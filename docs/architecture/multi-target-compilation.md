# Multi-Target Compilation - VelinScript 3.1.0

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert  
**Datum:** 2026-01-30

---

## Übersicht

VelinScript 3.1.0 unterstützt die Kompilierung zu verschiedenen Zielsprachen. Dies ermöglicht die Nutzung von VelinScript in bestehenden Projekten und Umgebungen, ohne die gesamte Codebase zu migrieren.

## Architektur

### CodeGenerator Trait

Alle Code-Generatoren implementieren das `CodeGenerator` Trait:

```rust
pub trait CodeGenerator {
    fn generate(&mut self, program: &Program, config: &CodegenConfig) -> Result<String>;
    fn get_target_language(&self) -> TargetLanguage;
}
```

### TargetLanguage Enum

Der Compiler unterstützt folgende Zielsprachen:

```rust
pub enum TargetLanguage {
    Rust,        // Default
    Php,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    CSharp,
}
```

### Dynamische Dispatch

Der `CodegenPass` wählt automatisch den passenden Generator basierend auf der `--target` Option:

```rust
match config.target {
    TargetLanguage::Rust => Box::new(RustCodeGenerator::new()),
    TargetLanguage::Php => Box::new(PhpCodeGenerator::new()),
    TargetLanguage::Python => Box::new(PythonCodeGenerator::new()),
    // ...
}
```

---

## Unterstützte Targets

### 1. Rust (Default)

**Status:** ✅ Vollständig implementiert

**Features:**
- High-Performance, Type-Safe
- Nutzt Axum/Tokio Stack
- Volle Feature-Unterstützung
- Async/Await Support
- Pattern Matching
- Ownership & Borrowing

**Framework-Unterstützung:**
- Axum (Default)
- Actix-Web

**Beispiel:**
```bash
velin compile -i main.velin --target rust
# oder (Default)
velin compile -i main.velin
```

**Generierter Code:**
```rust
use axum::{Router, routing::get, Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/users", get(get_users));
    // ...
}
```

---

### 2. PHP

**Status:** ✅ Vollständig implementiert

**Features:**
- Generiert PHP 8.2+ Code
- Strict Types (`declare(strict_types=1)`)
- PSR-konformer Code
- Klassen und Funktionen
- Type Hints

**Typ-Mapping:**
- `string` → `string`
- `number` → `float`
- `boolean` → `bool`
- `List<T>` → `array`
- `Map<K,V>` → `array`

**Beispiel:**
```bash
velin compile -i main.velin --target php
```

**Generierter Code:**
```php
<?php
declare(strict_types=1);

class User {
    public function __construct(
        public string $id,
        public string $name
    ) {}
}

function getUsers(): array {
    return [];
}
```

---

### 3. Python

**Status:** ✅ Vollständig implementiert

**Features:**
- Generiert Python 3.10+ Code
- Type Hints (`typing.*`)
- Data Classes für Structs
- Async/Await Support
- Module und Imports

**Framework-Unterstützung:**
- FastAPI
- Flask

**Typ-Mapping:**
- `string` → `str`
- `number` → `float`
- `boolean` → `bool`
- `List<T>` → `List[T]`
- `Map<K,V>` → `Dict[K, V]`

**Beispiel:**
```bash
velin compile -i main.velin --target python
```

**Generierter Code:**
```python
#!/usr/bin/env python3
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class User:
    id: str
    name: str

async def get_users() -> List[User]:
    return []
```

---

### 4. TypeScript

**Status:** ✅ Vollständig implementiert (Phase 3)

**Features:**
- Generiert TypeScript 5.0+ Code
- Interfaces und Klassen
- Generics (`List<T>` → `T[]`)
- Async/Await Support
- Type Safety

**Typ-Mapping:**
- `string` → `string`
- `number` → `number`
- `boolean` → `boolean`
- `List<T>` → `T[]`
- `Map<K,V>` → `Map<K, V>` oder `Record<K, V>`

