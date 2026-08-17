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
    /// Web framework for HTTP lowering (e.g. "axum", "actix"). Default axum when None.
    pub framework: Option<String>,
    /// When true (default): IR codegen. When false: AST legacy (debug/compare only).
    pub use_ir: bool,
    // KI-Compiler-Passes Feature Flags
    pub enable_ai_semantic: bool,
    pub enable_ai_bug_detection: bool,
    pub enable_ai_codegen: bool,
    pub enable_ai_code_review: bool,
    pub enable_ai_sandbox: bool,
    pub enable_ai_optimization: bool,
    pub ai_provider: Option<String>,
    pub ai_api_key: Option<String>,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        let _velisch_check = VELISCH_LANGUAGE_NAME;
        
        Self {
            enable_autofix: false,
            enable_type_check: true,
            enable_optimization: true,
            show_code: false,
            output_path: None,
            target: TargetLanguage::Rust,
            framework: None,
            use_ir: true,
            enable_ai_semantic: false,
            enable_ai_bug_detection: false,
            enable_ai_codegen: false,
            enable_ai_code_review: false,
            enable_ai_sandbox: false,
            enable_ai_optimization: false,
            ai_provider: None,
            ai_api_key: None,
        }
    }
}

impl CompilerConfig {
    /// Normalized framework name for Rust HTTP (default axum).
    pub fn rust_framework(&self) -> String {
        self.framework
            .as_deref()
            .map(|f| f.to_lowercase())
            .filter(|f| !f.is_empty())
            .unwrap_or_else(|| "axum".to_string())
    }
}
