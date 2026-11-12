# Fail when the built syx does not embed a tag-matching prelude snapshot
# (cold run would full-compile stdlib and write prelude_*.bin under target/cache).
# Usage: build syx first, then: .\scripts\check_prelude_embed.ps1 [path\to\syx.exe]
#
# This script never writes the checked-in assets. If you see a 0-byte
# prelude_snapshot.bin, gen_prelude_snapshot.ps1 truncated it and then failed
# before copying a fresh cold-store blob — re-run gen (fixed to restore on failure).
$ErrorActionPreference = "Stop"

function Get-RepoRoot {
    $here = $PSScriptRoot
    if (-not $here) {
        $here = Split-Path -Parent $MyInvocation.MyCommand.Path
    }
    return (Resolve-Path (Join-Path $here "..")).Path
}

function Find-PreludeBins([string]$CacheDir) {
    if (-not (Test-Path -LiteralPath $CacheDir)) {
        return @()
    }
    # Do NOT use -Filter with -Recurse on Windows PowerShell 5.1 (misses matches).
    return @(Get-ChildItem -LiteralPath $CacheDir -Recurse -File -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like 'prelude_*.bin' })
}

$Root = Get-RepoRoot
Set-Location -LiteralPath $Root

$SyxBin = $null
if ($args.Count -ge 1 -and $args[0]) {
    $SyxBin = $args[0]
}
if (-not $SyxBin) {
    $release = Join-Path $Root "target\release\syx.exe"
    $debug = Join-Path $Root "target\debug\syx.exe"
    if (Test-Path -LiteralPath $release) {
        $SyxBin = $release
    } elseif (Test-Path -LiteralPath $debug) {
        $SyxBin = $debug
    } else {
        Write-Error "syx binary missing (cargo build -p syx-cli [--release]) under $Root\target"
        exit 1
    }
}
if (-not [System.IO.Path]::IsPathRooted($SyxBin)) {
    $SyxBin = Join-Path $Root $SyxBin
}
$SyxBin = (Resolve-Path -LiteralPath $SyxBin).Path

$Asset = Join-Path $Root "crates\syx-driver\assets\prelude_snapshot.bin"
if (-not (Test-Path -LiteralPath $Asset)) {
    Write-Error @"
missing asset file:
  $Asset
  (repo root resolved as: $Root)
  fix: .\scripts\gen_prelude_snapshot.ps1
"@
    exit 1
}
$AssetLen = (Get-Item -LiteralPath $Asset).Length
if ($AssetLen -eq 0) {
    Write-Error @"
asset exists but is EMPTY (0 bytes) — gen truncated it then failed before cold-store copy:
  $Asset
  fix: .\scripts\gen_prelude_snapshot.ps1
  (do not commit a 0-byte prelude_snapshot.bin)
"@
    exit 1
}
Write-Host "prelude_embed_asset_bytes=$AssetLen"
Write-Host "prelude_embed_asset_path=$Asset"

$RkyvAsset = Join-Path $Root "crates\syx-driver\assets\prelude_vm_image.rkyv"
if ((Test-Path -LiteralPath $RkyvAsset) -and ((Get-Item -LiteralPath $RkyvAsset).Length -gt 0)) {
    Write-Host "prelude_rkyv_asset_bytes=$((Get-Item -LiteralPath $RkyvAsset).Length)"
} else {
    Write-Warning "empty or missing $RkyvAsset (rkyv fastpath disabled; bincode embed still required)"
}

$HelloSrc = Join-Path $Root "examples\hello.sy"
if (-not (Test-Path -LiteralPath $HelloSrc)) {
    Write-Error "missing $HelloSrc"
    exit 1
}

$Work = Join-Path ([System.IO.Path]::GetTempPath()) ("syx-check-prelude." + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $Work | Out-Null
try {
    Copy-Item -LiteralPath $HelloSrc -Destination (Join-Path $Work "hello.sy")
    $Hello = Join-Path $Work "hello.sy"

    $ErrorActionPreference = "Continue"
    & $SyxBin run $Hello | Out-Null
    $runExit = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($runExit -ne 0) {
        Write-Error "syx run failed (exit $runExit) bin=$SyxBin hello=$Hello"
        exit $runExit
    }
    $CacheDir = Join-Path $Work "target\cache"
    if (Test-Path -LiteralPath $CacheDir) {
        Write-Error "default syx run created target/cache (incremental_cache should be off)"
        Get-ChildItem -LiteralPath $CacheDir -Recurse -File | ForEach-Object { Write-Host $_.FullName }
        exit 1
    }

    $ErrorActionPreference = "Continue"
    & $SyxBin run --with-cache $Hello | Out-Null
    $runExit = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($runExit -ne 0) {
        Write-Error "syx run --with-cache failed (exit $runExit) bin=$SyxBin hello=$Hello"
        exit $runExit
    }

    $PreludeHits = Find-PreludeBins $CacheDir
    if ($PreludeHits.Count -gt 0) {
        $p = $PreludeHits[0]
        Write-Error @"
prelude snapshot STALE or unloadable (embed miss / tag mismatch)
  wrote: $($p.FullName) ($($p.Length) bytes)
  on-disk asset still present ($AssetLen bytes) but this syx binary did not load it
  fix: .\scripts\gen_prelude_snapshot.ps1
  then: cargo build -p syx-cli --release
  and re-run this check against the NEW syx.exe
"@
        exit 1
    }

    Write-Host "prelude_embed_ok=1"
    Write-Host "note: default run left no target/cache; --with-cache entry cache allowed"
    exit 0
}
finally {
    if (Test-Path -LiteralPath $Work) {
        Remove-Item -LiteralPath $Work -Recurse -Force -ErrorAction SilentlyContinue
    }
}
