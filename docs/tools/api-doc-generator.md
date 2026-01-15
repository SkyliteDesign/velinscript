# API Documentation Generator

Der VelinScript API Documentation Generator erstellt automatisch OpenAPI/Swagger Dokumentation aus deinem Code.

## Installation

Der API Doc Generator ist Teil des VelinScript Toolchains. Baue ihn mit:

```bash
cd tools/api-doc-generator
cargo build --release
```

## Verwendung

### OpenAPI JSON generieren

```bash
velin-api-doc generate -i main.velin -o openapi.json
```

### OpenAPI YAML generieren

```bash
velin-api-doc generate -i main.velin -o openapi.yaml --format yaml
```

### Markdown Dokumentation generieren

```bash
velin-api-doc generate -i main.velin -o api.md --format markdown
```

### Mit Custom Titel und Version

```bash
velin-api-doc generate -i api.velin -o openapi.json \
  --title "My Awesome API" \
  --version "2.0.0"
```

## Unterstützte Features

### HTTP Endpoints

Alle Funktionen mit HTTP-Decorators werden automatisch erkannt:

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    // ...
}

@POST("/api/users")
fn createUser(name: string, email: string): User {
    // ...
}
```

### Schemas

Structs und Enums werden automatisch als OpenAPI Schemas generiert:

```velin
struct User {
    id: string,
    name: string,
    email: string,
}

enum Status {
    Active,
    Inactive,
}
```

### Security

Security-Decorators werden als Security Schemes generiert:

```velin
@GET("/api/admin/users")
@Auth
fn getAdminUsers(): List<User> {
    // ...
}
```

Wird zu:
```json
{
  "security": [{"bearerAuth": []}],
  "components": {
    "securitySchemes": {
      "bearerAuth": {
        "type": "http",
        "scheme": "bearer",
        "bearerFormat": "JWT"
      }
    }
  }
}
```

## Output-Formate

### JSON (Standard)

```bash
velin-api-doc generate -i main.velin -o api.json --format json
```

### YAML

```bash
velin-api-doc generate -i main.velin -o api.yaml --format yaml
```

### Markdown

```bash
velin-api-doc generate -i main.velin -o api.md --format markdown
```

## Beispiel

**VelinScript Code:**
```velin
struct User {
    id: string,
    name: string,
    email: string,
}

@GET("/api/users/:id")
@Auth
fn getUser(id: string): User {
    return db.find(User, id);
}
```

**Generierte OpenAPI Spec:**
```json
{
  "openapi": "3.0.0",
  "info": {
    "title": "VelinScript API",
    "version": "1.0.0"
  },
  "paths": {
    "/api/users/{id}": {
      "get": {
        "operationId": "get_user",
        "parameters": [
          {
            "name": "id",
            "in": "path",
            "required": true,
            "schema": {"type": "string"}
          }
        ],
        "responses": {
          "200": {
            "description": "Success",
            "content": {
              "application/json": {
                "schema": {"$ref": "#/components/schemas/User"}
              }
            }
          }
        },
        "security": [{"bearerAuth": []}]
      }
    }
  },
  "components": {
    "schemas": {
      "User": {
        "type": "object",
        "properties": {
          "id": {"type": "string"},
          "name": {"type": "string"},
          "email": {"type": "string"}
        },
        "required": ["id", "name", "email"]
      }
    },
    "securitySchemes": {
      "bearerAuth": {
        "type": "http",
        "scheme": "bearer",
        "bearerFormat": "JWT"
      }
    }
  }
}
```

## Integration

### Swagger UI

1. Generiere OpenAPI Spec:
```bash
velin-api-doc generate -i main.velin -o openapi.json
```

2. Öffne in Swagger UI:
```bash
# Mit Docker
docker run -p 8080:8080 -e SWAGGER_JSON=/openapi.json -v $(pwd):/openapi swaggerapi/swagger-ui
```

### CI/CD Integration

```yaml
# .github/workflows/api-docs.yml
- name: Generate API Documentation
  run: |
    cd tools/api-doc-generator
    cargo build --release
    ./target/release/velin-api-doc generate \
      -i ../examples/api.velin \
      -o api-docs/openapi.json \
      --title "VelinScript API" \
      --version "1.0.0"
```

## Best Practices

1. **Regelmäßig aktualisieren** - Generiere Docs bei jedem Release
2. **Versionierung** - Verwende `--version` für API-Versionierung
3. **Titel** - Verwende beschreibende Titel mit `--title`
4. **CI/CD** - Integriere in deine Deployment-Pipeline
