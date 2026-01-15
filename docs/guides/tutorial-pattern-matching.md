# Erweiterte Pattern Matching

VelinScript unterstützt erweiterte Pattern Matching mit Guards, Range Patterns, Destructuring und mehr.

## Basis Pattern Matching

```velin
match (value) {
    "hello" => {
        return "greeting";
    },
    "world" => {
        return "planet";
    },
    _ => {
        return "unknown";
    }
}
```

## Pattern Guards

Guards erlauben zusätzliche Bedingungen in Pattern Matches:

```velin
match (result) {
    Ok(value) if value > 0 => {
        return "positive";
    },
    Ok(value) if value == 0 => {
        return "zero";
    },
    Ok(value) => {
        return "negative";
    },
    Error(err) => {
        return "error";
    }
}
```

## Range Patterns

Range Patterns erlauben das Matching von Zahlenbereichen:

```velin
match (age) {
    0..=12 => "child",
    13..=19 => "teenager",
    20..=64 => "adult",
    _ => "senior"
}
```

## Destructuring

### Struct Destructuring

```velin
match (user) {
    User { name: "admin", role } => {
        return "admin user";
    },
    User { name, role: "user" } => {
        return "regular user: " + name;
    },
    _ => {
        return "unknown";
    }
}
```

### Tuple Destructuring

```velin
match (coordinates) {
    (0, 0) => "origin",
    (x, 0) => "x-axis",
    (0, y) => "y-axis",
    (x, y) => "point at (" + x + ", " + y + ")"
}
```

### Enum Variant Destructuring

```velin
enum Result<T, E> {
    Ok(value: T),
    Error(error: E),
}

match (result) {
    Result::Ok(value) => {
        return value;
    },
    Result::Error(err: DatabaseError) => {
        return "database error";
    },
    Result::Error(err: NetworkError) => {
        return "network error";
    }
}
```

## Or Patterns

Mehrere Patterns können mit `|` kombiniert werden:

```velin
match (status) {
    "pending" | "processing" => {
        return "in progress";
    },
    "completed" | "done" => {
        return "finished";
    },
    _ => {
        return "unknown";
    }
}
```

## Wildcard Pattern

Der Wildcard `_` matched alles:

```velin
match (value) {
    1 => "one",
    2 => "two",
    _ => "other"
}
```

## Best Practices

- Verwende Guards für komplexe Bedingungen
- Nutze Range Patterns für Zahlenbereiche
- Destructure Structs für bessere Lesbarkeit
- Verwende Wildcard als Fallback
