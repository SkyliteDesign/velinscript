pub struct AgentStdlib;

impl AgentStdlib {
    pub fn generate_agent_runtime_code() -> String {
        r#"
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

// Global memory store for agents
static AGENT_MEMORY: Lazy<Arc<Mutex<HashMap<String, serde_json::Value>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub struct Agent {
    pub name: String,
}

impl Agent {
    pub fn new(name: &str) -> Self {
        Agent { name: name.to_string() }
    }

    pub fn think(&self, context: &str) -> String {
        format!("Agent {} is thinking about: {}", self.name, context)
    }
}

pub struct AgentMemory;
impl AgentMemory {
    pub fn store(key: &str, value: serde_json::Value) {
        let mut mem = AGENT_MEMORY.lock().unwrap();
        mem.insert(key.to_string(), value);
    }

    pub fn get(key: &str) -> Option<serde_json::Value> {
        let mem = AGENT_MEMORY.lock().unwrap();
        mem.get(key).cloned()
    }

    pub fn delete(key: &str) {
        let mut mem = AGENT_MEMORY.lock().unwrap();
        mem.remove(key);
    }

    pub fn search(query: &str) -> Vec<serde_json::Value> {
        let mem = AGENT_MEMORY.lock().unwrap();
        // Simple search: value contains query string
        mem.iter()
            .filter(|(k, v)| k.contains(query) || v.to_string().contains(query))
            .map(|(_, v)| v.clone())
            .collect()
    }
}

pub struct AgentTask;
impl AgentTask {
    pub fn run(description: &str) -> String {
        format!("Executed task: {}", description)
    }

    pub fn plan(goal: &str) -> Vec<String> {
        vec![
            format!("Analyze {}", goal),
            format!("Plan for {}", goal),
            format!("Execute {}", goal),
        ]
    }

    pub fn execute(plan: Vec<String>) -> String {
        format!("Executed {} steps", plan.len())
    }
}
"#
        .to_string()
    }

    pub fn generate_memory_store_code(key: &str, value: &str) -> String {
        format!("AgentMemory::store({}, {})", key, value)
    }

    pub fn generate_memory_get_code(key: &str) -> String {
        format!("AgentMemory::get({})", key)
    }

    pub fn generate_memory_delete_code(key: &str) -> String {
        format!("AgentMemory::delete({})", key)
    }

    pub fn generate_memory_search_code(query: &str) -> String {
        format!("AgentMemory::search({})", query)
    }

    pub fn generate_task_run_code(description: &str) -> String {
        format!("AgentTask::run({})", description)
    }

    pub fn generate_task_plan_code(goal: &str) -> String {
        format!("AgentTask::plan({})", goal)
    }

    pub fn generate_task_execute_code(plan: &str) -> String {
        format!("AgentTask::execute({})", plan)
    }

    pub fn generate_agent_create_code(name: &str) -> String {
        format!("Agent::new({})", name)
    }

    pub fn generate_agent_think_code(agent: &str, context: &str) -> String {
        format!("{}.think({})", agent, context)
    }
}
