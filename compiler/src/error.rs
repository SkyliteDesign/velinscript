// Zentrale Error-Types für VelinScript Compiler
// Verwendet thiserror für automatische Error-Implementierungen

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ErrorLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl ErrorLocation {
    pub fn new(line: usize, column: usize) -> Self {
        ErrorLocation {
            line,
            column,
            file: None,
        }
    }
    
    pub fn with_file(line: usize, column: usize, file: String) -> Self {
        ErrorLocation {
            line,
            column,
            file: Some(file),
        }
    }
}

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Parse error: {message} at line {line}, column {column}")]
    Parse {
        message: String,
        location: ErrorLocation,
        expected: Option<String>,
        found: Option<String>,
        line: usize,
        column: usize,
    },
    
    #[error("Type error: {message} at line {line}, column {column}")]
    Type {
        message: String,
        location: ErrorLocation,
        kind: Option<String>,
        line: usize,
        column: usize,
    },
    
    #[error("Code generation error: {message}")]
    CodeGen {
        message: String,
        context: Option<String>,
    },
    
    #[error("IO error: {message}")]
    Io {
        message: String,
    },
    
    #[error("Validation error: {message}")]
    Validation {
        message: String,
        field: Option<String>,
    },
    
    #[error("Configuration error: {message}")]
    Config {
        message: String,
    },
    
    #[error("Internal error: {message}")]
    Internal {
        message: String,
    },
}

pub type CompilerResult<T> = Result<T, CompilerError>;

impl CompilerError {
    pub fn parse_error(message: String, location: ErrorLocation) -> Self {
        CompilerError::Parse {
            message,
            location: location.clone(),
            expected: None,
            found: None,
            line: location.line,
            column: location.column,
        }
    }
    
    pub fn parse_error_with_context(
        message: String,
        location: ErrorLocation,
        expected: String,
        found: String,
    ) -> Self {
        CompilerError::Parse {
            message,
            location: location.clone(),
            expected: Some(expected),
            found: Some(found),
            line: location.line,
            column: location.column,
        }
    }
    
    pub fn type_error(message: String, location: ErrorLocation) -> Self {
        CompilerError::Type {
            message,
            location: location.clone(),
            kind: None,
            line: location.line,
            column: location.column,
        }
    }
    
    pub fn type_error_with_kind(message: String, location: ErrorLocation, kind: String) -> Self {
        CompilerError::Type {
            message,
            location: location.clone(),
            kind: Some(kind),
            line: location.line,
            column: location.column,
        }
    }
    
    pub fn codegen_error(message: String) -> Self {
        CompilerError::CodeGen {
            message,
            context: None,
        }
    }
    
    pub fn codegen_error_with_context(message: String, context: String) -> Self {
        CompilerError::CodeGen {
            message,
            context: Some(context),
        }
    }
    
    pub fn io_error(message: String) -> Self {
        CompilerError::Io {
            message,
        }
    }
    
    pub fn validation_error(message: String, field: Option<String>) -> Self {
        CompilerError::Validation {
            message,
            field,
        }
    }
}

// Konvertierungen von Standard-Errors
impl From<std::io::Error> for CompilerError {
    fn from(err: std::io::Error) -> Self {
        CompilerError::Io {
            message: err.to_string(),
        }
    }
}
