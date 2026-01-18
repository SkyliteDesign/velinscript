use crate::parser::ast::{Type, Struct, Enum};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub variables: HashMap<String, Type>,
    pub functions: HashMap<String, FunctionSignature>,
    pub types: HashMap<String, Type>,
    pub structs: HashMap<String, Struct>,
    pub enums: HashMap<String, Enum>,
    pub modules: HashMap<String, Box<Environment>>, // Make public for debug
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
        let mut functions = HashMap::new();
        
        // Add built-in Error function
        functions.insert("Error".to_string(), FunctionSignature {
            name: "Error".to_string(),
            params: vec![ParameterInfo {
                name: "message".to_string(),
                param_type: Type::String,
            }],
            return_type: Some(Type::Named("Error".to_string())),
        });

        Environment {
            variables: HashMap::new(),
            functions,
            types: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            modules: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            types: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            modules: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    pub fn define_module(&mut self, name: String, env: Environment) {
        self.modules.insert(name, Box::new(env));
    }

    pub fn get_module(&self, name: &str) -> Option<Environment> {
        if let Some(env) = self.modules.get(name) {
            Some(*env.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get_module(name)
        } else {
            None
        }
    }

    pub fn set_parent(&mut self, parent: Environment) {
        self.parent = Some(Box::new(parent));
    }

    pub fn define_variable(&mut self, name: String, var_type: Type) {
        self.variables.insert(name, var_type);
    }
    
    pub fn get_variable(&self, name: &str) -> Option<Type> {
        if let Some((module_name, rest)) = name.split_once('.') {
            if let Some(module_env) = self.get_module(module_name) {
                return module_env.get_variable(rest);
            }
        }

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
        if let Some((module_name, rest)) = name.split_once('.') {
            if let Some(module_env) = self.get_module(module_name) {
                return module_env.get_function(rest);
            }
        }
        
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
        if let Some((module_name, rest)) = name.split_once('.') {
            if let Some(module_env) = self.get_module(module_name) {
                return module_env.get_type(rest);
            }
        }

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
    
    pub fn define_struct(&mut self, name: String, struct_def: Struct) {
        self.structs.insert(name, struct_def);
    }
    
    pub fn get_struct(&self, name: &str) -> Option<Struct> {
        if let Some((module_name, rest)) = name.split_once('.') {
            if let Some(module_env) = self.get_module(module_name) {
                return module_env.get_struct(rest);
            }
        }

        if let Some(struct_def) = self.structs.get(name) {
            Some(struct_def.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get_struct(name)
        } else {
            None
        }
    }
    
    pub fn define_enum(&mut self, name: String, enum_def: Enum) {
        self.enums.insert(name, enum_def);
    }
    
    pub fn get_enum(&self, name: &str) -> Option<Enum> {
        if let Some((module_name, rest)) = name.split_once('.') {
            if let Some(module_env) = self.get_module(module_name) {
                return module_env.get_enum(rest);
            }
        }

        if let Some(enum_def) = self.enums.get(name) {
            Some(enum_def.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get_enum(name)
        } else {
            None
        }
    }
    pub fn get_all_function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
