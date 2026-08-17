// Variable Inspection

use serde_json::{Value, Map};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub var_type: String,
    pub children: Vec<Variable>,
}

pub struct VariableInspector {
    variables: HashMap<usize, Variable>,
    next_ref: usize,
}

impl VariableInspector {
    pub fn new() -> Self {
        VariableInspector {
            variables: HashMap::new(),
            next_ref: 1,
        }
    }

    pub fn get_variables(&self, args: &Value) -> Value {
        let mut vars = Vec::new();
        
        // Use args to determine which variables to return
        let _variables_ref = args.get("variablesReference").and_then(|v| v.as_u64()).unwrap_or(0);

        // In production, get actual variables from current scope
        // For now, return mock variables
        let mut var1 = Map::new();
        var1.insert("name".to_string(), Value::String("x".to_string()));
        var1.insert("value".to_string(), Value::String("42".to_string()));
        var1.insert("type".to_string(), Value::String("number".to_string()));
        var1.insert("variablesReference".to_string(), Value::Number(0.into()));
        vars.push(Value::Object(var1));

        let mut var2 = Map::new();
        var2.insert("name".to_string(), Value::String("name".to_string()));
        var2.insert("value".to_string(), Value::String("\"John\"".to_string()));
        var2.insert("type".to_string(), Value::String("string".to_string()));
        var2.insert("variablesReference".to_string(), Value::Number(0.into()));
        vars.push(Value::Object(var2));

        Value::Array(vars)
    }

    pub fn add_variable(&mut self, name: String, value: String, var_type: String) -> usize {
        let ref_id = self.next_ref;
        self.next_ref += 1;
        
        self.variables.insert(ref_id, Variable {
            name,
            value,
            var_type,
            children: Vec::new(),
        });
        
        ref_id
    }

    pub fn get_variable(&self, ref_id: usize) -> Option<&Variable> {
        self.variables.get(&ref_id)
    }
}

impl Default for VariableInspector {
    fn default() -> Self {
        Self::new()
    }
}
