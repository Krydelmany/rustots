# RusToTS - Analisador Léxico para TypeScript

Projeto de análise léxica (tokenização) para código TypeScript, implementado em Rust com interface Electron.

## 📋 Visão Geral

Este projeto implementa a **Parte 1 (Análise Léxica)** de um analisador estático para TypeScript. O lexer identifica e classifica tokens do código fonte em formato JSON estruturado.

## 🚀 Como Executar

### Núcleo Rust (Core)

#### Compilação:
```bash
cd core
cargo build --release
```

#### Execução:
```bash
# Analisar arquivo TypeScript
cargo run -- samples/example_ok.ts

# Ler da entrada padrão
cargo run -- --stdin < samples/example_ok.ts
cat samples/example_ok.ts | cargo run -- --stdin

# Usar binário compilado diretamente
./target/release/rustots samples/example_ok.ts
```

### Interface Electron (App)

```bash
cd app
npm install
npm start
```

## 📤 Formato de Saída

O lexer retorna JSON estruturado com todos os tokens identificados:

```json
{
  "tokens": [
    {
      "type": "keyword",
      "value": "let",
      "position": {
        "start": 0,
        "end": 3,
        "line": 1,
        "column": 1
      }
    },
    {
      "type": "whitespace",
      "value": " ",
      "position": {
        "start": 3,
        "end": 4,
        "line": 1,
        "column": 4
      }
    },
    {
      "type": "identifier",
      "value": "x",
      "position": {
        "start": 4,
        "end": 5,
        "line": 1,
        "column": 5
      }
    }
  ]
}
```

## 🏷️ Tipos de Token

| Tipo | Descrição | Exemplos |
|------|-----------|----------|
| `keyword` | Palavras-chave TypeScript | `let`, `const`, `function`, `class`, `interface` |
| `identifier` | Nomes de variáveis/funções | `x`, `myFunction`, `userName` |
| `literal` | Valores literais | `42`, `"texto"`, `'char'`, `` `template` `` |
| `operator` | Operadores | `=`, `+`, `-`, `==`, `=>`, `&&` |
| `punctuation` | Pontuação | `{`, `}`, `(`, `)`, `;`, `,`, `.` |
| `comment` | Comentários | `// linha`, `/* bloco */` |
| `whitespace` | Espaços e tabs | ` `, `\t` |
| `newline` | Quebras de linha | `\n`, `\r\n` |
| `unknown` | Caracteres não reconhecidos | Qualquer char fora da gramática |

## 📂 Estrutura do Projeto

```
rustots/
├── core/                    # Núcleo do analisador (Rust)
│   ├── src/
│   │   ├── main.rs         # CLI e orquestração
│   │   └── lexer.rs        # Implementação do lexer
│   ├── samples/            # Arquivos de exemplo
│   │   ├── example_ok.ts   # Código válido
│   │   └── example_error.ts # Código com tokens desconhecidos
│   └── Cargo.toml          # Dependências Rust
├── app/                     # Interface Electron (React + Vite)
│   ├── src/                # Código TypeScript/React
│   ├── electron/           # Processo principal Electron
│   └── package.json
└── README.md
```

## 🔧 Detalhes Técnicos

### Suporte UTF-8
O lexer suporta caracteres UTF-8, incluindo identificadores com letras não-ASCII:
```typescript
let variável = 42;  // ✅ Aceita acentos
const 변수 = "한글";  // ✅ Aceita Unicode
```

### Palavras-chave Reconhecidas
O lexer identifica todas as palavras-chave TypeScript/JavaScript padrão:
- **Declarações**: `let`, `const`, `var`, `function`, `class`, `interface`, `type`, `enum`
- **Controle**: `if`, `else`, `for`, `while`, `switch`, `case`, `break`, `continue`, `return`
- **Tipos**: `string`, `number`, `boolean`, `any`, `void`, `never`, `unknown`, `null`, `undefined`
- **Módulos**: `import`, `export`, `from`, `module`, `namespace`, `require`
- **Outros**: `async`, `await`, `typeof`, `keyof`, `readonly`, `public`, `private`, `static`

### Tokens Unknown
Quando o lexer encontra um caractere não reconhecido, gera um token do tipo `unknown`. Isso permite:
- Continuar a análise sem interrupção
- Identificar todos os problemas de uma vez
- Útil para debugging e detecção de caracteres inválidos

## 📝 Exemplos

### Exemplo 1: Declaração Simples
**Entrada** (`example_ok.ts`):
```typescript
let x = 42;
console.log(x);
// identifier teste
```

**Saída**: Tokens identificados incluindo keywords (`let`), identifiers (`x`, `console`, `log`), literals (`42`), operators (`=`), punctuation (`.`, `;`, `(`, `)`), comments, whitespace e newlines.

### Exemplo 2: Tokens Unknown
**Entrada** (`example_error.ts`):
```typescript
let valor = 100@;
```

**Saída**: O caractere `@` será identificado como token `unknown`, permitindo identificar problemas léxicos.

## 🎯 Integração com Electron

Este projeto está preparado para integração com Electron (pasta `app/`):
- Saída JSON facilita comunicação entre processos Rust ↔ Node.js
- CLI pode ser chamado via `child_process.spawn()` no Electron
- Tokens com posição exata permitem syntax highlighting no editor
- Arquitetura desacoplada: core Rust independente da UI

## 🧪 Testes

Para validar o funcionamento:

```bash
cd core

# Testar com exemplo válido
cargo run -- samples/example_ok.ts

# Testar com stdin
echo "let x = 42;" | cargo run -- --stdin

# Testar com caractere inválido
echo "let x = 100@;" | cargo run -- --stdin
```

## 📚 Dependências

### Core (Rust)
- `clap` - Parser de argumentos CLI
- `serde` + `serde_json` - Serialização JSON
- `anyhow` - Tratamento de erros

### App (Electron)
- `electron` - Framework desktop
- `react` - UI components
- `vite` - Build tool

## 🚧 Roadmap

- [x] Análise Léxica (Parte 1) ✅
- [ ] Parser/AST (Parte 2)
- [ ] Análise Semântica (Parte 3)
- [ ] Interface Electron completa
- [ ] Syntax highlighting em tempo real
- [ ] Exportação de relatórios

## 👥 Autor

Desenvolvido como projeto acadêmico - Disciplina de Compiladores

**Entrega**: Parte 1 - Análise Léxica

## 📄 Licença

Projeto acadêmico - uso educacional
