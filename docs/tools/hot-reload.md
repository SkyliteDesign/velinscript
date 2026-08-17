# Hot Reload & Development Server

Die Entwicklererfahrung (DX) steht bei VelinScript im Fokus. Das `velin-hot-reload` Tool ermöglicht extrem schnelle Feedback-Zyklen, indem es Code-Änderungen sofort kompiliert und die laufende Anwendung aktualisiert.

## Wofür ist Hot Reload ideal?

Hot Reload ist ideal für:
- ✅ **Schnelle Entwicklung** - Sofortiges Feedback bei Code-Änderungen
- ✅ **Development Server** - Kontinuierliche Entwicklung mit Live-Updates
- ✅ **API-Entwicklung** - Testen Sie API-Endpunkte sofort nach Änderungen
- ✅ **Inkrementelle Kompilierung** - Kompiliert nur geänderte Module
- ✅ **State Preservation** - Versucht, Anwendungs-State zu erhalten
- ✅ **VS Code Integration** - Nahtlose Integration in den Development-Workflow

## Wofür ist Hot Reload NICHT gedacht?

Hot Reload ist NICHT gedacht für:
- ❌ **Production-Builds** - Nur für Development, nicht für Production
- ❌ **Einmalige Kompilierung** - Für normale Builds nutzen Sie `velin compile`
- ❌ **Performance-Tests** - Für Performance-Messung nutzen Sie den Benchmark Runner
- ❌ **Strukturierte Tests** - Für Tests nutzen Sie den Test Runner
- ❌ **Security-Checks** - Für Security nutzen Sie den Security Scanner

---

## Inhaltsverzeichnis

1.  [Funktionsweise](#1-funktionsweise)
2.  [Nutzung](#2-nutzung)
    *   [Watch Mode](#watch-mode)
    *   [Dev Server Mode](#dev-server-mode)
3.  [Konfiguration](#3-konfiguration)
4.  [Integration mit VS Code](#4-integration-mit-vs-code)

---

## 1. Funktionsweise

Im Gegensatz zu einfachen Datei-Watchern (wie `nodemon`), die den Prozess bei jeder Änderung hart neustarten, versucht VelinScript intelligent zu sein:

1.  **File Watching:** Überwacht das Dateisystem auf Änderungen (`.velin` Dateien).
2.  **Inkrementelle Kompilierung:** Kompiliert nur die geänderten Module neu, was bei großen Projekten Millisekunden statt Sekunden dauert.
3.  **State Preservation (Experimentell):** Versucht, den Speicherzustand (z.B. Datenbankverbindungen) zu erhalten, während die Logik ausgetauscht wird.
4.  **Auto-Restart:** Wenn Hot-Swapping nicht möglich ist (z.B. bei Änderungen an Struct-Layouts), wird der Prozess automatisch und schnell neu gestartet.

---

## 2. Nutzung

### Watch Mode

Der Watch-Mode kompiliert Dateien im Hintergrund, führt sie aber nicht aus. Ideal, wenn Sie Fehler direkt im Editor sehen wollen.

```bash
velin-hot-reload --watch
```

**Optionen:**
*   `--directory <dir>`: Überwachtes Verzeichnis (Default: aktuelles).
*   `--compile-command <cmd>`: Befehl zum Kompilieren (Default: `velin compile`).

### Dev Server Mode

Startet Ihre Anwendung und startet sie bei Änderungen neu. Dies ist der Standard für die API-Entwicklung.

```bash
velin-hot-reload --server --run-command "velin run main.velin"
```

Wenn Sie ein Web-Projekt haben, wird oft auch ein Proxy für Frontend-Assets gestartet.

**Optionen:**
*   `--port <port>`: Port für den Dev-Server (Default: 3000).
*   `--delay <ms>`: Verzögerung vor Neustart (um "Flackern" bei "Save All" zu vermeiden).

---

## 3. Konfiguration

Sie können das Verhalten in der `velin.toml` unter `[dev]` anpassen.

```toml
[dev]
# Dateien, die ignoriert werden sollen
ignore = [
    "**/*.test.velin",
    "temp/**",
    ".git"
]

# Befehle
compile_cmd = "velin compile --debug"
run_cmd = "./build/app"

# Umgebungsvariablen für Dev
env = { "ENV": "development", "DEBUG": "true" }
```

---

## 4. Integration mit VS Code

Die VelinScript VS Code Extension nutzt `velin-hot-reload` im Hintergrund.

*   **Status Bar:** Zeigt "Watching..." an.
*   **On Save:** Bei jedem Speichern (`Ctrl+S`) wird eine schnelle Prüfung ausgelöst. Fehler werden direkt im "Problems"-Tab angezeigt.
*   **Debugging:** Wenn Sie den Debugger starten (F5), wird automatisch der Hot-Reload-Modus aktiviert, sodass Sie Breakpoints auch nach Code-Änderungen weiter nutzen können.

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Hot Reload                                 │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-hot-reload --server                            │
│                                                         │
│  🔥 Hot Reload aktiviert                                │
│  📁 Überwache: ./src                                     │
│                                                         │
│  [Watching for changes...]                              │
│                                                         │
│  ✓ main.velin geändert                                  │
│  ⚡ Inkrementelle Kompilierung...                       │
│  ✓ Kompilierung erfolgreich (45ms)                     │
│  🔄 Server neu gestartet                                │
│                                                         │
│  [Server läuft auf http://localhost:3000]               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```
