# Apex for Zed

Salesforce Apex, SOQL, and SOSL language support for the [Zed editor](https://zed.dev).

## Features

- **Syntax highlighting** for Apex (`.cls`, `.trigger`, `.apex`), SOQL (`.soql`), and SOSL (`.sosl`)
- **SOQL injection** — inline SOQL queries in Apex files are highlighted with SOQL grammar
- **Code outline** — classes, interfaces, enums, methods, constructors, triggers, and fields
- **Auto-indentation** and **bracket matching**
- **LSP integration** via Salesforce's official [apex-jorje-lsp](https://github.com/forcedotcom/salesforcedx-vscode), providing:
  - Code completion
  - Go-to-definition
  - Find all references
  - Diagnostics
  - Refactoring

## Requirements

- **Zed** editor
- **Java 11+** on your `PATH` (required for LSP features)

The extension automatically downloads `apex-jorje-lsp.jar` on first use. No manual setup needed.

## Installation

Search for **Apex** in Zed's extension marketplace (`zed: extensions`), or install as a dev extension:

1. Clone this repository
2. In Zed, run `zed: install dev extension` and select the cloned directory

## Configuration

The extension works out of the box. To customize LSP settings, add to your Zed `settings.json`:

```json
{
  "lsp": {
    "apex-jorje": {
      "initialization_options": {
        "enableEmbeddedSoqlCompletion": true
      }
    }
  }
}
```

## Development

Build the extension:

```sh
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --release
```

Run tests:

```sh
cargo test -p zed_apex_tests
```

## License

MIT
