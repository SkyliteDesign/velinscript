# Passwort-Generator

## 🔐 Beschreibung

Ein umfassender Passwort-Generator mit Stärkeprüfung, verschiedenen Generierungsoptionen und Sicherheitsanalyse. Dieses Beispiel zeigt:

- Sichere Passwort-Generierung
- Passwortstärke-Bewertung
- Entropie-Berechnung
- Passphrasen-Generierung
- PIN-Generierung
- Crack-Zeit-Schätzung
- Konfigurierbare Zeichensätze

## 🎯 Lernziele

- Kryptografische Best Practices
- Zufallszahlen-Generierung
- String-Manipulation für Sicherheit
- Komplexitätsbewertung
- Mathematische Berechnungen (Entropie, Kombinatorik)
- Pattern-Matching mit Regex
- Scoring-Algorithmen

## 🚀 Verwendung

### Passwort generieren (mit Optionen)
```bash
POST /api/password/generate
{
    "length": 16,
    "includeUppercase": true,
    "includeLowercase": true,
    "includeNumbers": true,
    "includeSymbols": true,
    "excludeSimilar": true,
    "excludeAmbiguous": false
}
```

Antwort:
```json
{
    "password": "Kp9$mX2nRt#Yz4Lq",
    "strength": "Sehr stark",
    "score": 95,
    "entropy": 95.2,
    "suggestions": []
}
```

### Mehrere Passwörter generieren
```bash
POST /api/password/generate-multiple
{
    "options": {
        "length": 12,
        "includeUppercase": true,
        "includeLowercase": true,
        "includeNumbers": true,
        "includeSymbols": false,
        "excludeSimilar": true,
        "excludeAmbiguous": false
    },
    "count": 5
}
```

Gibt 5 verschiedene Passwörter zurück.

### Passwortstärke prüfen
```bash
POST /api/password/check-strength
{
    "password": "MyP@ssw0rd123"
}
```

Antwort:
```json
{
    "score": 65,
    "level": "Mittel",
    "hasUppercase": true,
    "hasLowercase": true,
    "hasNumbers": true,
    "hasSymbols": true,
    "length": 13,
    "suggestions": [
        "Vermeide häufige Wörter wie 'password'",
        "Empfohlen: Mindestens 16 Zeichen"
    ]
}
```

### Passphrase generieren (aus Wörtern)
```bash
POST /api/password/generate-passphrase
{
    "wordCount": 4,
    "separator": "-"
}
```

Beispiel: `"tiger-mountain-sunset-wizard"`

Vorteile:
- Leichter zu merken
- Trotzdem sicher bei ausreichender Länge
- Gut für Master-Passwörter

### PIN generieren
```bash
POST /api/password/generate-pin
{
    "length": 6
}
```

Beispiel: `"749283"`

### Crack-Zeit schätzen
```bash
POST /api/password/crack-time
{
    "password": "Abc123"
}
```

Gibt zurück: `"2 Stunden"` (Beispiel)

Für ein starkes Passwort:
```bash
POST /api/password/crack-time
{
    "password": "K9$mX2n#Yz4L"
}
```

Gibt zurück: `"1847293 Jahre"` (Beispiel)

## 💡 Wichtige Konzepte

### 1. Entropie
Maß für Unvorhersagbarkeit eines Passworts:
```
Entropie = Länge × log₂(Zeichensatzgröße)
```

Beispiele:
- `abc123` (6 Zeichen, nur Kleinbuchstaben + Zahlen): ~31 Bit
- `Kp9$mX2n` (8 Zeichen, gemischt): ~52 Bit
- `Kp9$mX2nRt#Yz4Lq` (16 Zeichen, gemischt): ~105 Bit

**Empfohlung**: Mindestens 60-80 Bit Entropie

### 2. Zeichensatzgröße
- Nur Kleinbuchstaben: 26
- + Großbuchstaben: 52
- + Zahlen: 62
- + Symbole: ~94

### 3. Scoring-System
- **Länge**: 10-40 Punkte
- **Großbuchstaben**: +10 Punkte
- **Kleinbuchstaben**: +10 Punkte
- **Zahlen**: +15 Punkte
- **Symbole**: +25 Punkte
- **Häufige Muster**: -10 bis -20 Punkte

### 4. Stärke-Level
- 0-29: Sehr schwach
- 30-49: Schwach
- 50-69: Mittel
- 70-84: Stark
- 85-100: Sehr stark

### 5. Optionen
- **excludeSimilar**: Entfernt `il1Lo0O` (verhindert Verwechslungen)
- **excludeAmbiguous**: Entfernt `{}[]()` (verhindert Probleme bei Eingabe)

## 📊 Empfehlungen

### Für verschiedene Verwendungszwecke

**Online-Accounts (Standard)**:
```json
{
    "length": 16,
    "includeUppercase": true,
    "includeLowercase": true,
    "includeNumbers": true,
    "includeSymbols": true,
    "excludeSimilar": true
}
```

**Master-Passwort (sehr wichtig)**:
```json
{
    "wordCount": 6,
    "separator": "-"
}
```
Oder: 20+ Zeichen mit allen Zeichentypen

**WLAN-Passwort**:
```json
{
    "length": 24,
    "includeUppercase": true,
    "includeLowercase": true,
    "includeNumbers": true,
    "includeSymbols": false,
    "excludeSimilar": true,
    "excludeAmbiguous": true
}
```

**PIN (Smartphone, Bankkarte)**:
```
6-8 Ziffern, vermeide 0000, 1234, etc.
```

## 🔧 Erweiterungsmöglichkeiten

- Pwned Passwords API Integration (Check ob Passwort geleakt)
- Custom Wortlisten für Passphrasen
- Passwort-Manager-Integration
- QR-Code-Generierung für WLAN-Passwörter
- Aussprache-Hinweise für Passphrasen
- Passwortverlauf und -Rotation
- Compliance-Check (z.B. NIST-Richtlinien)
- Passwort-Sharing mit Verschlüsselung
- 2FA-Code-Generierung
- Biometrische Alternativen-Vorschläge

## ⚠️ Sicherheitshinweise

1. **Niemals** Passwörter im Klartext speichern
2. Verwende einen Passwort-Manager für einzigartige Passwörter
3. Aktiviere 2FA wo möglich
4. Ändere kompromittierte Passwörter sofort
5. Teile Passwörter niemals per E-Mail/Chat
6. Verwende verschiedene Passwörter für verschiedene Accounts
7. Bei Generierung: Nutze kryptografisch sichere Zufallszahlen (in Produktion)

## 📚 Best Practices

- ✅ Mindestens 12 Zeichen (besser 16+)
- ✅ Alle Zeichentypen gemischt
- ✅ Keine Wörterbuch-Wörter
- ✅ Keine persönlichen Informationen
- ✅ Keine Muster (123, abc, qwerty)
- ✅ Regelmäßig wechseln (alle 3-6 Monate)
- ✅ Einzigartig für jeden Account
