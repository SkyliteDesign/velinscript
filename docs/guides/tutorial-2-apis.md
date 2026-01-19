# Tutorial 2: REST API Entwicklung

Lerne, wie du REST APIs mit VelinScript entwickelst.

## Erster API Endpoint

### GET Endpoint

```velin
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript!";
}
```

Dies wird zu einem Rust actix-web Endpoint kompiliert.

### POST Endpoint

```velin
@POST("/api/users")
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
    };
    return db.save(user);
}
```

## Path Parameters

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

Der `:id` Parameter wird automatisch als Funktionsparameter übergeben.

## CRUD Operations

### Create

```velin
@POST("/api/users")
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
    };
    return db.save(user);
}
```

### Read

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}

@GET("/api/users")
fn getUsers(): List<User> {
    return db.findAll(User);
}
```

### Update

```velin
@PUT("/api/users/:id")
fn updateUser(id: string, name: string, email: string): User {
    let user = db.find(User, id);
    user.name = name;
    user.email = email;
    return db.save(user);
}
```

### Delete

```velin
@DELETE("/api/users/:id")
fn deleteUser(id: string): void {
    db.delete(User, id);
}
```

## Vollständiges Beispiel

```velin
struct User {
    id: string,
    name: string,
    email: string,
}

@GET("/api/users")
fn getUsers(): List<User> {
    return db.findAll(User);
}

@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}

@POST("/api/users")
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
    };
    return db.save(user);
}

@PUT("/api/users/:id")
fn updateUser(id: string, name: string, email: string): User {
    let user = db.find(User, id);
    user.name = name;
    user.email = email;
    return db.save(user);
}

@DELETE("/api/users/:id")
fn deleteUser(id: string): void {
    db.delete(User, id);
}
```

## Beispiel: Krypto-Portfolio-API

Eine kleine realistische API, die ein Krypto-Portfolio berechnet:

```velin
struct PortfolioRequest {
    btcAmount: number,
    ethAmount: number,
    solAmount: number,
}

struct PortfolioOverview {
    btcAmount: number,
    btcValueUsd: number,
    ethAmount: number,
    ethValueUsd: number,
    solAmount: number,
    solValueUsd: number,
    totalValueUsd: number,
}

@POST("/api/crypto/portfolio/custom")
fn calculateCustomPortfolio(request: PortfolioRequest): PortfolioOverview {
    let btc = getStaticBitcoinPrice();
    let eth = getStaticEthereumPrice();
    let sol = getStaticSolanaPrice();

    let btcValue = btc.priceUsd * request.btcAmount;
    let ethValue = eth.priceUsd * request.ethAmount;
    let solValue = sol.priceUsd * request.solAmount;

    let total = btcValue + ethValue + solValue;

    return PortfolioOverview {
        btcAmount: request.btcAmount,
        btcValueUsd: btcValue,
        ethAmount: request.ethAmount,
        ethValueUsd: ethValue,
        solAmount: request.solAmount,
        solValueUsd: solValue,
        totalValueUsd: total,
    };
}
```

Beispiel-Request:

```http
POST /api/crypto/portfolio/custom
Content-Type: application/json

{
  "btcAmount": 0.25,
  "ethAmount": 2.0,
  "solAmount": 15.0
}
```

Die Antwort enthält die berechneten USD-Werte je Coin und die Gesamtsumme.

## Nächste Schritte

- [Tutorial 3: Security](tutorial-3-security.md) - Security Features
- [Tutorial 4: Database](tutorial-4-database.md) - Database Integration
