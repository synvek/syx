# Fail when the built syx does not embed a tag-matching prelude snapshot
# (cold run would full-compile stdlib and write prelude_*.bin under target/cache).
# Usage: build syx first, then: .\scripts\check_prelude_embed.ps1 [path\to\syx.exe]
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$SyxBin = $args[0]
if (-not $SyxBin) {
    $release = Join-Path $Root "target\release\syx.exe"
    $debug = Join-Path $Root "target\debug\syx.exe"
    if (Test-Path $release) {
        $SyxBin = $release
    } elseif (Test-Path $debug) {
        $SyxBin = $debug
    } else {
        Write-Error "syx binary missing (cargo build -p syx-cli [--release])"
        exit 1
    }
}

$Asset = Join-Path $Root "crates\syx-driver\assets\prelude_snapshot.bin"
if (-not (Test-Path $Asset) -or (Get-Item $Asset).Length -eq 0) {
    Write-Error @"
missing or empty $Asset
  fix: .\scripts\gen_prelude_snapshot.ps1
  (or: bash scripts/gen_prelude_snapshot.sh)
"@
    exit 1
}
Write-Host "prelude_embed_asset_bytes=$((Get-Item $Asset).Length)"
$RkyvAsset = Join-Path $Root "crates\syx-driver\assets\prelude_vm_image.rkyv"
if ((Test-Path $RkyvAsset) -and (Get-Item $RkyvAsset).Length -gt 0) {
    Write-Host "prelude_rkyv_asset_bytes=$((Get-Item $RkyvAsset).Length)"
} else {
    Write-Warning "empty or missing $RkyvAsset (rkyv fastpath disabled; bincode embed still required)"
}

$HelloSrc = Join-Path $Root "examples\hello.sy"
if (-not (Test-Path $HelloSrc)) {
    Write-Error "missing $HelloSrc"
    exit 1
}

$Work = Join-Path ([System.IO.Path]::GetTempPath()) ("syx-check-prelude." + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $Work | Out-Null
try {
    Copy-Item $HelloSrc (Join-Path $Work "hello.sy")
    $Hello = Join-Path $Work "hello.sy"

    # Default run must leave no target/cache (incremental_cache off).
    $ErrorActionPreference = "Continue"
    & $SyxBin run $Hello | Out-Null
    $runExit = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($runExit -ne 0) {
        Write-Error "syx run failed (exit $runExit)"
        exit $runExit
    }
    $CacheDir = Join-Path $Work "target\cache"
    if (Test-Path $CacheDir) {
        Write-Error "default syx run created target/cache (incremental_cache should be off)"
        Get-ChildItem -Recurse -File $CacheDir | ForEach-Object { Write-Host $_.FullName }
        exit 1
    }

    # With --with-cache: embed HIT must not materialize prelude_*.bin (entry .bin is OK).
    $ErrorActionPreference = "Continue"
    & $SyxBin run --with-cache $Hello | Out-Null
    $runExit = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($runExit -ne 0) {
        Write-Error "syx run --with-cache failed (exit $runExit)"
        exit $runExit
    }

    $Prelude = $null
    if (Test-Path $CacheDir) {
        $Prelude = Get-ChildItem -Recurse -File $CacheDir -Filter "prelude_*.bin" |
            Select-Object -First 1
    }
    if ($Prelude) {
        Write-Error @"
prelude snapshot STALE or unloadable (embed miss)
  wrote: $($Prelude.FullName) ($($Prelude.Length) bytes)
  fix: .\scripts\gen_prelude_snapshot.ps1 && cargo build -p syx-cli --release
  then commit crates/syx-driver/assets/prelude_snapshot.bin
"@
        exit 1
    }

    Write-Host "prelude_embed_ok=1"
    Write-Host "note: default run left no target/cache; --with-cache entry cache allowed"
}
finally {
    if (Test-Path $Work) {
        Remove-Item -Recurse -Force $Work -ErrorAction SilentlyContinue
    }
}
