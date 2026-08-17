use crate::error::CompilerError;

#[derive(Debug, Clone)]
pub struct ErrorReport {
    pub errors: Vec<CompilerError>,
}

impl ErrorReport {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
    
    pub fn add(&mut self, error: CompilerError) {
        self.errors.push(error);
    }
}
