pub mod ai_bug_detection;
pub mod ai_code_review;
pub mod ai_codegen;
pub mod ai_optimization;
pub mod ai_sandbox;
pub mod ai_semantic;
pub mod autofix;
pub mod code_order;
pub mod codegen;
pub mod desugar;
pub mod parser;
pub mod type_check;

// Re-export Passes
pub use ai_code_review::{AICodeReviewPass, AICodeReviewer};
pub use ai_sandbox::{AICodeSandbox, AISandboxPass};
pub use code_order::CodeOrderingPass;
pub use desugar::DesugaringPass;
