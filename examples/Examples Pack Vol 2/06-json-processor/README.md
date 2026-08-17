# JSON-Prozessor

## 🔄 Beschreibung

Ein umfassendes Tool zur Verarbeitung, Validierung und Transformation von JSON-Daten. Dieses Beispiel zeigt:

- JSON-Parsing und -Validierung
- Formatierung (Pretty Print / Minify)
- JSON-Path-Operationen
- Vergleiche und Diffs
- Statistik-Analysen
- Datenkonvertierung (JSON → CSV)
- Flattening (verschachtelt → flach)

## 🎯 Lernziele

- JSON-Verarbeitung mit `JSON.parse()` und `JSON.stringify()`
- Rekursive Datenstruktur-Traversierung
- Try-Catch für Fehlerbehandlung
- Arbeiten mit `any` Typ für dynamische Daten
- Collections und Maps
- Type-Checking zur Laufzeit
- Datenstruktur-Analyse

## 🚀 Verwendung

### JSON validieren
```bash
POST /api/json/validate
{
    "jsonString": "{\"name\": \"Max\", \"age\": 30}"
}
```

Antwort:
```json
{
    "isValid": true,
    "errors": [],
    "warnings": [],
    "structure": "Object(2 keys)"
}
```

### JSON formatieren (Pretty Print)
```bash
POST /api/json/format
{
    "jsonString": "{\"name\":\"Max\",\"age\":30}",
    "indent": 2
}
```

Gibt zurück:
```json
{
  "name": "Max",
  "age": 30
}
```

### JSON minimieren
```bash
POST /api/json/minify
{
    "jsonString": "{\n  \"name\": \"Max\",\n  \"age\": 30\n}"
}
```

Gibt zurück: `{"name":"Max","age":30}`

### Wert extrahieren (JSON-Path)
```bash
POST /api/json/extract
{
    "jsonString": "{\"user\": {\"address\": {\"city\": \"Berlin\"}}}",
    "path": "user.address.city"
}
```

Gibt zurück: `"Berlin"`

Mit Array-Index:
```bash
POST /api/json/extract
{
    "jsonString": "{\"users\": [{\"name\": \"Max\"}, {\"name\": \"Anna\"}]}",
    "path": "users[1].name"
}
```

Gibt zurück: `"Anna"`

### Wert setzen
```bash
POST /api/json/set
{
    "jsonString": "{\"user\": {\"name\": \"Max\"}}",
    "path": "user.email",
    "value": "max@example.com"
}
```

Gibt zurück:
```json
{
  "user": {
    "name": "Max",
    "email": "max@example.com"
  }
}
```

### JSON-Objekte mergen
```bash
POST /api/json/merge
{
    "json1": "{\"name\": \"Max\", \"age\": 30}",
    "json2": "{\"age\": 31, \"city\": \"Berlin\"}"
}
```

Gibt zurück:
```json
{
  "name": "Max",
  "age": 31,
  "city": "Berlin"
}
```

### JSON-Objekte vergleichen
```bash
POST /api/json/diff
{
    "json1": "{\"name\": \"Max\", \"age\": 30}",
    "json2": "{\"name\": \"Max\", \"age\": 31, \"city\": \"Berlin\"}"
}
```

Gibt zurück:
```json
{
    "added": ["city"],
    "removed": [],
    "changed": ["age"]
}
```

### JSON-Statistiken
```bash
POST /api/json/statistics
{
    "jsonString": "{\"user\": {\"name\": \"Max\", \"age\": 30, \"active\": true}}"
}
```

Gibt zurück:
```json
{
    "totalKeys": 3,
    "maxDepth": 2,
    "arrayCount": 0,
    "objectCount": 2,
    "stringCount": 1,
    "numberCount": 1,
    "booleanCount": 1,
    "nullCount": 0
}
```

### JSON flatten (verschachtelt → flach)
```bash
POST /api/json/flatten
{
    "jsonString": "{\"user\": {\"name\": \"Max\", \"address\": {\"city\": \"Berlin\"}}}"
}
```

Gibt zurück:
```json
{
    "user.name": "Max",
    "user.address.city": "Berlin"
}
```

### JSON zu CSV konvertieren
```bash
POST /api/json/to-csv
{
    "jsonString": "[{\"name\":\"Max\",\"age\":30},{\"name\":\"Anna\",\"age\":25}]"
}
```

Gibt zurück:
```
name,age
Max,30
Anna,25
```

## 💡 Wichtige Konzepte

1. **JSON-Parsing**:
   ```velin
   let data = JSON.parse(jsonString);
   ```

2. **JSON-Stringification**:
   ```velin
   let jsonString = JSON.stringify(data, null, 2); // mit Indent
   ```

3. **Rekursive Traversierung**: Durchlaufen verschachtelter Strukturen

4. **Type-Checking**:
   ```velin
   let t = typeof(obj);
   if (isArray(obj)) { ... }
   ```

5. **JSON-Path**: Zugriff auf verschachtelte Werte
   - Objekt: `user.address.city`
   - Array: `users[0].name`

6. **Dynamic Typing**: `any` für unbekannte JSON-Strukturen

7. **Fehlerbehandlung**: Try-Catch bei JSON.parse()

## 🔧 Erweiterungsmöglichkeiten

- JSONPath-Standard vollständig implementieren
- JSON-Schema-Validierung
- JSON-Patch (RFC 6902)
- JSON-Pointer (RFC 6901)
- Tiefe Merge-Strategien
- JSON zu XML Konvertierung
- YAML Support
- JSON Compression
- Circular Reference Detection
- Custom Transformer-Funktionen

## 📋 Unterstützte JSON-Path-Syntax

Aktuell implementiert:
- ✅ `key` - Direkter Zugriff
- ✅ `key1.key2.key3` - Verschachtelt
- ✅ `array[0]` - Array-Index
- ✅ `key.array[0].field` - Kombiniert

Noch nicht implementiert:
- ❌ `$.key` - Root-Notation
- ❌ `key[*]` - Wildcard
- ❌ `key[?(@.field)]` - Filter

## ⚠️ Hinweise

- Sehr tiefe JSON-Strukturen (>10 Ebenen) können Performance-Probleme verursachen
- CSV-Konvertierung funktioniert nur für flache Arrays von Objekten
- Bei großen JSON-Dateien kann Memory-Verbrauch hoch sein
- Flatten kann bei sehr verschachtelten Strukturen lange Keys erzeugen
