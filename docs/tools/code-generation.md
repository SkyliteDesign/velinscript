# Code-Generierung mit VelinScript

VelinScript verfügt über einen leistungsstarken Code-Generator (`velin generate`), der entwickelt wurde, um wiederkehrende Aufgaben zu automatisieren und Best Practices von Anfang an zu erzwingen. Anstatt Boilerplate-Code manuell zu schreiben, können Sie vollständige Module, Tests und Konfigurationen mit einem einzigen Befehl erstellen.

## Wofür ist Code Generation ideal?

Code Generation ist ideal für:
- ✅ **Boilerplate-Reduktion** - Generiert CRUD-Module, APIs und Auth-Setup
- ✅ **Schnelles Prototyping** - Erstellt vollständige Module mit einem Befehl
- ✅ **Best Practices** - Erzwingt Best Practices von Anfang an
- ✅ **Client-Generierung** - Generiert TypeScript/Python Clients aus OpenAPI
- ✅ **Projekt-Initialisierung** - Erstellt Projekt-Struktur und Konfiguration
- ✅ **Konsistenz** - Stellt konsistente Code-Struktur sicher

## Wofür ist Code Generation NICHT gedacht?

Code Generation ist NICHT gedacht für:
- ❌ **Komplexe, spezifische Logik** - Für einmalige, komplexe Implementierungen
- ❌ **Code-Qualität** - Für Code-Qualitätsprüfung nutzen Sie den Linter
- ❌ **Security-Checks** - Für Security nutzen Sie den Security Scanner
- ❌ **Performance-Analyse** - Für Performance nutzen Sie den Profiler
- ❌ **Code-Optimierung** - Generierter Code muss manuell optimiert werden

---

## Inhaltsverzeichnis

1.  [Einführung](#1-einführung)
2.  [API & CRUD Generierung](#2-api--crud-generierung)
    *   [REST-Endpunkte generieren](#rest-endpunkte-generieren)
    *   [Vollständige CRUD-Module](#vollständige-crud-module)
3.  [Infrastruktur-Module](#3-infrastruktur-module)
    *   [Security & Auth](#security--auth)
    *   [Logging & Monitoring](#logging--monitoring)
    *   [Caching](#caching)
4.  [Client-Generierung](#4-client-generierung)
5.  [Projekt-Initialisierung](#5-projekt-initialisierung)
6.  [Templates anpassen](#6-templates-anpassen)

---

## 1. Einführung

Der Generator ist direkt in die CLI integriert:

```bash
velin generate <typ> [optionen]
```

Er erzeugt nicht nur leere Dateien, sondern voll funktionsfähigen, typisierten VelinScript-Code, der sofort kompiliert werden kann.

**Verfügbare Generatoren:**
*   `api`: Einzelne Controller
*   `crud`: Datenbank-Modelle + Controller + Service
*   `test`: Unit- und Integrationstests
*   `client`: TypeScript/Rust Clients aus OpenAPI
*   `security`: Auth-Setup
*   `logging`: Logger-Konfiguration
*   `cache`: Redis/Memory Cache Setup

---

## 2. API & CRUD Generierung

### REST-Endpunkte generieren

Erstellt einen neuen Controller mit Basis-Routen.

```bash
velin generate api --name Products --path /api/v1/products
```

**Generierter Output (`src/controllers/products.velin`):**
```velin
use http
use services

@Controller("/api/v1/products")
struct ProductsController {
    
    @GET("/")
    fn list(): List<any> {
        return [];
    }

    @GET("/:id")
    fn get(@Path("id") id: string): any {
        return null;
    }
    
    @POST("/")
    fn create(@Body data: any): any {
        return data;
    }
}
```

### Vollständige CRUD-Module

Der `crud`-Generator ist mächtiger. Er erstellt Model, Repository, Service und Controller in einem Zug.

```bash
velin generate crud --name User --fields "id:string,email:string,age:number,active:boolean"
```

**Generiert:**
1.  `src/models/user.velin`: Entity-Definition mit `@Validate`
2.  `src/controllers/user.velin`: REST-Controller
3.  `src/services/user.velin`: Business-Logik

**Beispiel Model (`src/models/user.velin`):**
```velin
@Entity(table: "users")
struct User {
    @Id
    id: string,
    
    @Validate(email: true)
    email: string,
    
    @Validate(min: 0)
    age: number,
    
    active: boolean
}
```

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Code Generator                             │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin generate crud --name User                      │
│                                                         │
│  ⚡ Generiere CRUD-Modul...                            │
│                                                         │
│  ✓ src/models/user.velin erstellt                      │
│  ✓ src/controllers/user.velin erstellt                 │
│  ✓ src/services/user.velin erstellt                   │
│  ✓ src/repositories/user.velin erstellt                 │
│                                                         │
│  📦 Vollständiges CRUD-Modul generiert!                │
│     - GET    /api/users                                 │
│     - GET    /api/users/:id                            │
│     - POST   /api/users                                │
│     - PUT    /api/users/:id                            │
│     - DELETE /api/users/:id                            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## 3. Infrastruktur-Module

VelinScript hilft Ihnen, "Production-Ready" zu starten.

### Security & Auth

Generiert ein komplettes Authentifizierungs-Modul mit JWT-Support.

```bash
velin generate security
```

**Features:**
*   `AuthService` für Login/Register
*   Passwort-Hashing (Argon2)
*   JWT-Strategie
*   Middleware für `@Auth`

### Logging & Monitoring

```bash
velin generate logging
```

Erstellt eine zentrale Logger-Konfiguration, die JSON-Logs für ELK/Splunk formatiert und Request-IDs durchreicht.

### Caching

```bash
velin generate cache --provider redis
```

Erstellt einen `CacheService`, der das `utils.cache`-Interface implementiert und Redis als Backend nutzt.

---

## 4. Client-Generierung

Wenn Sie eine API gebaut haben, wollen Sie diese oft in einem Frontend (React, Vue, Angular) konsumieren.

**Schritt 1: OpenAPI generieren**
```bash
velin open-api -i main.velin -o openapi.json
```

**Schritt 2: Client generieren**
```bash
velin generate client --openapi openapi.json --language typescript --output ./frontend/src/api
```

Der generierte Client ist **vollständig typisiert** und nutzt `fetch` oder `axios`.

**Verwendung im Frontend:**
```typescript
import { ApiClient } from "./api";

const client = new ApiClient({ baseUrl: "http://localhost:8080" });

// Vollständige Autovervollständigung!
const users = await client.users.list();
```

---

## 5. Projekt-Initialisierung

Der Befehl `velin init` legt das Fundament.

```bash
velin init my-new-project
```

**Struktur:**
```text
my-new-project/
├── velin.toml          # Projekt-Konfiguration
├── .gitignore
├── README.md
└── src/
    ├── main.velin      # Einstiegspunkt
    ├── models/
    ├── controllers/
    └── services/
```

Es wird automatisch eine `velin.toml` mit Standard-Dependencies (http-server, test-utils) erstellt.

---

## 6. Templates anpassen

Sie sind nicht auf die Standard-Templates beschränkt. VelinScript sucht im Ordner `.velin/templates` Ihres Projekts nach eigenen `.velin`-Vorlagen.

**Beispiel Custom Template (`.velin/templates/api.velin`):**
```velin
// Mein Firmen-Standard-Controller
use http
use shared::monitoring

@Controller("{{path}}")
struct {{name}}Controller {
    @GET("/ping")
    fn ping(): string {
        return "pong";
    }
}
```

Wenn Sie nun `velin generate api` ausführen, wird Ihr eigenes Template verwendet.
