# VelinScript Installation Script für Windows
# Usage:
#   .\install.ps1 -Prefix "I:\Projekte\VelinScript\test_sandbox\bin" -SourceRepo "I:\Projekte\VelinScript\velinscript"
#   .\install.ps1   # default: %ProgramFiles%\velin (legacy)

param(
    [string]$Prefix = "",
    [string]$SourceRepo = "",
    [switch]$SkipClone
)

$ErrorActionPreference = "Stop"

Write-Host "VelinScript Installation (Windows)" -ForegroundColor Green
Write-Host ""

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust/cargo nicht gefunden. Installiere zuerst: https://rustup.rs/" -ForegroundColor Red
    exit 1
}
Write-Host "Rust gefunden" -ForegroundColor Green

$startDir = Get-Location

if ($SourceRepo -ne "" -and (Test-Path $SourceRepo)) {
    $repoRoot = (Resolve-Path $SourceRepo).Path
    Write-Host "Verwende lokales Repo: $repoRoot" -ForegroundColor Yellow
    Set-Location (Join-Path $repoRoot "compiler")
} elseif ($SkipClone -and (Test-Path "compiler")) {
    Set-Location compiler
} elseif (Test-Path "velinscript\compiler") {
    Set-Location velinscript\compiler
} elseif (Test-Path "compiler\Cargo.toml") {
    Set-Location compiler
} else {
    Write-Host "Repository klonen..." -ForegroundColor Yellow
    git clone https://github.com/SkyliteDesign/velinscript.git
    if ($LASTEXITCODE -ne 0) { exit 1 }
    Set-Location velinscript\compiler
}

Write-Host "Compiler bauen (release)..." -ForegroundColor Yellow
cargo build --release --bin velin --bin velin-compiler
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build fehlgeschlagen" -ForegroundColor Red
    exit 1
}

$releaseDir = Join-Path (Get-Location) "target\release"
if ($env:CARGO_TARGET_DIR -and (Test-Path $env:CARGO_TARGET_DIR)) {
    $alt = Join-Path $env:CARGO_TARGET_DIR "release"
    if (Test-Path (Join-Path $alt "velin.exe")) { $releaseDir = $alt }
    elseif (Test-Path (Join-Path $alt "velin-compiler.exe")) { $releaseDir = $alt }
}
$velinSrc = Join-Path $releaseDir "velin.exe"
$compilerSrc = Join-Path $releaseDir "velin-compiler.exe"
if (-not (Test-Path $velinSrc)) {
    if (Test-Path $compilerSrc) {
        $velinSrc = $compilerSrc
    } else {
        Write-Host "Binary nicht gefunden unter $releaseDir" -ForegroundColor Red
        Get-ChildItem $releaseDir -ErrorAction SilentlyContinue | Select-Object -First 30 Name
        exit 1
    }
}

if ($Prefix -eq "") {
    $INSTALL_DIR = Join-Path $env:ProgramFiles "velin"
} else {
    $INSTALL_DIR = $Prefix
    if (-not [System.IO.Path]::IsPathRooted($INSTALL_DIR)) {
        $INSTALL_DIR = Join-Path $startDir $INSTALL_DIR
    }
}

New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
Copy-Item $velinSrc (Join-Path $INSTALL_DIR "velin.exe") -Force
if (Test-Path $compilerSrc) {
    Copy-Item $compilerSrc (Join-Path $INSTALL_DIR "velin-compiler.exe") -Force
}

Write-Host ""
Write-Host "Installiert nach: $INSTALL_DIR" -ForegroundColor Green
Write-Host "Session-PATH (Beispiel):"
Write-Host "  `$env:PATH = `"$INSTALL_DIR;`$env:PATH`""
Write-Host "  velin --version"
Write-Host ""
Set-Location $startDir
