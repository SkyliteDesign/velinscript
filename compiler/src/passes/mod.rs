pub mod autofix;
pub mod parser;
pub mod desugar;
pub mod type_check;
pub mod codegen;
pub mod ai_semantic;
pub mod ai_bug_detection;
pub mod ai_codegen;
pub mod ai_optimization;
pub mod ai_code_review;
pub mod ai_sandbox;
pub mod code_order;

// Re-export Passes
pub use desugar::DesugaringPass;
pub use ai_code_review::{AICodeReviewPass, AICodeReviewer};
pub use ai_sandbox::{AISandboxPass, AICodeSandbox};
pub use code_order::CodeOrderingPass;