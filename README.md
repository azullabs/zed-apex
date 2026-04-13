# Apex for Zed

Salesforce Apex, SOQL, and SOSL language support for the [Zed editor](https://zed.dev).

## Features

- **Syntax highlighting** for Apex (`.cls`, `.trigger`, `.apex`), SOQL (`.soql`), and SOSL (`.sosl`)
- **SOQL injection** — inline SOQL queries in Apex files are highlighted with SOQL grammar
- **Code outline** — classes, interfaces, enums, methods, constructors, triggers, and fields
- **Auto-indentation** and **bracket matching**

## Installation

Install as a dev extension:

1. Clone this repository
2. Clone the grammar: `git clone https://github.com/aheber/tree-sitter-sfapex vendor/tree-sitter-sfapex`
3. In Zed, run `zed: install dev extension` and select the cloned directory

## Development

Run tests:

```sh
cargo test --manifest-path tests/crate/Cargo.toml
```

## License

MIT
