# Collections Library

VelinScript bietet eine umfangreiche Collections Library mit List, Map und Set.

## List<T>

### Basis Operationen

```velin
let list = List<number>([1, 2, 3, 4, 5]);

// Filter
let evens = list.filter((x: number) => x % 2 == 0);

// Map
let doubled = list.map((x: number) => x * 2);

// Reduce
let sum = list.reduce((acc: number, x: number) => acc + x, 0);
```

### Weitere Methoden

```velin
// Find
let found = list.find((x: number) => x > 3);

// Contains
let hasFive = list.contains(5);

// IndexOf
let index = list.indexOf(3);

// Sort
let sorted = list.sort(); // oder mit Comparator
let sorted = list.sort((a: number, b: number) => a - b);

// Reverse
let reversed = list.reverse();

// Chunk
let chunks = list.chunk(2);

// Slice
let slice = list.slice(1, 3);
```

## Map<K, V>

```velin
let map = Map<string, number>();

// Set
map.set("one", 1);
map.set("two", 2);

// Get
let value = map.get("one");

// Has
if (map.has("one")) {
    // ...
}

// Keys
let keys = map.keys();

// Values
let values = map.values();

// Entries
let entries = map.entries();

// Delete
map.delete("one");

// Size
let size = map.size();
```

## Set<T>

```velin
let set = Set<number>();

// Add
set.add(1);
set.add(2);

// Remove
set.remove(1);

// Has
if (set.has(2)) {
    // ...
}

// Size
let size = set.size();

// Union
let union = set1.union(set2);

// Intersection
let intersection = set1.intersection(set2);

// Difference
let difference = set1.difference(set2);
```

## Best Practices

- Verwende `filter`, `map`, `reduce` für funktionale Programmierung
- Nutze `find` statt manueller Schleifen
- Verwende `Set` für eindeutige Werte
- Nutze `Map` für Key-Value-Zuordnungen
