# VelinScript Debugger

Der VelinScript Debugger ermöglicht es, VelinScript-Programme zu debuggen.

## Installation

Der Debugger ist Teil des VelinScript Toolchains. Stelle sicher, dass `velin-debugger` installiert ist:

```bash
cd tools/debugger
cargo build --release
```

## Verwendung

### DAP Server starten

```bash
velin-debugger start --port 4711
```

Der DAP Server lauscht standardmäßig auf Port 4711.

### VS Code Integration

Der Debugger ist automatisch in der VS Code Extension integriert. Siehe [Debugger Tutorial](../guides/tutorial-debugger.md) für Details.

## Features

- **Breakpoints**: Setzen und Verwalten von Breakpoints
- **Step Over/Into/Out**: Schrittweises Debugging
- **Variable Inspection**: Variablen im aktuellen Scope anzeigen
- **Call Stack**: Aufrufkette anzeigen
- **Watch Expressions**: Ausdrücke während des Debuggings überwachen
- **Evaluate Expressions**: Ausdrücke in der Debug Console evaluieren

## DAP Protocol

Der Debugger implementiert das Debug Adapter Protocol (DAP), was bedeutet, dass er mit jedem DAP-kompatiblen Editor funktioniert:

- VS Code
- Visual Studio
- JetBrains IDEs (mit DAP Plugin)
- Andere DAP-kompatible Editoren

## Konfiguration

### Port

Standard-Port ist 4711. Kann mit `--port` geändert werden:

```bash
velin-debugger start --port 5000
```

### VS Code Settings

In VS Code können folgende Einstellungen konfiguriert werden:

```json
{
    "velin.debugger.path": "velin-debugger",
    "velin.debugger.port": 4711
}
```

## Troubleshooting

### Port bereits belegt

Wenn der Port bereits belegt ist, ändere den Port:

```bash
velin-debugger start --port 5000
```

### Debugger startet nicht

- Prüfe, ob `velin-debugger` im PATH ist
- Prüfe die Port-Konfiguration
- Prüfe Firewall-Einstellungen

### Breakpoints werden nicht getroffen

- Stelle sicher, dass Debug-Informationen generiert wurden
- Prüfe, ob der Code tatsächlich ausgeführt wird
- Prüfe, ob die Zeilennummern korrekt sind
