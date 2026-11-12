# S3-I54: generate crates/syx-driver/assets/prelude_snapshot.bin (+ prelude_vm_image.rkyv)
# from a release syx run. Does not compile .sy inside build.rs — run after (or this
# script will trigger) `cargo build -p syx-cli --release`.
#
# Chicken-egg: a tag-matching embed short-circuits ensure_prelude and writes no
# prelude_*.bin. This script zeros the assets and rebuilds tip syx first so cold
# store always runs, then rebuilds again so include_bytes picks up the new blobs.
#
# Windows counterpart of scripts/gen_prelude_snapshot.sh
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$HelloSrc = Join-Path $Root "examples\hello.sy"
if (-not (Test-Path $HelloSrc)) {
    Write-Error "missing $HelloSrc"
    exit 1
}

$OutDir = Join-Path $Root "crates\syx-driver\assets"
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$OutBin = Join-Path $OutDir "prelude_snapshot.bin"
$OutRkyv = Join-Path $OutDir "prelude_vm_image.rkyv"

Write-Host "invalidating embed assets so cold store can run…"
# Truncate (keep path) so include_bytes still resolves during the empty-embed rebuild.
[System.IO.File]::WriteAllBytes($OutBin, [byte[]]@())
[System.IO.File]::WriteAllBytes($OutRkyv, [byte[]]@())

Write-Host "building tip syx (empty embed)…"
# cargo may write warnings to stderr; judge by $LASTEXITCODE (see build-full-native.ps1).
$ErrorActionPreference = "Continue"
& cargo build -p syx-cli --release
$cargoExit = $LASTEXITCODE
$ErrorActionPreference = "Stop"
if ($cargoExit -ne 0) { exit $cargoExit }

$SyxBin = Join-Path $Root "target\release\syx.exe"
if (-not (Test-Path $SyxBin)) {
    Write-Error "missing $SyxBin after build"
    exit 1
}

$Work = Join-Path ([System.IO.Path]::GetTempPath()) ("syx-gen-prelude." + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $Work | Out-Null
try {
    Copy-Item $HelloSrc (Join-Path $Work "hello.sy")
    $Hello = Join-Path $Work "hello.sy"
    Write-Host "cold-compiling prelude via $SyxBin run --with-cache…"
    $ErrorActionPreference = "Continue"
    & $SyxBin run --with-cache $Hello | Out-Null
    $runExit = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($runExit -ne 0) {
        Write-Error "syx run --with-cache failed (exit $runExit)"
        exit $runExit
    }

    $CacheDir = Join-Path $Work "target\cache"
    $Prelude = $null
    if (Test-Path $CacheDir) {
        $Prelude = Get-ChildItem -Recurse -File $CacheDir -Filter "prelude_*.bin" |
            Select-Object -First 1
    }
    if (-not $Prelude -or $Prelude.Length -eq 0) {
        Write-Error "no prelude_*.bin produced under $CacheDir"
        if (Test-Path $Work) {
            Get-ChildItem -Recurse -File $Work | Select-Object -First 50 |
                ForEach-Object { Write-Host $_.FullName }
        }
        exit 1
    }

    Copy-Item -Force $Prelude.FullName $OutBin
    Write-Host "wrote $OutBin ($($Prelude.Length) bytes) from $($Prelude.FullName)"

    $Rkyv = Get-ChildItem -Recurse -File $CacheDir -Filter "prelude_*.rkyv" -ErrorAction SilentlyContinue |
        Select-Object -First 1
    if ($Rkyv -and $Rkyv.Length -gt 0) {
        Copy-Item -Force $Rkyv.FullName $OutRkyv
        Write-Host "wrote $OutRkyv ($($Rkyv.Length) bytes) from $($Rkyv.FullName)"
    } else {
        Write-Warning "no prelude_*.rkyv produced (rkyv embed stays empty until next cold store)"
    }

    Write-Host "rebuilding syx so include_bytes picks up the assets…"
    $ErrorActionPreference = "Continue"
    & cargo build -p syx-cli --release
    $cargoExit = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($cargoExit -ne 0) { exit $cargoExit }

    Write-Host "verifying embed hit…"
    & (Join-Path $PSScriptRoot "check_prelude_embed.ps1") $SyxBin
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host "done — commit $OutBin / $OutRkyv if this is a source landing"
}
finally {
    if (Test-Path $Work) {
        Remove-Item -Recurse -Force $Work -ErrorAction SilentlyContinue
    }
}
