# Rust/Cargo Installation für VelinScript

## Windows Installation

### Option 1: rustup (Empfohlen)

1. Lade rustup-init.exe von https://rustup.rs/ herunter
2. Führe die Installationsdatei aus
3. Folge den Anweisungen im Installer
4. Starte PowerShell/CMD neu

### Option 2: Chocolatey

```powershell
choco install rust
```

### Option 3: Scoop

```powershell
scoop install rust
```

## Nach der Installation

Überprüfe die Installation:

```powershell
rustc --version
cargo --version
```

## Tests ausführen

Nach der Installation kannst du die Tests ausführen:

```powershell
cd compiler
cargo test
```

## Alternative: Online Compiler

Falls du Rust nicht installieren möchtest, kannst du den Code auch online testen:
- https://play.rust-lang.org/
- https://repl.it/languages/rust
