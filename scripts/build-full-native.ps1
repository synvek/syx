# Build syx-cli with --features full-native.
# On Windows MSVC, inject +crt-static /INCLUDE:malloc only for this invocation
# (must not live in .cargo/config.toml — that contaminates syx_rt for native link).
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$target = if ($env:CARGO_BUILD_TARGET) { $env:CARGO_BUILD_TARGET } else {
    ((rustc -vV | Select-String '^host:').ToString() -split '\s+')[1]
}

$cargoArgs = @("build", "--release", "-p", "syx-cli", "--features", "full-native")
if ($env:CARGO_BUILD_TARGET) {
    $cargoArgs += @("--target", $env:CARGO_BUILD_TARGET)
}

if ($target -like "*-pc-windows-msvc") {
    # Isolated target-dir so these flags never contaminate the workspace
    # `target/` used when packaging `syx_rt.lib` for end-user native link.
    $cargoArgs += @("--target-dir", "target-full-native")
    $env:RUSTFLAGS = "-C target-feature=+crt-static -C link-arg=/INCLUDE:malloc"
}

if (-not $env:SYX_LLVM_PREFIX) {
    $env:SYX_LLVM_PREFIX = Join-Path (Split-Path $Root -Parent) "llvm"
}
$env:LLVM_SYS_211_PREFIX = $env:SYX_LLVM_PREFIX
$llvmBin = Join-Path $env:SYX_LLVM_PREFIX "bin"
if (Test-Path $llvmBin) {
    $env:Path = "$llvmBin;$env:Path"
}

& cargo @cargoArgs
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

if ($target -like "*-pc-windows-msvc") {
    $src = if ($env:CARGO_BUILD_TARGET) {
        Join-Path $Root "target-full-native\$env:CARGO_BUILD_TARGET\release\syx.exe"
    } else {
        Join-Path $Root "target-full-native\release\syx.exe"
    }
    $dstDir = Join-Path $Root "target\release"
    New-Item -ItemType Directory -Force -Path $dstDir | Out-Null
    Copy-Item -Force $src (Join-Path $dstDir "syx.exe")
    Write-Host "Copied $src -> target\release\syx.exe"
}
