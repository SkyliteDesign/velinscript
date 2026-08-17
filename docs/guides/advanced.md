# Erweiterte Themen in VelinScript

VelinScript ist mehr als nur eine Sprache für Web-APIs. Sie bietet mächtige Werkzeuge für Systemprogrammierung, Modularisierung und Erweiterbarkeit. Dieses Handbuch richtet sich an fortgeschrittene Entwickler, die das volle Potenzial der Plattform ausschöpfen möchten.

---

## Inhaltsverzeichnis

1.  [Das Modulsystem](#1-das-modulsystem)
    *   [Dateien als Module](#dateien-als-module)
    *   [Explizite Sub-Module](#explizite-sub-module)
    *   [Sichtbarkeit (`pub`)](#sichtbarkeit-pub)
    *   [Re-Exporting](#re-exporting)
2.  [CLI & Systemprozesse (`process` Modul)](#2-cli--systemprozesse-process-modul)
    *   [Externe Befehle ausführen](#externe-befehle-ausführen)
    *   [Prozessmanagement und Überwachung](#prozessmanagement-und-überwachung)
    *   [Interaktive CLIs bauen](#interaktive-clis-bauen)
3.  [Plugin-Entwicklung](#3-plugin-entwicklung)
    *   [Architektur eines Plugins](#architektur-eines-plugins)
    *   [Lifecycle Hooks](#lifecycle-hooks)
    *   [Beispiel: Ein Metrik-Plugin](#beispiel-ein-metrik-plugin)
4.  [Paketmanagement mit `velin.toml`](#4-paketmanagement-mit-velintoml)
    *   [Abhängigkeiten verwalten](#abhängigkeiten-verwalten)
    *   [Workspaces](#workspaces)
    *   [Eigene Pakete veröffentlichen](#eigene-pakete-veröffentlichen)

---

## 1. Das Modulsystem

VelinScript verwendet ein modernes, dateibasiertes Modulsystem, das stark von Rust und Python inspiriert ist. Es fördert Kapselung und Wiederverwendbarkeit.

### Dateien als Module

Jede `.velin`-Datei ist implizit ein Modul. Der Name der Datei (ohne Endung) ist der Modulname.

**Dateistruktur:**
```text
src/
  main.velin
  utils.velin
  models/
    user.velin
```

**Verwendung in `main.velin`:**
```velin
use utils
use models::user

fn main() {
    utils.helper();
    let u = user.User { ... };
}
```

### Explizite Sub-Module

Sie können Module auch *innerhalb* einer Datei definieren, um logische Gruppen zu bilden, ohne neue Dateien anzulegen.

```velin
// Datei: math_utils.velin

// Privates Modul (nur in dieser Datei sichtbar)
mod internal {
    fn helper() { ... }
}

// Öffentliches Modul
pub mod geometry {
    pub struct Point { x: number, y: number }

    pub fn distance(p1: Point, p2: Point): number {
        return math.sqrt(math.pow(p2.x - p1.x, 2) + ...);
    }
}
```

### Sichtbarkeit (`pub`)

Standardmäßig ist alles in VelinScript **privat** (nur im aktuellen Modul sichtbar). Um Funktionen, Structs oder Konstanten nach außen freizugeben, nutzen Sie das `pub`-Keyword.

```velin
// Nur hier sichtbar
fn secretHelper() {}

// Von überall sichtbar, wo das Modul importiert wird
pub fn publicApi() {
    secretHelper(); // Interner Zugriff erlaubt
}

pub struct Config {
    pub host: string, // Feld ist öffentlich
    apiKey: string    // Feld ist privat (nur über Methoden zugreifbar)
}
```

### Re-Exporting

Sie können Typen aus Untermodulen "hochziehen", um eine sauberere API zu bieten.

```velin
// Datei: api/mod.velin
use api::users
use api::products

// Nutzer müssen nicht 'api::users::User' schreiben, sondern nur 'api::User'
pub use users::User
pub use products::Product
```

---

## 2. CLI & Systemprozesse (`process` Modul)

VelinScript eignet sich hervorragend als Ersatz für Bash- oder Python-Skripte im DevOps-Bereich.

### Externe Befehle ausführen

Nutzen Sie `process.spawn`, um Systembefehle auszuführen.

```velin
use process

fn backupDatabase() {
    log.info("Starte Backup...");
    
    // Führt 'pg_dump' aus
    let output = process.spawn("pg_dump", [
        "-U", "postgres",
        "-f", "./backup.sql",
        "my_database"
    ]);

    if (output.exitCode == 0) {
        log.info("Backup erfolgreich!");
    } else {
        log.error("Backup fehlgeschlagen: " + output.stderr);
        throw Error("Backup Failed");
    }
}
```

### Prozessmanagement und Überwachung

Sie können laufende Prozesse überwachen und steuern.

```velin
fn ensureServiceRunning(serviceName: string) {
    if (process.is_running(serviceName)) {
        let mem = process.get_memory(serviceName); // in MB
        log.info(serviceName + " läuft (RAM: " + mem + "MB)");
        
        if (mem > 1024) {
            log.warn("Speicherleck erkannt, starte neu...");
            process.restart(serviceName);
        }
    } else {
        log.warn(serviceName + " ist down. Starte...");
        process.spawn("systemctl", ["start", serviceName]);
    }
}
```

### Interaktive CLIs bauen

VelinScript kann Benutzereingaben lesen und farbige Ausgaben erzeugen.

```velin
use console

fn main() {
    console.print("Willkommen zum Setup-Wizard!", "green");
    
    let name = console.prompt("Wie heißt dein Projekt?");
    let type = console.select("Projekttyp wählen:", ["API", "CLI", "Web"]);
    
    if (console.confirm("Soll ich 'git init' ausführen?")) {
        process.spawn("git", ["init"]);
    }
    
    console.print("Fertig!", "bold");
}
```

---

## 3. Plugin-Entwicklung

Die Plugin-Architektur erlaubt es, VelinScript-Anwendungen zur Laufzeit zu erweitern, ohne den Kern neu zu kompilieren. Plugins sind kompilierte `.vplugin`-Dateien (Shared Libraries).

### Architektur eines Plugins

Ein Plugin muss das `Plugin`-Trait implementieren.

```velin
// Datei: my_plugin.velin

struct MyPlugin {
    config: Map<string, any>
}

impl Plugin for MyPlugin {
    // Wird beim Laden aufgerufen
    fn on_load(ctx: PluginContext) {
        log.info("Plugin wird initialisiert...");
        
        // Hooks registrieren
        ctx.register_hook("on_request", |req| {
            log.info("Request empfangen: " + req.path);
        });
        
        // Eigene Befehle hinzufügen
        ctx.register_command("hello", || log.info("Hallo vom Plugin!"));
    }
    
    // Wird beim Entladen (oder Shutdown) aufgerufen
    fn on_unload() {
        log.info("Plugin wird gestoppt.");
    }
}

// Export der Factory-Funktion
pub fn create(config: Map<string, any>): Plugin {
    return MyPlugin { config: config };
}
```

### Lifecycle Hooks

Plugins können sich in verschiedene Phasen der Anwendung einklinken:

*   `on_load`: Initialisierung.
*   `on_config_loaded`: Nach dem Laden der Konfiguration.
*   `on_server_start`: Bevor der HTTP-Server startet.
*   `on_request`: Middleware-ähnlicher Hook für jeden Request.
*   `on_error`: Globaler Error-Handler.
*   `on_unload`: Cleanup.

---

## 4. Paketmanagement mit `velin.toml`

Jedes VelinScript-Projekt wird durch eine `velin.toml`-Datei definiert.

### Abhängigkeiten verwalten

```toml
[package]
name = "my-awesome-app"
version = "1.0.0"
authors = ["Max Mustermann <max@example.com>"]
description = "Eine VelinScript Demo App"

[dependencies]
# Standard-Abhängigkeiten aus der Registry
http-server = "1.2.0"
sea-orm = "0.12"

# Git-Abhängigkeiten (für Private Repos oder Forks)
utils = { git = "https://github.com/velin/utils.git", branch = "dev" }

# Lokale Pfade (für Monorepos)
my-shared-lib = { path = "./libs/shared" }

[dev-dependencies]
test-utils = "0.5"
```

### Workspaces

Für große Projekte (Monorepos) können Sie Workspaces definieren, um Abhängigkeiten zu teilen und mehrere Pakete gemeinsam zu bauen.

```toml
# Root velin.toml
[workspace]
members = [
    "apps/api",
    "apps/worker",
    "libs/shared",
    "libs/database"
]
```

### Eigene Pakete veröffentlichen

1.  **Login:** `velin login`
2.  **Build & Test:** `velin test && velin build --release`
3.  **Publish:** `velin publish`

Ihr Paket ist dann sofort über die zentrale Registry verfügbar.

---

*Ende des Advanced-Guides. Sie haben nun einen tiefen Einblick in die Systemarchitektur von VelinScript.*