**Beispiel:**
```bash
velin compile -i main.velin --target typescript
# oder
velin compile -i main.velin --target ts
```

**Generierter Code:**
```typescript
interface User {
    id: string;
    name: string;
}

async function getUsers(): Promise<User[]> {
    return [];
}
```

---

### 5. JavaScript

**Status:** ✅ Vollständig implementiert (3.1.0)

**Features:**
- Generiert modernen JavaScript Code (ES2020+)
- Async/Await Support
- Classes und Functions
- Module System (ES Modules)
- Framework-Support: Express, NestJS (Neu in 3.1.0)
- Automatische Route-Generierung für Express

**Beispiel:**
```bash
velin compile -i main.velin --target javascript
# oder
velin compile -i main.velin --target js
```

**Generierter Code:**
```javascript
class User {
    constructor(id, name) {
        this.id = id;
        this.name = name;
    }
}

async function getUsers() {
    return [];
}
```

---

### 6. Go

**Status:** ✅ Vollständig implementiert

**Features:**
- Generiert Go 1.20+ Code
- Structs und Interfaces
- Error Handling (`error` Interface)
- Goroutines für Async

**Typ-Mapping:**
- `string` → `string`
- `number` → `float64`
- `boolean` → `bool`
- `List<T>` → `[]T`
- `Map<K,V>` → `map[K]V`

**Beispiel:**
```bash
velin compile -i main.velin --target go
# oder
velin compile -i main.velin --target golang
```

**Generierter Code:**
```go
package main

type User struct {
    ID   string
    Name string
}

func GetUsers() ([]User, error) {
    return []User{}, nil
}
```

---

### 7. Java

**Status:** ✅ Vollständig implementiert (Phase 4)

**Features:**
- Generiert Java 17+ Code
- Spring Boot kompatibel
- POJOs mit Gettern/Settern
- Lombok Support (optional)
- Generics

**Typ-Mapping:**
- `string` → `String`
- `number` → `double`
- `boolean` → `boolean`
- `List<T>` → `java.util.List<T>`
- `Map<K,V>` → `java.util.Map<K, V>`

**Beispiel:**
```bash
velin compile -i main.velin --target java
```

**Generierter Code:**
```java
public class User {
    private String id;
    private String name;
    
    public User(String id, String name) {
        this.id = id;
        this.name = name;
    }
    
    public String getId() { return id; }
    public String getName() { return name; }
}
```

---

### 8. C#

**Status:** ✅ Vollständig implementiert (Phase 4)

**Features:**
- Generiert C# 10+ Code
- ASP.NET Core kompatibel
- File-Scoped Namespaces
- `new()` Syntax
- PascalCase-Konventionen

**Typ-Mapping:**
- `string` → `string`
- `number` → `double`
- `boolean` → `bool`
- `List<T>` → `List<T>`
- `Map<K,V>` → `Dictionary<K, V>`

**Beispiel:**
```bash
velin compile -i main.velin --target csharp
# oder
velin compile -i main.velin --target cs
```

**Generierter Code:**
```csharp
namespace VelinApp;

public class User
{
    public string Id { get; set; }
    public string Name { get; set; }
    
    public User(string id, string name)
    {
        Id = id;
        Name = name;
    }
}
```

---

## CLI Nutzung

### Basis-Kompilierung

```bash
# Rust (Default)
velin compile -i main.velin

# PHP
velin compile -i main.velin --target php

# Python
velin compile -i main.velin --target python

# TypeScript
velin compile -i main.velin --target typescript

# JavaScript
velin compile -i main.velin --target javascript

# Go
velin compile -i main.velin --target go

# Java
velin compile -i main.velin --target java

# C#
velin compile -i main.velin --target csharp
```

### Mit Framework-Auswahl

```bash
# Python mit FastAPI
velin compile -i main.velin --target python --framework fastapi

# PHP mit Laravel
velin compile -i main.velin --target php --framework laravel
```

---

## Typ-Mapping Übersicht

