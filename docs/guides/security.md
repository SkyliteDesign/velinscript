# Sicherheit in VelinScript

Sicherheit ist kein Feature, sondern eine Notwendigkeit. VelinScript verfolgt den Ansatz "Secure by Default". Das bedeutet, dass unsichere Muster oft gar nicht erst kompilieren oder dass die Standardeinstellungen immer die sicherste Variante wählen.

Dieses Handbuch erklärt, wie Sie Ihre Anwendungen absichern, von der Benutzerauthentifizierung über die Abwehr von Angriffen bis hin zur proaktiven Code-Analyse.

---

## Inhaltsverzeichnis

1.  [Authentifizierung & Autorisierung](#1-authentifizierung--autorisierung)
    *   [Authentifizierung mit `@Auth`](#authentifizierung-mit-auth)
    *   [Rollenbasierte Zugriffskontrolle (RBAC)](#rollenbasierte-zugriffskontrolle-rbac)
    *   [Attribute-Based Access Control (ABAC)](#attribute-based-access-control-abac)
    *   [Manuelle Token-Verwaltung](#manuelle-token-verwaltung)
2.  [Schutz vor Angriffen](#2-schutz-vor-angriffen)
    *   [Rate Limiting](#rate-limiting)
    *   [Input Sanitization & XSS](#input-sanitization--xss)
    *   [SQL Injection Prävention](#sql-injection-prävention)
3.  [Verschlüsselung und Hashing](#3-verschlüsselung-und-hashing)
    *   [Passwörter hashen](#passwörter-hashen)
    *   [Daten verschlüsseln](#daten-verschlüsseln)
4.  [Sicherheitsscanner (SAST)](#4-sicherheitsscanner-sast)
    *   [Verwendung des Scanners](#verwendung-des-scanners)
    *   [Erkannte Sicherheitslücken](#erkannte-sicherheitslücken)

---

## 1. Authentifizierung & Autorisierung

VelinScript integriert Authentifizierung direkt in das Web-Framework, um sicherzustellen, dass Sicherheitslogik nicht vergessen wird.

### Authentifizierung mit `@Auth`

Der einfachste Weg, einen Endpunkt zu schützen, ist der `@Auth`-Decorator. Er stellt sicher, dass ein gültiges JWT (JSON Web Token) oder Session-Cookie vorhanden ist.

```velin
use http
use security

@Controller("/api/profile")
struct ProfileController {

    // Dieser Endpunkt ist NUR für eingeloggte Benutzer erreichbar.
    // VelinScript validiert das Token automatisch und extrahiert den Benutzer.
    @Auth
    @GET("/")
    fn getProfile(user: User): User {
        // 'user' wird automatisch injiziert
        return user;
    }
}
```

Wenn kein Token vorhanden oder ungültig ist, antwortet der Server automatisch mit `401 Unauthorized`.

### Rollenbasierte Zugriffskontrolle (RBAC)

Oft reicht ein einfacher Login nicht aus; Sie müssen zwischen normalen Benutzern und Administratoren unterscheiden.

```velin
@Controller("/api/admin")
struct AdminController {

    // Nur Benutzer mit der Rolle "admin" dürfen hier zugreifen
    @Auth
    @Role("admin")
    @GET("/users")
    fn listAllUsers(): List<User> {
        return db.findAll(User);
    }
    
    // Mehrere Rollen erlauben
    @Auth
    @Role("admin", "moderator")
    @DELETE("/comments/:id")
    fn deleteComment(@Path("id") id: string) {
        db.delete(Comment, id);
    }
}
```

### Attribute-Based Access Control (ABAC)

Für komplexe Regeln, die nicht durch statische Rollen abgebildet werden können (z.B. "Darf nur eigene Daten bearbeiten"), nutzen Sie `@Authorize` mit einer Closure.

```velin
@Controller("/api/documents")
struct DocumentController {

    @Auth
    @Authorize(|user, req| {
        // Custom Logik: Darf zugreifen, wenn Admin ODER Eigentümer des Dokuments
        if (user.roles.contains("admin")) return true;
        
        let docId = req.params.get("id");
        let doc = db.find(Document, docId);
        return doc.ownerId == user.id;
    })
    @GET("/:id")
    fn getDocument(@Path("id") id: string): Document {
        return db.find(Document, id);
    }
}
```

### Manuelle Token-Verwaltung

Wenn Sie Login-Endpunkte bauen, müssen Sie Token selbst erstellen.

```velin
use security

@POST("/login")
fn login(@Body creds: LoginDto): LoginResponse {
    let user = db.find(User, { email: creds.email });
    
    if (!user || !security.verifyPassword(creds.password, user.passwordHash)) {
        throw HttpError.Unauthorized("Falsche Zugangsdaten");
    }
    
    let auth = AuthService.new();
    
    // Token erstellen (Gültig für 1 Stunde)
    let accessToken = auth.createToken({
        sub: user.id,
        roles: user.roles,
        exp: datetime.now().addHours(1)
    });
    
    let refreshToken = auth.createRefreshToken(user.id);
    
    return LoginResponse { accessToken, refreshToken };
}
```

---

## 2. Schutz vor Angriffen

### Rate Limiting

Schützen Sie Ihre API vor DDoS-Angriffen und Brute-Force-Versuchen.

**Deklarativ (Empfohlen):**
```velin
// Maximal 5 Login-Versuche pro Minute pro IP
@RateLimit(limit: 5, window: "1m")
@POST("/login")
fn login(...) { ... }

// Maximal 1000 API-Calls pro Stunde für diesen Controller
@RateLimit(limit: 1000, window: "1h")
@Controller("/api/public")
struct PublicApi { ... }
```

**Manuell (in Logik):**
```velin
use utils

fn sendSms(phone: string) {
    // Throttling: Nur 1 SMS alle 30 Sekunden an dieselbe Nummer
    utils.throttle("sms_" + phone, "30s", || {
        smsGateway.send(phone, "Ihr Code ist 1234");
    });
}
```

### Input Sanitization & XSS

Obwohl VelinScript durch JSON-APIs weniger anfällig für XSS (Cross-Site Scripting) ist als klassische SSR-Apps, sollten Benutzereingaben bereinigt werden, wenn sie HTML enthalten könnten.

```velin
use security

fn saveComment(input: string) {
    // Entfernt gefährliche Tags wie <script>, <iframe> etc.
    let safeHtml = security.sanitizeHtml(input);
    
    // Verhindert Directory Traversal (../../etc/passwd)
    let safePath = security.sanitizePath(userInputPath);
    
    db.save(Comment { text: safeHtml });
}
```

### SQL Injection Prävention

Das `db`-Modul nutzt intern Prepared Statements. **SQL Injection ist damit strukturell unmöglich**, solange Sie die ORM-Methoden nutzen.

```velin
// SICHER: Parameter werden escaped
db.findMany(User, { name: userInput });

// SICHER: Auch bei Raw Queries
db.execute("SELECT * FROM users WHERE name = ?", [userInput]);
```

**Warnung:** Bauen Sie NIEMALS SQL-Strings manuell zusammen!
```velin
// UNSICHER - VERMEIDEN!
// db.execute("SELECT * FROM users WHERE name = '" + userInput + "'");
```
Der VelinScript-Scanner warnt Sie vor solchen Mustern.

---

## 3. Verschlüsselung und Hashing

### Passwörter hashen

Speichern Sie niemals Passwörter im Klartext. VelinScript nutzt standardmäßig Argon2id, den aktuellen Goldstandard für Passwort-Hashing.

```velin
use security

// Passwort hashen (beim Registrieren)
let hash = security.hashPassword("GeheimesPasswort123");
// hash ist z.B. "$argon2id$v=19$m=4096,t=3,p=1$..."

// Passwort verifizieren (beim Login)
let isValid = security.verifyPassword("Eingabe123", hash);
```

### Daten verschlüsseln

Für sensible Daten in der Datenbank (z.B. API-Keys von Nutzern, Gesundheitsdaten).

```velin
use crypto

let secretKey = config.get("ENCRYPTION_KEY");

// Verschlüsseln (AES-256-GCM)
let encrypted = crypto.encrypt("Sensible Daten", secretKey);

// Entschlüsseln
let plain = crypto.decrypt(encrypted, secretKey);
```

---

## 4. Sicherheitsscanner (SAST)

VelinScript bringt einen eigenen Static Application Security Testing (SAST) Scanner mit. Dieser analysiert Ihren Quellcode auf bekannte Sicherheitsmuster, *bevor* Sie deployen.

### Verwendung des Scanners

Der Scanner ist in die CLI integriert:

```bash
# Gesamtes Projekt scannen
velin scan

# Spezifisches Verzeichnis scannen
velin scan ./src/controllers
```

Sie können den Scan auch als Teil Ihrer CI/CD-Pipeline (GitHub Actions, GitLab CI) laufen lassen. Wenn Sicherheitslücken gefunden werden, bricht der Build ab (Exit Code 1).

### Erkannte Sicherheitslücken

Der Scanner prüft unter anderem auf:

*   **Hardcodierte Secrets:** Findet AWS Keys, Datenbank-Passwörter oder API-Tokens im Code.
*   **Unsichere Regex (ReDoS):** Warnt vor regulären Ausdrücken, die bei bestimmten Eingaben exponentiell langsam werden und die CPU blockieren.
*   **SQL Injection:** Erkennt manuelle String-Verkettung in `db.execute`-Aufrufen.
*   **Fehlende Authentifizierung:** Warnt, wenn sensible Routen (z.B. `/admin/...`) nicht mit `@Auth` geschützt sind.
*   **Veraltete Abhängigkeiten:** Prüft `velin.lock` auf Pakete mit bekannten Sicherheitslücken (CVEs).

**Beispiel-Ausgabe:**
```text
[CRITICAL] Hardcoded Secret found in src/config.velin:12
    let apiKey = "sk-1234567890abcdef";
    -> Use config.get("API_KEY") instead.

[HIGH] Potential SQL Injection in src/reports.velin:45
    db.execute("SELECT * FROM orders WHERE id = " + id);
    -> Use parameterized queries: db.execute("...", [id])
```

---

*Ende des Sicherheits-Guides. Sicherheit ist ein fortlaufender Prozess – nutzen Sie die Werkzeuge, die VelinScript Ihnen bietet!*
