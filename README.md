# Analisador de TypeScript (Rust + React)

Trabalho da disciplina de Compiladores
Este projeto é um analisador léxico e sintático para TypeScript, feito com um núcleo em **Rust** (para performance) e uma interface em **React/Electron** (para visualização).

## Como Rodar

### Pré-requisitos
*   **Rust** instalado (para o backend)
*   **Node.js** instalado (para o frontend)

### Passo a Passo

1.  **Prepare o Backend (Rust)**
    Abra um terminal na pasta `core` e compile o projeto:
    ```bash
    cd core
    cargo build --release
    ```
    *Isso vai criar o executável que analisa o código.*

2.  **Rode a Interface (App)**
    Abra outro terminal na pasta `app`, instale as dependências e inicie:
    ```bash
    cd app
    npm install
    npm start
    ```
    *Uma janela vai abrir onde você pode digitar código TypeScript e ver a análise.*

---

## Estrutura de Pastas

*   `core/`: O "cérebro" do projeto. Código em Rust que lê o arquivo e monta a árvore.
*   `app/`: A "cara" do projeto. Interface gráfica para facilitar o uso.
*   `explicacoes/`: Documentação extra para estudo.

---
**Autores:** [Giovani Claro Moraes, Gabriel de Almeida Cerqueira, Leonardo Donzelli Cantazini, Rennan Samuel de Souza]
