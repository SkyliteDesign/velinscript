use crate::compiler::language::VELISCH_LANGUAGE_NAME;

#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub enable_autofix: bool,
    pub enable_type_check: bool,
    pub enable_optimization: bool,
    pub show_code: bool,
    pub output_path: Option<std::path::PathBuf>,
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
        }
    }
}
