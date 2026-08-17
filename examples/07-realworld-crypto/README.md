# 07-realworld-crypto

**Kleines Realworld-Projekt mit Krypto-Portfolio.**

## Zweck

- Zeigt, wie man reale Daten (Krypto) in VelinScript kapselt
- Demonstriert GET- und POST-APIs
- Zeigt personalisierte Portfolio-Berechnung
- Enthält Portfolio-Presets für typische Strategien

## Endpoints

- `GET /api/crypto/btc`  
  Liefert einen `CryptoPrice` für BTC mit Beispielmarktdaten.

- `GET /api/crypto/portfolio`  
  Liefert ein Beispiel-Portfolio (`PortfolioOverview`) mit festen Mengen.

- `POST /api/crypto/portfolio/custom`  
  Berechnet dein persönliches Portfolio aus deinen Mengen.

- `POST /api/crypto/portfolio/from-preset`  
  Nimmt ein Preset (`presetId`) und ein Budget (`totalUsd`) und berechnet
  daraus ein Portfolio.

- `GET /api/crypto/portfolio/preset/conservative`  
  Liefert ein konservatives BTC-fokussiertes Preset.

- `GET /api/crypto/portfolio/preset/balanced`  
  Liefert ein ausgewogenes Preset.

- `GET /api/crypto/portfolio/preset/growth`  
  Liefert ein wachstumsorientiertes Preset.

## Schemas

### CryptoPrice

```json
{
  "symbol": "BTC",
  "priceUsd": 43000.15,
  "change24hPercent": -2.31,
  "marketCapUsd": 845000000000
}
```

### PortfolioRequest (POST Body)

```json
{
  "btcAmount": 0.5,
  "ethAmount": 5.0,
  "solAmount": 80.0
}
```

### PortfolioOverview (Response)

```json
{
  "btcAmount": 0.5,
  "btcValueUsd": 21500.075,
  "ethAmount": 5.0,
  "ethValueUsd": 11502.5,
  "solAmount": 80.0,
  "solValueUsd": 7616.0,
  "totalValueUsd": 40618.575
}
```

Die konkreten Zahlen hängen von den im Code hinterlegten Beispielpreisen ab.

### PortfolioFromPresetRequest (POST Body)

```json
{
  "presetId": "balanced",
  "totalUsd": 10000.0
}
```

Die Logik:

- Preset wird über `presetId` gewählt (`conservative`, `balanced`, `growth`).
- Budget wird nach Shares aufgeteilt:
  - `btcBudget = totalUsd * btcShare`
  - `ethBudget = totalUsd * ethShare`
  - `solBudget = totalUsd * solShare`
- Daraus werden Coin-Mengen abgeleitet:
  - `btcAmount = btcBudget / btcPrice`
  - usw.

Die Antwort ist wieder ein `PortfolioOverview`.

## Portfolio-Presets (Schema)

Für Presets gibt es den Typ `PortfolioPreset`:

```json
{
  "id": "balanced",
  "name": "Ausgewogen",
  "description": "40% BTC, 40% ETH, 20% SOL",
  "btcShare": 0.4,
  "ethShare": 0.4,
  "solShare": 0.2
}
```

Aktuell vorhandene Presets:

- `conservative`
  - 70% BTC, 20% ETH, 10% SOL
- `balanced`
  - 40% BTC, 40% ETH, 20% SOL
- `growth`
  - 20% BTC, 50% ETH, 30% SOL

Diese Presets kannst du clientseitig mit einem Budget kombinieren, z. B.:

- Gesamtbudget `totalUsd`
- Beitrag für BTC: `totalUsd * btcShare`
- Beitrag für ETH: `totalUsd * ethShare`
- Beitrag für SOL: `totalUsd * solShare`

Die resultierenden Coin-Mengen kannst du dann an  
`POST /api/crypto/portfolio/custom` schicken.
