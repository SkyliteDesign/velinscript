# 03-automation-pipeline

**Dein Spezialgebiet.**

## Zweck

- zeigt Autonomie
- zeigt Entscheidungslogik
- zeigt echten Mehrwert

## Use-Case

1. Eingangsdaten prüfen
2. bewerten
3. Aktion auslösen

## Architektur

```
Eingang → Validierung → Bewertung → Entscheidung → Aktion
```

## Kompilieren

```bash
cd examples/03-automation-pipeline
velin compile -i main.velin -o main.rs
```

## Ausführen

```bash
cargo run --release
```

Die API läuft dann auf `http://localhost:8080`

## Testen

### Einzelne Verarbeitung

```bash
curl -X POST http://localhost:8080/api/process \
  -H "Content-Type: application/json" \
  -d '{
    "id": "item-1",
    "value": 85,
    "category": "critical",
    "priority": "high"
  }'
```

**Response:**
```json
{
  "status": "success",
  "score": 135,
  "action": "gold",
  "message": "High priority item - immediate action required"
}
```

### Batch-Verarbeitung

```bash
curl -X POST http://localhost:8080/api/process/batch \
  -H "Content-Type: application/json" \
  -d '[
    {"id": "item-1", "value": 85, "category": "critical", "priority": "high"},
    {"id": "item-2", "value": 60, "category": "important", "priority": "medium"},
    {"id": "item-3", "value": 30, "category": "normal", "priority": "low"}
  ]'
```

## Regeln

Die Pipeline verwendet folgende Bewertungslogik:

1. **Basis-Score**: Direkt aus `value` (0-100)
2. **Kategorie-Bonus**:
   - `critical`: +30
   - `important`: +15
   - `normal`: +0
3. **Priority-Bonus**:
   - `high`: +20
   - `medium`: +10
   - `low`: +0

**Entscheidungslogik:**
- Score > 80 → `gold` (sofortige Aktion, Benachrichtigung)
- Score > 50 → `silver` (Verarbeitung innerhalb 24h)
- Score ≤ 50 → `bronze` (Verarbeitung wenn verfügbar)

## Pipeline-Denken

Dieses Beispiel zeigt:

1. **Modulare Funktionen**: Jede Funktion hat eine klare Aufgabe
2. **Entscheidungslogik**: Klare Regeln für verschiedene Szenarien
3. **Erweiterbarkeit**: Einfach neue Regeln oder Aktionen hinzufügen

## Erweiterungsmöglichkeiten

- **KI-Integration**: Nutze LLM für intelligente Bewertung
- **Datenbank**: Speichere Verarbeitungsergebnisse
- **Webhooks**: Sende Benachrichtigungen an externe Systeme
- **Retry-Logik**: Automatische Wiederholung bei Fehlern
- **Monitoring**: Metriken und Logging für Production

## Was du lernst

- Wie man Entscheidungslogik strukturiert
- Wie man Pipeline-Patterns implementiert
- Wie man autonome Systeme baut
- Wie man Code erweiterbar hält

## Nächste Schritte

- **Volles System?** → Siehe `04-custom-recommender`
