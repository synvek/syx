# S3-I54: generate crates/syx-driver/assets/prelude_snapshot.bin (+ prelude_vm_image.rkyv)
# from a release syx run. Windows counterpart of scripts/gen_prelude_snapshot.sh
#
# Chicken-egg: a tag-matching embed short-circuits ensure_prelude and writes no
# prelude_*.bin. This script zeros the assets and rebuilds tip syx first so cold
# store always runs, then rebuilds again so include_bytes picks up the new blobs.
#
# Safety: previous assets are backed up before truncate; any failure restores them
# so a failed gen cannot leave a 0-byte committed blob for check_prelude_embed.
$ErrorActionPreference = "Stop"

function Get-RepoRoot {
    $here = $PSScriptRoot
    if (-not $here) {
        $here = Split-Path -Parent $MyInvocation.MyCommand.Path
    }
    return (Resolve-Path (Join-Path $here "..")).Path
}

function Find-PreludeFiles([string]$CacheDir, [string]$Pattern) {
    if (-not (Test-Path -LiteralPath $CacheDir)) {
        return @()
    }
    # Do NOT use -Filter with -Recurse on Windows PowerShell 5.1 (misses matches).
    return @(Get-ChildItem -LiteralPath $CacheDir -Recurse -File -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like $Pattern })
}

function Invoke-CargoRelease {
    $ErrorActionPreference = "Continue"
    & cargo build -p syx-cli --release
    $code = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($code -ne 0) { exit $code }
}

$Root = Get-RepoRoot
Set-Location -LiteralPath $Root

$HelloSrc = Join-Path $Root "examples\hello.sy"
if (-not (Test-Path -LiteralPath $HelloSrc)) {
    Write-Error "missing $HelloSrc"
    exit 1
}

$OutDir = Join-Path $Root "crates\syx-driver\assets"
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$OutBin = Join-Path $OutDir "prelude_snapshot.bin"
$OutRkyv = Join-Path $OutDir "prelude_vm_image.rkyv"

# Backup before truncate so a failed cold-store cannot leave 0-byte assets.
$BackupDir = Join-Path ([System.IO.Path]::GetTempPath()) ("syx-prelude-bak." + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $BackupDir | Out-Null
$BakBin = Join-Path $BackupDir "prelude_snapshot.bin"
$BakRkyv = Join-Path $BackupDir "prelude_vm_image.rkyv"
$HadBin = $false
$HadRkyv = $false
if ((Test-Path -LiteralPath $OutBin) -and ((Get-Item -LiteralPath $OutBin).Length -gt 0)) {
    Copy-Item -LiteralPath $OutBin -Destination $BakBin -Force
    $HadBin = $true
}
if ((Test-Path -LiteralPath $OutRkyv) -and ((Get-Item -LiteralPath $OutRkyv).Length -gt 0)) {
    Copy-Item -LiteralPath $OutRkyv -Destination $BakRkyv -Force
    $HadRkyv = $true
}

function Restore-PreludeAssets {
    if ($HadBin -and (Test-Path -LiteralPath $BakBin)) {
        Copy-Item -LiteralPath $BakBin -Destination $OutBin -Force
        Write-Warning "restored $OutBin from pre-gen backup"
    }
    if ($HadRkyv -and (Test-Path -LiteralPath $BakRkyv)) {
        Copy-Item -LiteralPath $BakRkyv -Destination $OutRkyv -Force
        Write-Warning "restored $OutRkyv from pre-gen backup"
    }
}

$Work = $null
try {
    Write-Host "invalidating embed assets so cold store can run…"
    [System.IO.File]::WriteAllBytes($OutBin, [byte[]]@())
    [System.IO.File]::WriteAllBytes($OutRkyv, [byte[]]@())

    Write-Host "building tip syx (empty embed)…"
    Invoke-CargoRelease

    $SyxBin = Join-Path $Root "target\release\syx.exe"
    if (-not (Test-Path -LiteralPath $SyxBin)) {
        throw "missing $SyxBin after build"
    }

    $Work = Join-Path ([System.IO.Path]::GetTempPath()) ("syx-gen-prelude." + [guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Force -Path $Work | Out-Null
    Copy-Item -LiteralPath $HelloSrc -Destination (Join-Path $Work "hello.sy")
    $Hello = Join-Path $Work "hello.sy"

    Write-Host "cold-compiling prelude via $SyxBin run --with-cache…"
    $ErrorActionPreference = "Continue"
    & $SyxBin run --with-cache $Hello | Out-Null
    $runExit = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($runExit -ne 0) {
        throw "syx run --with-cache failed (exit $runExit)"
    }

    $CacheDir = Join-Path $Work "target\cache"
    $PreludeHits = Find-PreludeFiles $CacheDir 'prelude_*.bin'
    if ($PreludeHits.Count -eq 0 -or $PreludeHits[0].Length -eq 0) {
        Write-Host "cache tree under $CacheDir :"
        if (Test-Path -LiteralPath $Work) {
            Get-ChildItem -LiteralPath $Work -Recurse -File -ErrorAction SilentlyContinue |
                Select-Object -First 50 |
                ForEach-Object { Write-Host $_.FullName }
        }
        throw "no prelude_*.bin produced under $CacheDir"
    }
    $Prelude = $PreludeHits[0]
    Copy-Item -LiteralPath $Prelude.FullName -Destination $OutBin -Force
    Write-Host "wrote $OutBin ($($Prelude.Length) bytes) from $($Prelude.FullName)"

    $RkyvHits = Find-PreludeFiles $CacheDir 'prelude_*.rkyv'
    if ($RkyvHits.Count -gt 0 -and $RkyvHits[0].Length -gt 0) {
        Copy-Item -LiteralPath $RkyvHits[0].FullName -Destination $OutRkyv -Force
        Write-Host "wrote $OutRkyv ($($RkyvHits[0].Length) bytes) from $($RkyvHits[0].FullName)"
    } else {
        Write-Warning "no prelude_*.rkyv produced (rkyv embed stays empty until next cold store)"
    }

    Write-Host "rebuilding syx so include_bytes picks up the assets…"
    Invoke-CargoRelease

    Write-Host "verifying embed hit…"
    $check = Join-Path $PSScriptRoot "check_prelude_embed.ps1"
    $ErrorActionPreference = "Continue"
    & $check $SyxBin
    $checkExit = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($checkExit -ne 0) {
        throw "check_prelude_embed.ps1 failed (exit $checkExit)"
    }

    Write-Host "done — commit $OutBin / $OutRkyv if this is a source landing"
}
catch {
    # Restore before Write-Error: Stop preference would otherwise skip restore.
    Restore-PreludeAssets
    Write-Host "ERROR: $_" -ForegroundColor Red
    exit 1
}
finally {
    if ($Work -and (Test-Path -LiteralPath $Work)) {
        Remove-Item -LiteralPath $Work -Recurse -Force -ErrorAction SilentlyContinue
    }
    if (Test-Path -LiteralPath $BackupDir) {
        Remove-Item -LiteralPath $BackupDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}
