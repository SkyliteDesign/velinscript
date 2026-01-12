#!/bin/bash

# VelinScript Installation Script
# Für Linux und macOS

set -e

echo "🚀 VelinScript Installation"
echo ""

# Prüfe ob Rust installiert ist
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust ist nicht installiert."
    echo "Bitte installiere Rust zuerst:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust gefunden"

# Repository klonen oder aktualisieren
if [ -d "velinscript" ]; then
    echo "📦 Repository aktualisieren..."
    cd velinscript
    git pull
else
    echo "📦 Repository klonen..."
    git clone https://github.com/SkyliteDesign/velinscript.git
    cd velinscript
fi

# Compiler bauen
echo "🔨 Compiler bauen..."
cd compiler
cargo build --release

# Binary Pfad
BINARY_PATH="$(pwd)/target/release/velin-compiler"
INSTALL_PATH="/usr/local/bin/velin"

# Binary installieren
echo "📦 Binary installieren..."
if [ -w "/usr/local/bin" ]; then
    sudo cp "$BINARY_PATH" "$INSTALL_PATH"
    sudo chmod +x "$INSTALL_PATH"
else
    echo "⚠️  Keine Schreibrechte für /usr/local/bin"
    echo "Bitte manuell installieren:"
    echo "  sudo cp $BINARY_PATH $INSTALL_PATH"
    echo "  sudo chmod +x $INSTALL_PATH"
    exit 1
fi

echo ""
echo "✅ VelinScript erfolgreich installiert!"
echo ""
echo "Verwendung:"
echo "  velin compile -i main.velin"
echo "  velin check -i main.velin"
echo "  velin init my-project"
echo ""
echo "Dokumentation: https://github.com/SkyliteDesign/velinscript"
