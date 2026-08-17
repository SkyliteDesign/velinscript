# Kontaktbuch

## 📇 Beschreibung

Ein vollständiges Kontaktverwaltungssystem mit Such-, Filter-, Gruppen- und Export-Funktionen. Dieses Beispiel demonstriert:

- Vollständiges CRUD für Kontakte
- Erweiterte Suchfunktionen
- Gruppen-Organisation
- Favoriten-System
- Tag-Management
- Export (CSV, vCard)
- Geburtstags-Erinnerungen
- Statistiken

## 🎯 Lernziele

- Komplexe Datenmodelle mit verschachtelten Strukturen
- Erweiterte Suche mit mehreren Filtern
- Datenvalidierung (E-Mail, Telefon)
- Datenformatierung und -bereinigung
- Export in verschiedene Formate
- Viele-zu-Viele-Beziehungen (Kontakte ↔ Gruppen)
- Aggregation und Statistiken

## 🚀 Verwendung

### Kontakte verwalten

#### Alle Kontakte auflisten
```bash
GET /api/contacts?page=0&pageSize=20
```

#### Einzelnen Kontakt anzeigen
```bash
GET /api/contacts/:id
```

#### Neuen Kontakt erstellen
```bash
POST /api/contacts
{
    "firstName": "Max",
    "lastName": "Mustermann",
    "email": "max@example.com",
    "phone": "+49 123 456789",
    "company": "TechCorp",
    "jobTitle": "Software Engineer",
    "address": {
        "street": "Hauptstraße 1",
        "city": "Berlin",
        "state": "Berlin",
        "zipCode": "10115",
        "country": "Deutschland"
    },
    "birthday": "1990-05-15",
    "notes": "Kontakt von Konferenz XYZ"
}
```

#### Kontakt aktualisieren
```bash
PUT /api/contacts/:id
{
    "firstName": "Max",
    "lastName": "Mustermann",
    ...
}
```

#### Kontakt löschen
```bash
DELETE /api/contacts/:id
```

#### Als Favorit markieren
```bash
PATCH /api/contacts/:id/favorite
```

Toggle zwischen Favorit und nicht-Favorit.

#### Tags hinzufügen
```bash
POST /api/contacts/:id/tags
{
    "tags": ["Kunde", "VIP", "Berlin"]
}
```

### Suchen und Filtern

#### Kontakte suchen
```bash
POST /api/contacts/search
{
    "query": "max",
    "tags": ["Kunde"],
    "favoriteOnly": false,
    "company": "TechCorp"
}
```

**Filter-Optionen**:
- `query`: Textsuche in Name, E-Mail, Firma
- `tags`: Nur Kontakte mit bestimmten Tags
- `favoriteOnly`: Nur Favoriten
- `company`: Nur Kontakte einer Firma

#### Favoriten anzeigen
```bash
GET /api/contacts/favorites
```

#### Geburtstage diesen Monat
```bash
GET /api/contacts/birthdays
```

### Gruppen

#### Neue Gruppe erstellen
```bash
POST /api/groups
{
    "name": "Projektteam Alpha",
    "description": "Alle Mitglieder des Alpha-Projekts"
}
```

#### Kontakt zu Gruppe hinzufügen
```bash
POST /api/groups/:groupId/contacts/:contactId
```

#### Kontakte einer Gruppe anzeigen
```bash
GET /api/groups/:groupId/contacts
```

### Export

#### Alle Kontakte als CSV
```bash
GET /api/contacts/export/csv
```

Beispiel-Ausgabe:
```csv
Vorname,Nachname,E-Mail,Telefon,Firma,Position,Stadt,Land
Max,Mustermann,max@example.com,+49123456789,TechCorp,Software Engineer,Berlin,Deutschland
Anna,Schmidt,anna@example.com,+49987654321,DesignCo,Designer,München,Deutschland
```

#### Einzelnen Kontakt als vCard
```bash
GET /api/contacts/:id/vcard
```

Beispiel-Ausgabe:
```
BEGIN:VCARD
VERSION:3.0
FN:Max Mustermann
N:Mustermann;Max;;;
EMAIL:max@example.com
TEL:+49123456789
ORG:TechCorp
TITLE:Software Engineer
END:VCARD
```

### Statistiken

```bash
GET /api/contacts/stats
```

Antwort:
```json
{
    "totalContacts": 150,
    "favoriteCount": 23,
    "groupCount": 8,
    "companiesCount": 42
}
```

## 💡 Wichtige Konzepte

### 1. Datenmodell

**Contact** (Hauptstruktur):
```velin
struct Contact {
    id: string,
    firstName: string,
    lastName: string,
    email: string,
    phone: string,
    company: string,
    jobTitle: string,
    address: Address,      // Verschachtelt!
    birthday: string,
    notes: string,
    favorite: boolean,
    tags: List<string>,
    createdAt: string,
    updatedAt: string,
}
```

**Address** (Verschachtelte Struktur):
```velin
struct Address {
    street: string,
    city: string,
    state: string,
    zipCode: string,
    country: string,
}
```

