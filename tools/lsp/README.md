# VelinScript Language Server

Language Server Protocol (LSP) Implementation für VelinScript.

## Features

- Auto-Completion
- Go-to-Definition
- Hover-Informationen
- Error Highlighting
- Code Formatting
- Refactoring

## Installation

```bash
cd tools/lsp
cargo build --release
```

## Verwendung

Der LSP Server wird von IDEs automatisch verwendet. Für manuelle Tests:

```bash
velin-lsp
```

## Entwicklung

Dies ist ein separates Cargo-Projekt, das `velin-compiler` als Dependency nutzt.
