# API Documentation - Standard Library

Die VelinScript Standard Library bietet vordefinierte Funktionen für häufige Aufgaben.

## Database Funktionen

### db.find

Findet ein Entity anhand der ID.

```velin
let user = db.find(User, "123");
```

**Signatur:**
```velin
fn<T>(T, string) -> Optional<T>
```

**Parameter:**
- `Entity` (Type) - Der Entity-Typ
- `id` (string) - Die ID

**Rückgabewert:**
- `Optional<T>` - Das gefundene Entity oder null

**Transformiert zu:**
```rust
db.find::<User>("123").await
```

### db.findAll

Findet alle Entities eines Typs.

```velin
let users = db.findAll(User);
```

**Signatur:**
```velin
fn<T>(T) -> List<T>
```

**Parameter:**
- `Entity` (Type) - Der Entity-Typ

**Rückgabewert:**
- `List<T>` - Liste aller Entities

**Transformiert zu:**
```rust
db.find_all::<User>().await
```

### db.save

Speichert ein Entity.

```velin
let savedUser = db.save(user);
```

**Signatur:**
```velin
fn<T>(T) -> T
```

**Parameter:**
- `entity` (T) - Das Entity zum Speichern

**Rückgabewert:**
- `T` - Das gespeicherte Entity

**Transformiert zu:**
```rust
db.save(user).await
```

### db.delete

Löscht ein Entity.

```velin
let deleted = db.delete(User, "123");
```

**Signatur:**
```velin
fn<T>(T, string) -> boolean
```

**Parameter:**
- `Entity` (Type) - Der Entity-Typ
- `id` (string) - Die ID

**Rückgabewert:**
- `boolean` - true wenn gelöscht, false wenn nicht gefunden

**Transformiert zu:**
```rust
db.delete::<User>("123").await
```

## Assert Funktionen

### assert

Prüft eine Bedingung.

```velin
assert(condition);
assert(value == expected);
```

**Signatur:**
```velin
fn(boolean) -> void
```

**Parameter:**
- `condition` (boolean) - Die zu prüfende Bedingung

**Transformiert zu:**
- `assert!(condition)` - Einfache Bedingung
- `assert_eq!(left, right)` - Wenn `==` verwendet wird
- `assert_ne!(left, right)` - Wenn `!=` verwendet wird

**Beispiele:**

```velin
@test
fn testAddition() {
    let result = add(2, 3);
    assert(result == 5);  // Wird zu assert_eq!(result, 5)
    assert(result != 0); // Wird zu assert_ne!(result, 0)
    assert(result > 0);  // Wird zu assert!(result > 0)
}
```

## String Funktionen

### String Concatenation

```velin
let greeting = "Hello, " + name + "!";
```

### String Interpolation

Format-Strings ermöglichen die Interpolation von Ausdrücken:

```velin
let name = "John";
let message = "Hello, {name}!";
// Ergebnis: "Hello, John!"

let x = 10;
let y = 20;
let result = "Sum: {x + y}";
// Ergebnis: "Sum: 30"
```

**Syntax:**
- Format-Strings verwenden geschweifte Klammern `{}` für Interpolation
- Beliebige Ausdrücke können innerhalb der Klammern verwendet werden
- Escaping: `\{` für literal `{`, `\}` für literal `}`

**Kompilierung:**
Format-Strings werden zu Rust `format!` Macros kompiliert.

## Collection Funktionen

### List Operations

```velin
let users = db.findAll(User);
let firstUser = users[0];        // Index Access
let count = users.length;        // Length (geplant)
```

### Map Operations

```velin
let map = Map<string, number>();
map["key"] = 42;                 // Set
let value = map["key"];          // Get
```

## Type Conversion

### Explizite Konvertierung (geplant)

```velin
let number = stringToNumber("42");
let string = numberToString(42);
```

## Utility Funktionen

### generateId

Generiert eine eindeutige ID.

```velin
let id = generateId();
```

**Signatur:**
```velin
fn() -> string
```

### currentUser

Gibt den aktuell authentifizierten Benutzer zurück.

```velin
@Auth
@GET("/api/profile")
fn getProfile(): User {
    return currentUser();
}
```

**Signatur:**
```velin
fn() -> User
```

**Voraussetzung:**
- Funktion muss mit `@Auth` Decorator markiert sein

## Error Handling

### Error Types (geplant)

```velin
enum Error {
    NotFound(message: string),
    ValidationError(field: string, message: string),
    ServerError(message: string),
}
```

