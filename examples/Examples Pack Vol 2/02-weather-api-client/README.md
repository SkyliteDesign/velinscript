# Wetter API Client

## 🌤️ Beschreibung

Ein Wetter-Client, der externe APIs aufruft und Wetterdaten verarbeitet. Dieses Beispiel zeigt:

- HTTP-Requests an externe APIs
- JSON-Datenverarbeitung
- Datenkonvertierung und Formatierung
- String-Interpolation in mehrzeiligen Strings
- Vergleichslogik
- Umgebungsvariablen

## 🎯 Lernziele

- Externe APIs aufrufen mit `http.get()`
- JSON-Daten parsen und verarbeiten
- Datenkonvertierung (Kelvin → Celsius)
- Formatierte Ausgaben erstellen
- Fehlerbehandlung bei API-Aufrufen
- Arbeiten mit Umgebungsvariablen

## 🔑 Vorbereitung

1. Registriere dich bei [OpenWeatherMap](https://openweathermap.org/api) für einen kostenlosen API-Key
2. Setze die Umgebungsvariable:
   ```bash
   export WEATHER_API_KEY="dein_api_key_hier"
   ```

## 🚀 Verwendung

### Aktuelles Wetter abrufen
```bash
GET /api/weather/Berlin
```

Gibt strukturierte Wetterdaten zurück:
```json
{
    "city": "Berlin",
    "temperature": 15.5,
    "feelsLike": 14.2,
    "humidity": 65,
    "description": "partly cloudy",
    "windSpeed": 12.5,
    "timestamp": "2026-01-28 14:30:00"
}
```

### 5-Tages-Vorhersage
```bash
GET /api/weather/Berlin/forecast
```

### Komplette Wetterdaten (Aktuell + Vorhersage)
```bash
GET /api/weather/Berlin/complete
```

### Formatierte lesbare Ausgabe
```bash
GET /api/weather/Berlin/formatted
```

Beispielausgabe:
```
☁️ Wetter in Berlin
🌡️  Temperatur: 15.5°C
🤚 Gefühlt wie: 14.2°C
💧 Luftfeuchtigkeit: 65%
💨 Windgeschwindigkeit: 12.5 km/h
📝 Beschreibung: partly cloudy
```

### Wetter zwischen Städten vergleichen
```bash
GET /api/weather/compare/Berlin/München
```

## 💡 Wichtige Konzepte

1. **HTTP-Requests**: `http.get(url)` für externe API-Aufrufe
2. **JSON-Parsing**: `.json()` zum Parsen der Response
3. **Umgebungsvariablen**: `env("VARIABLE_NAME")` für sichere API-Keys
4. **Datenkonvertierung**: Temperatur von Kelvin zu Celsius
5. **String-Interpolation**: `{variable}` in Strings
6. **Bedingte Logik**: `if`-Ausdrücke für Emoji-Auswahl

## 🔧 Erweiterungsmöglichkeiten

- Historische Wetterdaten speichern
- Push-Benachrichtigungen bei Wetteränderungen
- Wetteralarme bei extremen Bedingungen
- Pollenflug-Informationen hinzufügen
- UV-Index und Sonnenauf-/untergangszeiten
- Mehrere Wetter-APIs kombinieren
