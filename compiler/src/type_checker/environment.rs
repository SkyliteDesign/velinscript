use crate::parser::ast::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Type>,
    functions: HashMap<String, FunctionSignature>,
    types: HashMap<String, Type>,
    parent: Option<Box<Environment>>,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<ParameterInfo>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: Type,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            types: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            types: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    pub fn define_variable(&mut self, name: String, var_type: Type) {
        self.variables.insert(name, var_type);
    }
    
    pub fn get_variable(&self, name: &str) -> Option<Type> {
        if let Some(var_type) = self.variables.get(name) {
            Some(var_type.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get_variable(name)
        } else {
            None
        }
    }
    
    pub fn define_function(&mut self, name: String, signature: FunctionSignature) {
        self.functions.insert(name, signature);
    }
    
    pub fn get_function(&self, name: &str) -> Option<FunctionSignature> {
        if let Some(sig) = self.functions.get(name) {
            Some(sig.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get_function(name)
        } else {
            None
        }
    }
    
    pub fn define_type(&mut self, name: String, type_def: Type) {
        self.types.insert(name, type_def);
    }
    
    pub fn get_type(&self, name: &str) -> Option<Type> {
        if let Some(type_def) = self.types.get(name) {
            Some(type_def.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get_type(name)
        } else {
            None
        }
    }
    
    pub fn has_variable(&self, name: &str) -> bool {
        self.get_variable(name).is_some()
    }
    
    pub fn has_function(&self, name: &str) -> bool {
        self.get_function(name).is_some()
    }
    
    pub fn has_type(&self, name: &str) -> bool {
        self.get_type(name).is_some()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
