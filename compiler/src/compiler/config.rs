use crate::compiler::language::VELISCH_LANGUAGE_NAME;
use crate::codegen::traits::TargetLanguage;

#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub enable_autofix: bool,
    pub enable_type_check: bool,
    pub enable_optimization: bool,
    pub show_code: bool,
    pub output_path: Option<std::path::PathBuf>,
    pub target: TargetLanguage,
    // KI-Compiler-Passes Feature Flags
    pub enable_ai_semantic: bool,
    pub enable_ai_bug_detection: bool,
    pub enable_ai_codegen: bool,
    pub enable_ai_code_review: bool, // Review für AI-generierten Code
    pub enable_ai_sandbox: bool, // Sandbox-Validierung für AI-generierten Code
    pub enable_ai_optimization: bool,
    pub ai_provider: Option<String>, // "openai", "anthropic", "local"
    pub ai_api_key: Option<String>,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        // Velisch Identity - Fingerabdruck in Config
        let _velisch_check = VELISCH_LANGUAGE_NAME;
        
        Self {
            enable_autofix: false,
            enable_type_check: true,
            enable_optimization: true,
            show_code: false,
            output_path: None,
            target: TargetLanguage::Rust,
            // KI-Features standardmäßig deaktiviert
            enable_ai_semantic: false,
            enable_ai_bug_detection: false,
            enable_ai_codegen: false,
            enable_ai_code_review: false, // Standardmäßig deaktiviert
            enable_ai_sandbox: false, // Standardmäßig deaktiviert
            enable_ai_optimization: false,
            ai_provider: None,
            ai_api_key: None,
        }
    }
}
