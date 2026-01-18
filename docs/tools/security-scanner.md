# VelinScript Security Scanner

Der VelinScript Security Scanner analysiert Code auf Security-Vulnerabilities und Sicherheitsprobleme.

## Installation

Der Security Scanner ist Teil des VelinScript Toolchains. Baue ihn mit:

```bash
cd tools/security-scanner
cargo build --release
```

## Verwendung

### Code scannen

```bash
velin-security scan
```

Scannt das aktuelle Verzeichnis auf Security-Vulnerabilities.

### Spezifisches Verzeichnis scannen

```bash
velin-security scan src/
```

### JSON-Report generieren

```bash
velin-security scan --format json > security-report.json
```

### HTML-Report generieren

```bash
velin-security scan --format html
```

Generiert `security-report.html` mit detailliertem Report.

### Dependencies auditieren

```bash
velin-security audit
```

Prüft Dependencies in `velin.toml` auf bekannte Vulnerabilities.

### Custom Config-Datei

```bash
velin-security audit --config custom-velin.toml
```

## Erkannte Vulnerabilities

### Critical

- **SQL Injection** - Ungeprüfte SQL-Queries
- **Command Injection** - Ungeprüfte System-Commands
- **Path Traversal** - Ungeprüfte Dateipfade
- **Hardcoded Secrets** - Passwörter, API-Keys im Code

### High

- **XSS (Cross-Site Scripting)** - Ungeprüfte User-Input in HTML
- **CSRF (Cross-Site Request Forgery)** - Fehlende CSRF-Token
- **Insecure Random** - Unsichere Zufallszahlen-Generierung
- **Unsafe Deserialization** - Unsichere Deserialisierung

### Medium

- **Weak Cryptography** - Schwache Verschlüsselungsalgorithmen
- **Missing Authentication** - Fehlende Authentifizierung
- **Insecure Direct Object Reference** - Ungeprüfte Objekt-Referenzen

### Low

- **Information Disclosure** - Zu detaillierte Error-Messages
- **Weak Password Policy** - Schwache Passwort-Anforderungen

## Beispiel-Output

### Text-Format

```
🔍 VelinScript Security Scanner
===============================

[CRITICAL] SQL Injection
  Location: src/api/users.velin:42
  Message: Ungeprüfte SQL-Query mit User-Input
  Recommendation: Verwende Prepared Statements oder ORM

[HIGH] XSS Vulnerability
  Location: src/templates/render.velin:15
  Message: User-Input wird ohne Escaping ausgegeben
  Recommendation: Verwende html::escape() oder Template-Engine mit Auto-Escaping

[MEDIUM] Hardcoded Secret
  Location: src/config.velin:8
  Message: API-Key ist im Code hardcodiert
  Recommendation: Verwende Environment-Variablen oder Secrets-Management

⚠ 3 Vulnerabilities gefunden
```

### JSON-Format

```json
[
  {
    "rule": "SQL Injection",
    "severity": "Critical",
    "location": "src/api/users.velin:42",
    "message": "Ungeprüfte SQL-Query mit User-Input",
    "recommendation": "Verwende Prepared Statements oder ORM"
  },
  {
    "rule": "XSS Vulnerability",
    "severity": "High",
    "location": "src/templates/render.velin:15",
    "message": "User-Input wird ohne Escaping ausgegeben",
    "recommendation": "Verwende html::escape() oder Template-Engine mit Auto-Escaping"
  }
]
```

## Security Rules

### SQL Injection Detection

Erkennt ungeprüfte SQL-Queries:

```velin
// ❌ VULNERABLE
fn getUser(id: string): User {
    let query = "SELECT * FROM users WHERE id = " + id;
    return db.query(query);
}

// ✅ SAFE
fn getUser(id: string): User {
    return db.find(User, id);
}
```

### XSS Detection

Erkennt ungeprüfte User-Input in HTML:

