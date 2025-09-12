use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Primitive(PrimitiveType),
    Object(ObjectType),
    Array(Box<Type>),
    Function(FunctionType),
    Union(Vec<Type>),
    Intersection(Vec<Type>),
    Tuple(Vec<Type>),
    Literal(LiteralType),
    Generic(String),
    Unknown,
    Never,
    Void,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveType {
    String,
    Number,
    Boolean,
    BigInt,
    Symbol,
    Undefined,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectType {
    pub properties: HashMap<String, Type>,
    pub methods: HashMap<String, FunctionType>,
    pub index_signature: Option<(Type, Type)>, // key type, value type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionType {
    pub parameters: Vec<Parameter>,
    pub return_type: Box<Type>,
    pub type_parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub optional: bool,
    pub rest: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Boolean(bool),
}

pub struct TypeChecker {
    type_context: HashMap<String, Type>,
    current_scope: Vec<HashMap<String, Type>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            type_context: HashMap::new(),
            current_scope: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_scope.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.current_scope.pop();
    }

    pub fn declare_variable(&mut self, name: String, var_type: Type) {
        if let Some(scope) = self.current_scope.last_mut() {
            scope.insert(name, var_type);
        }
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&Type> {
        for scope in self.current_scope.iter().rev() {
            if let Some(var_type) = scope.get(name) {
                return Some(var_type);
            }
        }
        None
    }

    pub fn check_type_compatibility(&self, expected: &Type, actual: &Type) -> bool {
        // TODO: Implement proper type compatibility checking
        // For now, just check for exact matches
        match (expected, actual) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            (Type::Primitive(p1), Type::Primitive(p2)) => p1 == p2,
            // Add more cases as needed
            _ => false,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