| VelinScript | Rust | PHP | Python | TypeScript | JavaScript | Go | Java | C# |
|-------------|------|-----|--------|------------|------------|----|----|-----|
| `string` | `String` | `string` | `str` | `string` | `string` | `string` | `String` | `string` |
| `number` | `f64` | `float` | `float` | `number` | `number` | `float64` | `double` | `double` |
| `boolean` | `bool` | `bool` | `bool` | `boolean` | `boolean` | `bool` | `boolean` | `bool` |
| `List<T>` | `Vec<T>` | `array` | `List[T]` | `T[]` | `Array` | `[]T` | `List<T>` | `List<T>` |
| `Map<K,V>` | `HashMap<K,V>` | `array` | `Dict[K,V]` | `Map<K,V>` | `Map` | `map[K]V` | `Map<K,V>` | `Dictionary<K,V>` |
| `Result<T,E>` | `Result<T,E>` | Exception | `Result[T,E]` | Custom | Custom | `(T, error)` | Custom | Custom |

---

## Feature-Unterstützung

| Feature | Rust | PHP | Python | TS | JS | Go | Java | C# |
|---------|------|-----|--------|----|----|----|----|-----|
| Structs | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Enums | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Functions | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Async/Await | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Pattern Matching | ✅ | ⚠️ | ⚠️ | ⚠️ | ❌ | ⚠️ | ⚠️ | ⚠️ |
| Generics | ✅ | ⚠️ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ |
| Decorators | ✅ | ⚠️ | ⚠️ | ✅ | ❌ | ❌ | ✅ | ✅ |
| Traits/Interfaces | ✅ | ⚠️ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ |

**Legende:**
- ✅ Vollständig unterstützt
- ⚠️ Teilweise unterstützt (mit Einschränkungen)
- ❌ Nicht unterstützt

---

## IR-basierte Code-Generierung

Der IR Code Generator unterstützt Multi-Target Code-Generierung:

```rust
let generator = IRCodeGenerator::new(TargetLanguage::Rust);
let code = generator.generate(&ir_module)?;
```

**Unterstützte Targets im IR Generator:**
- Rust ✅
- PHP ✅
- Python ✅
- Andere Targets nutzen Rust als Fallback

---

## Best Practices

### 1. Target-Auswahl

- **Rust**: Für maximale Performance und Type Safety
- **PHP**: Für bestehende PHP-Projekte
- **Python**: Für Data Science und ML-Projekte
- **TypeScript**: Für Frontend-Integration
- **JavaScript**: Für Browser-Kompatibilität
- **Go**: Für Microservices
- **Java**: Für Enterprise-Anwendungen
- **C#**: Für .NET-Integration

### 2. Framework-Integration

Wähle das passende Framework für dein Target:
- Rust: Axum oder Actix-Web
- Python: FastAPI oder Flask
- PHP: Laravel oder Symfony
- Java: Spring Boot
- C#: ASP.NET Core

### 3. Typ-Kompatibilität

Achte auf Typ-Mapping-Unterschiede:
- `List<T>` wird unterschiedlich gemappt
- `Map<K,V>` hat verschiedene Implementierungen
- `Result<T,E>` wird unterschiedlich behandelt

---

## Implementierung

**Dateien:**
- `compiler/src/codegen/traits.rs` - CodeGenerator Trait und TargetLanguage Enum
- `compiler/src/codegen/rust.rs` - Rust Generator
- `compiler/src/codegen/php.rs` - PHP Generator
- `compiler/src/codegen/python.rs` - Python Generator
- `compiler/src/codegen/typescript.rs` - TypeScript Generator
- `compiler/src/codegen/javascript.rs` - JavaScript Generator
- `compiler/src/codegen/go.rs` - Go Generator
- `compiler/src/codegen/java.rs` - Java Generator
- `compiler/src/codegen/csharp.rs` - C# Generator
- `compiler/src/codegen/ir_codegen.rs` - IR-basierte Code-Generierung

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
