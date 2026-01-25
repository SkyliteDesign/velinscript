// Input Validation Framework

use regex::Regex;

pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub struct Validator {
    pub errors: Vec<ValidationError>,
}

impl Validator {
    pub fn new() -> Self {
        Validator { errors: Vec::new() }
    }

    pub fn required(&mut self, field: &str, value: Option<&String>) -> &mut Self {
        if value.is_none() || value.unwrap().is_empty() {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} ist erforderlich", field),
            });
        }
        self
    }

    pub fn min_length(&mut self, field: &str, value: &str, min: usize) -> &mut Self {
        if value.len() < min {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} muss mindestens {} Zeichen lang sein", field, min),
            });
        }
        self
    }

    pub fn max_length(&mut self, field: &str, value: &str, max: usize) -> &mut Self {
        if value.len() > max {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} darf maximal {} Zeichen lang sein", field, max),
            });
        }
        self
    }

    pub fn email(&mut self, field: &str, value: &str) -> &mut Self {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(value) {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} muss eine gültige E-Mail-Adresse sein", field),
            });
        }
        self
    }

    pub fn pattern(&mut self, field: &str, value: &str, pattern: &str, message: &str) -> &mut Self {
        match Regex::new(pattern) {
            Ok(regex) => {
                if !regex.is_match(value) {
                    self.errors.push(ValidationError {
                        field: field.to_string(),
                        message: message.to_string(),
                    });
                }
            }
            Err(_) => {
                self.errors.push(ValidationError {
                    field: field.to_string(),
                    message: format!("Ungültiges Regex-Pattern für {}", field),
                });
            }
        }
        self
    }

    pub fn min(&mut self, field: &str, value: f64, min: f64) -> &mut Self {
        if value < min {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} muss mindestens {} sein", field, min),
            });
        }
        self
    }

    pub fn max(&mut self, field: &str, value: f64, max: f64) -> &mut Self {
        if value > max {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} darf maximal {} sein", field, max),
            });
        }
        self
    }

    pub fn range(&mut self, field: &str, value: f64, min: f64, max: f64) -> &mut Self {
        if value < min || value > max {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} muss zwischen {} und {} liegen", field, min, max),
            });
        }
        self
    }

    pub fn custom<F>(&mut self, field: &str, value: &str, validator: F) -> &mut Self
    where
        F: Fn(&str) -> Option<String>,
    {
        if let Some(error_msg) = validator(value) {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: error_msg,
            });
        }
        self
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn errors(&self) -> &Vec<ValidationError> {
        &self.errors
    }
}

pub struct ValidationStdlib;

impl ValidationStdlib {
    pub fn generate_new_code() -> String {
        "Validator::new()".to_string()
    }

    pub fn generate_required_code(validator: &str, field: &str, value: &str) -> String {
        format!("{}.required({}, {})", validator, field, value)
    }

    pub fn generate_min_length_code(
        validator: &str,
        field: &str,
        value: &str,
        min: &str,
    ) -> String {
        format!("{}.min_length({}, {}, {})", validator, field, value, min)
    }

    pub fn generate_max_length_code(
        validator: &str,
        field: &str,
        value: &str,
        max: &str,
    ) -> String {
        format!("{}.max_length({}, {}, {})", validator, field, value, max)
    }

    pub fn generate_email_code(validator: &str, field: &str, value: &str) -> String {
        format!("{}.email({}, {})", validator, field, value)
    }

    pub fn generate_pattern_code(
        validator: &str,
        field: &str,
        value: &str,
        pattern: &str,
        message: &str,
    ) -> String {
        format!(
            "{}.pattern({}, {}, {}, {})",
            validator, field, value, pattern, message
        )
    }

    pub fn generate_min_code(validator: &str, field: &str, value: &str, min: &str) -> String {
        format!("{}.min({}, {}, {})", validator, field, value, min)
    }

    pub fn generate_max_code(validator: &str, field: &str, value: &str, max: &str) -> String {
        format!("{}.max({}, {}, {})", validator, field, value, max)
    }

    /// Generiert Rust-Code für Validator-Import
    pub fn generate_import() -> String {
        "use regex::Regex;\n".to_string()
    }

    /// Generiert Rust-Code für Validator-Struktur
    pub fn generate_validator_struct() -> String {
        r#"#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub struct Validator {
    pub errors: Vec<ValidationError>,
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            errors: Vec::new(),
        }
    }
    
    pub fn required(&mut self, field: &str, value: Option<&String>) -> &mut Self {
        if value.is_none() || value.unwrap().is_empty() {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} ist erforderlich", field),
            });
        }
        self
    }
    
    pub fn min_length(&mut self, field: &str, value: &str, min: usize) -> &mut Self {
        if value.len() < min {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} muss mindestens {} Zeichen lang sein", field, min),
            });
        }
        self
    }
    
    pub fn max_length(&mut self, field: &str, value: &str, max: usize) -> &mut Self {
        if value.len() > max {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} darf maximal {} Zeichen lang sein", field, max),
            });
        }
        self
    }
    
    pub fn email(&mut self, field: &str, value: &str) -> &mut Self {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(value) {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} muss eine gültige E-Mail-Adresse sein", field),
            });
        }
        self
    }
    
    pub fn pattern(&mut self, field: &str, value: &str, pattern: &str, message: &str) -> &mut Self {
        match Regex::new(pattern) {
            Ok(regex) => {
                if !regex.is_match(value) {
                    self.errors.push(ValidationError {
                        field: field.to_string(),
                        message: message.to_string(),
                    });
                }
            }
            Err(_) => {
                self.errors.push(ValidationError {
                    field: field.to_string(),
                    message: format!("Ungültiges Regex-Pattern für {}", field),
                });
            }
        }
        self
    }
    
    pub fn min(&mut self, field: &str, value: f64, min: f64) -> &mut Self {
        if value < min {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} muss mindestens {} sein", field, min),
            });
        }
        self
    }
    
    pub fn max(&mut self, field: &str, value: f64, max: f64) -> &mut Self {
        if value > max {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} darf maximal {} sein", field, max),
            });
        }
        self
    }
    
    pub fn range(&mut self, field: &str, value: f64, min: f64, max: f64) -> &mut Self {
        if value < min || value > max {
            self.errors.push(ValidationError {
                field: field.to_string(),
                message: format!("{} muss zwischen {} und {} liegen", field, min, max),
            });
        }
        self
    }
    
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
    
    pub fn errors(&self) -> &Vec<ValidationError> {
        &self.errors
    }
}
"#
        .to_string()
    }
}
