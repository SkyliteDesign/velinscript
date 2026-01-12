pub mod checker;
pub mod environment;
pub mod errors;

pub use checker::TypeChecker;
pub use errors::{TypeError, TypeErrorKind};
