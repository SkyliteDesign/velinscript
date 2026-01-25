# VelinScript Systemdiagnose - Vollständige Dokumentation

**Version:** 1.0.0  
**Status:** ✅ Vollständig implementiert  
**Letzte Aktualisierung:** 2024

---

## 📋 Inhaltsverzeichnis

1. [Übersicht](#übersicht)
2. [Installation & Setup](#installation--setup)
3. [Schnellstart](#schnellstart)
4. [Funktionsübersicht](#funktionsübersicht)
5. [API-Referenz](#api-referenz)
6. [Sicherheitsprüfungen](#sicherheitsprüfungen)
7. [Tests](#tests)
8. [Best Practices](#best-practices)
9. [Troubleshooting](#troubleshooting)
10. [Beispiele](#beispiele)

---

## 🎯 Übersicht

Das VelinScript Systemdiagnose-System ist ein umfassendes Tool zur Überwachung und Analyse von Systemressourcen, Sicherheit und Services. Es bietet:

- ✅ **Vollständige Systemressourcen-Überprüfung** (CPU, Memory, Disk, Network)
- ✅ **Umfassende Sicherheitsprüfungen** (Auth, Encryption, Certificates, etc.)
- ✅ **Service-Status-Überwachung**
- ✅ **Log-Analyse** mit Pattern-Erkennung
- ✅ **Automatische Empfehlungen** basierend auf Diagnose-Ergebnissen
- ✅ **Health-Score-Berechnung** (0-100)
- ✅ **Vollständige Test-Suite**
- ✅ **JSON-Export** für Integration mit Monitoring-Tools

### Hauptkomponenten

1. **`system_diagnosis.velin`** - Hauptmodul für Systemdiagnose
2. **`security_checks.velin`** - Sicherheitsprüfungen
3. **`tests/system_diagnosis_test.velin`** - Vollständige Test-Suite

---

## 🚀 Installation & Setup

### Voraussetzungen

- VelinScript 3.1.0 oder höher
- Zugriff auf Systemressourcen (CPU, Memory, Disk, Network)
- Berechtigung zum Lesen von Log-Dateien

### Installation

1. **Kopieren Sie die Module in Ihr Projekt:**

```bash
cp examples/system-diagnosis/system_diagnosis.velin src/
cp examples/system-diagnosis/security_checks.velin src/
```

2. **Importieren Sie die Module in Ihrem Code:**

```velin
use system_diagnosis;
use security_checks;
```

3. **Initialisieren Sie das System:**

```velin
// Automatisch beim Laden des Moduls
// Keine manuelle Initialisierung erforderlich
```

---

## ⚡ Schnellstart

### Einfache Diagnose durchführen

```velin
use system_diagnosis;

// Vollständige Systemdiagnose durchführen
let report = system_diagnosis.runFullDiagnosis();

// Status ausgeben
println("Gesamtstatus: " + report.overallStatus);
println("Health-Score: " + report.score);
println("Empfehlungen: " + report.recommendations.length);
```

### Nur Ressourcen prüfen

```velin
use system_diagnosis;

let resources = system_diagnosis.checkResources();

println("CPU-Auslastung: " + resources.cpu.usage + "%");
println("Speicherauslastung: " + resources.memory.usagePercent + "%");
println("Festplattenauslastung: " + resources.disk.usagePercent + "%");
```

### Nur Sicherheit prüfen

```velin
use security_checks;

let security = security_checks.runSecurityChecks();

println("Sicherheitsstatus: " + security.overallStatus);
println("Sicherheits-Score: " + security.score);
println("Gefundene Vulnerabilities: " + security.vulnerabilities.length);
```

### Als API-Endpoint verwenden

```velin
use system_diagnosis;
use json;

@GET("/api/diagnosis")
fn getDiagnosis(): string {
    let report = system_diagnosis.runFullDiagnosis();
    return json.stringify(report);
}

@GET("/api/diagnosis/health")
fn getHealth(): string {
    let report = system_diagnosis.runFullDiagnosis();
    return json.stringify({
        status: report.overallStatus,
        score: report.score,
        timestamp: report.timestamp,
    });
}
```

---

## 🔧 Funktionsübersicht

### Systemdiagnose-Modul

#### Hauptfunktionen

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `runFullDiagnosis()` | Führt vollständige Systemdiagnose durch | `SystemDiagnosisReport` |
| `checkResources()` | Überprüft alle Systemressourcen | `ResourceStatus` |
| `checkServices()` | Überprüft alle Services | `ServiceStatusList` |
| `analyzeLogs()` | Analysiert System-Logs | `LogAnalysis` |
| `collectSystemInfo()` | Sammelt System-Informationen | `SystemInfo` |

#### Ressourcen-Funktionen

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `checkCPU()` | Überprüft CPU-Status | `CPUStatus` |
| `checkMemory()` | Überprüft Memory-Status | `MemoryStatus` |
| `checkDisk()` | Überprüft Disk-Status | `DiskStatus` |
| `checkNetwork()` | Überprüft Network-Status | `NetworkStatus` |

#### Helper-Funktionen

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `determineOverallStatus()` | Bestimmt Gesamtstatus | `string` |
| `calculateHealthScore()` | Berechnet Health-Score (0-100) | `number` |
| `generateRecommendations()` | Generiert Empfehlungen | `List<string>` |

### Sicherheitsprüfungs-Modul

#### Hauptfunktionen

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `runSecurityChecks()` | Führt alle Sicherheitsprüfungen durch | `SecurityStatus` |
| `scanVulnerabilities()` | Scannt nach Vulnerabilities | `List<Vulnerability>` |

#### Authentifizierung & Autorisierung

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `checkAuthentication()` | Prüft Authentifizierung | `SecurityCheck` |
| `checkAuthorization()` | Prüft Autorisierung | `SecurityCheck` |
| `checkPasswordPolicy()` | Prüft Passwort-Richtlinien | `SecurityCheck` |
| `checkSessionSecurity()` | Prüft Session-Sicherheit | `SecurityCheck` |

#### Verschlüsselung & Zertifikate

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `checkEncryption()` | Prüft Verschlüsselung | `SecurityCheck` |
| `checkCertificates()` | Prüft Zertifikate | `SecurityCheck` |
| `checkTLSConfiguration()` | Prüft TLS-Konfiguration | `SecurityCheck` |

#### Netzwerk-Sicherheit

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `checkFirewall()` | Prüft Firewall | `SecurityCheck` |
| `checkOpenPorts()` | Prüft offene Ports | `SecurityCheck` |
| `checkNetworkEncryption()` | Prüft Netzwerk-Verschlüsselung | `SecurityCheck` |

#### Dateisystem-Sicherheit

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `checkFilePermissions()` | Prüft Dateiberechtigungen | `SecurityCheck` |
| `checkSensitiveFiles()` | Prüft sensible Dateien | `SecurityCheck` |
| `checkBackupSecurity()` | Prüft Backup-Sicherheit | `SecurityCheck` |

#### Konfigurations-Sicherheit

| Funktion | Beschreibung | Rückgabetyp |
|----------|--------------|--------------|
| `checkConfigSecurity()` | Prüft Konfigurations-Sicherheit | `SecurityCheck` |
| `checkEnvironmentVariables()` | Prüft Umgebungsvariablen | `SecurityCheck` |
| `checkSecretsManagement()` | Prüft Secrets-Management | `SecurityCheck` |

---

## 📚 API-Referenz

### Datenstrukturen

#### SystemDiagnosisReport

```velin
struct SystemDiagnosisReport {
    timestamp: string,              // ISO-8601 Timestamp
    overallStatus: string,          // "healthy" | "degraded" | "critical" | "unknown"
    systemInfo: SystemInfo,        // System-Informationen
    resources: ResourceStatus,      // Ressourcen-Status
    security: SecurityStatus,       // Sicherheits-Status
    services: ServiceStatusList,   // Service-Status-Liste
    logs: LogAnalysis,             // Log-Analyse
    recommendations: List<string>,  // Empfehlungen
    score: number,                  // Health-Score (0-100)
}
```

#### ResourceStatus

```velin
struct ResourceStatus {
    cpu: CPUStatus,        // CPU-Status
    memory: MemoryStatus,  // Memory-Status
    disk: DiskStatus,      // Disk-Status
    network: NetworkStatus, // Network-Status
}
```

#### CPUStatus

```velin
struct CPUStatus {
    usage: number,              // CPU-Auslastung in Prozent (0-100)
    cores: number,              // Anzahl CPU-Kerne
    loadAverage: List<number>,  // Load Average [1min, 5min, 15min]
    temperature: number,        // CPU-Temperatur in Celsius (optional)
    status: string,             // "healthy" | "warning" | "critical"
}
```

#### MemoryStatus

```velin
struct MemoryStatus {
    total: number,        // Gesamtspeicher in Bytes
    used: number,         // Verwendeter Speicher in Bytes
    free: number,         // Freier Speicher in Bytes
    cached: number,       // Gecachter Speicher in Bytes
    usagePercent: number, // Speicherauslastung in Prozent (0-100)
    status: string,       // "healthy" | "warning" | "critical"
}
```

#### DiskStatus

```velin
struct DiskStatus {
    total: number,              // Gesamtspeicherplatz in Bytes
    used: number,               // Verwendeter Speicherplatz in Bytes
    free: number,               // Freier Speicherplatz in Bytes
    usagePercent: number,       // Festplattenauslastung in Prozent (0-100)
    ioRead: number,             // Lese-IO in Bytes/s
    ioWrite: number,            // Schreib-IO in Bytes/s
    status: string,             // "healthy" | "warning" | "critical"
    partitions: List<PartitionInfo>, // Partition-Informationen
}
```

#### SecurityStatus

```velin
struct SecurityStatus {
    overallStatus: string,           // "secure" | "warning" | "vulnerable"
    checks: List<SecurityCheck>,     // Durchgeführte Sicherheits-Checks
    vulnerabilities: List<Vulnerability>, // Gefundene Vulnerabilities
    score: number,                   // Sicherheits-Score (0-100)
}
```

#### SecurityCheck

```velin
struct SecurityCheck {
    name: string,        // Name des Checks
    status: string,      // "passed" | "failed" | "warning"
    severity: string,    // "low" | "medium" | "high" | "critical"
    message: string,     // Beschreibung des Ergebnisses
    timestamp: string,   // ISO-8601 Timestamp
}
```

#### Vulnerability

```velin
struct Vulnerability {
    id: string,                    // Eindeutige ID
    name: string,                  // Name der Vulnerability
    severity: string,              // "low" | "medium" | "high" | "critical"
    description: string,           // Beschreibung
    affectedComponents: List<string>, // Betroffene Komponenten
    recommendation: string,         // Empfehlung zur Behebung
    cve: string,                   // CVE-ID (optional)
}
```

---

## 🔒 Sicherheitsprüfungen

### Übersicht der Sicherheitsprüfungen

Das System führt folgende Sicherheitsprüfungen durch:

#### 1. Authentifizierung & Autorisierung

- ✅ **JWT/OAuth2-Verwendung** - Prüft ob sichere Authentifizierung verwendet wird
- ✅ **RBAC-System** - Prüft ob Role-Based Access Control implementiert ist
- ✅ **Passwort-Richtlinien** - Prüft Mindestlänge, Komplexität, etc.
- ✅ **Session-Sicherheit** - Prüft Timeout, Secure-Cookies, HttpOnly-Cookies

#### 2. Verschlüsselung & Zertifikate

- ✅ **Verschlüsselungs-Algorithmen** - Prüft auf veraltete Algorithmen (DES, MD5)
- ✅ **Zertifikats-Gültigkeit** - Prüft abgelaufene oder bald ablaufende Zertifikate
- ✅ **TLS-Konfiguration** - Prüft TLS-Version und Cipher-Suites

#### 3. Netzwerk-Sicherheit

- ✅ **Firewall-Status** - Prüft ob Firewall aktiv ist
- ✅ **Offene Ports** - Prüft auf potenziell unsichere Ports
- ✅ **Netzwerk-Verschlüsselung** - Prüft ob alle Verbindungen verschlüsselt sind

#### 4. Dateisystem-Sicherheit

- ✅ **Dateiberechtigungen** - Prüft sensible Dateien auf unsichere Berechtigungen
- ✅ **Exponierte Dateien** - Prüft auf exponierte sensible Dateien
- ✅ **Backup-Verschlüsselung** - Prüft ob Backups verschlüsselt sind

#### 5. Konfigurations-Sicherheit

- ✅ **Konfigurations-Dateien** - Prüft auf sensible Daten in Configs
- ✅ **Umgebungsvariablen** - Prüft auf exponierte Umgebungsvariablen
- ✅ **Secrets-Management** - Prüft ob Secrets-Manager verwendet wird

#### 6. Vulnerability-Scanning

- ✅ **CVE-Scanning** - Scannt nach bekannten CVE
- ✅ **Veraltete Dependencies** - Prüft auf veraltete Abhängigkeiten
- ✅ **SQL-Injection-Risiken** - Prüft auf SQL-Injection-Schwachstellen
- ✅ **XSS-Risiken** - Prüft auf XSS-Schwachstellen

### Sicherheits-Score-Berechnung

Der Sicherheits-Score wird wie folgt berechnet:

- **Basis-Score:** 100 Punkte
- **Abzug für fehlgeschlagene Checks:**
  - Critical: -20 Punkte
  - High: -10 Punkte
  - Medium: -5 Punkte
  - Low: -2 Punkte
- **Abzug für Vulnerabilities:**
  - Critical: -15 Punkte
  - High: -8 Punkte
  - Medium: -4 Punkte
  - Low: -1 Punkt

**Gesamtstatus:**
- **secure:** Score ≥ 80, keine Critical-Issues
- **warning:** Score 50-79, oder mehrere Failed-Checks
- **vulnerable:** Score < 50, oder Critical-Issues

---

## 🧪 Tests

### Test-Suite ausführen

```velin
use system_diagnosis_test;

// Alle Tests ausführen
let results = system_diagnosis_test.runAllTests();

println("Tests: " + results.total);
println("Erfolgreich: " + results.passed);
println("Fehlgeschlagen: " + results.failed);
```

### Verfügbare Tests

#### Systemdiagnose-Tests

- ✅ `testSystemInfoCollection()` - Testet System-Informationen-Sammlung
- ✅ `testResourceChecking()` - Testet Ressourcen-Überprüfung
- ✅ `testServiceChecking()` - Testet Service-Überprüfung
- ✅ `testLogAnalysis()` - Testet Log-Analyse
- ✅ `testOverallStatusDetermination()` - Testet Gesamtstatus-Bestimmung
- ✅ `testHealthScoreCalculation()` - Testet Health-Score-Berechnung
- ✅ `testRecommendationsGeneration()` - Testet Empfehlungen-Generierung

#### Sicherheits-Tests

- ✅ `testSecurityChecks()` - Testet Sicherheitsprüfungen
- ✅ `testAuthenticationCheck()` - Testet Authentifizierungs-Check
- ✅ `testEncryptionCheck()` - Testet Verschlüsselungs-Check
- ✅ `testCertificateCheck()` - Testet Zertifikats-Check
- ✅ `testFirewallCheck()` - Testet Firewall-Check
- ✅ `testFilePermissionsCheck()` - Testet Dateiberechtigungs-Check
- ✅ `testVulnerabilityScanning()` - Testet Vulnerability-Scanning

#### Integration-Tests

- ✅ `testFullDiagnosisFlow()` - Testet vollständigen Diagnose-Flow
- ✅ `testSecurityStatusCalculation()` - Testet Sicherheitsstatus-Berechnung
- ✅ `testReportGeneration()` - Testet Report-Generierung

### Test-Ausführung

```bash
# Mit VelinScript CLI
velin test examples/system-diagnosis/tests/system_diagnosis_test.velin

# Oder direkt im Code
velin run examples/system-diagnosis/tests/system_diagnosis_test.velin
```

---

## 💡 Best Practices

### 1. Regelmäßige Diagnose

Führen Sie regelmäßig Systemdiagnosen durch:

```velin
// Als Scheduled Task
@scheduled("0 */6 * * *")  // Alle 6 Stunden
fn scheduledDiagnosis() {
    let report = system_diagnosis.runFullDiagnosis();
    
    if (report.overallStatus == "critical") {
        // Alert senden
        alert.sendCritical("Systemdiagnose: Kritischer Status!");
    }
}
```

### 2. Monitoring-Integration

Integrieren Sie die Diagnose in Ihr Monitoring-System:

```velin
@GET("/metrics")
fn getMetrics(): string {
    let report = system_diagnosis.runFullDiagnosis();
    
    // Prometheus-Format
    return metrics.exportPrometheus(report);
}
```

### 3. Logging

Loggen Sie Diagnose-Ergebnisse:

```velin
fn logDiagnosis() {
    let report = system_diagnosis.runFullDiagnosis();
    
    log.info("Systemdiagnose", {
        status: report.overallStatus,
        score: report.score,
        cpuUsage: report.resources.cpu.usage,
        memoryUsage: report.resources.memory.usagePercent,
        securityScore: report.security.score,
    });
}
```

### 4. Automatische Empfehlungen umsetzen

Implementieren Sie automatische Behebungen für häufige Probleme:

```velin
fn autoFixCommonIssues(report: SystemDiagnosisReport) {
    // Automatische Behebung für kritische Disk-Auslastung
    if (report.resources.disk.usagePercent > 90) {
        cleanupOldLogs();
        cleanupTempFiles();
    }
    
    // Automatische Behebung für Memory-Leaks
    if (report.resources.memory.usagePercent > 90) {
        restartServices();
    }
}
```

### 5. Security-First

Priorisieren Sie Sicherheitsprüfungen:

```velin
fn checkSecurityFirst() {
    let security = security_checks.runSecurityChecks();
    
    if (security.overallStatus == "vulnerable") {
        // Stoppe alle nicht-kritischen Services
        stopNonCriticalServices();
        
        // Alert an Security-Team
        securityTeam.alert(security);
    }
}
```

---

## 🔧 Troubleshooting

### Häufige Probleme

#### Problem: "CPU-Auslastung ist kritisch"

**Lösung:**
1. Prüfen Sie laufende Prozesse
2. Identifizieren Sie CPU-intensive Prozesse
3. Optimieren Sie Code oder skalierten Sie horizontal

```velin
// CPU-intensive Prozesse finden
let processes = process.getTopCPUProcesses(10);
for (proc in processes) {
    println(proc.name + ": " + proc.cpuUsage + "%");
}
```

#### Problem: "Speicherauslastung ist kritisch"

**Lösung:**
1. Prüfen Sie auf Memory-Leaks
2. Erhöhen Sie verfügbaren RAM
3. Optimieren Sie Speicherverwendung

```velin
// Memory-Leaks identifizieren
let memoryReport = system_diagnosis.checkMemory();
if (memoryReport.status == "critical") {
    // Memory-Profiling aktivieren
    profiler.startMemoryProfiling();
}
```

#### Problem: "Sicherheitsprüfung hat Schwachstellen gefunden"

**Lösung:**
1. Prüfen Sie die gefundenen Vulnerabilities
2. Setzen Sie die Empfehlungen um
3. Aktualisieren Sie Dependencies

```velin
let security = security_checks.runSecurityChecks();
for (vuln in security.vulnerabilities) {
    if (vuln.severity == "critical") {
        println("KRITISCH: " + vuln.name);
        println("Empfehlung: " + vuln.recommendation);
    }
}
```

#### Problem: "Services sind nicht gesund"

**Lösung:**
1. Prüfen Sie Service-Logs
2. Prüfen Sie Service-Konfiguration
3. Starten Sie Services neu

```velin
let services = system_diagnosis.checkServices();
for (service in services.services) {
    if (service.health != "healthy") {
        println("Service " + service.name + " ist nicht gesund");
        println("Status: " + service.status);
        println("PID: " + service.pid);
        
        // Service neu starten
        process.restartService(service.name);
    }
}
```

---

## 📖 Beispiele

### Beispiel 1: Vollständige Diagnose mit Alerting

```velin
use system_diagnosis;
use alerting;

fn fullDiagnosisWithAlerts() {
    let report = system_diagnosis.runFullDiagnosis();
    
    // Status-basierte Alerts
    if (report.overallStatus == "critical") {
        alerting.sendCritical("Systemdiagnose: Kritischer Status!", report);
    } else if (report.overallStatus == "degraded") {
        alerting.sendWarning("Systemdiagnose: Degradierter Status", report);
    }
    
    // Score-basierte Alerts
    if (report.score < 50) {
        alerting.sendCritical("Health-Score ist kritisch: " + report.score, report);
    }
    
    // Security-Alerts
    if (report.security.overallStatus == "vulnerable") {
        alerting.sendSecurityAlert("Sicherheitsprüfung: Vulnerabilities gefunden!", report.security);
    }
    
    return report;
}
```

### Beispiel 2: Ressourcen-Monitoring Dashboard

```velin
use system_diagnosis;
use json;

@GET("/dashboard/resources")
fn getResourceDashboard(): string {
    let resources = system_diagnosis.checkResources();
    
    return json.stringify({
        cpu: {
            usage: resources.cpu.usage,
            cores: resources.cpu.cores,
            status: resources.cpu.status,
            loadAverage: resources.cpu.loadAverage,
        },
        memory: {
            total: resources.memory.total,
            used: resources.memory.used,
            free: resources.memory.free,
            usagePercent: resources.memory.usagePercent,
            status: resources.memory.status,
        },
        disk: {
            total: resources.disk.total,
            used: resources.disk.used,
            free: resources.disk.free,
            usagePercent: resources.disk.usagePercent,
            status: resources.disk.status,
        },
        network: {
            latency: resources.network.latency,
            status: resources.network.status,
            totalBytesIn: resources.network.totalBytesIn,
            totalBytesOut: resources.network.totalBytesOut,
        },
    });
}
```

### Beispiel 3: Automatische Behebung

```velin
use system_diagnosis;

fn autoRemediation() {
    let report = system_diagnosis.runFullDiagnosis();
    
    // Automatische Behebung für Disk-Space
    if (report.resources.disk.usagePercent > 90) {
        println("Disk-Space kritisch! Starte automatische Bereinigung...");
        
        // Alte Logs löschen
        fs.deleteOldLogs(30); // Älter als 30 Tage
        
        // Temp-Dateien löschen
        fs.cleanupTempFiles();
        
        // Cache bereinigen
        cache.clearOldEntries(7); // Älter als 7 Tage
        
        println("Bereinigung abgeschlossen!");
    }
    
    // Automatische Behebung für Memory
    if (report.resources.memory.usagePercent > 90) {
        println("Memory kritisch! Starte automatische Optimierung...");
        
        // Services mit hohem Memory-Verbrauch neu starten
        for (service in report.services.services) {
            if (service.memoryUsage > 1000000000) { // > 1GB
                println("Starte Service " + service.name + " neu...");
                process.restartService(service.name);
            }
        }
        
        // Garbage Collection forcieren
        gc.force();
        
        println("Memory-Optimierung abgeschlossen!");
    }
    
    // Automatische Behebung für Security
    if (report.security.overallStatus == "vulnerable") {
        println("Sicherheitsprobleme gefunden! Starte automatische Behebung...");
        
        // Abgelaufene Zertifikate erneuern
        for (vuln in report.security.vulnerabilities) {
            if (vuln.name.contains("Certificate")) {
                crypto.renewCertificate(vuln.affectedComponents[0]);
            }
        }
        
        println("Sicherheitsbehebung abgeschlossen!");
    }
}
```

### Beispiel 4: Scheduled Monitoring

```velin
use system_diagnosis;
use scheduler;

// Alle 5 Minuten
@scheduled("*/5 * * * *")
fn monitorSystem() {
    let report = system_diagnosis.runFullDiagnosis();
    
    // Metriken sammeln
    metrics.record("system.health_score", report.score);
    metrics.record("system.cpu_usage", report.resources.cpu.usage);
    metrics.record("system.memory_usage", report.resources.memory.usagePercent);
    metrics.record("system.disk_usage", report.resources.disk.usagePercent);
    metrics.record("system.security_score", report.security.score);
    
    // Alerts bei kritischen Werten
    if (report.score < 50) {
        alerting.sendCritical("Health-Score kritisch: " + report.score);
    }
}

// Täglich um Mitternacht
@scheduled("0 0 * * *")
fn dailySecurityCheck() {
    let security = security_checks.runSecurityChecks();
    
    // Security-Report generieren
    let report = {
        date: DateTime.now().toDateString(),
        status: security.overallStatus,
        score: security.score,
        checks: security.checks.length,
        vulnerabilities: security.vulnerabilities.length,
    };
    
    // Report speichern
    fs.writeFile("/reports/security_" + DateTime.now().toDateString() + ".json", 
                 json.stringify(report));
    
    // Email-Report senden
    email.sendReport("security-team@example.com", report);
}
```

---

## 📊 Health-Score-Erklärung

Der Health-Score wird wie folgt berechnet:

### Gewichtung

- **Ressourcen (40%):** CPU, Memory, Disk, Network
- **Sicherheit (30%):** Alle Sicherheitsprüfungen
- **Services (20%):** Service-Status
- **Logs (10%):** Fehlerrate in Logs

### Score-Bereiche

- **90-100:** Exzellent - System läuft optimal
- **70-89:** Gut - System läuft gut, kleine Optimierungen möglich
- **50-69:** Akzeptabel - System läuft, aber Verbesserungen nötig
- **30-49:** Problematisch - System hat Probleme, sofortige Maßnahmen nötig
- **0-29:** Kritisch - System ist instabil, sofortige Intervention erforderlich

---

## 🔗 Integration mit anderen Tools

### Prometheus

```velin
@GET("/metrics")
fn getPrometheusMetrics(): string {
    let report = system_diagnosis.runFullDiagnosis();
    
    return "# HELP system_health_score System Health Score (0-100)\n" +
           "# TYPE system_health_score gauge\n" +
           "system_health_score " + report.score + "\n" +
           "# HELP system_cpu_usage CPU Usage Percentage\n" +
           "# TYPE system_cpu_usage gauge\n" +
           "system_cpu_usage " + report.resources.cpu.usage + "\n";
}
```

### Grafana

Die Diagnose-Daten können direkt in Grafana visualisiert werden:

1. Erstellen Sie eine Datenquelle (Prometheus/JSON)
2. Erstellen Sie Dashboards mit den Metriken
3. Setzen Sie Alerts basierend auf den Werten

### ELK Stack

```velin
fn sendToELK(report: SystemDiagnosisReport) {
    let elkData = {
        "@timestamp": report.timestamp,
        "system": {
            "health_score": report.score,
            "status": report.overallStatus,
            "cpu": report.resources.cpu.usage,
            "memory": report.resources.memory.usagePercent,
            "disk": report.resources.disk.usagePercent,
            "security_score": report.security.score,
        },
    };
    
    http.post("http://elk:9200/system-diagnosis/_doc", json.stringify(elkData));
}
```

---

## 📝 Changelog

### Version 1.0.0 (2024)

- ✅ Initiale Implementierung
- ✅ Vollständige Systemdiagnose
- ✅ Umfassende Sicherheitsprüfungen
- ✅ Vollständige Test-Suite
- ✅ Vollständige Dokumentation

---

## 🤝 Beitragen

Beiträge sind willkommen! Bitte:

1. Forken Sie das Repository
2. Erstellen Sie einen Feature-Branch
3. Schreiben Sie Tests für neue Features
4. Stellen Sie einen Pull Request

---

## 📄 Lizenz

Dieses Modul ist Teil von VelinScript und unter der MIT-Lizenz lizenziert.

---

## 🆘 Support

Bei Fragen oder Problemen:

- 📧 Email: support@velinscript.com
- 💬 Forum: https://forum.velinscript.com
- 🐛 Issues: https://github.com/velinscript/velinscript/issues

---

**VelinScript Systemdiagnose** - Vollständige Systemüberwachung und Sicherheitsprüfung in einer Lösung.
