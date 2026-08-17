# HTTP Client Library

VelinScript bietet eine vollständige HTTP Client Library für API-Aufrufe.

## Basis Verwendung

```velin
let client = HttpClient.new();

// GET Request
let response = await client.get("https://api.example.com/users");
let data = response.json();
```

## GET Request

```velin
let response = await client.get("https://api.example.com/users");

// Mit Headers
let headers = Map<string, string>([
    ("Authorization", "Bearer token"),
    ("Content-Type", "application/json")
]);
let response = await client.get("https://api.example.com/users", headers);
```

## POST Request

```velin
let user = {
    name: "John",
    email: "john@example.com"
};

let response = await client.post(
    "https://api.example.com/users",
    user,
    headers
);
```

## PUT, DELETE, PATCH

```velin
// PUT
let response = await client.put(
    "https://api.example.com/users/123",
    updatedUser
);

// DELETE
let response = await client.delete("https://api.example.com/users/123");

// PATCH
let response = await client.patch(
    "https://api.example.com/users/123",
    partialUpdate
);
```

## Response Handling

```velin
let response = await client.get("https://api.example.com/users");

// JSON
let data = response.json();

// Text
let text = response.text();

// Status
let status = response.status();
```

## Error Handling

```velin
match (await client.get("https://api.example.com/users")) {
    Ok(response) => {
        let data = response.json();
        // ...
    },
    Error(err: NetworkError) => {
        logError("Network error: " + err.message);
    },
    Error(err: HttpError) => {
        logError("HTTP error: " + err.status);
    }
}
```

## Retry Logic

Die HTTP Client Library unterstützt automatische Retries:

```velin
// Mit Retry (automatisch bei Fehlern)
let response = await client.getWithRetry(
    "https://api.example.com/users",
    3, // max retries
    1000 // delay in ms
);
```

## Best Practices

- Verwende immer Error Handling
- Nutze Retry Logic für kritische Requests
- Setze angemessene Timeouts
- Verwende Headers für Authentication
