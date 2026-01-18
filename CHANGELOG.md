# Changelog

All notable changes to this project will be documented in this file.

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
