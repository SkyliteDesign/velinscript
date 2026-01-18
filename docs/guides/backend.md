# APIs & Backend Entwicklung mit VelinScript

VelinScript wurde entwickelt, um die Erstellung moderner, skalierbarer Backend-Services so einfach wie möglich zu machen, ohne dabei Kompromisse bei der Performance einzugehen. Durch die tiefe Integration von Web-Server, Datenbankzugriff und Validierung in die Sprache selbst können Sie mit extrem wenig Boilerplate-Code produktive Anwendungen schreiben.

Dieses Handbuch führt Sie durch alle Aspekte der Backend-Entwicklung: von der Erstellung Ihrer ersten REST-API über komplexe Datenbankmodellierung bis hin zu Performance-Optimierung und Deployment.

---

## Inhaltsverzeichnis

1.  [REST APIs erstellen](#1-rest-apis-erstellen)
    *   [Controller und Routing](#controller-und-routing)
    *   [Request-Handling (Body, Query, Path)](#request-handling)
    *   [Antworten und Statuscodes](#antworten-und-statuscodes)
2.  [Validierung](#2-validierung)
    *   [Deklarative Validierung](#deklarative-validierung)
    *   [Benutzerdefinierte Validatoren](#benutzerdefinierte-validatoren)
3.  [Datenbanken und ORM](#3-datenbanken-und-orm)
    *   [Modelle definieren](#modelle-definieren)
    *   [CRUD Operationen](#crud-operationen)
    *   [Beziehungen (Relations)](#beziehungen-relations)
    *   [Transaktionen](#transaktionen)
4.  [Performance und Monitoring](#4-performance-und-monitoring)
    *   [PerformanceMonitor](#performancemonitor)
    *   [Caching Strategien](#caching-strategien)
    *   [Metriken sammeln](#metriken-sammeln)
5.  [Fehlerbehandlung im API-Kontext](#5-fehlerbehandlung-im-api-kontext)

---

## 1. REST APIs erstellen

Das `http`-Modul von VelinScript ist das Herzstück Ihrer Webanwendung. Es nutzt ein Decorator-basiertes System, das stark von Frameworks wie NestJS oder Spring Boot inspiriert ist, aber dank der Kompilierung zu Rust extrem performant ist.

### Controller und Routing

Ein Controller ist eine Struct, die zusammengehörige Endpunkte gruppiert. Der `@Controller`-Decorator definiert den Basispfad.

```velin
use http
use services

@Controller("/api/v1/products")
struct ProductController {
    
    // Dieser Endpunkt ist unter GET /api/v1/products erreichbar
    @GET("/")
    fn listAll(): List<Product> {
        return db.findAll(Product);
    }

    // Verschachtelte Pfade: GET /api/v1/products/featured
    @GET("/featured")
    fn listFeatured(): List<Product> {
        return db.findMany(Product, { "isFeatured": true });
    }
}
```

### Request-Handling

Um auf Daten des HTTP-Requests zuzugreifen, nutzen Sie Parameter-Decorators. VelinScript übernimmt das Parsen und die Typkonvertierung automatisch.

#### Body (`@Body`)
Liest den JSON-Body des Requests und mappt ihn auf ein Struct.

```velin
@POST("/")
fn createProduct(@Body payload: CreateProductDto): Product {
    // payload ist hier bereits ein typisiertes Objekt!
    // Wenn das JSON ungültig ist, wird automatisch 400 Bad Request zurückgegeben.
    return productService.create(payload);
}
```

#### Pfad-Parameter (`@Path`)
Extrahiert variable Teile aus der URL.

```velin
// Route: /api/v1/products/:id
@GET("/:id")
fn getProduct(@Path("id") productId: string): Product {
    let product = db.find(Product, productId);
    if (!product) {
        throw HttpError.NotFound("Produkt nicht gefunden");
    }
    return product;
}
```

#### Query-Parameter (`@Query`)
Liest Parameter aus dem Query-String (z.B. `?limit=10&sort=desc`).

```velin
@GET("/")
fn search(
    @Query("q") searchTerm: string,
    @Query("limit") limit: number = 20, // Default-Wert
    @Query("offset") offset: number = 0
): List<Product> {
    return db.search(Product, searchTerm, limit, offset);
}
```

#### Header (`@Header`)
Liest HTTP-Header, z.B. für Authentifizierung oder Metadaten.

```velin
@GET("/secure-data")
fn getSecureData(@Header("X-API-Key") apiKey: string): string {
    // ...
}
```

### Antworten und Statuscodes

Standardmäßig sendet VelinScript Status `200 OK` (oder `201 Created` bei POST) und serialisiert den Rückgabewert als JSON. Sie können dies aber anpassen.

```velin
use http

@POST("/")
@Status(201) // Expliziter Statuscode
fn create(@Body p: Product): Product {
    return db.save(p);
}

// Manuelle Response-Steuerung
@GET("/download")
fn downloadPdf(): HttpResponse {
    let pdfData = generatePdf();
    
    return HttpResponse.builder()
        .status(200)
        .header("Content-Type", "application/pdf")
        .header("Content-Disposition", "attachment; filename=invoice.pdf")
        .body(pdfData);
}
```

---

## 2. Validierung

In VelinScript ist Validierung keine "Afterthought", sondern tief integriert. Anstatt Validierungslogik in Ihren Controllern zu schreiben, definieren Sie Regeln direkt an Ihren Datenmodellen.

### Deklarative Validierung

Nutzen Sie den `@Validate`-Decorator an Struct-Feldern.

```velin
struct UserRegistration {
    @Validate(min: 3, max: 20, message: "Benutzername muss zwischen 3 und 20 Zeichen lang sein")
    username: string,

    @Validate(email: true, message: "Ungültige E-Mail-Adresse")
    email: string,

    @Validate(min: 18)
    age: number,
    
    @Validate(pattern: "^[A-Z0-9]+$") // Regex für alphanumerische Codes
    referralCode: string?, // Optionales Feld
    
    @Validate(custom: "passwordComplexity") // Custom Validator
    password: string
}
```

Wenn dieses Struct als `@Body` in einem Controller verwendet wird, führt VelinScript die Validierung **automatisch** durch, *bevor* Ihre Funktion aufgerufen wird.

**Verfügbare Regeln:**
*   `min`, `max`: Für Zahlen (Wert) oder Strings (Länge).
*   `email`: Prüft auf gültiges E-Mail-Format.
*   `url`: Prüft auf gültige URL.
*   `uuid`: Prüft auf UUID-Format.
*   `pattern`: Prüft gegen einen regulären Ausdruck.
*   `notEmpty`: Darf nicht leer sein (für Listen/Strings).

### Benutzerdefinierte Validatoren

Sie können eigene Validierungslogik registrieren.

```velin
use validation

// Validator registrieren
validation.register("passwordComplexity", |value| {
    if (value.length < 8) return false;
    if (!value.match(/[A-Z]/)) return false; // Braucht Großbuchstaben
    if (!value.match(/[0-9]/)) return false; // Braucht Zahl
    return true;
});
```

---

## 3. Datenbanken und ORM

Das `db`-Modul (basierend auf SeaORM) ermöglicht typisierten Datenbankzugriff. Es unterstützt PostgreSQL, MySQL und SQLite.

### Modelle definieren

Modelle sind normale Structs mit `@Entity`-Annotationen.

```velin
@Entity(table: "users")
struct User {
    @Id
    @Generated // Auto-Increment oder UUID
    id: string,

    @Column(unique: true)
    email: string,
    
    name: string,
    
    @Column(name: "created_at")
    createdAt: string,
    
    isActive: boolean
}
```

### CRUD Operationen

Das `db`-Objekt bietet intuitive Methoden für Standard-Operationen.

**Erstellen (Create):**
```velin
let u = User { 
    id: utils.uuid(), 
    email: "test@example.com", 
    name: "Test", 
    createdAt: datetime.now(), 
    isActive: true 
};
db.save(u); // Insert oder Update
```

**Lesen (Read):**
```velin
// Nach ID
let user = db.find(User, "uuid-123");

// Mit Bedingungen
let activeUsers = db.findMany(User, { 
    "isActive": true 
});

// Komplexe Queries (Fluent API)
let recentUsers = db.query(User)
    .filter("createdAt", ">", "2023-01-01")
    .orderBy("name", "ASC")
    .limit(10)
    .all();
```

**Aktualisieren (Update):**
```velin
user.name = "Neuer Name";
db.save(user); // Erkennt existierende ID und macht Update
```

**Löschen (Delete):**
```velin
db.delete(User, "uuid-123");
```

### Transaktionen

Transaktionen sind essentiell für Datenkonsistenz.

```velin
try {
    db.transaction(|tx| {
        // Alle Operationen auf 'tx' statt 'db' ausführen
        let order = tx.save(newOrder);
        
        // Inventar verringern
        let product = tx.find(Product, order.productId);
        product.stock -= order.quantity;
        tx.save(product);
        
        // Zahlung verarbeiten
        paymentService.charge(order.total);
    });
} catch (e) {
    log.error("Transaktion fehlgeschlagen, Rollback durchgeführt: " + e.message);
}
```

---

## 4. Performance und Monitoring

VelinScript hilft Ihnen, Flaschenhälse zu identifizieren und zu beheben.

### PerformanceMonitor

Messen Sie granular, wie lange bestimmte Abschnitte Ihres Codes dauern.

```velin
use metrics

let monitor = PerformanceMonitor.new();

fn processOrder(order: Order) {
    monitor.start("validate_stock");
    inventory.check(order);
    monitor.stop("validate_stock");
    
    monitor.start("payment");
    payment.process(order);
    monitor.stop("payment");
    
    log.info(monitor.report()); // Gibt JSON-Bericht zurück
}
```

### Caching Strategien

Verwenden Sie den `@Cache`-Decorator, um teure Berechnungen oder Datenbankabfragen zu cachen.

```velin
@Controller("/api/stats")
struct StatsController {

    // Cache-Ergebnis für 5 Minuten
    // Cache-Key wird automatisch aus Parametern generiert
    @GET("/daily")
    @Cache(ttl: "5m")
    fn getDailyStats(): Stats {
        // Teure DB Aggregation...
        return db.query(Stats).aggregate(...);
    }
}
```

Für manuelles Caching nutzen Sie das `utils`-Modul:
```velin
let result = utils.cache("my_cache_key", "10m", || {
    return fetchRemoteData();
});
```

### Metriken sammeln

Der `MetricsCollector` integriert sich nahtlos mit Prometheus.

```velin
let metrics = MetricsCollector.new();

// In Middleware oder Interceptor
metrics.increment("http_requests_total", { 
    "method": req.method, 
    "status": res.status 
});

// Histogramm für Latenz
let timer = metrics.startTimer("http_request_duration_seconds");
// ... request verarbeiten ...
timer.observe();
```

---

## 5. Fehlerbehandlung im API-Kontext

VelinScript mappt Exceptions automatisch auf sinnvolle HTTP-Statuscodes.

*   `throw HttpError.NotFound("User nicht gefunden")` -> 404
*   `throw HttpError.Forbidden("Kein Zugriff")` -> 403
*   `throw Error("Oops")` -> 500

Sie können globale Exception-Filter definieren, um Fehler einheitlich zu formatieren.

```velin
// Globaler Error Handler (Pseudocode)
app.onError(|error, req, res| {
    log.error(error);
    res.status(500).json({
        "error": true,
        "message": "Ein interner Fehler ist aufgetreten",
        "requestId": req.id
    });
});
```

---

*Ende des Backend-Guides. Für Informationen zur Absicherung Ihrer API lesen Sie den [Sicherheits-Guide](security.md).*
