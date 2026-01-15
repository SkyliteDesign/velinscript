# VelinScript Debugger Tutorial

Der VelinScript Debugger ermöglicht es, VelinScript-Programme direkt in VS Code zu debuggen.

## Installation

Der Debugger ist Teil der VelinScript VS Code Extension. Stelle sicher, dass die Extension installiert ist.

## Debugger starten

### Automatisch (VS Code)

1. Öffne eine `.velin` Datei
2. Setze Breakpoints durch Klick auf den linken Rand
3. Drücke F5 oder gehe zu "Run and Debug"
4. Wähle "Debug VelinScript" aus der Konfiguration

### Manuell (Command Line)

```bash
velin-debugger start --port 4711
```

## Debug-Konfiguration

Erstelle eine `.vscode/launch.json` Datei:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "velin",
            "request": "launch",
            "name": "Debug VelinScript",
            "program": "${workspaceFolder}/main.velin",
            "stopOnEntry": false,
            "args": []
        },
        {
            "type": "velin",
            "request": "attach",
            "name": "Attach to VelinScript",
            "port": 4711,
            "host": "localhost"
        }
    ]
}
```

## Breakpoints

### Breakpoints setzen

- Klicke auf den linken Rand neben einer Zeilennummer
- Oder verwende `F9` auf der aktuellen Zeile
- Breakpoints werden als rote Punkte angezeigt

### Conditional Breakpoints

Rechtsklick auf einen Breakpoint → "Edit Breakpoint" → Bedingung eingeben:

```
x > 10
```

### Logpoints

Rechtsklick auf einen Breakpoint → "Add Logpoint" → Log-Nachricht eingeben:

```
Variable x = {x}
```

## Debugging-Features

### Step Over (F10)

Führt die aktuelle Zeile aus und pausiert bei der nächsten Zeile.

### Step Into (F11)

Tritt in Funktionsaufrufe ein.

### Step Out (Shift+F11)

Verlässt die aktuelle Funktion und pausiert bei der aufrufenden Funktion.

### Continue (F5)

Setzt die Ausführung fort bis zum nächsten Breakpoint.

### Restart (Ctrl+Shift+F5)

Startet das Debugging neu.

### Stop (Shift+F5)

Beendet das Debugging.

## Variablen inspizieren

### Variables Panel

Im "Variables" Panel siehst du alle Variablen im aktuellen Scope:

- Lokale Variablen
- Funktionsparameter
- Globale Variablen

### Watch Expressions

Füge Ausdrücke zum "Watch" Panel hinzu, um sie während des Debuggings zu überwachen:

```
x + y
user.name
list.length()
```

### Hover

Bewege die Maus über eine Variable, um ihren aktuellen Wert zu sehen.

## Call Stack

Das "Call Stack" Panel zeigt die Aufrufkette:

```
main() - main.velin:10
  createUser() - main.velin:25
    validateEmail() - validation.velin:5
```

## Debug Console

Die Debug Console ermöglicht es, Ausdrücke während des Debuggings zu evaluieren:

```
> x
42
> x + 10
52
> getUserName()
"John"
```

## Beispiel

```velin
fn calculateTotal(items: List<Item>): number {
    let mut total = 0.0;  // Breakpoint hier
    
    for (item in items) {
        total = total + item.price;  // Breakpoint hier
    }
    
    return total;  // Breakpoint hier
}

fn main(): void {
    let items = List<Item>([
        Item { name: "Apple", price: 1.5 },
        Item { name: "Banana", price: 2.0 }
    ]);
    
    let total = calculateTotal(items);  // Breakpoint hier
    // Inspect: total should be 3.5
}
```

## Tipps

1. **Verwende Conditional Breakpoints** für wiederholte Schleifen
2. **Nutze Logpoints** statt `println` für temporäres Logging
3. **Watch Expressions** für komplexe Berechnungen
4. **Call Stack** um den Ausführungsfluss zu verstehen

## Troubleshooting

### Debugger startet nicht

- Prüfe, ob `velin-debugger` im PATH ist
- Prüfe die Port-Konfiguration (Standard: 4711)
- Prüfe die VS Code Extension Logs

### Breakpoints werden nicht getroffen

- Stelle sicher, dass der Code kompiliert wurde
- Prüfe, ob Debug-Informationen generiert wurden
- Stelle sicher, dass der Code tatsächlich ausgeführt wird

### Variablen werden nicht angezeigt

- Prüfe, ob du an einem Breakpoint pausiert bist
- Stelle sicher, dass die Variablen im aktuellen Scope sind
- Prüfe, ob die Variablen initialisiert wurden
