use crate::lexer::{Token, TokenType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: String },
    UnexpectedEOF,
    InvalidSyntax(String),
}

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
    UnaryExpression {
        operator: String,
        argument: Box<Expression>,
        prefix: bool,
    },
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
        // Filtramos tokens que não afetam a sintaxe (espaços, comentários, quebras de linha)
        // Isso simplifica muito a lógica do parser, pois não precisamos ficar pulando eles manualmente toda hora
        let tokens = tokens
            .into_iter()
            .filter(|t| !matches!(t.token_type, TokenType::Whitespace | TokenType::Comment | TokenType::Newline))
            .collect();
        
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut body = Vec::new();

        while !self.is_at_end() {
            let stmt = self.parse_statement()?;
            body.push(stmt);
        }

        Ok(Program { body })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
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
            return Ok(Statement::BlockStatement(self.parse_block_statement()?));
        }

        self.parse_expression_statement()
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, ParseError> {
        let id = self.parse_identifier()?;
        
        self.consume(TokenType::Punctuation, "(")?;
        let mut params = Vec::new();
        if !self.check_value(")") {
            loop {
                let param = self.parse_identifier()?;
                params.push(param);
                // TODO: Implementar análise de tipos completa.
                // Por enquanto, apenas consumimos a anotação de tipo para não quebrar o parser.
                if self.match_operator(":") {
                    self.consume_type_annotation();
                }
                
                if !self.match_punctuation(",") {
                    break;
                }
            }
        }
        self.consume(TokenType::Punctuation, ")")?;

        let mut return_type = None;
        if self.match_operator(":") {
            return_type = Some(self.consume_type_annotation());
        }

        let body = self.parse_block_statement()?;

        Ok(Statement::FunctionDeclaration {
            id,
            params,
            return_type,
            body,
        })
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParseError> {
        let kind = self.previous().value.clone();
        let mut declarations = Vec::new();

        loop {
            let id = self.parse_identifier()?;
            let mut init = None;

            if self.match_operator("=") {
                init = Some(self.parse_expression()?);
            }

            declarations.push(VariableDeclarator { id, init });

            if !self.match_punctuation(",") {
                break;
            }
        }

        self.consume(TokenType::Punctuation, ";")?;

        Ok(Statement::VariableDeclaration { kind, declarations })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let mut argument = None;
        if !self.check(TokenType::Punctuation) || self.peek().unwrap().value != ";" {
            argument = Some(self.parse_expression()?);
        }
        self.consume(TokenType::Punctuation, ";")?;
        Ok(Statement::ReturnStatement { argument })
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParseError> {
        self.consume(TokenType::Punctuation, "{")?;
        let mut body = Vec::new();
        while !self.check_value("}") && !self.is_at_end() {
            let stmt = self.parse_statement()?;
            body.push(stmt);
        }
        self.consume(TokenType::Punctuation, "}")?;
        Ok(BlockStatement { body })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expression = self.parse_expression()?;
        self.consume(TokenType::Punctuation, ";")?;
        Ok(Statement::ExpressionStatement { expression })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_logical_and()?;

        while self.match_operator("||") {
            let operator = self.previous().value.clone();
            let right = self.parse_logical_and()?;
            left = Expression::BinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_equality()?;

        while self.match_operator("&&") {
            let operator = self.previous().value.clone();
            let right = self.parse_equality()?;
            left = Expression::BinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_relational()?;

        while self.match_operator("==") || self.match_operator("!=") {
            let operator = self.previous().value.clone();
            let right = self.parse_relational()?;
            left = Expression::BinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_relational(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_additive()?;

        while self.match_operator("<") || self.match_operator(">") || self.match_operator("<=") || self.match_operator(">=") {
            let operator = self.previous().value.clone();
            let right = self.parse_additive()?;
            left = Expression::BinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_multiplicative()?;

        while self.match_operator("+") || self.match_operator("-") {
            let operator = self.previous().value.clone();
            let right = self.parse_multiplicative()?;
            left = Expression::BinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_unary()?;

        while self.match_operator("*") || self.match_operator("/") {
            let operator = self.previous().value.clone();
            let right = self.parse_unary()?;
            left = Expression::BinaryExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if self.match_operator("!") || self.match_operator("-") || self.match_keyword("typeof") {
            let operator = self.previous().value.clone();
            let argument = self.parse_unary()?;
            Ok(Expression::UnaryExpression {
                operator,
                argument: Box::new(argument),
                prefix: true,
            })
        } else {
            self.parse_member_call_expression()
        }
    }

    fn parse_member_call_expression(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_punctuation("(") {
                expr = self.finish_call(expr)?;
            } else if self.match_punctuation(".") {
                let property = self.parse_identifier()?;
                expr = Expression::MemberExpression {
                    object: Box::new(expr),
                    property: Box::new(Expression::Identifier(property)),
                    computed: false,
                };
            } else if self.match_punctuation("[") {
                let property = self.parse_expression()?;
                self.consume(TokenType::Punctuation, "]")?;
                expr = Expression::MemberExpression {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: true,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParseError> {
        let mut arguments = Vec::new();
        if !self.check_value(")") {
            loop {
                arguments.push(self.parse_expression()?);
                if !self.match_punctuation(",") {
                    break;
                }
            }
        }
        self.consume(TokenType::Punctuation, ")")?;

        Ok(Expression::CallExpression {
            callee: Box::new(callee),
            arguments,
        })
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        if self.match_type(TokenType::Literal) {
            return Ok(Expression::Literal {
                value: self.previous().value.clone(),
                raw: self.previous().value.clone(),
            });
        }
        if self.match_keyword("true") {
            return Ok(Expression::Literal {
                value: "true".to_string(),
                raw: "true".to_string(),
            });
        }
        if self.match_keyword("false") {
            return Ok(Expression::Literal {
                value: "false".to_string(),
                raw: "false".to_string(),
            });
        }
        if self.match_type(TokenType::Identifier) {
            return Ok(Expression::Identifier(Identifier {
                name: self.previous().value.clone(),
            }));
        }
        if self.match_punctuation("(") {
            let expr = self.parse_expression()?;
            self.consume(TokenType::Punctuation, ")")?;
            return Ok(expr);
        }
        
        Err(ParseError::UnexpectedToken {
            expected: "Expressão".to_string(),
            found: self.peek().map(|t| t.value.clone()).unwrap_or_else(|| "Fim de Arquivo".to_string()),
        })
    }

    fn parse_identifier(&mut self) -> Result<Identifier, ParseError> {
        if self.match_type(TokenType::Identifier) {
            Ok(Identifier {
                name: self.previous().value.clone(),
            })
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "Identificador".to_string(),
                found: self.peek().map(|t| t.value.clone()).unwrap_or_else(|| "Fim de Arquivo".to_string()),
            })
        }
    }

    // Helpers
    fn consume_type_annotation(&mut self) -> String {
    // Função auxiliar para pular a anotação de tipo (ex: : string)
    // Necessário porque nossa AST ainda não guarda informações de tipo complexas
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

    fn consume(&mut self, token_type: TokenType, value: &str) -> Result<&Token, ParseError> {
        if self.check(token_type.clone()) && self.peek().unwrap().value == value {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: value.to_string(),
                found: self.peek().map(|t| t.value.clone()).unwrap_or_else(|| "Fim de Arquivo".to_string()),
            })
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


