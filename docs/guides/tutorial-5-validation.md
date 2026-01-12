# Tutorial 5: Input Validation

Lerne, wie du Input Validation in VelinScript implementierst.

## Validator verwenden

### Basis-Validierung

```velin
@POST("/api/users")
fn createUser(name: string, email: string): User {
    let mut validator = Validator::new();
    
    validator
        .required("name", &name)
        .min_length("name", &name, 3)
        .max_length("name", &name, 50)
        .email("email", &email);
    
    if (!validator.is_valid()) {
        return HttpResponse::bad_request(
            validator.errors().map(|e| e.message).join(", ")
        );
    }
    
    // Validierung erfolgreich, weiter mit Logik
    return db.save(User { name, email });
}
```

### Erweiterte Validierung

```velin
@POST("/api/products")
fn createProduct(name: string, price: number, sku: string): Product {
    let mut validator = Validator::new();
    
    validator
        .required("name", &name)
        .min_length("name", &name, 3)
        .max_length("name", &name, 100)
        .required("sku", &sku)
        .pattern("sku", &sku, "^[A-Z0-9-]+$", "SKU muss alphanumerisch sein");
    
    if (!validator.is_valid()) {
        let errors = validator.errors();
        return HttpResponse::bad_request(
            errors.map(|e| format!("{}: {}", e.field, e.message)).join(", ")
        );
    }
    
    return db.save(Product { name, price, sku });
}
```

## Best Practices

1. **Immer validieren** für User-Input
2. **Klare Fehlermeldungen** bereitstellen
3. **Konsistente Validierung** über alle Endpoints
4. **Type Safety** nutzen

## Nächste Schritte

- [Tutorial 6: Authentication](tutorial-6-authentication.md) - JWT/OAuth2
- [Tutorial 7: ML Integration](tutorial-7-ml.md) - KI/ML Features
