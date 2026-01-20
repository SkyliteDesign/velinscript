# Changelog

All notable changes to this project will be documented in this file.

## [2.7.0] - 2026-01-30

### Added

- **Bibliotheks-Generator**: Neues Tool zur automatischen Generierung von Standardbibliotheks-Modulen ✅
  - CLI-Tool (`velin-library-generator`) für schnelle Bibliotheks-Erstellung
  - Interaktiver Modus für benutzerfreundliche Konfiguration
  - YAML-basierte Konfigurationsdateien für wiederholbare Generierung
  - Automatische Integration in alle System-Komponenten:
    - Modul-Datei-Generierung (`compiler/src/stdlib/{name}.rs`)
    - Integration in `mod.rs` (alphabetisch sortiert)
    - Type Checker Integration (vollständige Typ- und Funktions-Registrierung)
    - Code Generator Integration (Dispatch-Logik und Funktions-Generierung)
    - Test-Generierung mit Validierungen
    - Vollständige API-Dokumentation
  - Template-System mit 3 Modul-Typen:
    - Simple Functions (einfache Funktionen ohne Structs)
    - Struct Based (Module mit benutzerdefinierten Typen)
    - Service Based (Service-basierte Module mit State)
  - Vollständige Code-Generierung ohne TODOs
  - Validierung von Konfigurationen
  - Integration-Tests
- **Dokumentation**:
  - Vollständige README für Bibliotheks-Generator
  - Tool-Dokumentation in `docs/tools/library-generator.md`
  - Aktualisierter Plan in `bauplan/BIBLIOTHEKS_GENERATOR_PLAN.md`

### Changed

- Verbesserte Entwickler-Erfahrung durch automatische Bibliotheks-Generierung
- Reduzierte Entwicklungszeit für neue Standardbibliotheks-Module von Stunden auf Minuten

## [2.6.0] - 2026-01-30

### Added

- **Standard Library Expansion**: Added 5 new critical modules with 50+ functions.
  - `path`: Cross-platform path manipulation (`join`, `dirname`, `basename`, `extname`, `normalize`, `resolve`, `relative`, `is_absolute`, `separator`).
  - `url`: URL parsing and manipulation (`parse`, `protocol`, `hostname`, `port`, `pathname`, `search`, `hash`, `format`, `parse_query`, `stringify_query`).
  - `stream`: Stream processing for large datasets (`create`, `map`, `filter`, `reduce`, `batch`, `buffer`, `merge`, `zip`).
  - `redis`: Redis integration for caching and pub/sub (`connect`, `set`, `get`, `delete`, `hset`, `hget`, `hgetall`, `lpush`, `rpush`, `lpop`, `llen`, `sadd`, `sismember`, `smembers`, `publish`).
  - `tracing`: Distributed tracing for microservices (`start_span`, `set_attribute`, `child_span`, `end_span`, `export`).
- **Implementation Improvements**:
  - All mock functions replaced with real implementations.
  - Process management now uses real platform-specific commands (Unix/Windows).
  - Sandbox functions now use actual velin CLI integration.
  - WebSocket functions include real event handling with tokio::spawn.
  - Utils functions (debounce, throttle, memoize, cache) now have full implementations.
  - Log functions include real file appender and context support.
- **Dependencies**:
  - Added `url`, `pathdiff`, `futures`, `urlencoding`, `once_cell`, `tempfile` dependencies.

### Changed

- Fixed all compiler warnings (unused imports, unused variables).
- Improved error handling in all standard library modules.
- Enhanced pipeline optimizer with real variable dependency tracking.
- Updated documentation to reflect all new modules.

### Fixed

- Fixed type checker issues with `Type::Optional` vs `Type::Option`.
- Fixed borrow checker errors in pipeline optimizer.
- Fixed expression variant names in pipeline optimizer (BinaryOp, UnaryOp, If).

## [2.5.0] - 2026-01-30

### Added

- **Standard Library Expansion**: Added 13 new modules with over 117 functions.
  - `string`: Advanced string manipulation (`split`, `join`, `slugify`, `capitalize`, etc.).
  - `math`: Mathematical utilities (`clamp`, `lerp`, `random_range`, `round_to`, etc.).
  - `date`: Extended date/time functions (`add_days`, `is_weekend`, `format_relative`, etc.).
  - `fs`: File system operations (`read_json`, `write_json`, `copy`, `get_size`, etc.).
  - `llm`: AI/LLM integration (`summarize`, `chat`, `translate`, `sentiment`, etc.).
  - `embedding`: Vector embedding utilities (`similarity`, `find_nearest`, `cluster`, etc.).
  - `agent`: AI Agent capabilities (`memory.store`, `task.plan`, `think`, etc.).
  - `process`: System process management (`spawn`, `kill`, `status`, `get_output`, etc.).
  - `sandbox`: Code execution and validation (`run`, `lint`, `test`, `optimize`, etc.).
  - `websocket`: Real-time communication (`connect`, `send`, `on_message`, etc.).
  - `utils`: Utility functions (`uuid`, `sleep`, `retry`, `debounce`, `throttle`, etc.).
  - `log`: Enhanced logging (`trace`, `to_file`, `with_context`, etc.).
  - `config`: Configuration and environment management (`get_env`, `load_dotenv`).
  - `flow`: VelinFlow Runtime (`@Flow`, `flow.snapshot_input`, automatic rollback/commit).
- **VelinAutoDoc**:
  - Compiler now captures `///` doc comments.
  - New `AutoDocGenerator` extracts API docs, types, and decorators into structured JSON.
  - Includes `llm_prompt_context` for AI-powered documentation generation.
- **VelinPipeline**:
  - New `PipelineOptimizer` analyzes module data flow.
  - Detects parallelizable async blocks in `@VelinPipeline` modules.
  - Prepares codegen for automatic `tokio::join!` optimization.
- **Core Stabilization**:
  - **Routing**: Improved parameter extraction for `@GET`, `@POST`, etc. Automatic `Path` and `Json` extraction.
  - **Validation**: Integrated `validator` crate. Structs now automatically derive `Validate`.
  - **Error Handling**: Global `AppError` handler for Axum prevents silent failures and ensures 500 responses.
  - **Observability**: Automatic `#[tracing::instrument]` on all async handlers.
- **Type Checker**:
  - Full support for new standard library modules.
  - Improved `any` type compatibility.
  - Support for nested module calls (e.g., `agent.memory.store`).
- **Code Generator**:
  - Rust code generation for all new modules.
  - Integration with `reqwest`, `tokio`, `rand`, `chrono`, and other crates.

### Changed

- Updated core dependencies in `Cargo.toml`.
- Improved error handling in standard library functions.
- Documentation updated to reflect new API surface.

## [2.0.0] - 2025-12-01

### Added

- Initial release of VelinScript 2.0.
- Compiler core (Parser, Type Checker, Code Generator).
- Basic Standard Library (HTTP, JSON, Auth).
- Developer Tools (Linter, Formatter, LSP, Debugger).
