// Mock Manager
// Verwaltet Mock-Objekte für Tests

use std::collections::HashMap;
use anyhow::Result;

pub struct MockManager {
    mocks: HashMap<String, MockDefinition>,
}

struct MockDefinition {
    name: String,
    return_value: String,
}

impl MockManager {
    pub fn new() -> Self {
        Self {
            mocks: HashMap::new(),
        }
    }
    
    pub fn register_mock(&mut self, name: String, return_value: String) {
        self.mocks.insert(name.clone(), MockDefinition {
            name,
            return_value,
        });
    }
    
    pub fn get_mock(&self, name: &str) -> Option<&MockDefinition> {
        self.mocks.get(name)
    }
    
    pub fn clear(&mut self) {
        self.mocks.clear();
    }
}
