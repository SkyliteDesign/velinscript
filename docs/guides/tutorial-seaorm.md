# Tutorial: SeaORM Integration

Lerne, wie du SeaORM in VelinScript verwendest.

## Config-Setup

In `velin.config.json`:

```json
{
  "orm": "seaorm",
  "database": {
    "orm": "seaorm",
    "connectionString": "${DATABASE_URL}"
  }
}
```

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
    
    // Generiert zu: User::ActiveModel::insert()
    return db.save(user);
}
```

### Read

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    // Generiert zu: User::Entity::find_by_id()
    return db.find(User, id);
}

@GET("/api/users")
fn getUsers(): List<User> {
    // Generiert zu: User::Entity::find().all()
    return db.findAll(User);
}
```

### Update

```velin
@PUT("/api/users/:id")
fn updateUser(id: string, name: string): User {
    let user = db.find(User, id);
    user.name = name;
    
    // Generiert zu: User::ActiveModel::update()
    return db.save(user);
}
```

### Delete

```velin
@DELETE("/api/users/:id")
fn deleteUser(id: string): void {
    // Generiert zu: User::Entity::delete_by_id()
    db.delete(User, id);
}
```

## Entity-Generierung

Structs werden automatisch zu SeaORM Entities:

```velin
struct User {
    id: string,
    name: string,
    email: string,
}
```

Wird generiert zu:

```rust
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub email: String,
}
```

## Vollständiges Beispiel

Siehe [examples/seaorm-crud.velin](../../examples/seaorm-crud.velin) für ein vollständiges Beispiel.
