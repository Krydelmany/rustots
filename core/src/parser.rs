use crate::lexer::{Token, TokenType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Statement>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Statement {
    FunctionDeclaration {
        id: Identifier,
        params: Vec<Identifier>,
        return_type: Option<String>,
        body: BlockStatement,
    },
    VariableDeclaration {
        kind: String, // const, let, var
        declarations: Vec<VariableDeclarator>,
    },
    ExpressionStatement {
        expression: Expression,
    },
    ReturnStatement {
        argument: Option<Expression>,
    },
    BlockStatement(BlockStatement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStatement {
    pub body: Vec<Statement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: Option<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    BinaryExpression {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    CallExpression {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    MemberExpression {
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
    },
    Literal {
        value: String,
        raw: String,
    },
    Identifier(Identifier),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        // Filter out whitespace and comments for parsing
        let tokens = tokens
            .into_iter()
            .filter(|t| !matches!(t.token_type, TokenType::Whitespace | TokenType::Comment | TokenType::Newline))
            .collect();
        
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Program {
        let mut body = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                // Panic recovery or skip token could go here
                self.advance();
            }
        }

        Program { body }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.match_keyword("function") {
            return self.parse_function_declaration();
        }
        if self.match_keyword("const") || self.match_keyword("let") || self.match_keyword("var") {
            return self.parse_variable_declaration();
        }
        if self.match_keyword("return") {
            return self.parse_return_statement();
        }
        if self.check(TokenType::Punctuation) && self.peek().unwrap().value == "{" {
            return Some(Statement::BlockStatement(self.parse_block_statement()));
        }

        self.parse_expression_statement()
    }

    fn parse_function_declaration(&mut self) -> Option<Statement> {
        let id = self.parse_identifier()?;
        
        self.consume(TokenType::Punctuation, "(");
        let mut params = Vec::new();
        if !self.check_value(")") {
            loop {
                if let Some(param) = self.parse_identifier() {
                    params.push(param);
                    // Skip type annotation if present
                    if self.match_punctuation(":") {
                        self.consume_type_annotation();
                    }
                }
                if !self.match_punctuation(",") {
                    break;
                }
            }
        }
        self.consume(TokenType::Punctuation, ")");

        let mut return_type = None;
        if self.match_punctuation(":") {
            return_type = Some(self.consume_type_annotation());
        }

        let body = self.parse_block_statement();

        Some(Statement::FunctionDeclaration {
            id,
            params,
            return_type,
            body,
        })
    }

    fn parse_variable_declaration(&mut self) -> Option<Statement> {
        let kind = self.previous().value.clone();
        let mut declarations = Vec::new();

        loop {
            let id = self.parse_identifier()?;
            let mut init = None;

            if self.match_operator("=") {
                init = self.parse_expression();
            }

            declarations.push(VariableDeclarator { id, init });

            if !self.match_punctuation(",") {
                break;
            }
        }

        self.consume(TokenType::Punctuation, ";");

        Some(Statement::VariableDeclaration { kind, declarations })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let mut argument = None;
        if !self.check(TokenType::Punctuation) || self.peek().unwrap().value != ";" {
            argument = self.parse_expression();
        }
        self.consume(TokenType::Punctuation, ";");
        Some(Statement::ReturnStatement { argument })
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        self.consume(TokenType::Punctuation, "{");
        let mut body = Vec::new();
        while !self.check_value("}") && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                self.advance();
            }
        }
        self.consume(TokenType::Punctuation, "}");
        BlockStatement { body }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression()?;
        self.consume(TokenType::Punctuation, ";");
        Some(Statement::ExpressionStatement { expression })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_binary_expression()
    }

    fn parse_binary_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_call_expression()?;

        while self.match_operator("+") || self.match_operator("-") || self.match_operator("*") || self.match_operator("/") {
            let operator = self.previous().value.clone();
            let right = self.parse_call_expression()?;
            left = Expression::BinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_call_expression(&mut self) -> Option<Expression> {
        let mut expr = self.parse_primary()?;

        while self.match_punctuation("(") {
            let mut arguments = Vec::new();
            if !self.check_value(")") {
                loop {
                    if let Some(arg) = self.parse_expression() {
                        arguments.push(arg);
                    }
                    if !self.match_punctuation(",") {
                        break;
                    }
                }
            }
            self.consume(TokenType::Punctuation, ")");
            expr = Expression::CallExpression {
                callee: Box::new(expr),
                arguments,
            };
        }

        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        if self.match_type(TokenType::Literal) {
            return Some(Expression::Literal {
                value: self.previous().value.clone(),
                raw: self.previous().value.clone(),
            });
        }
        if self.match_type(TokenType::Identifier) {
            return Some(Expression::Identifier(Identifier {
                name: self.previous().value.clone(),
            }));
        }
        None
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        if self.match_type(TokenType::Identifier) {
            Some(Identifier {
                name: self.previous().value.clone(),
            })
        } else {
            None
        }
    }

    // Helpers
    fn consume_type_annotation(&mut self) -> String {
        // Simple skipper for type annotations: string, number, etc.
        if self.match_type(TokenType::Identifier) || self.match_type(TokenType::Keyword) {
            self.previous().value.clone()
        } else {
            String::new()
        }
    }

    fn match_keyword(&mut self, keyword: &str) -> bool {
        if self.check(TokenType::Keyword) && self.peek().unwrap().value == keyword {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_operator(&mut self, op: &str) -> bool {
        if self.check(TokenType::Operator) && self.peek().unwrap().value == op {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_punctuation(&mut self, p: &str) -> bool {
        if self.check(TokenType::Punctuation) && self.peek().unwrap().value == p {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_type(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            // Simple enum variant check
            std::mem::discriminant(&self.peek().unwrap().token_type) == std::mem::discriminant(&token_type)
        }
    }

    fn check_value(&self, value: &str) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().unwrap().value == value
        }
    }

    fn consume(&mut self, token_type: TokenType, value: &str) -> Option<&Token> {
        if self.check(token_type.clone()) && self.peek().unwrap().value == value {
            Some(self.advance())
        } else {
            None
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
