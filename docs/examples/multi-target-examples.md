# Multi-Target Compilation Beispiele

**Version:** 3.1.0  
**Datum:** 2026-01-30

---

## Übersicht

Diese Datei zeigt Beispiele für die Kompilierung des gleichen VelinScript-Codes zu verschiedenen Zielsprachen.

## Beispiel 1: Einfache API-Funktion

### VelinScript Source

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}

struct User {
    id: string,
    name: string,
    email: string,
}
```

### Rust Output

```rust
use axum::{Router, routing::get, extract::Path, Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

async fn get_user(Path(id): Path<String>) -> Json<User> {
    let user = db.find::<User>(&id).await;
    Json(user)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/users/:id", get(get_user));
    // ...
}
```

### PHP Output

```php
<?php
declare(strict_types=1);

class User {
    public function __construct(
        public string $id,
        public string $name,
        public string $email
    ) {}
}

function getUser(string $id): User {
    return db::find(User::class, $id);
}
```

### Python Output

```python
from dataclasses import dataclass
from typing import Optional
from fastapi import FastAPI, Path

@dataclass
class User:
    id: str
    name: str
    email: str

app = FastAPI()

@app.get("/api/users/{id}")
async def get_user(id: str) -> User:
    return db.find(User, id)
```

### TypeScript Output

```typescript
interface User {
    id: string;
    name: string;
    email: string;
}

async function getUser(id: string): Promise<User> {
    return await db.find(User, id);
}
```

### Go Output

```go
package main

type User struct {
    ID    string `json:"id"`
    Name  string `json:"name"`
    Email string `json:"email"`
}

func GetUser(id string) (User, error) {
    return db.Find(User{}, id)
}
```

### Java Output

```java
public class User {
    private String id;
    private String name;
    private String email;
    
    public User(String id, String name, String email) {
        this.id = id;
        this.name = name;
        this.email = email;
    }
    
    // Getters and Setters...
}

@GetMapping("/api/users/{id}")
public User getUser(@PathVariable String id) {
    return db.find(User.class, id);
}
```

### C# Output

```csharp
namespace VelinApp;

public class User
{
    public string Id { get; set; }
    public string Name { get; set; }
    public string Email { get; set; }
}

[HttpGet("/api/users/{id}")]
public User GetUser(string id)
{
    return db.Find<User>(id);
}
```

---

## Beispiel 2: Async-Funktion mit LLM

### VelinScript Source

```velin
@POST("/api/analyze")
async fn analyzeText(text: string): string {
    return await @llm.analyze(text);
}
```

### Rust Output

```rust
use axum::{Router, routing::post, Json};

async fn analyze_text(Json(text): Json<String>) -> Json<String> {
    let client = LLMClient::new("openai");
    let result = client.analyze(&text).await;
    Json(result)
}
```

### Python Output

```python
from fastapi import FastAPI, Body
from llm_client import LLMClient

app = FastAPI()

@app.post("/api/analyze")
async def analyze_text(text: str = Body(...)) -> str:
    client = LLMClient("openai")
    return await client.analyze(text)
```

### TypeScript Output

```typescript
import { LLMClient } from './llm-client';

async function analyzeText(text: string): Promise<string> {
    const client = new LLMClient('openai');
    return await client.analyze(text);
}
```

---

## Beispiel 3: Collections

### VelinScript Source

```velin
fn processNumbers(numbers: List<number>): number {
    return numbers
        .filter((x) => x > 0)
        .map((x) => x * 2)
        .reduce((acc, x) => acc + x, 0);
}
```

### Rust Output

```rust
fn process_numbers(numbers: Vec<f64>) -> f64 {
    numbers
        .iter()
        .filter(|x| **x > 0.0)
        .map(|x| x * 2.0)
        .sum()
}
```

### Python Output

```python
def process_numbers(numbers: List[float]) -> float:
    return sum(x * 2 for x in numbers if x > 0)
```

### TypeScript Output

```typescript
function processNumbers(numbers: number[]): number {
    return numbers
        .filter(x => x > 0)
        .map(x => x * 2)
        .reduce((acc, x) => acc + x, 0);
}
```

### Go Output

```go
func ProcessNumbers(numbers []float64) float64 {
    sum := 0.0
    for _, x := range numbers {
        if x > 0 {
            sum += x * 2
        }
    }
    return sum
}
```

---

## Beispiel 4: Pattern Matching

### VelinScript Source

```velin
fn handleStatus(status: Status): string {
    match (status) {
        Status::Pending => "Waiting",
        Status::Active => "Running",
        Status::Inactive => "Stopped",
        _ => "Unknown"
    }
}
```

### Rust Output

```rust
fn handle_status(status: Status) -> String {
    match status {
        Status::Pending => "Waiting".to_string(),
        Status::Active => "Running".to_string(),
        Status::Inactive => "Stopped".to_string(),
    }
}
```

### Python Output

```python
def handle_status(status: Status) -> str:
    match status:
        case Status.PENDING:
            return "Waiting"
        case Status.ACTIVE:
            return "Running"
        case Status.INACTIVE:
            return "Stopped"
        case _:
            return "Unknown"
```

### TypeScript Output

```typescript
function handleStatus(status: Status): string {
    switch (status) {
        case Status.Pending:
            return "Waiting";
        case Status.Active:
            return "Running";
        case Status.Inactive:
            return "Stopped";
        default:
            return "Unknown";
    }
}
```

---

## Vergleich: Typ-Mapping

| VelinScript | Rust | PHP | Python | TypeScript | Go | Java | C# |
|-------------|------|-----|--------|------------|----|----|-----|
| `string` | `String` | `string` | `str` | `string` | `string` | `String` | `string` |
| `number` | `f64` | `float` | `float` | `number` | `float64` | `double` | `double` |
| `List<T>` | `Vec<T>` | `array` | `List[T]` | `T[]` | `[]T` | `List<T>` | `List<T>` |
| `Map<K,V>` | `HashMap<K,V>` | `array` | `Dict[K,V]` | `Map<K,V>` | `map[K]V` | `Map<K,V>` | `Dictionary<K,V>` |

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
