// Standard Library für Result<T, E> Type
// Bietet Methoden wie unwrap(), unwrap_or(), map(), etc.

use std::collections::HashMap;
use std::fmt;

/// Velin Error mit Context und Stack Trace
#[derive(Debug)]
pub struct VelinError {
    pub message: String,
    pub error_type: String,
    pub context: HashMap<String, String>,
    pub stack_trace: Vec<String>,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Clone for VelinError {
    fn clone(&self) -> Self {
        VelinError {
            message: self.message.clone(),
            error_type: self.error_type.clone(),
            context: self.context.clone(),
            stack_trace: self.stack_trace.clone(),
            source: None, // Source kann nicht geklont werden
        }
    }
}

impl VelinError {
    /// Erstellt einen neuen Velin Error
    pub fn new(message: String, error_type: String) -> Self {
        VelinError {
            message,
            error_type,
            context: HashMap::new(),
            stack_trace: Vec::new(),
            source: None,
        }
    }

    /// Fügt Context hinzu
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }

    /// Fügt Stack Trace Eintrag hinzu
    pub fn add_stack_frame(mut self, frame: String) -> Self {
        self.stack_trace.push(frame);
        self
    }

    /// Setzt die Source-Error
    pub fn with_source(mut self, source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        self.source = Some(source);
        self
    }

    /// Gibt den Error als strukturierten String zurück
    pub fn to_structured_string(&self) -> String {
        let mut output = format!("[{}] {}\n", self.error_type, self.message);

        if !self.context.is_empty() {
            output.push_str("Context:\n");
            for (key, value) in &self.context {
                output.push_str(&format!("  {}: {}\n", key, value));
            }
        }

        if !self.stack_trace.is_empty() {
            output.push_str("Stack Trace:\n");
            for (i, frame) in self.stack_trace.iter().enumerate() {
                output.push_str(&format!("  {}: {}\n", i, frame));
            }
        }

        if let Some(ref source) = self.source {
            output.push_str(&format!("Source: {}\n", source));
        }

        output
    }

    /// Exportiert Error als JSON
    pub fn to_json(&self) -> String {
        let mut json = format!(
            r#"{{"error_type":"{}","message":"{}","#,
            self.error_type,
            self.message.replace('"', "\\\"")
        );

        if !self.context.is_empty() {
            let context_json = serde_json::to_string(&self.context).unwrap_or_default();
            json.push_str(&format!(r#""context":{},"#, context_json));
        }

        if !self.stack_trace.is_empty() {
            let stack_json = serde_json::to_string(&self.stack_trace).unwrap_or_default();
            json.push_str(&format!(r#""stack_trace":{},"#, stack_json));
        }

        json.push('}');
        json
    }
}

impl fmt::Display for VelinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_structured_string())
    }
}

impl std::error::Error for VelinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &dyn std::error::Error)
    }
}

/// Error Recovery Mechanism
pub struct ErrorRecovery {
    pub retry_count: u32,
    pub backoff_strategy: BackoffStrategy,
    pub max_retries: u32,
}

#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    Linear { delay_ms: u64 },
    Exponential { base_ms: u64, max_ms: u64 },
    Fixed { delay_ms: u64 },
}

impl ErrorRecovery {
    /// Erstellt einen neuen Error Recovery
    pub fn new() -> Self {
        ErrorRecovery {
            retry_count: 0,
            backoff_strategy: BackoffStrategy::Fixed { delay_ms: 1000 },
            max_retries: 3,
        }
    }

    /// Setzt die Backoff-Strategie
    pub fn with_backoff(mut self, strategy: BackoffStrategy) -> Self {
        self.backoff_strategy = strategy;
        self
    }

    /// Setzt die maximale Anzahl von Retries
    pub fn with_max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    /// Berechnet die Verzögerung für den nächsten Retry
    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        match &self.backoff_strategy {
            BackoffStrategy::Linear { delay_ms } => *delay_ms * attempt as u64,
            BackoffStrategy::Exponential { base_ms, max_ms } => {
                let delay = base_ms * 2_u64.pow(attempt);
                delay.min(*max_ms)
            }
            BackoffStrategy::Fixed { delay_ms } => *delay_ms,
        }
    }

    /// Prüft, ob ein Retry möglich ist
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }

    /// Erhöht den Retry-Counter
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Error Reporter für zentrale Error-Erfassung
pub struct ErrorReporter {
    errors: Vec<VelinError>,
    max_errors: usize,
}

impl ErrorReporter {
    /// Erstellt einen neuen Error Reporter
    pub fn new() -> Self {
        ErrorReporter {
            errors: Vec::new(),
            max_errors: 1000,
        }
    }

