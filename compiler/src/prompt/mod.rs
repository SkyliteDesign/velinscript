/// Prompt Optimizer - Optimiert LLM-Prompts
/// 
/// Dieses Modul implementiert automatische Prompt-Optimierung für 90%+ Token-Ersparnis:
/// - Prompt-Kürzung
/// - Token-Optimierung
/// - System-Prompt-Caching
/// - Kompakte Syntax
/// 
/// # Beispiel
/// 
/// ```rust
/// use velin_compiler::prompt::optimizer::PromptOptimizer;
/// 
/// let mut optimizer = PromptOptimizer::new();
/// let optimized = optimizer.optimize("Bitte analysiere den folgenden Text...");
/// println!("Ersparnis: {:.1}%", optimized.savings_percent);
/// ```

pub mod optimizer;
pub mod sanitizer;

pub use optimizer::{PromptOptimizer, OptimizedPrompt};
pub use sanitizer::PromptSanitizer;