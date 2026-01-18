# Debugging in VelinScript

Der VelinScript Debugger (`velin-debugger`) ist ein vollwertiger DAP (Debug Adapter Protocol) Server. Das bedeutet, er integriert sich nahtlos in moderne IDEs wie VS Code, NeoVim oder IntelliJ, ohne dass spezielle Plugins nötig sind (abgesehen von der Basis-Integration).

---

## Inhaltsverzeichnis

1.  [Features](#1-features)
2.  [Nutzung in VS Code](#2-nutzung-in-vs-code)
3.  [CLI-Nutzung](#3-cli-nutzung)
4.  [Erweiterte Debugging-Techniken](#4-erweiterte-debugging-techniken)
    *   [Conditional Breakpoints](#conditional-breakpoints)
    *   [Logpoints](#logpoints)
    *   [Exception Breakpoints](#exception-breakpoints)
5.  [Remote Debugging](#5-remote-debugging)

---

## 1. Features

*   **Breakpoints:** Halten Sie die Ausführung an jeder Zeile an.
*   **Stepping:** Step Over, Step Into, Step Out.
*   **Variable Inspection:** Untersuchen Sie lokale Variablen, Globals und Closures.
*   **Expression Evaluation:** Führen Sie VelinScript-Code im Kontext des gestoppten Programms aus.
*   **Call Stack:** Navigieren Sie durch die Aufrufhierarchie.
*   **Threads:** Debuggen Sie asynchrone Tasks und Worker.

---

## 2. Nutzung in VS Code

Die offizielle Extension bringt bereits eine Konfiguration mit. Erstellen Sie eine `launch.json` im Ordner `.vscode`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "velin",
            "request": "launch",
            "name": "Debug Main",
            "program": "${workspaceFolder}/src/main.velin",
            "args": [],
            "env": { "DEBUG": "true" }
        }
    ]
}
```

Drücken Sie dann **F5**, um das Debugging zu starten.

---

## 3. CLI-Nutzung

Sie können den Debug-Server auch manuell starten, z.B. für die Integration in andere Editoren.

```bash
velin-debugger start --port 4711
```

Der Server lauscht nun auf DAP-Nachrichten.

Um ein Programm im Debug-Modus zu starten, ohne eine IDE zu verbinden (nur Wait-For-Attach):

```bash
velin run main.velin --debug --wait-for-debugger
```

---

## 4. Erweiterte Debugging-Techniken

### Conditional Breakpoints

Manchmal wollen Sie nur anhalten, wenn eine bestimmte Bedingung wahr ist (z.B. in einer Schleife).

*   Rechtsklick auf den Breakpoint in VS Code.
*   "Edit Breakpoint..." wählen.
*   Bedingung eingeben: `i == 100` oder `user.name == "Alice"`.

### Logpoints

Logpoints geben Nachrichten in die Konsole aus, ohne die Ausführung zu stoppen. Ideal für "Print-Debugging" ohne den Code zu verunreinigen.

*   Rechtsklick -> "Add Logpoint..."
*   Nachricht: `Schleifenindex ist: {i}`

### Exception Breakpoints

VelinScript kann automatisch anhalten, wenn eine Exception geworfen wird ("Uncaught Exceptions") oder sogar bei jeder Exception ("All Exceptions"). Dies aktivieren Sie im "Breakpoints"-Fenster Ihrer IDE.

---

## 5. Remote Debugging

Sie können Anwendungen debuggen, die auf einem anderen Server oder in einem Docker-Container laufen.

**Auf dem Server:**
```bash
velin run main.velin --debug --debug-port 4711 --debug-host 0.0.0.0
```

**Lokal (VS Code `launch.json`):**
```json
{
    "type": "velin",
    "request": "attach",
    "name": "Attach to Remote",
    "host": "192.168.1.50",
    "port": 4711,
    "remoteRoot": "/app/src",
    "localRoot": "${workspaceFolder}/src"
}
```

VelinScript mappt die Dateipfade automatisch, sodass Sie lokal Breakpoints setzen können, die remote greifen.
