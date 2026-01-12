# Tutorial: Privacy & DSGVO-Compliance

Lerne, wie du Privacy-Features in VelinScript verwendest.

## Privacy Decorator

Markiere sensitive Felder mit `@Privacy`:

```velin
struct User {
    id: string,
    name: string,
    @Privacy
    email: string,
    @Privacy
    phone: string,
    @Privacy
    ip_address: string,
}
```

Felder werden automatisch als `PrivacyWrapper<T>` behandelt und in Logs verschleiert.

## Automatische PII-Detection

Feldnamen mit PII-Keywords werden automatisch erkannt:

- `email`, `phone`, `ssn`, `passport`, `credit_card`
- `ip`, `address`, `name`, `birthdate`

## Secure Deletion

Verwende `secureDelete()` für sichere Löschung:

```velin
@DELETE("/api/users/:id")
fn deleteUser(id: string): void {
    let user = db.find(User, id);
    secureDelete(user);  // Automatisch zeroize()
    db.delete(User, id);
}
```

## Zero-Knowledge Encryption

Verschlüssele Daten während Verarbeitung:

```velin
fn processSensitiveData(data: string): string {
    let encrypted = zeroKnowledgeEncrypt(data);
    // ... Verarbeitung ...
    return zeroKnowledgeDecrypt(encrypted);
}
```

## Config-Setup

In `velin.config.json`:

```json
{
  "privacy": {
    "enabled": true,
    "piiDetection": true,
    "zeroKnowledge": false
  }
}
```

## Vollständiges Beispiel

Siehe [examples/privacy-compliant.velin](../../examples/privacy-compliant.velin) für ein vollständiges Beispiel.