    /// Setzt die maximale Anzahl von gespeicherten Errors
    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }

    /// Meldet einen Error
    pub fn report(&mut self, error: VelinError) {
        if self.errors.len() >= self.max_errors {
            self.errors.remove(0);
        }
        self.errors.push(error);
    }

    /// Gibt alle Errors zurück
    pub fn get_errors(&self) -> &[VelinError] {
        &self.errors
    }

    /// Exportiert alle Errors als JSON
    pub fn export_json(&self) -> String {
        let errors_json: Vec<String> = self.errors.iter().map(|e| e.to_json()).collect();
        format!("[{}]", errors_json.join(","))
    }

    /// Löscht alle Errors
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ResultStdlib;

impl ResultStdlib {
    /// Generiert Rust-Code für Result-Methoden
    /// Diese werden als Extension-Methoden auf Result<T, E> aufgerufen
    pub fn generate_result_methods() -> String {
        r#"
// Result Extension Methods
// Diese Methoden werden automatisch für Result<T, E> verfügbar gemacht

impl<T, E> Result<T, E> {
    /// Unwraps a Result, yielding the content of an Ok.
    /// Panics if the value is an Err.
    pub fn unwrap(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
        }
    }
    
    /// Unwraps a Result, yielding the content of an Ok.
    /// Panics if the value is an Err with a custom panic message.
    pub fn expect(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => panic!("{}: {:?}", msg, err),
        }
    }
    
    /// Returns the contained Ok value or a provided default.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Ok(val) => val,
            Err(_) => default,
        }
    }
    
    /// Returns the contained Ok value or computes it from a closure.
    pub fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Ok(val) => val,
            Err(err) => op(err),
        }
    }
    
    /// Maps a Result<T, E> to Result<U, E> by applying a function to a contained Ok value.
    pub fn map<U, F>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Ok(val) => Ok(op(val)),
            Err(err) => Err(err),
        }
    }
    
    /// Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value.
    pub fn map_err<F, O>(self, op: O) -> Result<T, F>
    where
        O: FnOnce(E) -> F,
    {
        match self {
            Ok(val) => Ok(val),
            Err(err) => Err(op(err)),
        }
    }
    
    /// Returns true if the result is Ok.
    pub fn is_ok(&self) -> bool {
        matches!(self, Ok(_))
    }
    
    /// Returns true if the result is Err.
    pub fn is_err(&self) -> bool {
        matches!(self, Err(_))
    }
    
    /// Converts from Result<T, E> to Option<T>.
    pub fn ok(self) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
    
    /// Converts from Result<T, E> to Option<E>.
    pub fn err(self) -> Option<E> {
        match self {
            Ok(_) => None,
            Err(err) => Some(err),
        }
    }
}
"#
        .to_string()
    }

    /// Prüft, ob ein Expression ein Result-Methoden-Aufruf ist
    pub fn is_result_method(method_name: &str) -> bool {
        matches!(
            method_name,
            "unwrap"
                | "expect"
                | "unwrap_or"
                | "unwrap_or_else"
                | "map"
                | "map_err"
                | "is_ok"
                | "is_err"
                | "ok"
                | "err"
        )
    }

    /// Generiert Code für einen Result-Methoden-Aufruf
    pub fn generate_result_method_call(method_name: &str, object: &str, args: &[String]) -> String {
        match method_name {
            "unwrap" => format!("{}.unwrap()", object),
            "expect" => {
                if let Some(msg) = args.get(0) {
                    format!("{}.expect({})", object, msg)
                } else {
                    format!("{}.expect(\"error\")", object)
                }
            }
            "unwrap_or" => {
                if let Some(default) = args.get(0) {
                    format!("{}.unwrap_or({})", object, default)
                } else {
                    format!("{}.unwrap_or(Default::default())", object)
                }
            }
            "unwrap_or_else" => {
                if let Some(closure) = args.get(0) {
                    format!("{}.unwrap_or_else({})", object, closure)
                } else {
                    format!("{}.unwrap_or_else(|e| Default::default())", object)
                }
            }
            "map" => {
                if let Some(closure) = args.get(0) {
                    format!("{}.map({})", object, closure)
                } else {
                    format!("{}.map(|x| x)", object)
                }
            }
            "map_err" => {
                if let Some(closure) = args.get(0) {
                    format!("{}.map_err({})", object, closure)
                } else {
                    format!("{}.map_err(|e| e)", object)
                }
            }
            "is_ok" => format!("{}.is_ok()", object),
            "is_err" => format!("{}.is_err()", object),
            "ok" => format!("{}.ok()", object),
            "err" => format!("{}.err()", object),
            _ => format!("{}.{}()", object, method_name),
        }
    }
}
