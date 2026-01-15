# Closure/Lambda Functions

VelinScript unterstützt Lambda Functions (Closures) für funktionale Programmierung.

## Basis Lambda Syntax

```velin
let add = (a: number, b: number) => a + b;
let result = add(5, 3); // 8
```

## Lambda mit Block Body

```velin
let multiply = (a: number, b: number) => {
    let result = a * b;
    return result;
};
```

## Higher-Order Functions

Lambdas können als Parameter übergeben werden:

```velin
fn applyOperation(a: number, b: number, op: fn(number, number) -> number): number {
    return op(a, b);
}

let result = applyOperation(10, 5, (x: number, y: number) => x - y);
```

## Mit Collections

Lambdas arbeiten perfekt mit Collections:

```velin
let numbers = List<number>([1, 2, 3, 4, 5]);

// Map
let doubled = numbers.map((x: number) => x * 2);

// Filter
let evens = numbers.filter((x: number) => x % 2 == 0);

// Reduce
let sum = numbers.reduce((acc: number, x: number) => acc + x, 0);
```

## Type Inference

Lambdas unterstützen Type Inference:

```velin
// Type wird automatisch inferiert
let square = (x) => x * x; // x ist number
```

## Closure Capture

Lambdas können Variablen aus dem umgebenden Scope erfassen:

```velin
let factor = 10;
let multiply = (x: number) => x * factor;
let result = multiply(5); // 50
```

## Currying

Lambdas können für Currying verwendet werden:

```velin
let add = (a: number) => (b: number) => a + b;
let addFive = add(5);
let result = addFive(3); // 8
```

## Best Practices

- Verwende Lambdas für kurze, einfache Operationen
- Nutze Named Functions für komplexe Logik
- Kombiniere Lambdas mit Collections für funktionale Programmierung
- Nutze Type Inference wo möglich
