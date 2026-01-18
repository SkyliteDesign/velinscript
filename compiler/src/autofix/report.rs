use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoFixReport {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub rule: String,
    pub original: String,
    pub fixed: String,
}

impl AutoFixReport {
    pub fn new(
        file: impl Into<String>,
        line: usize,
        column: usize,
        rule: impl Into<String>,
        original: impl Into<String>,
        fixed: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            rule: rule.into(),
            original: original.into(),
            fixed: fixed.into(),
        }
    }
}
