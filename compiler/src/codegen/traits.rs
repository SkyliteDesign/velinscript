use crate::parser::ast::Program;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetLanguage {
    #[default]
    Rust,
    Php,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    CSharp,
}

impl std::fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetLanguage::Rust => write!(f, "Rust"),
            TargetLanguage::Php => write!(f, "PHP"),
            TargetLanguage::Python => write!(f, "Python"),
            TargetLanguage::JavaScript => write!(f, "JavaScript"),
            TargetLanguage::TypeScript => write!(f, "TypeScript"),
            TargetLanguage::Go => write!(f, "Go"),
            TargetLanguage::Java => write!(f, "Java"),
            TargetLanguage::CSharp => write!(f, "C#"),
        }
    }
}

impl std::str::FromStr for TargetLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(TargetLanguage::Rust),
            "php" => Ok(TargetLanguage::Php),
            "python" | "py" => Ok(TargetLanguage::Python),
            "javascript" | "js" => Ok(TargetLanguage::JavaScript),
            "typescript" | "ts" => Ok(TargetLanguage::TypeScript),
            "go" | "golang" => Ok(TargetLanguage::Go),
            "java" => Ok(TargetLanguage::Java),
            "csharp" | "c#" | "cs" => Ok(TargetLanguage::CSharp),
            _ => Err(format!("Unknown target language: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodegenConfig {
    pub target: TargetLanguage,
    pub framework: Option<String>,
    pub orm: Option<String>,
    pub output_path: Option<PathBuf>,
}

pub trait CodeGenerator {
    fn generate(&mut self, program: &Program, config: &CodegenConfig) -> Result<String>;
    fn get_target_language(&self) -> TargetLanguage;
}
