// LSP Handlers

pub mod completion;
pub mod hover;
pub mod definition;
pub mod formatting;
pub mod references;
pub mod rename;
pub mod code_actions;
pub mod imports;

pub use completion::get_completions;
pub use hover::get_hover;
pub use definition::find_definition;
pub use formatting::format_document;
pub use references::find_references;
pub use rename::rename_symbol;
pub use code_actions::get_code_actions;
// organize_imports is imported directly in code_actions.rs, no need to re-export here
