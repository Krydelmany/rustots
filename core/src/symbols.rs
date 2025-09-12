use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::Type;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolKind {
    Variable,
    Function,
    Class,
    Interface,
    Type,
    Namespace,
    Enum,
    EnumMember,
    Parameter,
    Property,
    Method,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub symbol_type: Type,
    pub location: Option<crate::lexer::Position>,
    pub exported: bool,
    pub modifiers: Vec<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<usize>, // Index to parent scope
    pub children: Vec<usize>,  // Indices to child scopes
}

pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
    global_scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let global_scope = Scope {
            symbols: HashMap::new(),
            parent: None,
            children: vec![],
        };

        Self {
            scopes: vec![global_scope],
            current_scope: 0,
            global_scope: 0,
        }
    }

    pub fn enter_scope(&mut self) -> usize {
        let new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(self.current_scope),
            children: vec![],
        };

        let scope_id = self.scopes.len();
        self.scopes.push(new_scope);
        
        // Add child reference to parent
        self.scopes[self.current_scope].children.push(scope_id);
        self.current_scope = scope_id;
        
        scope_id
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    pub fn declare_symbol(&mut self, symbol: Symbol) -> Result<(), String> {
        let current_scope = &mut self.scopes[self.current_scope];
        
        if current_scope.symbols.contains_key(&symbol.name) {
            return Err(format!("Symbol '{}' already declared in current scope", symbol.name));
        }

        current_scope.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        let mut current = Some(self.current_scope);
        
        while let Some(scope_id) = current {
            let scope = &self.scopes[scope_id];
            
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
            
            current = scope.parent;
        }
        
        None
    }

    pub fn lookup_symbol_in_scope(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        if let Some(scope) = self.scopes.get(scope_id) {
            scope.symbols.get(name)
        } else {
            None
        }
    }

    pub fn get_current_scope(&self) -> usize {
        self.current_scope
    }

    pub fn get_global_scope(&self) -> usize {
        self.global_scope
    }

    pub fn get_scope_symbols(&self, scope_id: usize) -> Option<&HashMap<String, Symbol>> {
        self.scopes.get(scope_id).map(|scope| &scope.symbols)
    }

    pub fn get_all_symbols(&self) -> Vec<&Symbol> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.symbols.values())
            .collect()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
