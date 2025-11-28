# TypeScript Analyzer

A TypeScript static analysis tool built with Rust core and Electron UI.

## Structure

- `core/` - Rust binary for lexical analysis and parsing
- `app/` - Electron application with React UI
- `schema/` - JSON contracts between UI and core
- `docs/` - Documentation and diagrams

## Usage

### Core (Rust)
```bash
cd core
cargo build --release
./target/release/rustots --lex input.ts
./target/release/rustots --stdin < input.ts
```

### App (Electron)
```bash
cd app
npm install
npm start
```

## Development

Build both components:
```bash
# Core
cd core && cargo build

# App
cd app && npm run build
```
