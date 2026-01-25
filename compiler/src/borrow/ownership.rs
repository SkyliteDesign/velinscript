/// Ownership-System für VelinScript
///
/// Definiert Ownership-Semantik ähnlich Rust:
/// - Owned: Variable besitzt den Wert (move semantics)
/// - Borrowed: Immutable Referenz (&T)
/// - BorrowedMut: Mutable Referenz (&mut T)
/// - Shared: Shared ownership (Arc/Rc)
/// - Copy: Copy-Semantik (primitive types)
use crate::borrow::lifetime::Lifetime;
use std::fmt;

/// Ownership-Information
///
/// Definiert die Ownership-Semantik für eine Variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ownership {
    /// Variable besitzt den Wert (move semantics)
    Owned,

    /// Variable ist eine immutable Referenz (&T)
    Borrowed { lifetime: Lifetime },

    /// Variable ist eine mutable Referenz (&mut T)
    BorrowedMut { lifetime: Lifetime },

    /// Shared ownership (Arc<T> / Rc<T>)
    Shared,

    /// Copy-Semantik (primitive types)
    Copy,
}

impl Ownership {
    /// Prüft ob Ownership Copy-Semantik hat
    pub fn is_copy(&self) -> bool {
        matches!(self, Ownership::Copy)
    }

    /// Prüft ob Ownership Owned ist
    pub fn is_owned(&self) -> bool {
        matches!(self, Ownership::Owned)
    }

    /// Prüft ob Ownership eine Referenz ist
    pub fn is_borrowed(&self) -> bool {
        matches!(
            self,
            Ownership::Borrowed { .. } | Ownership::BorrowedMut { .. }
        )
    }

    /// Prüft ob Ownership eine mutable Referenz ist
    pub fn is_borrowed_mut(&self) -> bool {
        matches!(self, Ownership::BorrowedMut { .. })
    }

    /// Holt Lifetime (falls vorhanden)
    pub fn lifetime(&self) -> Option<&Lifetime> {
        match self {
            Ownership::Borrowed { lifetime } | Ownership::BorrowedMut { lifetime } => {
                Some(lifetime)
            }
            _ => None,
        }
    }
}

impl fmt::Display for Ownership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ownership::Owned => write!(f, "owned"),
            Ownership::Borrowed { lifetime } => write!(f, "&{}", lifetime.id.0),
            Ownership::BorrowedMut { lifetime } => write!(f, "&mut {}", lifetime.id.0),
            Ownership::Shared => write!(f, "shared"),
            Ownership::Copy => write!(f, "copy"),
        }
    }
}

/// Ownership-Regeln
///
/// Definiert welche Ownership-Regeln für verschiedene Typen gelten.
pub struct OwnershipRules;

impl OwnershipRules {
    /// Prüft ob ein Typ Copy-Semantik hat
    pub fn is_copy_type(ty: &crate::ir::ir::IRType) -> bool {
        match ty {
            crate::ir::ir::IRType::Bool
            | crate::ir::ir::IRType::Int
            | crate::ir::ir::IRType::Float => true,
            _ => false,
        }
    }

    /// Bestimmt Standard-Ownership für einen Typ
    pub fn default_ownership(ty: &crate::ir::ir::IRType) -> Ownership {
        if Self::is_copy_type(ty) {
            Ownership::Copy
        } else {
            Ownership::Owned
        }
    }
}
