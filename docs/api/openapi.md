# OpenAPI Integration

VelinScript kann automatisch OpenAPI Specifications aus deinem Code generieren.

## Verwendung

### CLI Befehl

```bash
# Mit velin-api-doc Tool
velin-api-doc generate -i main.velin -o api.json

# Mit Format-Optionen
velin-api-doc generate -i main.velin -o api.yaml --format yaml
velin-api-doc generate -i main.velin -o api.md --format markdown

# Mit Custom Titel und Version
velin-api-doc generate -i main.velin -o api.json --title "My API" --version "1.0.0"
```

Dies generiert eine OpenAPI 3.0 Specification aus allen API-Endpoints in deinem VelinScript Code.

## Beispiel

**VelinScript:**
```velin
@GET("/api/users/:id")
@Auth
fn getUser(id: string): User {
    return db.find(User, id);
}

@POST("/api/users")
fn createUser(name: string, email: string): User {
    // ...
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
        "security": [{"bearerAuth": []}],
        "responses": {
          "200": {
            "description": "Success",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/User"
                }
              }
            }
          }
        }
      }
    }
  }
}
```

## Features

- Automatische Extraktion von HTTP Endpoints
- Security Decorators werden zu Security Requirements
- Parameter werden automatisch erkannt
- Request/Response Schemas werden generiert
- Type Information wird verwendet

## Integration

Die generierte OpenAPI Spec kann verwendet werden für:

- API Documentation (Swagger UI, ReDoc)
- Client Code Generation
- API Testing
- API Gateway Configuration
