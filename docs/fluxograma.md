# Fluxograma do Sistema

## Processo de Análise

```mermaid
graph TD
    A[Usuário insere código TS] --> B[Electron UI]
    B --> C[Spawn processo rustots]
    C --> D[Lexer - Tokenização]
    D --> E[Parser - AST]
    E --> F[Análise de Tipos]
    F --> G[Coleta de Símbolos]
    G --> H[Geração de Diagnósticos]
    H --> I[JSON Output]
    I --> J[IPC para UI]
    J --> K[Monaco Editor + Tabela]
```

## Componentes

- **UI (Electron/React)**: Interface do usuário
- **Core (Rust)**: Motor de análise
- **IPC**: Comunicação entre processos
- **Monaco**: Editor de código
- **JSON Schema**: Contrato de dados
