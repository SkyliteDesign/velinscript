# URL-Shortener

## 🔗 Beschreibung

Ein vollständiger URL-Verkürzer mit Analytics, Custom-Codes und Ablaufdatum. Dieses Beispiel demonstriert:

- URL-Verkürzung mit zufälligen oder Custom-Codes
- Click-Tracking und Analytics
- URL-Validierung
- Redirects
- Ablaufzeiten
- Statistiken (Clicks, Unique Visitors, Referers)
- Authentifizierung für Verwaltung

## 🎯 Lernziele

- HTTP-Redirects implementieren
- Analytics und Event-Tracking
- Code-Generierung und Eindeutigkeit
- Datum-/Zeit-Operationen
- Aggregation und Statistiken
- Datenbeziehungen (1:n)
- Validierung und Fehlerbehandlung

## 🚀 Verwendung

### URL verkürzen

#### Mit zufälligem Code
```bash
POST /api/shorten
{
    "url": "https://www.example.com/very/long/url/that/needs/shortening",
    "customCode": "",
    "expiresInDays": 0
}
```

Antwort:
```json
{
    "id": "uuid-123",
    "shortCode": "aB3xYz",
    "originalUrl": "https://www.example.com/very/long/url/that/needs/shortening",
    "createdAt": "2026-01-28 15:00:00",
    "expiresAt": "",
    "clickCount": 0,
    "createdBy": "user-123"
}
```

Verkürzte URL: `https://yourdomain.com/s/aB3xYz`

#### Mit Custom-Code
```bash
POST /api/shorten
{
    "url": "https://github.com/username/project",
    "customCode": "myproject",
    "expiresInDays": 30
}
```

Verkürzte URL: `https://yourdomain.com/s/myproject`

### URL aufrufen (Redirect)
```bash
GET /s/aB3xYz
```

→ Leitet automatisch zu Original-URL weiter
→ Erhöht Click-Counter
→ Speichert Analytics-Daten

### URL-Details abrufen
```bash
GET /api/url/aB3xYz
```

### Statistiken anzeigen
```bash
GET /api/url/aB3xYz/stats
```

Antwort:
```json
{
    "shortCode": "aB3xYz",
    "originalUrl": "https://www.example.com/...",
    "totalClicks": 142,
    "uniqueVisitors": 87,
    "clicksByDay": {
        "2026-01-28": 45,
        "2026-01-27": 52,
        "2026-01-26": 45
    },
    "topReferers": [
        "https://twitter.com (32 clicks)",
        "https://facebook.com (28 clicks)",
        "https://reddit.com (15 clicks)"
    ],
    "createdAt": "2026-01-25 10:00:00"
}
```

### Eigene URLs auflisten (Auth erforderlich)
```bash
GET /api/urls/my
```

### URL löschen (Auth erforderlich)
```bash
DELETE /api/url/aB3xYz
```

### Beliebte URLs anzeigen
```bash
GET /api/urls/popular?limit=10
```

### Code-Verfügbarkeit prüfen
```bash
GET /api/check-availability/mycode
```

Gibt `true` oder `false` zurück.

## 💡 Wichtige Konzepte

### 1. Short-Code-Generierung
- **Zufällig**: 6 Zeichen aus `[a-zA-Z0-9]` = 62^6 ≈ 56 Milliarden Kombinationen
- **Custom**: 3-10 Zeichen, benutzerdefiniert
- **Eindeutigkeits-Check**: Verhindert Kollisionen

### 2. Click-Tracking
Für jeden Aufruf wird gespeichert:
- Zeitstempel
- IP-Adresse (für Unique Visitors)
- User-Agent (Browser/Device)
- Referer (woher kam der Besucher)

### 3. Analytics
- **Total Clicks**: Alle Aufrufe
- **Unique Visitors**: Basierend auf IP (vereinfacht)
- **Clicks by Day**: Zeitliche Verteilung
- **Top Referers**: Von wo kommen die Besucher

### 4. Ablaufzeiten
- `expiresInDays: 0` → Kein Ablauf
- `expiresInDays: 30` → Läuft nach 30 Tagen ab
- Abgelaufene URLs geben 410 Gone zurück

### 5. Validierung
URLs müssen:
- Mit `http://` oder `https://` beginnen
- Mindestens 12 Zeichen lang sein

Custom-Codes müssen:
- 3-10 Zeichen lang sein
- Nur alphanumerisch `[a-zA-Z0-9]`
- Eindeutig sein

## 📊 Datenstruktur

```
ShortUrl (1) -------- (n) UrlClick
    ↓
shortCode, originalUrl, clickCount
    ↓
UrlClick: clickedAt, ipAddress, userAgent, referer
```

## 🔧 Erweiterungsmöglichkeiten

- QR-Code-Generierung für Short-URLs
- Browser-Extension für schnelles Verkürzen
- API-Rate-Limiting
- Spam-Schutz (Google Safe Browsing API)
- Custom Domains (z.B. go.company.com/code)
- Link-Preview mit Open Graph
- Geo-Location-Tracking
- Device/OS-Statistiken
- A/B-Testing mit mehreren URLs
- Bulk-URL-Import
- URL-Collections/Folders
- Password-Protected Links
- Link-Expiry-Notifications
- UTM-Parameter automatisch hinzufügen
- Link-Cloaking (verstecken der Original-URL)

## 🔐 Sicherheitsaspekte

### Implementiert
- ✅ URL-Validierung (Protokoll-Check)
- ✅ Short-Code-Validierung
- ✅ Authentifizierung für Verwaltung
- ✅ Berechtigungs-Check beim Löschen

### Empfohlen für Produktion
- 🔒 Rate-Limiting (verhindert Spam)
- 🔒 CAPTCHA für öffentliche Erstellung
- 🔒 Blacklist für bekannte Malware-URLs
- 🔒 IP-basiertes Blocking
- 🔒 Abuse-Report-System
- 🔒 GDPR-konforme Analytics (IP-Anonymisierung)

## 📈 Use Cases

1. **Social Media**: Kurze, teilbare Links
2. **Marketing**: Trackbare Campaign-Links
3. **Print**: QR-Codes mit kurzen URLs
4. **Branding**: Custom-Codes für Marken
5. **Analytics**: Detaillierte Click-Analysen
6. **Temporäre Links**: Mit Ablaufdatum für Events

## ⚙️ Performance-Tipps

- Index auf `shortCode` für schnelle Lookups
- Cache häufig aufgerufene URLs
- Batch-Insert für Click-Events
- Asynchrones Analytics-Processing
- CDN für Redirect-Endpoint

## 📝 Beispiel-Workflow

```
1. User erstellt Short-URL
   POST /api/shorten → Code: "abc123"

2. User teilt Link
   https://yourdomain.com/s/abc123

3. Besucher klicken
   GET /s/abc123
   → Redirect zu Original-URL
   → Click-Event gespeichert

4. User prüft Statistiken
   GET /api/url/abc123/stats
   → Sieht Clicks, Referers, etc.
```
