# Framework-Integration in VelinScript

**Version:** 3.0.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

VelinScript unterstützt automatische Framework-Erkennung und Code-Generierung für moderne HTTP-Frameworks in **Rust**, **PHP** und **Python**.

## Unterstützte Frameworks

### Rust
- **Axum** (Default): Moderne, async-first Architektur.
- **Actix-Web**: High-Performance Framework.

### PHP
- **Laravel** (Empfohlen): Generiert Controller-Klassen und `Route::get` Definitionen.
- **Symfony**: Generiert Controller mit `#[Route]` Attributen.

### Python
- **FastAPI** (Empfohlen): Generiert Pydantic-Modelle und Async-Handler.
- **Flask**: Generiert Standard Flask-Routen und View-Functions.

### TypeScript
- **Express** (Default): Generiert Router, Request-Handler und Interfaces.
- **NestJS**: Generiert Controller (`@Controller`), Module und DTOs.

### Java
- **Spring Boot**: Generiert RestController (`@RestController`), RequestMappings und Services.

### C#
- **ASP.NET Core**: Generiert Controller (`Microsoft.AspNetCore.Mvc`), Attributes (`[HttpGet]`) und Models.

### Go
- **Gin** (Empfohlen): High-Performance HTTP Web Framework. Generiert Struct-Tags für JSON und Gin-Handler.

## Framework-Erkennung

Der Compiler erkennt das Framework auf drei Arten:

### 1. Config-basiert

```json
// velin.config.json
{
  "target": "go",
  "framework": "gin"
}
```

### 2. Decorator-basiert

```velin
@Gin
@GET("/api/users")
fn getUsers(): List<User> {
    // ...
}
```

### 3. Default

- **Rust**: Axum
- **PHP**: Laravel
- **Python**: FastAPI
- **TypeScript**: Express
- **Java**: Spring Boot
- **C#**: ASP.NET Core
- **Go**: Gin

## Code-Generierung

### Automatische Imports

Der Compiler generiert automatisch Framework-spezifische Imports für alle unterstützten Sprachen.

**Laravel (PHP):**
```php
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\Controller;
```

**FastAPI (Python):**
```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
```

**Gin (Go):**
```go
import (
    "github.com/gin-gonic/gin"
    "net/http"
    "strconv"
)
```

**Express (TypeScript):**
```typescript
import express, { Request, Response } from 'express';
```

**Spring Boot (Java):**
```java
import org.springframework.web.bind.annotation.*;
import org.springframework.http.ResponseEntity;
import java.util.*;
```

**ASP.NET Core (C#):**
```csharp
using Microsoft.AspNetCore.Mvc;
using System.Collections.Generic;
```

### Router-Initialisierung

**Laravel:**
Generiert Routen am Ende der Datei, die auf Controller-Methoden verweisen:
```php
Route::get('/api/users', [AppController::class, 'get_users']);
```

**Symfony:**
Nutzt Attribute direkt an den Methoden:
```php
#[Route('/api/users', methods: ['GET'])]
public function get_users() { ... }
```

**FastAPI:**
```python
app.add_api_route("/api/users", get_users, methods=["GET"])
```

**Gin:**
```go
func main() {
    r := gin.Default()
    r.GET("/api/users", getUsersHandler)
    r.Run()
}
```

**Express:**
```typescript
const app = express();
app.get("/api/users", getUsers);
```

**NestJS:**
```typescript
@Controller("/api/users")
export class UsersController {
    @Get()
    getUsers() { ... }
}
```

**Spring Boot:**
```java
@RestController
@RequestMapping("/api/users")
public class UsersController {
    @GetMapping
    public List<User> getUsers() { ... }
}
```

**ASP.NET Core:**
```csharp
[ApiController]
[Route("/api/users")]
public class UsersController : ControllerBase {
    [HttpGet]
    public ActionResult<List<User>> GetUsers() { ... }
}
```

### Handler-Signaturen

Der Compiler generiert Framework-spezifische Handler-Signaturen:

**Axum:**
```rust
async fn get_users_handler() -> impl IntoResponse {
    // ...
}
```

**Actix-Web:**
```rust
async fn get_users_handler(req: HttpRequest) -> impl Responder {
    // ...
}
```

**Gin:**
```go
func getUsersHandler(c *gin.Context) {
    // ...
}
```

## Parameter-Extraktion

### Path-Parameter

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    // ...
}
```

**Axum:**
```rust
async fn get_user_handler(Path(id): Path<String>) -> impl IntoResponse {
    // ...
}
```

**Gin:**
```go
func getUserHandler(c *gin.Context) {
    id := c.Param("id")
    // ...
}
```

### Query-Parameter

```velin
@GET("/api/users")
fn getUsers(limit: number): List<User> {
    // ...
}
```

**Axum:**
```rust
async fn get_users_handler(Query(params): Query<GetUsersParams>) -> impl IntoResponse {
    // ...
}
```

**Gin:**
```go
func getUsersHandler(c *gin.Context) {
    limitStr := c.Query("limit")
    limit, _ := strconv.ParseFloat(limitStr, 64)
    // ...
}
```

### Body-Parameter

```velin
@POST("/api/users")
fn createUser(user: UserInput): User {
    // ...
}
```

**Axum:**
```rust
async fn create_user_handler(Json(user): Json<UserInput>) -> impl IntoResponse {
    // ...
}
```

**Gin:**
```go
func createUserHandler(c *gin.Context) {
    var user UserInput
    if err := c.ShouldBindJSON(&user); err != nil {
        c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
        return
    }
    // ...
}
```

## Error Handling

### Global Error Handler

Der Compiler generiert automatisch einen globalen Error Handler:

```rust
struct AppError(anyhow::Error);

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // Konvertiert Fehler in saubere JSON-Responses
        // Status Code: 500 für interne Fehler
    }
}
```

### Result-Typen

```velin
@GET("/api/users/:id")
fn getUser(id: string): Result<User, string> {
    // ...
}
```

Der Compiler konvertiert automatisch:
- `Ok(value)` → 200 OK mit JSON-Body
- `Error(message)` → 400 Bad Request oder 500 Internal Server Error

## Middleware-Integration

### Auth Middleware

```velin
@Auth
@GET("/api/profile")
fn getProfile(): User {
    // ...
}
```

Der Compiler generiert automatisch Auth-Middleware-Integration für beide Frameworks.

### Rate Limiting

```velin
@RateLimit(requests: 100, window: "1m")
@GET("/api/users")
fn getUsers(): List<User> {
    // ...
}
```

## Observability

### Structured Logging

Alle Handler werden automatisch mit `#[tracing::instrument]` versehen:

```rust
#[tracing::instrument]
async fn get_users_handler() -> impl IntoResponse {
    // Automatisches Logging:
    // - Request-ID
    // - Latenz
    // - Parameter
}
```

## Implementierung

**Datei:** `compiler/src/codegen/framework.rs`

**Features:**
- `FrameworkSelector::detect_framework()` - Framework-Erkennung
- `FrameworkSelector::generate_imports()` - Import-Generierung
- `FrameworkSelector::generate_app_init()` - Router/App-Initialisierung
- `FrameworkSelector::generate_handler_signature()` - Handler-Signaturen

---

**Letzte Aktualisierung:** 2026-02-01  
**Version:** 3.0.0
