// Debugger Core Logic

use std::path::PathBuf;

pub struct Debugger {
    program_path: Option<PathBuf>,
    is_running: bool,
    is_paused: bool,
    current_line: Option<usize>,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            program_path: None,
            is_running: false,
            is_paused: false,
            current_line: None,
        }
    }

    pub fn launch(&mut self, program: PathBuf) -> Result<(), String> {
        self.program_path = Some(program);
        self.is_running = true;
        self.is_paused = false;
        Ok(())
    }

    pub fn continue_execution(&mut self) {
        self.is_paused = false;
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn step_over(&mut self) -> Result<(), String> {
        // In production, execute one line and pause
        self.current_line = self.current_line.map(|l| l + 1);
        Ok(())
    }

    pub fn step_into(&mut self) -> Result<(), String> {
        // In production, step into function call
        self.current_line = self.current_line.map(|l| l + 1);
        Ok(())
    }

    pub fn step_out(&mut self) -> Result<(), String> {
        // In production, step out of current function
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn current_line(&self) -> Option<usize> {
        self.current_line
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}
