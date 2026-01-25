pub mod checker;
pub mod lifetime;
/// Borrow Checker - Ownership & Borrowing System
///
/// Dieses Modul implementiert ein Borrow-Checker-System ähnlich Rust:
/// - Ownership-Tracking
/// - Borrow-Checks
/// - Lifetime-Analyse
/// - Memory-Safety-Garantien
///
/// # Beispiel
///
/// ```rust
/// use velin_compiler::borrow::checker::BorrowChecker;
///
/// let mut checker = BorrowChecker::new();
/// match checker.check(&ir_module) {
///     Ok(_) => println!("Borrow-Checks erfolgreich"),
///     Err(errors) => println!("Borrow-Fehler: {:?}", errors),
/// }
/// ```
pub mod ownership;

pub use checker::BorrowChecker;
pub use lifetime::*;
pub use ownership::*;
