# VelinScript Systemdiagnose

Vollständiges Systemdiagnose-System mit Sicherheitsprüfungen, Tests und Dokumentation.

## 📁 Struktur

```
system-diagnosis/
├── system_diagnosis.velin      # Hauptmodul für Systemdiagnose
├── security_checks.velin       # Sicherheitsprüfungen
├── tests/
│   └── system_diagnosis_test.velin  # Vollständige Test-Suite
└── README.md                   # Diese Datei
```

## 🚀 Schnellstart

```velin
use system_diagnosis;

// Vollständige Diagnose durchführen
let report = system_diagnosis.runFullDiagnosis();
println("Status: " + report.overallStatus);
println("Score: " + report.score);
```

## 📚 Dokumentation

Vollständige Dokumentation: [docs/system-diagnose.md](../../docs/system-diagnose.md)

## 🧪 Tests ausführen

```bash
velin test tests/system_diagnosis_test.velin
```

## ✨ Features

- ✅ Vollständige Systemressourcen-Überprüfung
- ✅ Umfassende Sicherheitsprüfungen
- ✅ Service-Status-Überwachung
- ✅ Log-Analyse
- ✅ Automatische Empfehlungen
- ✅ Health-Score-Berechnung
- ✅ Vollständige Test-Suite
