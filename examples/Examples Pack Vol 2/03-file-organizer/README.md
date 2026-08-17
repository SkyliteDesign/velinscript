# Datei-Organizer

## 📁 Beschreibung

Ein praktisches Tool zum automatischen Organisieren von Dateien nach verschiedenen Kriterien. Dieses Beispiel demonstriert:

- Dateisystem-Operationen
- String-Verarbeitung und Pattern Matching
- Datenstrukturen für Dateiinformationen
- Gruppierung und Kategorisierung
- Fehlerbehandlung bei Dateioperationen
- Statistiken und Analysen

## 🎯 Lernziele

- Arbeiten mit dem Dateisystem (`fs` Modul)
- Dateien lesen, verschieben und organisieren
- Pattern Matching mit String-Operationen
- Collections und Maps verwenden
- Fehlerbehandlung mit try-catch
- Datenverarbeitung und -analyse

## 🚀 Verwendung

### Dateien auflisten
```bash
GET /api/files/list?directory=/pfad/zu/dateien
```

Gibt alle Dateien mit Details zurück:
```json
[
    {
        "name": "foto.jpg",
        "path": "/pfad/zu/dateien/foto.jpg",
        "extension": "jpg",
        "sizeBytes": 2048576,
        "createdAt": "2026-01-15 10:30:00",
        "category": "Bilder"
    }
]
```

### Nach Typ organisieren
```bash
POST /api/files/organize
{
    "sourceDir": "/pfad/zu/dateien",
    "targetDir": "/pfad/zu/organisierten/dateien"
}
```

Erstellt Unterordner:
- `/Bilder/` - JPG, PNG, GIF, SVG
- `/Videos/` - MP4, AVI, MKV
- `/Musik/` - MP3, WAV, FLAC
- `/Dokumente/` - PDF, DOC, TXT, MD
- `/Archive/` - ZIP, RAR, 7Z
- `/Programme/` - EXE, MSI, DMG
- `/Code/` - JS, PY, RS, VELIN
- `/Sonstiges/` - Alle anderen

### Nach Datum organisieren
```bash
POST /api/files/organize-by-date
{
    "sourceDir": "/pfad/zu/dateien",
    "targetDir": "/pfad/zu/organisierten/dateien"
}
```

Erstellt Ordner nach Jahr-Monat: `/2026-01/`, `/2026-02/`, etc.

### Datei-Statistiken abrufen
```bash
GET /api/files/statistics?directory=/pfad/zu/dateien
```

Gibt Übersicht zurück:
```json
{
    "totalFiles": 150,
    "totalSizeBytes": 5368709120,
    "largestFile": "video.mp4 (1.2 GB)",
    "smallestFile": "readme.txt (1.5 KB)",
    "filesByType": {
        "jpg": 45,
        "png": 20,
        "pdf": 15,
        "mp4": 10
    }
}
```

### Duplikate finden
```bash
GET /api/files/find-duplicates?directory=/pfad/zu/dateien
```

## 💡 Wichtige Konzepte

1. **Dateisystem-Operationen**:
   - `fs.readDir()` - Verzeichnis auslesen
   - `fs.move()` - Datei verschieben
   - `fs.createDir()` - Ordner erstellen

2. **Pattern Matching**: Kategorisierung nach Dateierweiterung

3. **Collections**:
   - `List<T>` für geordnete Sammlungen
   - `Map<K, V>` für Key-Value-Paare

4. **String-Operationen**:
   - `.split()` - String aufteilen
   - `.toLowerCase()` - Kleinschreibung
   - `.contains()` - Prüfen auf Teilstring

5. **Fehlerbehandlung**: Try-Catch für sichere Dateioperationen

## 🔧 Erweiterungsmöglichkeiten

- Duplikate basierend auf Dateiinhalt (Hash) erkennen
- Große Dateien automatisch archivieren
- Alte Dateien automatisch löschen
- Undo-Funktion für Organisationsschritte
- Vorschau-Modus (ohne tatsächliches Verschieben)
- Filter nach Dateigröße oder Datum
- Batch-Umbenennung von Dateien
- Integration mit Cloud-Storage

## ⚠️ Hinweise

- Stelle sicher, dass du Schreibrechte für die Verzeichnisse hast
- Erstelle Backups vor dem Organisieren wichtiger Dateien
- Die Duplikat-Erkennung basiert nur auf Dateinamen, nicht auf Inhalt
