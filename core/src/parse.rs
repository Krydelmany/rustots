use serde::{Deserialize, Serialize};
use crate::lexer::Token;
use crate::diagnostics::DiagnosticCollector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub ast: Option<AstNode>,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstNode {
    pub node_type: AstNodeType,
    pub value: Option<String>,
    pub children: Vec<AstNode>,
    pub position: Option<crate::lexer::Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AstNodeType {
    Program,
    VariableDeclaration,
    FunctionDeclaration,
    ClassDeclaration,
    InterfaceDeclaration,
    TypeAlias,
    ImportDeclaration,
    ExportDeclaration,
    Block,
    Expression,
    Statement,
    Identifier,
    Literal,
    TypeAnnotation,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult {
        // TODO: Implement actual parsing logic
        // For now, return a placeholder AST
        let ast = AstNode {
            node_type: AstNodeType::Program,
            value: None,
            children: vec![],
            position: None,
        };

        ParseResult {
            ast: Some(ast),
            diagnostics: vec![],
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.current_token()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
