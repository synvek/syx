# Collect MSVC / Windows SDK import libs into syx/lib/sysroot/<triple>/.
# NOT invoked by release.yml for now (Microsoft .lib redistrib concerns).
# Kept for experiments / a future self-built import-lib pipeline.
# Usage: powershell -File collect-sysroot.ps1 -Dest <dir> [-Triple x86_64-pc-windows-msvc]
param(
  [Parameter(Mandatory = $true)][string]$Dest,
  [string]$Triple = "x86_64-pc-windows-msvc"
)

$ErrorActionPreference = "Stop"
New-Item -ItemType Directory -Force -Path $Dest | Out-Null

$arch = "x64"
if ($Triple -match "aarch64|arm64") { $arch = "arm64" }
elseif ($Triple -match "i686|x86[^_]") { $arch = "x86" }

$names = @(
  "kernel32.lib", "user32.lib", "advapi32.lib", "shell32.lib", "ole32.lib",
  "oleaut32.lib", "ws2_32.lib", "bcrypt.lib", "ntdll.lib", "synchronization.lib",
  "ucrt.lib", "vcruntime.lib", "msvcrt.lib", "legacy_stdio_definitions.lib",
  "oldnames.lib", "uuid.lib", "gdi32.lib", "imm32.lib", "winmm.lib",
  "crypt32.lib", "secur32.lib", "ncrypt.lib", "iphlpapi.lib", "userenv.lib",
  "dbghelp.lib", "psapi.lib", "shlwapi.lib"
)

$searchRoots = New-Object System.Collections.Generic.List[string]

# Prefer LIB from the environment (Developer PowerShell / rustc-detected).
if ($env:LIB) {
  foreach ($p in ($env:LIB -split ';')) {
    if ($p -and (Test-Path -LiteralPath $p)) { [void]$searchRoots.Add($p) }
  }
}

# vswhere → latest VS with VC tools
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path -LiteralPath $vswhere) {
  $vsPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
  if ($vsPath) {
    $vcTools = Get-ChildItem -Path (Join-Path $vsPath "VC\Tools\MSVC") -Directory -ErrorAction SilentlyContinue |
      Sort-Object Name -Descending | Select-Object -First 1
    if ($vcTools) {
      [void]$searchRoots.Add((Join-Path $vcTools.FullName "lib\$arch"))
    }
  }
}

# Windows SDK um + ucrt
$sdkRoot = $env:WindowsSdkDir
if (-not $sdkRoot) {
  $sdkRoot = "C:\Program Files (x86)\Windows Kits\10\"
}
$sdkLib = Join-Path $sdkRoot "Lib"
if (Test-Path -LiteralPath $sdkLib) {
  $ver = Get-ChildItem -Path $sdkLib -Directory -ErrorAction SilentlyContinue |
    Sort-Object Name -Descending | Select-Object -First 1
  if ($ver) {
    [void]$searchRoots.Add((Join-Path $ver.FullName "um\$arch"))
    [void]$searchRoots.Add((Join-Path $ver.FullName "ucrt\$arch"))
  }
}

$copied = 0
foreach ($name in $names) {
  $found = $null
  foreach ($root in $searchRoots) {
    $cand = Join-Path $root $name
    if (Test-Path -LiteralPath $cand) {
      $found = $cand
      break
    }
  }
  if ($found) {
    Copy-Item -LiteralPath $found -Destination (Join-Path $Dest $name) -Force
    Write-Host "  + $name"
    $copied++
  } else {
    Write-Host "  - missing $name"
  }
}

@"
SYX link sysroot (Windows MSVC import libs)
triple=$Triple
arch=$arch
copied=$copied
Dynamic CRT / system DLL imports (ucrt, kernel32, ...). End users do not need VS installed.
"@ | Set-Content -Path (Join-Path $Dest "README.txt") -Encoding utf8

New-Item -ItemType File -Force -Path (Join-Path $Dest ".syx-sysroot") | Out-Null

if ($copied -lt 5) {
  Write-Error "Too few import libs copied ($copied). Is MSVC + Windows SDK installed?"
}

Write-Host "Windows sysroot: $copied libs → $Dest"
