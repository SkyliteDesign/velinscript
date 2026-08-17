# Auto-Import Management

VelinScript LSP Server unterstützt automatisches Import-Management für bessere Developer Experience.

## Features

### Automatisches Hinzufügen von Imports

Wenn du einen Typ verwendest, der nicht importiert ist, bietet der LSP Server automatisch an, den fehlenden Import hinzuzufügen.

**Beispiel:**
```velin
// Du verwendest User, aber es ist nicht importiert
fn processUser(user: User) {
    // ...
}
```

Der LSP Server erkennt den Fehler "undefined type: User" und bietet eine Code Action an:
- **"Add import: use module::User;"**

### Import-Organisierung

Der LSP Server kann Imports automatisch organisieren:

- **Sortierung** - Alphabetische Sortierung
- **Gruppierung** - std, extern, local Imports werden gruppiert
- **Bereinigung** - Ungenutzte Imports werden entfernt

## Verwendung in VS Code

### Code Actions

1. Öffne eine `.velin` Datei
2. Wenn ein "undefined type" Fehler auftritt, siehst du eine Glühbirne 💡
3. Klicke auf die Glühbirne oder drücke `Ctrl+.` (Windows/Linux) oder `Cmd+.` (Mac)
4. Wähle "Add missing import"

### Organize Imports

1. Rechtsklick im Editor
2. Wähle "Organize Imports" oder verwende den Command:
   - `Ctrl+Shift+P` → "Organize Imports"

## Beispiel

**Vorher:**
```velin
use std::collections::HashMap;
use crate::models::Product;
use std::io::Read;
use crate::utils::helpers;
```

**Nach "Organize Imports":**
```velin
use std::collections::HashMap;
use std::io::Read;

use crate::models::Product;
use crate::utils::helpers;
```

## Konfiguration

Die Auto-Import Features sind standardmäßig aktiviert. In zukünftigen Versionen können sie in der VS Code Extension konfiguriert werden.

## Best Practices

1. **Regelmäßig organisieren** - Nutze "Organize Imports" vor Commits
2. **Code Actions nutzen** - Nutze Quick Fixes für fehlende Imports
3. **Manuelle Kontrolle** - Prüfe automatisch hinzugefügte Imports
