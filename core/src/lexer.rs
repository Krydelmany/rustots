use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "type")]
    pub token_type: TokenType,
    pub value: String,
    pub position: Position,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malformed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Identifier,
    Keyword,
    Operator,
    Literal,
    Punctuation,
    Comment,
    Whitespace,
    Newline,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }

        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let start_pos = self.position;
        let start_line = self.line;
        let start_column = self.column;

        if let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\t' => {
                    self.consume_whitespace();
                    Some(Token {
                        token_type: TokenType::Whitespace,
                        value: self.input[start_pos..self.position].to_string(),
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                        malformed: None,
                    })
                }
                '\n' | '\r' => {
                    self.advance();
                    if ch == '\r' && self.current_char() == Some('\n') {
                        self.advance();
                    }
                    self.line += 1;
                    self.column = 1;
                    Some(Token {
                        token_type: TokenType::Newline,
                        value: self.input[start_pos..self.position].to_string(),
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                        malformed: None,
                    })
                }
                _ if ch.is_alphabetic() || ch == '_' || ch == '$' => {
                    let value = self.consume_identifier();
                    let token_type = if self.is_keyword(&value) {
                        TokenType::Keyword
                    } else {
                        TokenType::Identifier
                    };
                    Some(Token {
                        token_type,
                        value,
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                        malformed: None,
                    })
                }
                _ if ch.is_numeric() => {
                    let (value, malformed) = self.consume_number();
                    Some(Token {
                        token_type: TokenType::Literal,
                        value,
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                        malformed,
                    })
                }
                '"' | '\'' | '`' => {
                    let (value, malformed) = self.consume_string(ch);
                    Some(Token {
                        token_type: TokenType::Literal,
                        value,
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                        malformed,
                    })
                }
                '/' => {
                    if self.peek_char() == Some('/') || self.peek_char() == Some('*') {
                        let (value, malformed) = self.consume_comment();
                        Some(Token {
                            token_type: TokenType::Comment,
                            value,
                            position: Position {
                                start: start_pos,
                                end: self.position,
                                line: start_line,
                                column: start_column,
                            },
                            malformed,
                        })
                    } else {
                        self.advance();
                        Some(Token {
                            token_type: TokenType::Operator,
                            value: "/".to_string(),
                            position: Position {
                                start: start_pos,
                                end: self.position,
                                line: start_line,
                                column: start_column,
                            },
                            malformed: None,
                        })
                    }
                }
                '+' | '-' | '*' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '%' | '?' | ':' => {
                    let value = self.consume_operator();
                    Some(Token {
                        token_type: TokenType::Operator,
                        value,
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                        malformed: None,
                    })
                }
                '{' | '}' | '(' | ')' | '[' | ']' | ';' | ',' | '.' => {
                    self.advance();
                    Some(Token {
                        token_type: TokenType::Punctuation,
                        value: ch.to_string(),
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                        malformed: None,
                    })
                }
                _ => {
                    // Se chegamos aqui, o caractere não bateu com nenhuma regra conhecida.
                    // Marcamos como erro léxico para o usuário corrigir.
                    self.advance();
                    return Some(Token {
                        token_type: TokenType::Unknown,
                        value: ch.to_string(),
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                        malformed: Some(format!("Caractere não reconhecido: '{}'", ch)),
                    });
                }
            }
        } else {
            None
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().nth(1)
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            self.position += ch.len_utf8();
            self.column += 1;
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == ' ' || ch == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn consume_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn consume_number(&mut self) -> (String, Option<String>) {
        let start = self.position;
        let mut dot_count = 0;
        
        while let Some(ch) = self.current_char() {
            if ch.is_numeric() {
                self.advance();
            } else if ch == '.' {
                dot_count += 1;
                self.advance();
            } else {
                break;
            }
        }
        
        let value = self.input[start..self.position].to_string();
        let malformed = if dot_count > 1 {
            Some(format!("Número com múltiplos pontos decimais ({})", dot_count))
        } else {
            None
        };
        
        (value, malformed)
    }

    fn consume_string(&mut self, quote: char) -> (String, Option<String>) {
        let start = self.position;
        let start = self.position;
        self.advance();
        let mut terminated = false;

        while let Some(ch) = self.current_char() {
            if ch == quote {
                self.advance();
                terminated = true;
                break;
            } else if ch == '\\' {
                self.advance();
                if self.current_char().is_some() {
                    self.advance();
                }
            } else if ch == '\n' || ch == '\r' {
                // Detectamos se a string quebra a linha.
                // Em JS/TS isso geralmente é erro, mas deixamos passar por enquanto.
                break;
            } else {
                self.advance();
            }
        }

        let value = self.input[start..self.position].to_string();
        let malformed = if !terminated {
            Some("String não terminada".to_string())
        } else {
            None
        };

        (value, malformed)
    }

    fn consume_comment(&mut self) -> (String, Option<String>) {
        let start = self.position;
        let mut terminated = true;

        if self.peek_char() == Some('/') {
            // Single-line comment - sempre terminado
            while let Some(ch) = self.current_char() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                self.advance();
            }
        } else if self.peek_char() == Some('*') {
            // Multi-line comment - pode não terminar
            self.advance();
            self.advance();
            terminated = false;

            while self.position < self.input.len() {
                if self.current_char() == Some('*') && self.peek_char() == Some('/') {
                    self.advance();
                    self.advance();
                    terminated = true;
                    break;
                }
                if self.current_char().is_some() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let value = self.input[start..self.position].to_string();
        let malformed = if !terminated {
            Some("Comentário multilinha não fechado".to_string())
        } else {
            None
        };

        (value, malformed)
    }

    fn consume_operator(&mut self) -> String {
        let start = self.position;
        let ch = self.current_char().unwrap();
        self.advance();

        match ch {
            '=' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                    }
                } else if self.current_char() == Some('>') {
                    self.advance();
                }
            }
            '!' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                    }
                }
            }
            '<' | '>' => {
                if self.current_char() == Some('=') {
                    self.advance();
                } else if self.current_char() == Some(ch) {
                    self.advance();
                }
            }
            '&' | '|' => {
                if self.current_char() == Some(ch) {
                    self.advance();
                }
            }
            '+' | '-' => {
                if self.current_char() == Some(ch) || self.current_char() == Some('=') {
                    self.advance();
                }
            }
            '*' => {
                if self.current_char() == Some('=') || self.current_char() == Some('*') {
                    self.advance();
                }
            }
            _ => {}
        }

        self.input[start..self.position].to_string()
    }

    fn is_keyword(&self, value: &str) -> bool {
        matches!(
            value,
            "abstract"
                | "any"
                | "as"
                | "asserts"
                | "bigint"
                | "boolean"
                | "break"
                | "case"
                | "catch"
                | "class"
                | "const"
                | "continue"
                | "debugger"
                | "declare"
                | "default"
                | "delete"
                | "do"
                | "else"
                | "enum"
                | "export"
                | "extends"
                | "false"
                | "finally"
                | "for"
                | "from"
                | "function"
                | "get"
                | "if"
                | "implements"
                | "import"
                | "in"
                | "infer"
                | "instanceof"
                | "interface"
                | "is"
                | "keyof"
                | "let"
                | "module"
                | "namespace"
                | "never"
                | "new"
                | "null"
                | "number"
                | "object"
                | "of"
                | "package"
                | "private"
                | "protected"
                | "public"
                | "readonly"
                | "require"
                | "return"
                | "set"
                | "static"
                | "string"
                | "super"
                | "switch"
                | "symbol"
                | "this"
                | "throw"
                | "true"
                | "try"
                | "type"
                | "typeof"
                | "undefined"
                | "unique"
                | "unknown"
                | "var"
                | "void"
                | "while"
                | "with"
                | "yield"
        )
    }
}
