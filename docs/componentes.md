# Componentes do Sistema

## Core (Rust)

### Módulos

- **lexer.rs**: Tokenização do código TypeScript
- **parse.rs**: Construção da AST
- **types.rs**: Sistema de tipos
- **symbols.rs**: Tabela de símbolos
- **collect.rs**: Coleta de informações
- **diagnostics.rs**: Geração de diagnósticos

### Flags de Comando

- `--lex`: Apenas tokenização
- `--parse`: Tokenização + parsing
- `--analyze`: Análise completa
- `--stdin`: Lê de stdin
- `--file <path>`: Lê arquivo específico

## App (Electron)

### Estrutura

- **main.ts**: Processo principal do Electron
- **preload.ts**: Script de bridge para renderer
- **App.tsx**: Componente principal React
- **Monaco Editor**: Editor de código integrado
- **Token Table**: Visualização de tokens

### APIs

- `window.api.analyze(code)`: Analisa código TypeScript
- `window.api.openFile()`: Abre arquivo
- `window.api.saveFile(content)`: Salva arquivo
