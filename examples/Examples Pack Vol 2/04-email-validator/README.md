# Email-Validator

## 📧 Beschreibung

Ein umfassender E-Mail-Validator mit Format-Prüfung, Domain-Validierung und intelligenten Vorschlägen. Dieses Beispiel zeigt:

- Regex-Pattern für Validierung
- String-Manipulation und -Parsing
- Detaillierte Fehler- und Warnungsmeldungen
- Intelligente Vorschläge bei Tippfehlern
- Bulk-Validierung
- E-Mail-Maskierung für Datenschutz

## 🎯 Lernziele

- Regular Expressions (Regex) verwenden
- String-Operationen: `.split()`, `.contains()`, `.substring()`
- Validierungslogik implementieren
- Fehler-Handling und detailliertes Feedback
- Collections durchlaufen und verarbeiten
- Datenstrukturen für strukturierte Antworten

## 🚀 Verwendung

### Einzelne E-Mail validieren
```bash
POST /api/email/validate
{
    "email": "user@example.com"
}
```

Antwort:
```json
{
    "email": "user@example.com",
    "isValid": true,
    "errors": [],
    "warnings": [],
    "suggestions": []
}
```

Bei Tippfehler:
```bash
POST /api/email/validate
{
    "email": "user@gmial.com"
}
```

Antwort:
```json
{
    "email": "user@gmial.com",
    "isValid": true,
    "errors": [],
    "warnings": [],
    "suggestions": ["user@gmail.com"]
}
```

### Mehrere E-Mails validieren
```bash
POST /api/email/validate-bulk
{
    "emails": [
        "user1@example.com",
        "invalid.email",
        "user2@domain.co.uk"
    ]
}
```

Antwort:
```json
{
    "totalEmails": 3,
    "validEmails": 2,
    "invalidEmails": 1,
    "results": [...]
}
```

### Domain extrahieren
```bash
GET /api/email/extract-domain/user@example.com
```

Gibt zurück: `"example.com"`

### E-Mail normalisieren
```bash
POST /api/email/normalize
{
    "email": "  User@EXAMPLE.com  "
}
```

Gibt zurück: `"user@example.com"`

### Domain prüfen
```bash
POST /api/email/check-domain
{
    "email": "user@company.com",
    "allowedDomains": ["company.com", "company.de"]
}
```

Gibt zurück: `true` oder `false`

### E-Mail maskieren (Datenschutz)
```bash
POST /api/email/mask
{
    "email": "john.doe@example.com"
}
```

Gibt zurück: `"j***@example.com"`

## 💡 Wichtige Konzepte

1. **Regular Expressions**: Pattern Matching für E-Mail-Format
   ```velin
   let pattern = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$";
   email.matches(pattern)
   ```

2. **String-Parsing**: E-Mail in Komponenten zerlegen
   ```velin
   let parts = email.split("@");
   let localPart = parts[0];
   let domain = parts[1];
   ```

3. **Validierungsregeln**:
   - Local-Part: Max 64 Zeichen, keine doppelten Punkte
   - Domain: Mindestens ein Punkt, gültiges Format
   - TLD: Bekannte oder mindestens 2 Zeichen

4. **Smart Suggestions**: Erkennt häufige Tippfehler
   - `gmial.com` → `gmail.com`
   - `yahooo.com` → `yahoo.com`

5. **Fehler vs. Warnungen**:
   - **Errors**: Machen E-Mail ungültig
   - **Warnings**: Hinweise auf potenzielle Probleme

## 🔧 Erweiterungsmöglichkeiten

- DNS-Lookup für Domain-Existenz-Prüfung
- MX-Record-Validierung
- Disposable E-Mail-Erkennung
- Role-Based E-Mail-Erkennung (info@, admin@)
- Internationalisierte E-Mail-Adressen (IDN)
- E-Mail-Verifizierung per Token-Versand
- Blacklist/Whitelist-Verwaltung
- Rate-Limiting für Bulk-Validierung

## 📋 Validierungskriterien

### Gültige E-Mails
- ✅ `user@example.com`
- ✅ `john.doe@company.co.uk`
- ✅ `info+tag@domain.org`

### Ungültige E-Mails
- ❌ `user@` (keine Domain)
- ❌ `@example.com` (kein Local-Part)
- ❌ `user..name@example.com` (doppelte Punkte)
- ❌ `.user@example.com` (beginnt mit Punkt)
- ❌ `user@example` (keine TLD)
- ❌ `user name@example.com` (Leerzeichen)

## ⚠️ Hinweise

- Diese Validierung ist Format-basiert, nicht Existenz-basiert
- Für Produktionsumgebungen DNS/MX-Record-Prüfung hinzufügen
- Maskierung schützt nur vor einfachem Scraping
