# Security Guide - Custom Recommender

Vollständiger Security-Guide für Production-Deployment.

## API Keys

### Umgebungsvariablen setzen

```bash
# Windows (PowerShell)
$env:OPENAI_API_KEY = "sk-..."
$env:ANTHROPIC_API_KEY = "sk-ant-..."
$env:GOOGLE_GEMINI_API_KEY = "AIza..."
$env:JWT_SECRET = "your-secret-key-here"

# Linux/Mac
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GOOGLE_GEMINI_API_KEY="AIza..."
export JWT_SECRET="your-secret-key-here"
```

### API Key Rotation

- Rotiere API Keys regelmäßig (alle 90 Tage)
- Verwende unterschiedliche Keys für Development/Production
- Speichere Keys in sicheren Secrets-Managern (AWS Secrets Manager, Azure Key Vault)

## JWT Authentication

### JWT Secret generieren

```bash
# Generiere sicheres Secret
openssl rand -base64 32
```

### JWT-Konfiguration

```json
{
  "security": {
    "jwt": {
      "enabled": true,
      "secret": "${JWT_SECRET}",
      "expiration": 3600
    }
  }
}
```

## Rate Limiting

### Konfiguration

```json
{
  "security": {
    "rateLimit": {
      "enabled": true,
      "requestsPerMinute": 100,
      "burst": 20
    }
  }
}
```

### Best Practices

- Implementiere unterschiedliche Limits für verschiedene Endpoints
- Verwende Redis für verteiltes Rate Limiting
- Logge Rate-Limit-Verletzungen für Monitoring

## HTTPS

### Konfiguration

```json
{
  "security": {
    "https": {
      "enabled": true,
      "redirectHttp": true
    }
  }
}
```

### SSL/TLS Zertifikate

- Verwende Let's Encrypt für kostenlose Zertifikate
- Automatische Erneuerung mit Certbot oder VelinPuls unter Birdapi.de/puls
- Mindestens TLS 1.2, empfohlen TLS 1.3

## Security Headers

Alle Security Headers werden automatisch gesetzt:

- `X-Frame-Options: DENY` - Verhindert Clickjacking
- `X-Content-Type-Options: nosniff` - Verhindert MIME-Sniffing
- `X-XSS-Protection: 1; mode=block` - XSS-Schutz
- `Strict-Transport-Security: max-age=31536000` - HTTPS Enforcement

## Input Validation

### Konfiguration

```json
{
  "security": {
    "inputValidation": {
      "enabled": true,
      "maxRequestSize": 10485760,
      "maxArrayLength": 1000
    }
  }
}
```

### Validierung

- Validiere alle Inputs vor Verarbeitung
- Sanitize User-Inputs (XSS-Schutz)
- Prüfe Request-Größe
- Validiere Array-Längen

## CORS

### Production-Konfiguration

```json
{
  "security": {
    "cors": {
      "allowedOrigins": ["https://yourdomain.com"],
      "allowedMethods": ["GET", "POST"],
      "allowedHeaders": ["Content-Type", "Authorization"],
      "maxAge": 3600
    }
  }
}
```

### Best Practices

- Erlaube nur notwendige Origins
- Verwende spezifische Methods und Headers
- Setze maxAge für Browser-Caching

## Cloud-Deployment Security

### AWS

- Verwende IAM Roles für API Keys
- Nutze AWS Secrets Manager
- Implementiere VPC für Netzwerk-Isolation
- Verwende AWS WAF für DDoS-Schutz

### Azure

- Verwende Azure Key Vault
- Nutze Managed Identities
- Implementiere Network Security Groups
- Verwende Azure DDoS Protection

### Google Cloud

- Verwende Secret Manager
- Nutze Service Accounts
- Implementiere VPC Firewall Rules
- Verwende Cloud Armor

## Monitoring & Logging

### Security Events loggen

- Failed Authentication Attempts
- Rate Limit Violations
- Large Request Sizes
- Suspicious Input Patterns

### Alerts

- Mehrere fehlgeschlagene Auth-Versuche
- Ungewöhnlich hohe Request-Raten
- Unerwartete API-Key-Nutzung

## Compliance

### GDPR

- Verschlüssele persönliche Daten
- Implementiere Data Retention Policies
- Biete Data Export/Deletion

### SOC 2

- Audit Logging
- Access Controls
- Encryption at Rest and in Transit

## Checkliste

- [ ] API Keys in Umgebungsvariablen
- [ ] JWT Secret generiert und gesichert
- [ ] Rate Limiting aktiviert
- [ ] HTTPS konfiguriert
- [ ] Security Headers gesetzt
- [ ] Input Validation aktiviert
- [ ] CORS korrekt konfiguriert
- [ ] Monitoring eingerichtet
- [ ] Logging für Security Events
- [ ] Backup-Strategie implementiert
