// Breakpoint Management

use serde_json::{Value, Map};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: usize,
    pub file: PathBuf,
    pub line: usize,
    pub condition: Option<String>,
    pub hit_count: u64,
}

pub struct BreakpointManager {
    breakpoints: HashMap<usize, Breakpoint>,
    next_id: usize,
}

impl BreakpointManager {
    pub fn new() -> Self {
        BreakpointManager {
            breakpoints: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn set_breakpoints(&mut self, args: &Value) -> Value {
        let mut result = Map::new();
        let mut breakpoints = Vec::new();

        if let Some(source) = args.get("source") {
            if let Some(path) = source.get("path").and_then(|v| v.as_str()) {
                // Use path variable - convert to PathBuf for breakpoint tracking
                let path_buf = std::path::PathBuf::from(path);
                
                if let Some(lines) = args.get("breakpoints").and_then(|v| v.as_array()) {
                    for line_obj in lines {
                        if let Some(line) = line_obj.get("line").and_then(|v| v.as_u64()) {
                            // Add breakpoint using the path
                            let id = self.add_breakpoint(path_buf.clone(), line as usize, None);
                            
                            let mut bp = Map::new();
                            bp.insert("id".to_string(), Value::Number(id.into()));
                            bp.insert("verified".to_string(), Value::Bool(true));
                            bp.insert("line".to_string(), Value::Number(line.into()));
                            breakpoints.push(Value::Object(bp));
                        }
                    }
                }
            }
        }

        result.insert("breakpoints".to_string(), Value::Array(breakpoints));
        Value::Object(result)
    }

    pub fn add_breakpoint(&mut self, file: PathBuf, line: usize, condition: Option<String>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        self.breakpoints.insert(id, Breakpoint {
            id,
            file,
            line,
            condition,
            hit_count: 0,
        });
        
        id
    }

    pub fn remove_breakpoint(&mut self, id: usize) -> bool {
        self.breakpoints.remove(&id).is_some()
    }

    pub fn get_breakpoint(&self, id: usize) -> Option<&Breakpoint> {
        self.breakpoints.get(&id)
    }

    pub fn get_breakpoints_at_line(&self, file: &PathBuf, line: usize) -> Vec<&Breakpoint> {
        self.breakpoints.values()
            .filter(|bp| &bp.file == file && bp.line == line)
            .collect()
    }
}

impl Default for BreakpointManager {
    fn default() -> Self {
        Self::new()
    }
}