### Result Type (geplant)

```velin
type Result<T, E> = Ok(value: T) | Error(err: E);
```

## Async/Await

### Async Functions

```velin
async fn fetchData(url: string): Data {
    // Async operation
    return await http.get(url);
}
```

### Await

```velin
let data = await fetchData("https://api.example.com/data");
```

## HTTP Funktionen

### HttpRequest

HTTP Request Handling.

```velin
let request = HttpRequest::new("GET", "/api/users");
request.header("Authorization", "Bearer token");
let auth = request.get_header("Authorization");
```

### HttpResponse

HTTP Response Handling.

```velin
let response = HttpResponse::ok("Success");
let error = HttpResponse::bad_request("Invalid input");
let notFound = HttpResponse::not_found("Resource not found");
```

## Validation Funktionen

### Validator

Input Validation Framework.

```velin
let mut validator = Validator::new();
validator
    .required("name", &name)
    .min_length("name", &name, 3)
    .max_length("name", &name, 50)
    .email("email", &email);

if (!validator.is_valid()) {
    for error in validator.errors() {
        println!("{}: {}", error.field, error.message);
    }
}
```

**Methoden:**
- `required(field, value)` - Prüft ob Feld vorhanden ist
- `min_length(field, value, min)` - Minimale Länge
- `max_length(field, value, max)` - Maximale Länge
- `email(field, value)` - E-Mail-Validierung
- `pattern(field, value, pattern, message)` - Pattern-Matching

## Authentication Funktionen

### AuthService

JWT Token Management.

```velin
let auth = AuthService::new("secret-key");
let claims = UserClaims {
    user_id: "123",
    email: "user@example.com",
    roles: ["user", "admin"],
};
let token = auth.generate_token(claims);
let verified = auth.verify_token(&token.token);
```

### OAuth2Provider

OAuth2 Integration.

```velin
let oauth = OAuth2Provider::new(
    "client-id",
    "client-secret",
    "https://redirect.uri"
);
let auth_url = oauth.get_authorization_url("state");
let token = oauth.exchange_code("code");
```

## ML/LLM Funktionen

### ModelLoader

ML Model Loading & Prediction.

```velin
let mut loader = ModelLoader::new();
loader.load_model("sentiment", ModelType::Sentiment, "model.onnx");
let prediction = loader.predict("sentiment", "This is great!");
```

### LLMClient

Large Language Model Integration.

```velin
let llm = LLMClient::new(LLMProvider::OpenAI, "api-key");
let response = llm.generate("What is AI?");
let embeddings = llm.embed("Hello, world!");
```

### VectorDB

Vector Database Support.

```velin
let db = VectorDB::new(VectorDBProvider::Pinecone, "connection-string");
db.upsert("collection", "id", [0.1, 0.2, 0.3]);
let results = db.search("collection", [0.1, 0.2, 0.3], 10);
```

### TrainingService

Model Training mit ONNX Runtime und TensorFlow.

```velin
let mut training = TrainingService::new();
training.add_example("input", "output");

// Basis-Training
training.train("model-name");

// ONNX Training
let onnxConfig = ONNXTrainingConfig {
    epochs: 100,
    batch_size: 32,
    learning_rate: 0.001,
    optimizer: "Adam",
    loss_function: "CrossEntropy"
};
let result = training.train_with_onnx("model", onnxConfig);

// TensorFlow Training
let tfConfig = TensorFlowTrainingConfig {
    epochs: 100,
    batch_size: 32,
    learning_rate: 0.001,
    optimizer: "Adam",
    loss_function: "SparseCategoricalCrossentropy",
    validation_split: 0.2
};
let result = training.train_with_tensorflow("model", tfConfig);

// Model Evaluation
let testData = [TrainingExample { input: "test", output: "expected" }];
let evalResult = training.evaluate_model("model", testData);
```

**Features:**
- ONNX Runtime Integration
- TensorFlow Integration
- Hyperparameter Tuning
- Model Evaluation & Metrics
- Automatic Logging (VelinLogger)
- Metrics Collection

## Best Practices

1. **Immer Typen angeben** für öffentliche APIs
2. **Type Inference nutzen** für lokale Variablen
3. **Decorators verwenden** für API-Endpoints
4. **Error Handling** implementieren
5. **Tests schreiben** mit `@test`
6. **Input Validation** für alle User-Inputs
7. **Authentication** für geschützte Endpoints
8. **Performance** mit Optimizer verbessern