```velin
// ❌ VULNERABLE
fn renderUser(name: string): string {
    return "<div>" + name + "</div>";
}

// ✅ SAFE
fn renderUser(name: string): string {
    return "<div>" + html::escape(name) + "</div>";
}
```

### Hardcoded Secrets

Erkennt Passwörter, API-Keys, etc. im Code:

```velin
// ❌ VULNERABLE
let api_key = "sk-1234567890abcdef";

// ✅ SAFE
let api_key = config::get_env("API_KEY");
```

### Command Injection

Erkennt ungeprüfte System-Commands:

```velin
// ❌ VULNERABLE
fn executeCommand(cmd: string): string {
    return process::spawn("sh", ["-c", cmd]);
}

// ✅ SAFE
fn executeCommand(cmd: string): string {
    let allowed_commands = ["ls", "pwd", "date"];
    if !allowed_commands.contains(cmd) {
        return "Command not allowed";
    }
    return process::spawn("sh", ["-c", cmd]);
}
```

## Integration in CI/CD

```yaml
# .github/workflows/security.yml
- name: Security Scan
  run: |
    cd tools/security-scanner
    cargo build --release
    ./target/release/velin-security scan --format json > security-report.json
    if [ -s security-report.json ]; then
      echo "Security Vulnerabilities gefunden!"
      cat security-report.json
      exit 1
    fi

- name: Dependency Audit
  run: |
    ./target/release/velin-security audit
```

## Best Practices

1. **Regelmäßig scannen** - Integriere Security Scanning in CI/CD
2. **Vor Releases prüfen** - Führe Security Scan vor jedem Release aus
3. **Dependencies auditieren** - Prüfe regelmäßig Dependencies auf Vulnerabilities
4. **Fix Critical Issues sofort** - Behebe Critical Vulnerabilities sofort
5. **Security Reviews** - Führe regelmäßige Security Reviews durch
6. **Training** - Bilde Team in Security Best Practices aus

## Konfiguration

Erstelle eine `.velinsecurityrc.json` Datei:

```json
{
  "rules": {
    "sql-injection": "error",
    "xss": "error",
    "hardcoded-secret": "warning",
    "weak-cryptography": "warning"
  },
  "ignore": [
    "test/**",
    "examples/**"
  ],
  "severity-threshold": "medium"
}
```

## VS Code Integration

Der Security Scanner ist in der VS Code Extension integriert. Vulnerabilities werden direkt im Editor angezeigt.

## Dependency Audit

Der Scanner unterstützt Dependency-Auditing:

```bash
velin-security audit
```

Prüft `velin.toml` auf bekannte Vulnerabilities in Dependencies.

**Unterstützte Quellen:**
- CVE Database
- GitHub Security Advisories
- RustSec Advisory Database

## False Positives

Bei False Positives kannst du Regeln deaktivieren:

```velin
// velin-security-disable-next-line sql-injection
let query = "SELECT * FROM users WHERE id = " + sanitized_id;
```

Oder für ganze Dateien:

```velin
// velin-security-disable-file
```

## Reporting

### HTML Report

```bash
velin-security scan --format html
```

Generiert einen detaillierten HTML-Report mit:
- Übersicht aller Vulnerabilities
- Schweregrad-Verteilung
- Empfehlungen pro Vulnerability
- Code-Snippets

### JSON Report

```bash
velin-security scan --format json > report.json
```

Für Integration in andere Tools.

## Troubleshooting

### Zu viele False Positives

- Passe die Konfiguration an (`.velinsecurityrc.json`)
- Deaktiviere bestimmte Regeln
- Verwende `// velin-security-disable` Kommentare

### Performance-Probleme

- Scanne nur relevante Verzeichnisse
- Ignoriere große Verzeichnisse (z.B. `vendor/`)
- Verwende Caching für wiederholte Scans

### Dependencies nicht gefunden

- Prüfe, ob `velin.toml` korrekt ist
- Prüfe, ob Dependencies installiert sind
- Prüfe Internet-Verbindung für Audit-Datenbanken
