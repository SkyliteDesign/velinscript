
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

// Helper struct to inject into generated code
pub struct FlowStdlib;

impl FlowStdlib {
    pub fn generate_flow_runtime_code() -> String {
        // Returns the code for the FlowManager and related structs
        // In a real compiler, this might be a separate crate, but here we inject it.
        // Since we are compiling this file as part of the compiler, we don't return self-code.
        // Instead, the CodeGenerator will use the `FlowManager` struct definition if we expose it via a crate,
        // or we generate the source code directly.
        // For this MVP, we will assume the runtime struct is available or generated.
        
        r#"
        // --- VelinFlow Runtime ---
        use std::sync::{Arc, Mutex};
        use std::time::SystemTime;
        use std::collections::HashMap;

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
                tracing::info!(flow_id = %state.id, flow_name = %state.name, "Flow started");
            }

            pub fn commit(&self) {
                let mut state = self.state.lock().unwrap();
                state.status = FlowStatus::Completed;
                let duration = state.start_time.elapsed().unwrap_or_default().as_millis();
                tracing::info!(flow_id = %state.id, duration_ms = %duration, "Flow completed successfully");
            }

            pub fn rollback(&self, error: &str) {
                let mut state = self.state.lock().unwrap();
                state.status = FlowStatus::Failed;
                tracing::error!(flow_id = %state.id, error = %error, "Flow failed, initiating rollback");
                // Compensation logic would go here
                state.status = FlowStatus::Compensated;
            }
        }
        "#.to_string()
    }
}