### 2. Validierung

**E-Mail**:
```velin
fn isValidEmail(email: string): boolean {
    return email.matches("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$");
}
```

**Telefon**:
```velin
fn formatPhoneNumber(phone: string): string {
    // Entfernt alle Nicht-Zahlen außer +
    // "+49 123 456-789" → "+49123456789"
}
```

### 3. Suche

Multi-Kriterien-Suche:
1. Textsuche (Name OR E-Mail OR Firma)
2. UND Tag-Filter
3. UND Favoriten-Filter
4. UND Firmen-Filter

Alle Bedingungen müssen erfüllt sein.

### 4. Gruppen

Viele-zu-Viele-Beziehung:
- Ein Kontakt kann in mehreren Gruppen sein
- Eine Gruppe hat mehrere Kontakte

Implementation:
```velin
struct ContactGroup {
    contactIds: List<string>  // Referenzen zu Contacts
}
```

### 5. Export-Formate

**CSV**: Für Excel, Google Sheets, etc.
- Einfach zu parsen
- Gut für Bulk-Import/Export

**vCard (VCF)**: Für Kontakt-Apps
- Standard-Format
- Direkt in Smartphones importierbar
- Unterstützt von Outlook, Apple Contacts, etc.

## 📊 Use Cases

### 1. Business
- **CRM-Light**: Kundenverwaltung
- **Networking**: Konferenz-Kontakte
- **Team-Management**: Mitarbeiter-Verzeichnis

### 2. Personal
- **Adressbuch**: Freunde & Familie
- **Geburtstage**: Nie mehr vergessen
- **Organisation**: Mit Tags und Gruppen

### 3. Events
- **Gästeliste**: Event-Management
- **Seating**: Gruppierung nach Tisch
- **Follow-up**: Notizen zu Gesprächen

## 🔧 Erweiterungsmöglichkeiten

### Features
- **Bilder**: Profilfotos hochladen
- **Social Media**: Links zu LinkedIn, Twitter, etc.
- **Beziehungen**: "Partner von", "Arbeitet mit"
- **Notizen-Timeline**: Chronologische Interaktionen
- **Erinnerungen**: Automatische Follow-up-Benachrichtigungen
- **Duplikat-Erkennung**: Ähnliche Kontakte zusammenführen
- **Import**: Von CSV, vCard, Google Contacts
- **Backup**: Automatisches Backup
- **Synchronisation**: Mit Cloud-Services
- **Verschlüsselung**: Ende-zu-Ende für sensible Daten

### Integrationen
- **E-Mail**: Automatisches Hinzufügen aus E-Mails
- **Kalender**: Geburtstage automatisch eintragen
- **Maps**: Adresse in Karte anzeigen
- **QR-Code**: Kontakt als QR teilen
- **LinkedIn**: Automatischer Datenabgleich

### Collaboration
- **Sharing**: Kontakte mit Team teilen
- **Permissions**: Lese-/Schreibrechte
- **Comments**: Team-Notizen zu Kontakten
- **Activity Log**: Wer hat was geändert

## 🔐 Datenschutz

### Implementiert
- ✅ Daten-Minimierung (nur notwendige Felder)
- ✅ Soft-Delete möglich (mit Erweiterung)

### Empfohlen
- 🔒 GDPR-Compliance
  - Recht auf Vergessenwerden
  - Datenexport für User
  - Einwilligung tracken
- 🔒 Verschlüsselung sensibler Daten
- 🔒 Zugriffs-Logging
- 🔒 Data Retention Policies

## 📱 Mobile Optimierung

Features für Mobile:
- Pagination für große Listen
- ContactSummary für schnelle Übersicht
- Search für schnelles Finden
- QR-Code-Scanner für vCard
- "Click to Call" / "Click to Email"
- Swipe-Gesten (Favoriten, Löschen)

## 📈 Statistiken & Analytics

Tracking:
- Anzahl Kontakte insgesamt
- Anzahl Favoriten
- Anzahl Gruppen
- Anzahl verschiedener Firmen
- Wachstum über Zeit
- Häufigste Tags
- Vollständigkeit (Felder ausgefüllt)

## 🎨 UI-Komponenten

Empfohlene Views:
- **Liste**: Scrollbare Kontaktliste mit Avatar
- **Detail**: Alle Infos auf einen Blick
- **Edit**: Formular mit Validation
- **Search**: Mit Filter-Chips
- **Groups**: Drag & Drop zu Gruppen
- **Map**: Kontakte auf Karte

## 💾 Datenbank-Schema

```
contacts
  ├── id (PK)
  ├── firstName
  ├── lastName
  ├── email (indexed)
  ├── phone
  ├── company (indexed)
  ├── address (JSON)
  ├── tags (JSON array)
  └── favorite (indexed)

groups
  ├── id (PK)
  ├── name
  └── contactIds (JSON array)
```

**Indices für Performance**:
- Email (für Suche)
- Company (für Filter)
- Favorite (für schnelle Favoriten-Liste)
- lastName (für alphabetische Sortierung)
