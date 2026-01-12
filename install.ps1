# VelinScript Installation Script für Windows
# PowerShell Script

# Fehlerbehandlung: Script stoppt bei Fehlern
$ErrorActionPreference = "Stop"

Write-Host "🚀 VelinScript Installation" -ForegroundColor Green
Write-Host ""

# Prüfe ob Rust installiert ist
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust ist nicht installiert." -ForegroundColor Red
    Write-Host "Bitte installiere Rust zuerst:"
    Write-Host "  https://rustup.rs/"
    exit 1
}

Write-Host "✓ Rust gefunden" -ForegroundColor Green

# Repository klonen oder aktualisieren
if (Test-Path "velinscript") {
    Write-Host "📦 Repository aktualisieren..." -ForegroundColor Yellow
    Set-Location velinscript
    git pull
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Fehler beim Aktualisieren des Repositories" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "📦 Repository klonen..." -ForegroundColor Yellow
    git clone https://github.com/SkyliteDesign/velinscript.git
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Fehler beim Klonen des Repositories" -ForegroundColor Red
        exit 1
    }
    Set-Location velinscript
}

# Compiler bauen
Write-Host "🔨 Compiler bauen..." -ForegroundColor Yellow
Set-Location compiler
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Fehler beim Bauen des Compilers" -ForegroundColor Red
    exit 1
}

# Binary Pfad
$BINARY_PATH = Join-Path (Get-Location) "target\release\velin-compiler.exe"
$INSTALL_PATH = Join-Path $env:ProgramFiles "velin\velin.exe"

# Installations-Verzeichnis erstellen
$INSTALL_DIR = Split-Path $INSTALL_PATH
if (-not (Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
}

# Binary kopieren
Write-Host "📦 Binary installieren..." -ForegroundColor Yellow
if (-not (Test-Path $BINARY_PATH)) {
    Write-Host "❌ Binary nicht gefunden: $BINARY_PATH" -ForegroundColor Red
    exit 1
}
Copy-Item $BINARY_PATH $INSTALL_PATH -Force

# PATH aktualisieren (optional)
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$INSTALL_DIR*") {
    Write-Host "⚠️  Bitte füge $INSTALL_DIR zu deinem PATH hinzu" -ForegroundColor Yellow
    $newPath = "$currentPath;$INSTALL_DIR"
    Write-Host "  Oder verwende: [Environment]::SetEnvironmentVariable('Path', '$newPath', 'User')"
}

Write-Host ""
Write-Host "✅ VelinScript erfolgreich installiert!" -ForegroundColor Green
Write-Host ""
Write-Host "Verwendung:"
Write-Host "  velin compile -i main.velin"
Write-Host "  velin check -i main.velin"
Write-Host "  velin init my-project"
Write-Host ""
Write-Host "Dokumentation: https://github.com/SkyliteDesign/velinscript"
