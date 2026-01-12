use crate::error::{CompilerError, ErrorLocation};

#[derive(Debug, Clone)]
pub struct TypeError {
    pub kind: TypeErrorKind,
    pub message: String,
    pub location: Option<ErrorLocation>,
}

impl From<TypeError> for CompilerError {
    fn from(err: TypeError) -> Self {
        let location = err.location.unwrap_or_else(|| ErrorLocation::new(0, 0));
        let kind = format!("{:?}", err.kind);
        CompilerError::type_error_with_kind(err.message, location, kind)
    }
}

#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    TypeMismatch {
        expected: String,
        found: String,
    },
    UndefinedVariable(String),
    UndefinedFunction(String),
    UndefinedType(String),
    DuplicateDefinition(String),
    CannotInferType,
    InvalidOperation {
        operation: String,
        operand_type: String,
    },
    MissingReturn,
    WrongArgumentCount {
        expected: usize,
        found: usize,
    },
    InvalidArgumentType {
        position: usize,
        expected: String,
        found: String,
    },
    InvalidMemberAccess,
}

impl TypeError {
    pub fn new(kind: TypeErrorKind, message: String) -> Self {
        TypeError {
            kind,
            message,
            location: None,
        }
    }
    
    pub fn with_location(mut self, location: ErrorLocation) -> Self {
        self.location = Some(location);
        self
    }
    
    pub fn type_mismatch(expected: &str, found: &str) -> Self {
        let message = format!("Type mismatch: expected {}, found {}", expected, found);
        TypeError::new(
            TypeErrorKind::TypeMismatch {
                expected: expected.to_string(),
                found: found.to_string(),
            },
            message,
        )
    }
    
    pub fn undefined_variable(name: &str) -> Self {
        let message = format!("Undefined variable: {}", name);
        TypeError::new(TypeErrorKind::UndefinedVariable(name.to_string()), message)
    }
    
    pub fn undefined_function(name: &str) -> Self {
        let message = format!("Undefined function: {}", name);
        TypeError::new(TypeErrorKind::UndefinedFunction(name.to_string()), message)
    }
    
    pub fn undefined_type(name: &str) -> Self {
        let message = format!("Undefined type: {}", name);
        TypeError::new(TypeErrorKind::UndefinedType(name.to_string()), message)
    }
    
    pub fn cannot_infer_type() -> Self {
        let message = "Cannot infer type. Please provide explicit type annotation.".to_string();
        TypeError::new(TypeErrorKind::CannotInferType, message)
    }
    
    pub fn invalid_operation(operation: &str, operand_type: &str) -> Self {
        let message = format!("Invalid operation '{}' on type {}", operation, operand_type);
        TypeError::new(
            TypeErrorKind::InvalidOperation {
                operation: operation.to_string(),
                operand_type: operand_type.to_string(),
            },
            message,
        )
    }
    
    pub fn missing_return() -> Self {
        let message = "Function must return a value".to_string();
        TypeError::new(TypeErrorKind::MissingReturn, message)
    }
    
    pub fn wrong_argument_count(expected: usize, found: usize) -> Self {
        let message = format!("Wrong argument count: expected {}, found {}", expected, found);
        TypeError::new(
            TypeErrorKind::WrongArgumentCount { expected, found },
            message,
        )
    }
}
