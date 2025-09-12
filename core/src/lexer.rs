use serde::{Deserialize, Serialize};
use crate::diagnostics::DiagnosticCollector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "type")]
    pub token_type: TokenType,
    pub value: String,
    pub position: Position,
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
    diagnostics: &'a mut DiagnosticCollector,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, diagnostics: &'a mut DiagnosticCollector) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
            diagnostics,
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
                    })
                }
                'a'..='z' | 'A'..='Z' | '_' | '$' => {
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
                    })
                }
                '0'..='9' => {
                    let value = self.consume_number();
                    Some(Token {
                        token_type: TokenType::Literal,
                        value,
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                    })
                }
                '"' | '\'' | '`' => {
                    let value = self.consume_string(ch);
                    Some(Token {
                        token_type: TokenType::Literal,
                        value,
                        position: Position {
                            start: start_pos,
                            end: self.position,
                            line: start_line,
                            column: start_column,
                        },
                    })
                }
                '/' => {
                    if self.peek_char() == Some('/') || self.peek_char() == Some('*') {
                        let value = self.consume_comment();
                        Some(Token {
                            token_type: TokenType::Comment,
                            value,
                            position: Position {
                                start: start_pos,
                                end: self.position,
                                line: start_line,
                                column: start_column,
                            },
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
                    })
                }
                _ => {
                    self.advance();
                    None
                }
            }
        } else {
            None
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
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

    fn consume_number(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.current_char() {
            if ch.is_numeric() || ch == '.' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn consume_string(&mut self, quote: char) -> String {
        let start = self.position;
        self.advance(); // consume opening quote
        
        while let Some(ch) = self.current_char() {
            if ch == quote {
                self.advance(); // consume closing quote
                break;
            } else if ch == '\\' {
                self.advance(); // consume backslash
                if self.current_char().is_some() {
                    self.advance(); // consume escaped character
                }
            } else {
                self.advance();
            }
        }
        
        self.input[start..self.position].to_string()
    }

    fn consume_comment(&mut self) -> String {
        let start = self.position;
        
        if self.peek_char() == Some('/') {
            // Single line comment
            while let Some(ch) = self.current_char() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                self.advance();
            }
        } else if self.peek_char() == Some('*') {
            // Multi-line comment
            self.advance(); // consume '/'
            self.advance(); // consume '*'
            
            while self.position < self.input.len() - 1 {
                if self.current_char() == Some('*') && self.peek_char() == Some('/') {
                    self.advance(); // consume '*'
                    self.advance(); // consume '/'
                    break;
                }
                self.advance();
            }
        }
        
        self.input[start..self.position].to_string()
    }

    fn consume_operator(&mut self) -> String {
        let start = self.position;
        let ch = self.current_char().unwrap();
        self.advance();
        
        // Handle multi-character operators
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
        matches!(value, 
            "abstract" | "any" | "as" | "asserts" | "bigint" | "boolean" | "break" | "case" |
            "catch" | "class" | "const" | "continue" | "debugger" | "declare" | "default" |
            "delete" | "do" | "else" | "enum" | "export" | "extends" | "false" | "finally" |
            "for" | "from" | "function" | "get" | "if" | "implements" | "import" | "in" |
            "infer" | "instanceof" | "interface" | "is" | "keyof" | "let" | "module" | 
            "namespace" | "never" | "new" | "null" | "number" | "object" | "of" | "package" |
            "private" | "protected" | "public" | "readonly" | "require" | "return" | "set" |
            "static" | "string" | "super" | "switch" | "symbol" | "this" | "throw" | "true" |
            "try" | "type" | "typeof" | "undefined" | "unique" | "unknown" | "var" | "void" |
            "while" | "with" | "yield"
        )
    }
}
