# Backup & Rollback Strategien

VelinScript bietet integrierte Werkzeuge für Datensicherung und Versionskontrolle Ihrer Deployments. Diese Tools sind entscheidend für den "Day 2 Operations"-Betrieb, um bei Fehlern schnell reagieren zu können.

Das Backup-System ist zweigeteilt:
1.  **Daten-Backup:** Sichert Datenbanken und persistente Dateien.
2.  **Code-Rollback:** Ermöglicht das Zurückspringen auf vorherige Versionen der Anwendung.

## Wofür sind Backup & Rollback ideal?

Backup & Rollback sind ideal für:
- ✅ **Production-Sicherheit** - Schnelle Wiederherstellung bei Fehlern
- ✅ **Deployment-Rollback** - Zurückspringen auf vorherige Versionen
- ✅ **Daten-Sicherung** - Automatische Datenbank-Backups
- ✅ **Disaster Recovery** - Wiederherstellung nach Systemfehlern
- ✅ **Version-Management** - Verwaltung von Deployment-Versionen
- ✅ **Transaktionale Deployments** - Atomare Deployment-Operationen

## Wofür sind Backup & Rollback NICHT gedacht?

Backup & Rollback sind NICHT gedacht für:
- ❌ **Code-Versionierung** - Für Code-Versionierung nutzen Sie Git
- ❌ **Code-Qualität** - Für Code-Qualität nutzen Sie den Linter
- ❌ **Development** - Primär für Production-Umgebungen
- ❌ **Performance-Analyse** - Für Performance nutzen Sie den Profiler
- ❌ **Security-Checks** - Für Security nutzen Sie den Security Scanner

---

## Inhaltsverzeichnis

1.  [Daten-Backups (`velin backup`)](#1-daten-backups-velin-backup)
    *   [Backup erstellen](#backup-erstellen)
    *   [Backup wiederherstellen](#backup-wiederherstellen)
    *   [Verifizierung](#verifizierung)
2.  [Deployment Rollbacks (`velin rollback`)](#2-deployment-rollbacks-velin-rollback)
    *   [Versionen und Snapshots](#versionen-und-snapshots)
    *   [Rollback durchführen](#rollback-durchführen)
    *   [Transaktionale Deployments](#transaktionale-deployments)
3.  [Best Practices](#3-best-practices)

---

## 1. Daten-Backups (`velin backup`)

Der `velin backup`-Befehl ist ein einheitliches Interface für verschiedene Backup-Ziele. Er abstrahiert die spezifischen Befehle der Datenbanken (Postgres, MySQL).

### Backup erstellen

Erstellt einen Schnappschuss der konfigurierten Datenquellen.

```bash
# Vollständiges Backup (Standard)
velin backup create --destination ./backups

# Inkrementelles Backup (Nur Änderungen seit letztem Full)
velin backup create --strategy incremental
```

**Konfiguration (`velin.toml`):**
```toml
[backup]
include = ["db", "storage/uploads"]
compression = "gzip"
retention_days = 30
```

### Backup wiederherstellen

Im Katastrophenfall ("Disaster Recovery") können Sie den Zustand zu einem bestimmten Zeitpunkt wiederherstellen.

```bash
# Liste aller Backups anzeigen
velin backup list

# ID: 20231027-1400-full
velin backup restore 20231027-1400-full
```

**Warnung:** Ein Restore überschreibt in der Regel die aktuellen Daten. VelinScript fragt standardmäßig nach Bestätigung.

### Verifizierung

Ein Backup ist nutzlos, wenn es korrupt ist. Der `verify`-Befehl prüft die Integrität der Archive.

```bash
velin backup verify 20231027-1400-full
```

Dies führt Prüfsummen-Checks durch und testet optional, ob das Backup in eine temporäre Datenbank eingespielt werden kann.

---

## 2. Deployment Rollbacks (`velin rollback`)

Wenn ein neues Deployment Fehler verursacht, müssen Sie sofort zurück zur letzten funktionierenden Version.

### Versionen und Snapshots

VelinScript unterstützt das Konzept von "Versionen". Jedes Mal, wenn Sie `velin compile` für ein Release nutzen, kann eine Version markiert werden.

```bash
velin rollback create-version "v1.2.0 - Feature X Release"
```

Ein **Snapshot** umfasst zusätzlich zur Code-Version auch die Konfiguration zum Zeitpunkt des Releases.

```bash
velin rollback create-snapshot "Pre-Upgrade Snapshot"
```

### Rollback durchführen

Wenn Sie feststellen, dass v1.2.0 Fehler wirft:

```bash
velin rollback to-version v1.1.9
```

VelinScript stoppt den aktuellen Prozess, tauscht die Binaries/Skripte aus und startet den Dienst neu.

### Transaktionale Deployments

Für kritische Updates können Sie eine "Transaktion" starten.

```bash
velin rollback begin
# ... deployment schritte ...
# ... tests laufen lassen ...

if [ $TESTS_PASSED ]; then
    velin rollback commit
else
    velin rollback rollback
fi
```

Wenn `commit` nicht innerhalb eines Timeouts aufgerufen wird (z.B. weil der Server abgestürzt ist), führt VelinScript automatisch ein Rollback durch ("Auto-Rollback").

---

## 3. Best Practices

1.  **Regelmäßige Tests:** Testen Sie Ihre Backups! Führen Sie einmal im Monat einen `velin backup verify` und einen echten Restore auf einem Testsystem durch.
2.  **Offsite Backups:** Speichern Sie Backups nicht auf demselben Server. Nutzen Sie Plugins für S3 oder FTP.
3.  **Automatisierung:** Integrieren Sie `backup create` in einen Cronjob.
4.  **Database Migrations:** Seien Sie vorsichtig bei Rollbacks, wenn Datenbank-Schema-Änderungen im Spiel sind. VelinScript kann Code zurückrollen, aber Datenbank-Migrationen müssen oft manuell rückgängig gemacht werden (VelinScript bietet hierfür `db migrate down` Integration).

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Backup & Rollback                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin backup create                                  │
│                                                         │
│  💾 Erstelle Backup...                                  │
│  ✓ Datenbank-Backup erstellt                           │
│  ✓ Datei-Backup erstellt                               │
│  ✓ Backup komprimiert (gzip)                           │
│  ✓ Backup-ID: 20260130-1430-full                       │
│                                                         │
│  $ velin rollback list                                  │
│                                                         │
│  📋 Verfügbare Versionen:                               │
│    v1.2.0 - Feature X Release                          │
│    v1.1.9 - Bugfix Release                              │
│    v1.1.8 - Initial Release                             │
│                                                         │
│  $ velin rollback to-version v1.1.9                     │
│  ✓ Rollback zu v1.1.9 durchgeführt                     │
│                                                         │
└─────────────────────────────────────────────────────────┘
```
