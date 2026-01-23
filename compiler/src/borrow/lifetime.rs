/// Lifetime-System für VelinScript
/// 
/// Definiert Lifetime-Information für Referenzen ähnlich Rust.

use std::fmt;

/// Lifetime-Information
/// 
/// Definiert die Lifetime einer Referenz.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lifetime {
    pub id: LifetimeId,
    pub scope: ScopeId,
}

impl Lifetime {
    pub fn new(id: LifetimeId, scope: ScopeId) -> Self {
        Lifetime { id, scope }
    }
}

impl fmt::Display for Lifetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}", self.id.0)
    }
}

/// Lifetime-ID
/// 
/// Eindeutige ID für eine Lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LifetimeId(pub usize);

impl LifetimeId {
    pub fn new(id: usize) -> Self {
        LifetimeId(id)
    }
}

/// Scope-ID
/// 
/// Eindeutige ID für einen Scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);

impl ScopeId {
    pub fn new(id: usize) -> Self {
        ScopeId(id)
    }
}

/// Lifetime-Analyzer
/// 
/// Analysiert Lifetimes in IR-Code.
pub struct LifetimeAnalyzer {
    lifetime_counter: usize,
    scope_counter: usize,
    lifetimes: Vec<Lifetime>,
    scopes: Vec<ScopeId>,
}

impl LifetimeAnalyzer {
    pub fn new() -> Self {
        LifetimeAnalyzer {
            lifetime_counter: 0,
            scope_counter: 0,
            lifetimes: Vec::new(),
            scopes: Vec::new(),
        }
    }
    
    /// Erstellt eine neue Lifetime
    pub fn create_lifetime(&mut self, scope: ScopeId) -> Lifetime {
        let id = LifetimeId::new(self.lifetime_counter);
        self.lifetime_counter += 1;
        let lifetime = Lifetime::new(id, scope);
        self.lifetimes.push(lifetime);
        lifetime
    }
    
    /// Erstellt einen neuen Scope
    pub fn create_scope(&mut self) -> ScopeId {
        let scope = ScopeId::new(self.scope_counter);
        self.scope_counter += 1;
        self.scopes.push(scope);
        scope
    }
    
    /// Prüft ob eine Lifetime einen Scope überlebt
    pub fn outlives(&self, lifetime: &Lifetime, scope: ScopeId) -> bool {
        // Lifetime überlebt Scope wenn Scope-ID kleiner ist
        lifetime.scope.0 <= scope.0
    }
}

impl Default for LifetimeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
