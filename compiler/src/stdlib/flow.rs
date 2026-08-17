
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::SystemTime;

/// Represents the state of a Flow execution
#[derive(Debug, Clone)]
pub struct FlowState {
    pub id: String,
    pub name: String,
    pub start_time: SystemTime,
    pub steps: Vec<FlowStep>,
    pub status: FlowStatus,
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct FlowStep {
    pub name: String,
    pub status: StepStatus,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Compensating,
    Compensated,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepStatus {
    Pending,
    Success,
    Failed,
}

/// The FlowManager orchestrates the execution of a flow
pub struct FlowManager {
    state: Arc<Mutex<FlowState>>,
}

impl FlowManager {
    pub fn new(name: &str) -> Self {
        FlowManager {
            state: Arc::new(Mutex::new(FlowState {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.to_string(),
                start_time: SystemTime::now(),
                steps: Vec::new(),
                status: FlowStatus::Pending,
                context: HashMap::new(),
            })),
        }
    }

    /// Starts the flow execution
    pub fn start(&self) {
        let mut state = self.state.lock().unwrap();
        state.status = FlowStatus::Running;
        println!("[Flow:{}] Started flow '{}'", state.id, state.name);
    }

    /// Records a successful step
    pub fn step_success(&self, name: &str, duration_ms: u64) {
        let mut state = self.state.lock().unwrap();
        state.steps.push(FlowStep {
            name: name.to_string(),
            status: StepStatus::Success,
            duration_ms,
        });
        println!("[Flow:{}] Step '{}' succeeded ({}ms)", state.id, name, duration_ms);
    }

    /// Records a failed step and triggers compensation logic
    pub fn step_failed(&self, name: &str, error: &str) {
        let mut state = self.state.lock().unwrap();
        state.steps.push(FlowStep {
            name: name.to_string(),
            status: StepStatus::Failed,
            duration_ms: 0,
        });
        state.status = FlowStatus::Failed;
        println!("[Flow:{}] Step '{}' failed: {}", state.id, name, error);
        
        // In a real implementation, this would trigger rollback logic
        self.compensate(&mut state);
    }

    /// Commits the flow (success)
    pub fn commit(&self) {
        let mut state = self.state.lock().unwrap();
        state.status = FlowStatus::Completed;
        let duration = state.start_time.elapsed().unwrap().as_millis();
        println!("[Flow:{}] Completed successfully in {}ms", state.id, duration);
    }

    /// Rolls back the flow (failure)
    pub fn rollback(&self) {
        let mut state = self.state.lock().unwrap();
        if state.status != FlowStatus::Compensated {
            self.compensate(&mut state);
        }
    }

    fn compensate(&self, state: &mut FlowState) {
        println!("[Flow:{}] Initiating compensation sequence...", state.id);
        state.status = FlowStatus::Compensating;
        
        // Reverse iterate through successful steps
        for step in state.steps.iter().rev() {
            if step.status == StepStatus::Success {
                println!("[Flow:{}] Compensating step '{}'...", state.id, step.name);
                // Here we would call registered compensation hooks
            }
        }
        
        state.status = FlowStatus::Compensated;
        println!("[Flow:{}] Compensation completed.", state.id);
    }
}

// Global instance for generated code usage
use once_cell::sync::Lazy;

pub static GLOBAL_FLOW_MANAGER: Lazy<FlowManager> = Lazy::new(|| {
    FlowManager::new("GlobalFlow")
});

pub struct FlowStdlib;

impl FlowStdlib {
    pub fn generate_start_code(name: Option<&str>) -> String {
        if let Some(_n) = name {
            format!(
                "{{ 
                    // Note: resetting name on global flow manager not fully supported in this simple binding
                    crate::stdlib::flow::GLOBAL_FLOW_MANAGER.start(); 
                }}"
            )
        } else {
            "crate::stdlib::flow::GLOBAL_FLOW_MANAGER.start()".to_string()
        }
    }
    
    pub fn generate_checkpoint_code(step_name: &str) -> String {
        format!(
            "crate::stdlib::flow::GLOBAL_FLOW_MANAGER.step_success({}, 0)",
            step_name
        )
    }
    
    pub fn generate_fail_code(step_name: &str, error: &str) -> String {
        format!(
            "crate::stdlib::flow::GLOBAL_FLOW_MANAGER.step_failed({}, {})",
            step_name, error
        )
    }
    
    pub fn generate_commit_code() -> String {
        "crate::stdlib::flow::GLOBAL_FLOW_MANAGER.commit()".to_string()
    }
    
    pub fn generate_flow_runtime_code() -> String {
        r#"
        // --- VelinFlow Runtime ---
        use std::sync::{Arc, Mutex};
        use std::time::SystemTime;
        use std::collections::HashMap;
        use once_cell::sync::Lazy;

        #[derive(Debug, Clone)]
        pub struct FlowState {
            pub id: String,
            pub name: String,
            pub start_time: SystemTime,
            pub steps: Vec<FlowStep>,
            pub status: FlowStatus,
        }

        #[derive(Debug, Clone)]
        pub struct FlowStep {
            pub name: String,
            pub status: StepStatus,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum FlowStatus {
            Pending, Running, Completed, Failed, Compensated
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum StepStatus { Success, Failed }

        pub struct FlowManager {
            pub state: Arc<Mutex<FlowState>>,
        }

        impl FlowManager {
            pub fn new(name: &str) -> Self {
                FlowManager {
                    state: Arc::new(Mutex::new(FlowState {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: name.to_string(),
                        start_time: SystemTime::now(),
                        steps: Vec::new(),
                        status: FlowStatus::Pending,
                    })),
                }
            }

            pub fn start(&self) {
                let mut state = self.state.lock().unwrap();
                state.status = FlowStatus::Running;
                println!("Flow started: {} ({})", state.name, state.id);
            }

            pub fn step_success(&self, name: &str, duration_ms: u64) {
                let mut state = self.state.lock().unwrap();
                state.steps.push(FlowStep {
                    name: name.to_string(),
                    status: StepStatus::Success,
                });
                println!("Step succeeded: {}", name);
            }

            pub fn step_failed(&self, name: &str, error: &str) {
                let mut state = self.state.lock().unwrap();
                state.steps.push(FlowStep {
                    name: name.to_string(),
                    status: StepStatus::Failed,
                });
                state.status = FlowStatus::Failed;
                println!("Step failed: {} - {}", name, error);
                self.rollback(error);
            }

            pub fn commit(&self) {
                let mut state = self.state.lock().unwrap();
                state.status = FlowStatus::Completed;
                let duration = state.start_time.elapsed().unwrap_or_default().as_millis();
                println!("Flow completed successfully: {} ({}ms)", state.name, duration);
            }

            pub fn rollback(&self, error: &str) {
                let mut state = self.state.lock().unwrap();
                // Avoid double rollback
                if state.status == FlowStatus::Compensated {
                    return;
                }
                state.status = FlowStatus::Failed;
                println!("Flow failed, initiating rollback: {} - {}", state.name, error);
                state.status = FlowStatus::Compensated;
            }
        }

        pub static GLOBAL_FLOW_MANAGER: Lazy<FlowManager> = Lazy::new(|| {
            FlowManager::new("GlobalFlow")
        });
        "#.to_string()
    }
}
