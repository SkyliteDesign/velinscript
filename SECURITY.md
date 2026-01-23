# Security Policy

## Supported Versions
VelinScript (Velisch) befindet sich in aktiver Entwicklung. Sicherheitsrelevante Updates werden ausschließlich für die jeweils aktuelle Hauptversion bereitgestellt.

| Version | Status |
|--------|--------|
| 3.1.0  | ✅ Aktiv unterstützt (Aktuell) |
| 3.0.x  | ✅ Aktiv unterstützt |
| 2.7    | ⚠️ Legacy (Sicherheitsupdates) |
| < 2.7  | ❌ Keine Sicherheitsupdates |
---

## Reporting a Vulnerability
Sicherheitslücken oder verdächtiges Verhalten können vertraulich gemeldet werden.

Bitte sende eine E‑Mail an:

**security@skylite.design**

Folgende Informationen helfen bei der Analyse:

- Beschreibung der Schwachstelle  
- Schritte zur Reproduktion  
- Erwartetes vs. tatsächliches Verhalten  
- Umgebung (OS, Version, Konfiguration)  
- Optional: Proof‑of‑Concept  

Wir bestätigen den Eingang innerhalb von **48 Stunden** und melden uns mit einer Einschätzung.

---

## Security Architecture Overview
VelinScript enthält mehrere eingebaute Sicherheitsmechanismen:

### 🔸 Adaptive Guard
Ein autonomes Sicherheitssystem, das:
- Anomalien erkennt  
- temporäre Regeln generiert  
- IP‑Adressen automatisch blockiert  
- Rate‑Limits dynamisch anpasst  

### 🔸 Self‑Healing Engine
Überwacht Ressourcen und reagiert auf:
- CPU‑Spikes  
- Memory‑Anomalien  
- hängende Agents  
- ungewöhnliche Prozessmuster  

### 🔸 ML‑basierte Anomaly Detection
Erkennt Muster wie:
- ungewöhnliche Zugriffsmuster  
- brute‑force‑ähnliches Verhalten  
- API‑Missbrauch  
- untypische Kompilierungsabläufe  

### 🔸 Secure‑by‑Design Language Features
VelinScript bietet:
- strikte Typisierung  
- sichere Standardmodule  
- isolierte Execution‑Environments  
- automatische Sanitization für API‑Parameter  

---

## Responsible Disclosure
Wir bitten darum, gefundene Schwachstellen **nicht öffentlich** zu posten, bevor:

1. wir die Lücke bestätigt haben  
2. ein Fix bereitsteht  
3. ein koordinierter Release‑Plan abgestimmt wurde  

Wir veröffentlichen Security‑Fixes transparent im Changelog.

---

## Security Hardening Recommendations
Für produktive Umgebungen empfehlen wir:

- Aktivierung des Adaptive Guard  
- Nutzung der integrierten Rate‑Limiter  
- regelmäßige Updates auf die neueste Version  
- Monitoring über die Velin‑Telemetry‑Module  
- Einsatz von SSH‑Key‑Authentifizierung statt Passwörtern  

---

## Hall of Fame
Forscher, die verantwortungsvoll melden, können auf Wunsch öffentlich erwähnt werden.

Hall of Fame: https://birdapi.de/security-hall-of-fame  
Bug Bounty Programm: https://birdapi.de/bug-bounty
