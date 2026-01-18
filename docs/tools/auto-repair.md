# Auto-Repair & Self-Healing Builds

VelinScript geht einen Schritt weiter als klassische Compiler: Es sagt Ihnen nicht nur, was falsch ist, sondern versucht aktiv, es zu reparieren. Die **AutoFix Engine** ist tief in den Kompilierungsprozess integriert und kann eine Vielzahl von Syntax- und Flüchtigkeitsfehlern vollautomatisch beheben.

---

## Inhaltsverzeichnis

1.  [Funktionsweise](#1-funktionsweise)
2.  [Nutzung](#2-nutzung)
    *   [Im CLI (`--autofix`)](#im-cli---autofix)
    *   [In der IDE](#in-der-ide)
3.  [Was wird repariert?](#3-was-wird-repariert)
4.  [Konfiguration](#4-konfiguration)
5.  [Sicherheit & Grenzen](#5-sicherheit--grenzen)

---

## 1. Funktionsweise

Wenn Sie den Compiler starten, durchläuft Ihr Code mehrere Phasen (Parsing, Type Checking). Wenn der Parser auf einen Fehler stößt, bricht er normalerweise ab.

Mit aktiviertem Auto-Repair passiert folgendes:

1.  **Fehlererkennung:** Der Parser meldet z.B. "Unerwartetes Dateiende, `}` erwartet".
2.  **Analyse:** Die AutoFix Engine analysiert den Kontext (Einrückung, offene Blöcke).
3.  **Patching:** Der Code wird im Speicher korrigiert (z.B. wird `}` eingefügt).
4.  **Re-Run:** Der Compiler versucht erneut, den gepatchten Code zu verarbeiten.
5.  **Persistierung:** Wenn die Kompilierung erfolgreich ist, werden die Änderungen (optional) in Ihre Datei zurückgeschrieben.

Der Prozess ist **iterativ**: VelinScript kann bis zu 5 Reparatur-Durchläufe machen, um Kaskadenfehler zu beheben.

---

## 2. Nutzung

### Im CLI (`--autofix`)

Sie können die Reparatur explizit anfordern:

```bash
# Versucht zu kompilieren und repariert Fehler automatisch
velin compile -i main.velin --autofix

# Prüft nur und zeigt an, was repariert würde (Dry Run)
velin check -i main.velin --autofix
```

Wenn Reparaturen durchgeführt wurden, erhalten Sie einen Bericht:

```text
[AutoFix] 3 Fehler behoben:
  - main.velin:45 -> Fehlende schließende Klammer '}' ergänzt
  - main.velin:12 -> Typ-Signatur 'List<String' zu 'List<String>' korrigiert
  - utils.velin:8 -> Semikolon am Zeilenende eingefügt
```

### In der IDE

Die VS Code Extension nutzt dieselbe Engine für "Quick Fixes".

*   **Rote Unterstreichung:** Klicken Sie darauf oder drücken Sie `Ctrl+.`.
*   **Aktion:** Wählen Sie "Fix syntax error automatically".

---

## 3. Was wird repariert?

Die Engine konzentriert sich auf eindeutige, mechanische Fehler:

*   **Klammersetzung:**
    *   Fehlende `}`, `]`, `)` am Block- oder Dateiende.
    *   Unbalancierte Generics: `List<string` -> `List<string>`.
*   **Satzzeichen:**
    *   Fehlende Semikolons (wo sie syntaktisch zwingend sind).
    *   Falsche Kommas in Listen oder Maps.
*   **Typos (Experimentell):**
    *   Erkennt Buchstabendreher bei Keywords (`funtion` -> `fn`, `retrun` -> `return`).
*   **Imports:**
    *   (Via LSP) Schlägt vor, fehlende Module automatisch zu importieren, wenn der Typname eindeutig ist.

---

## 4. Konfiguration

Sie können in `velin.toml` steuern, wie aggressiv die Engine vorgehen soll.

```toml
[compiler.autofix]
# Aktiviert AutoFix standardmäßig für 'velin compile'
enabled = true

# Maximale Anzahl der Iterationen (Verhindert Endlos-Schleifen)
max_passes = 5

# Welche Kategorien sollen repariert werden?
rules = [
    "braces",       # Klammern
    "semicolons",   # Semikolons
    "keywords",     # Tippfehler in Keywords
    "imports"       # Auto-Imports (nur LSP)
]

# Soll vor dem Überschreiben der Datei ein Backup erstellt werden?
backup = true
```

---

## 5. Sicherheit & Grenzen

Die AutoFix Engine ist **konservativ**. Sie wird niemals Logik ändern oder raten, was Sie gemeint haben könnten, wenn es mehrdeutig ist.

*   **Beispiel (Reparierbar):** `if (x > 0 {` -> `if (x > 0) {`
*   **Beispiel (Nicht reparierbar):** `let x = y +` (Hier fehlt ein Operand, der Compiler kann nicht wissen, was addiert werden soll).

**Best Practice:** Nutzen Sie AutoFix als Lernwerkzeug. Schauen Sie sich die Änderungen an (z.B. via `git diff`), um zu verstehen, was falsch war.
