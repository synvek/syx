# Prebuild slim syx_rt.lib + syx-host-{sql,http,tls} for `syx build --native`.
# Windows counterpart of scripts/build_native_shake_libs.sh.
# Link path never runs cargo — this script (or the release packager) must run first.
#
# Usage:
#   powershell -File scripts/build_native_shake_libs.ps1
#   powershell -File scripts/build_native_shake_libs.ps1 release
#   powershell -File scripts/build_native_shake_libs.ps1 debug
#
# Env:
#   SYX_NATIVE_SHAKE_TARGET_DIR  — cargo target dir (default: <repo>/target/native-host-shake)
#   SYX_NATIVE_ASYNC=1|true      — include async-runtime in slim rt features
$ErrorActionPreference = "Stop"

function Get-RepoRoot {
    $here = $PSScriptRoot
    if (-not $here) {
        $here = Split-Path -Parent $MyInvocation.MyCommand.Path
    }
    return (Resolve-Path (Join-Path $here "..")).Path
}

$Root = Get-RepoRoot
Set-Location -LiteralPath $Root

$ProfileName = "release"
if ($args.Count -ge 1 -and -not [string]::IsNullOrWhiteSpace($args[0])) {
    $ProfileName = $args[0].ToString().Trim().ToLowerInvariant()
}
if ($ProfileName -ne "release" -and $ProfileName -ne "debug") {
    Write-Error "usage: build_native_shake_libs.ps1 [release|debug]"
    exit 2
}

$TargetDir = $env:SYX_NATIVE_SHAKE_TARGET_DIR
if ([string]::IsNullOrWhiteSpace($TargetDir)) {
    $TargetDir = Join-Path $Root "target\native-host-shake"
}

$Features = "file"
$async = $env:SYX_NATIVE_ASYNC
if ($async -eq "1" -or ($async -and $async.Equals("true", [System.StringComparison]::OrdinalIgnoreCase))) {
    $Features = "async-runtime,file"
}

$cargoArgs = @(
    "build",
    "-p", "syx-rt",
    "-p", "syx-host-sql",
    "-p", "syx-host-http",
    "-p", "syx-host-tls",
    "--no-default-features",
    "--features", $Features
)
if ($ProfileName -eq "release") {
    $cargoArgs += "--release"
}

Write-Host "building native shake libs → $TargetDir\$ProfileName (features=$Features)"
$prevTarget = $env:CARGO_TARGET_DIR
$env:CARGO_TARGET_DIR = $TargetDir
try {
    $ErrorActionPreference = "Continue"
    & cargo @cargoArgs
    $code = $LASTEXITCODE
    $ErrorActionPreference = "Stop"
    if ($code -ne 0) {
        exit $code
    }
}
finally {
    if ($null -eq $prevTarget) {
        Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
    } else {
        $env:CARGO_TARGET_DIR = $prevTarget
    }
}

$Dir = Join-Path $TargetDir $ProfileName
$names = @(
    "libsyx_rt.a", "syx_rt.lib",
    "libsyx_host_sql.a", "syx_host_sql.lib",
    "libsyx_host_http.a", "syx_host_http.lib",
    "libsyx_host_tls.a", "syx_host_tls.lib"
)
foreach ($name in $names) {
    $path = Join-Path $Dir $name
    if (Test-Path -LiteralPath $path) {
        $bytes = (Get-Item -LiteralPath $path).Length
        Write-Host "  ok $path ($bytes bytes)"
    }
}

$hasRt = (Test-Path -LiteralPath (Join-Path $Dir "libsyx_rt.a")) -or
    (Test-Path -LiteralPath (Join-Path $Dir "syx_rt.lib"))
if (-not $hasRt) {
    Write-Error "slim syx-rt archive missing under $Dir"
    exit 1
}

Write-Host "done — syx build --native will resolve these without spawning cargo"
Write-Host "note: Unix — use bash scripts/build_native_shake_libs.sh"
