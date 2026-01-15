# WebAssembly (WASM) Support

VelinScript kann zu WebAssembly kompiliert werden für Browser und Edge Computing.

## Kompilierung

### Zu WASM kompilieren

```bash
# Kompiliere zu WebAssembly
velin compile --target wasm32-unknown-unknown -i main.velin -o main.wasm

# Mit Optimierungen
velin compile --target wasm32-unknown-unknown --optimize size -i main.velin -o main.wasm
```

## Konfiguration

### velin.config.json

```json
{
  "wasm": {
    "enabled": true,
    "target": "wasm32-unknown-unknown",
    "optimization": "size",
    "features": ["std", "web"]
  }
}
```

## Verwendung im Browser

### HTML

```html
<!DOCTYPE html>
<html>
<head>
    <title>VelinScript WASM Demo</title>
</head>
<body>
    <script type="module">
        import init, { wasmGetRecommendations, wasmGenerateEmbedding } from './main.js';
        
        async function run() {
            // Initialisiere WASM-Modul
            await init();
            
            // Verwende WASM-Funktionen
            const recommendations = wasmGetRecommendations('user123', 10);
            console.log(recommendations);
            
            const embedding = wasmGenerateEmbedding('Hello, world!');
            console.log(embedding);
        }
        
        run();
    </script>
</body>
</html>
```

### JavaScript Integration

```javascript
// main.js (generiert von wasm-pack)
import init, { wasmGetRecommendations } from './pkg/main.js';

async function initWASM() {
    await init();
    
    // Jetzt können WASM-Funktionen verwendet werden
    const result = wasmGetRecommendations('user123', 10);
    return JSON.parse(result);
}
```

## WASM-Export in VelinScript

### Funktionen exportieren

```velin
// @WASMExport - Exportiert Funktion für Browser
@WASMExport
fn wasmGetRecommendations(userId: string, limit: number): string {
    let recommendations = hybridRecommend(
        userId,
        generateUserEmbedding(userId, allItems, allPreferences),
        allItems,
        allPreferences
    );
    
    return JSON.stringify(recommendations);
}
```

## Performance

### Vorteile

- **Schnell** - Nahezu native Performance im Browser
- **Kompakt** - Kleine Bundle-Größen
- **Sicher** - Sandboxed Execution
- **Portabel** - Läuft überall wo WASM unterstützt wird

### Optimierungen

```bash
# Size-Optimierung (für kleinere Bundles)
velin compile --target wasm32-unknown-unknown --optimize size

# Speed-Optimierung (für bessere Performance)
velin compile --target wasm32-unknown-unknown --optimize speed
```

## Edge Computing

### Cloudflare Workers

```javascript
// cloudflare-worker.js
import { wasmGetRecommendations } from './main.wasm';

export default {
    async fetch(request) {
        const userId = new URL(request.url).searchParams.get('userId');
        const recommendations = wasmGetRecommendations(userId, 10);
        return new Response(recommendations, {
            headers: { 'Content-Type': 'application/json' }
        });
    }
};
```

### Vercel Edge Functions

```javascript
// api/recommendations.js
import { wasmGetRecommendations } from './main.wasm';

export const config = {
    runtime: 'edge'
};

export default async function handler(req) {
    const { userId } = req.query;
    const recommendations = wasmGetRecommendations(userId, 10);
    return new Response(recommendations, {
        headers: { 'Content-Type': 'application/json' }
    });
}
```

## Best Practices

1. **Minimize WASM Size** - Nutze `--optimize size` für kleinere Bundles
2. **Lazy Loading** - Lade WASM-Module nur wenn benötigt
3. **Error Handling** - Behandle WASM-Fehler gracefully
4. **Memory Management** - Verwende WASM Memory API für große Daten
5. **Testing** - Teste WASM-Module in verschiedenen Browsern

## Browser-Support

- Chrome/Edge: ✅ Vollständig unterstützt
- Firefox: ✅ Vollständig unterstützt
- Safari: ✅ Vollständig unterstützt (ab Version 11)
- Opera: ✅ Vollständig unterstützt

## Beispiele

Siehe `wasm.velin` für vollständige WASM-Implementierung.
