# Multi-Target Compilation

VelinScript 3.5 unterstützt mehrere Zielsprachen.
Der stabile Laufzeitpfad in dieser Version ist **Rust/Axum**.
Weitere Targets befinden sich im Entwicklungs- bzw. Experimentalstatus.

Unterstützte Emit-Targets (Codegenerierung): Rust, PHP, Python, TypeScript, JavaScript, Go, Java und C#.

Für HTTP-APIs, die du bauen und starten willst, ist Rust mit Axum der empfohlene und belegte Weg: daraus entstehen Router und Handler, die sich mit dem Rust-Toolchain weiterbauen lassen.

```bash
velin compile -i main.velin -o main.rs --target rust
velin compile -i main.velin -o main.py --target python
```

**Codegenerierung** (Emit) ≠ **Runtime**. Nur Rust/Axum ist in 3.5.1 als stabiler Laufzeitpfad für HTTP/`@Auth`/`@Role` dokumentiert und getestet. Andere Targets liefern Code-Ausgabe zum Weiterentwickeln — ohne gleichwertiges Runtime-Versprechen. Interpolation wird für alle acht Emit-Targets erzeugt; ausgeführt und gegen HTTP geprüft ist der Rust/Axum-Pfad.

Siehe auch: [QUICK_START.md](../../QUICK_START.md), Beispiele unter `examples/`.
