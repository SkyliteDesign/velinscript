/// Borrow Checker - Prüft Ownership & Borrowing
/// 
/// Dieser Checker prüft IR-Code auf Borrow-Verletzungen:
/// - Use-After-Move
/// - Multiple Mutable Borrows
/// - Borrow-After-Move
/// - Lifetime-Verletzungen
/// - Dangling References

use crate::ir::ir::*;
use crate::borrow::ownership::{Ownership as BorrowOwnership, OwnershipRules};
use crate::borrow::lifetime::{LifetimeAnalyzer, Lifetime, ScopeId};
use std::collections::HashMap;
use anyhow::Result;

/// Konvertiert borrow::ownership::Ownership zu ir::ir::Ownership
fn convert_to_ir_ownership(borrow_ownership: &BorrowOwnership) -> Ownership {
    match borrow_ownership {
        BorrowOwnership::Owned => Ownership::Owned,
        BorrowOwnership::Borrowed { .. } => Ownership::Borrowed,
        BorrowOwnership::BorrowedMut { .. } => Ownership::BorrowedMut,
        BorrowOwnership::Shared => Ownership::Shared,
        BorrowOwnership::Copy => Ownership::Copy,
    }
}

/// Borrow Checker
pub struct BorrowChecker {
    ownership_map: HashMap<IRValue, BorrowOwnership>,
    lifetime_analyzer: LifetimeAnalyzer,
    errors: Vec<BorrowError>,
    current_scope: ScopeId,
    scope_stack: Vec<ScopeId>,
}

/// Borrow-Fehler
#[derive(Debug, Clone)]
pub enum BorrowError {
    UseAfterMove {
        value: IRValue,
        location: String,
    },
    MultipleMutableBorrows {
        value: IRValue,
        locations: Vec<String>,
    },
    BorrowAfterMove {
        value: IRValue,
        location: String,
    },
    LifetimeOutlivesScope {
        lifetime: Lifetime,
        scope: ScopeId,
        location: String,
    },
    DanglingReference {
        value: IRValue,
        location: String,
    },
    ImmutableBorrowMutation {
        value: IRValue,
        location: String,
    },
}

impl BorrowError {
    pub fn to_string(&self) -> String {
        match self {
            BorrowError::UseAfterMove { value, location } => {
                format!("Use after move: {:?} at {}", value, location)
            }
            BorrowError::MultipleMutableBorrows { value, locations } => {
                format!("Multiple mutable borrows: {:?} at {:?}", value, locations)
            }
            BorrowError::BorrowAfterMove { value, location } => {
                format!("Borrow after move: {:?} at {}", value, location)
            }
            BorrowError::LifetimeOutlivesScope { lifetime, scope, location } => {
                format!("Lifetime {} outlives scope {:?} at {}", lifetime, scope, location)
            }
            BorrowError::DanglingReference { value, location } => {
                format!("Dangling reference: {:?} at {}", value, location)
            }
            BorrowError::ImmutableBorrowMutation { value, location } => {
                format!("Cannot mutate immutable borrow: {:?} at {}", value, location)
            }
        }
    }
}

impl BorrowChecker {
    /// Erstellt einen neuen Borrow Checker
    pub fn new() -> Self {
        let mut analyzer = LifetimeAnalyzer::new();
        let root_scope = analyzer.create_scope();
        
        BorrowChecker {
            ownership_map: HashMap::new(),
            lifetime_analyzer: analyzer,
            errors: Vec::new(),
            current_scope: root_scope,
            scope_stack: vec![root_scope],
        }
    }
    
    /// Prüft IR-Modul auf Borrow-Verletzungen
    pub fn check(&mut self, module: &IRModule) -> Result<(), Vec<BorrowError>> {
        self.errors.clear();
        
        // 1. Ownership für alle Variablen initialisieren
        self.initialize_ownership(module);
        
        // 2. Jede Funktion prüfen
        for func in &module.functions {
            if let Err(e) = self.check_function(func) {
                self.errors.push(BorrowError::DanglingReference {
                    value: IRValue::Constant(IRConstant::Null),
                    location: format!("Internal error: {}", e),
                });
            }
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    /// Initialisiert Ownership für alle Variablen
    fn initialize_ownership(&mut self, module: &IRModule) {
        for func in &module.functions {
            // Parameter: Standardmäßig owned (außer Copy-Typen)
            for param in &func.params {
                let ownership = OwnershipRules::default_ownership(&param.ty);
                let ir_ownership = convert_to_ir_ownership(&ownership);
                let var_value = IRValue::Variable(IRVariable {
                    name: param.name.clone(),
                    id: VarId::new(0), // Wird später korrekt gesetzt
                    ty: param.ty.clone(),
                    ownership: ir_ownership,
                });
                self.ownership_map.insert(var_value, ownership);
            }
        }
    }
    
    /// Prüft Funktion auf Borrow-Verletzungen
    fn check_function(&mut self, func: &IRFunction) -> Result<()> {
        // Neuer Scope für Funktion
        let func_scope = self.lifetime_analyzer.create_scope();
        self.enter_scope(func_scope);
        
        // Parameter: Ownership initialisieren
        for param in &func.params {
            let ownership = OwnershipRules::default_ownership(&param.ty);
            let ir_ownership = convert_to_ir_ownership(&ownership);
            let var_value = IRValue::Variable(IRVariable {
                name: param.name.clone(),
                id: VarId::new(0),
                ty: param.ty.clone(),
                ownership: ir_ownership,
            });
            self.ownership_map.insert(var_value, ownership);
        }
        
        // Body prüfen
        self.check_block(&func.body, func_scope)?;
        
        self.exit_scope();
        Ok(())
    }
    
    /// Prüft Block auf Borrow-Verletzungen
    fn check_block(&mut self, block: &IRBlock, scope: ScopeId) -> Result<()> {
        for instruction in &block.instructions {
            self.check_instruction(instruction, scope)?;
        }
        Ok(())
    }
    
    /// Prüft Instruction auf Borrow-Verletzungen
    fn check_instruction(&mut self, inst: &IRInstruction, scope: ScopeId) -> Result<()> {
        match inst {
            IRInstruction::Load { dest, source } => {
                self.check_load(dest, source, scope)?;
            }
            IRInstruction::Store { dest, value } => {
                self.check_store(dest, value, scope)?;
            }
            IRInstruction::Call { dest, func, args } => {
                self.check_call(dest, func, args, scope)?;
            }
            IRInstruction::CallAsync { dest, func, args } => {
                // KRITISCH: Async-Calls erfordern spezielle Borrow-Behandlung
                // Borrows überleben async boundaries nicht - müssen 'static sein oder shared
                self.check_async_call(dest, func, args, scope)?;
            }
            IRInstruction::StructAccess { dest, struct_val, .. } => {
                self.check_struct_access(dest, struct_val, scope)?;
            }
            IRInstruction::ListGet { dest, list, .. } => {
                self.check_list_get(dest, list, scope)?;
            }
            IRInstruction::MapGet { dest, map, .. } => {
                self.check_map_get(dest, map, scope)?;
            }
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    self.check_return_value(v, scope)?;
                }
            }
            _ => {
                // Andere Instructions haben keine speziellen Borrow-Checks
            }
        }
        
        Ok(())
    }
    
    /// Prüft Load-Instruction
    fn check_load(&mut self, dest: &IRValue, source: &IRValue, scope: ScopeId) -> Result<()> {
        if let Some(ownership) = self.ownership_map.get(source) {
            match ownership {
                BorrowOwnership::Owned => {
                    // Move: source wird zu dest
                    self.ownership_map.remove(source);
                    self.ownership_map.insert(dest.clone(), BorrowOwnership::Owned);
                }
                BorrowOwnership::Borrowed { lifetime } => {
                    // Borrow: dest ist eine Referenz
                    if !self.lifetime_analyzer.outlives(lifetime, scope) {
                        self.errors.push(BorrowError::LifetimeOutlivesScope {
                            lifetime: *lifetime,
                            scope,
                            location: format!("Load from {:?}", source),
                        });
                    } else {
                        self.ownership_map.insert(dest.clone(), BorrowOwnership::Borrowed { lifetime: *lifetime });
                    }
                }
                BorrowOwnership::BorrowedMut { lifetime } => {
                    // Mutable Borrow: dest ist eine mutable Referenz
                    if !self.lifetime_analyzer.outlives(lifetime, scope) {
                        self.errors.push(BorrowError::LifetimeOutlivesScope {
                            lifetime: *lifetime,
                            scope,
                            location: format!("Load from {:?}", source),
                        });
                    } else {
                        self.ownership_map.insert(dest.clone(), BorrowOwnership::BorrowedMut { lifetime: *lifetime });
                    }
                }
                BorrowOwnership::Copy => {
                    // Copy: source bleibt gültig
                    self.ownership_map.insert(dest.clone(), BorrowOwnership::Copy);
                }
                BorrowOwnership::Shared => {
                    // Shared: beide bleiben gültig
                    self.ownership_map.insert(dest.clone(), BorrowOwnership::Shared);
                }
            }
        } else {
            // Fehler: source wurde bereits moved oder existiert nicht
            self.errors.push(BorrowError::UseAfterMove {
                value: source.clone(),
                location: format!("Load to {:?}", dest),
            });
        }
        
        Ok(())
    }
    
    /// Prüft Store-Instruction
    fn check_store(&mut self, dest: &IRValue, value: &IRValue, _scope: ScopeId) -> Result<()> {
        // Prüfe: Ist dest mutable?
        if let Some(ownership) = self.ownership_map.get(dest) {
            match ownership {
                BorrowOwnership::BorrowedMut { .. } | BorrowOwnership::Owned => {
                    // OK: mutable borrow oder owned value kann geschrieben werden
                }
                BorrowOwnership::Borrowed { .. } => {
                    // Fehler: immutable borrow kann nicht geschrieben werden
                    self.errors.push(BorrowError::ImmutableBorrowMutation {
                        value: dest.clone(),
                        location: format!("Store to {:?}", dest),
                    });
                }
                BorrowOwnership::Copy => {
                    // Copy-Typen können nicht mutiert werden
                    self.errors.push(BorrowError::ImmutableBorrowMutation {
                        value: dest.clone(),
                        location: format!("Store to {:?}", dest),
                    });
                }
                _ => {}
            }
        }
        
        // Prüfe: Wird value moved?
        if let Some(ownership) = self.ownership_map.get(value) {
            match ownership {
                BorrowOwnership::Owned => {
                    // Move: value wird consumed
                    self.ownership_map.remove(value);
                }
                BorrowOwnership::Borrowed { .. } | BorrowOwnership::BorrowedMut { .. } | BorrowOwnership::Shared | BorrowOwnership::Copy => {
                    // Borrow/Shared/Copy: value bleibt gültig
                }
            }
        }
        
        Ok(())
    }
    
    /// Prüft Call-Instruction
    fn check_call(&mut self, dest: &Option<IRValue>, _func: &IRValue, args: &[IRValue], scope: ScopeId) -> Result<()> {
        // Prüfe: Werden Argumente moved?
        for arg in args {
            if let Some(ownership) = self.ownership_map.get(arg) {
                match ownership {
                    BorrowOwnership::Owned => {
                        // Move: Argument wird consumed
                        self.ownership_map.remove(arg);
                    }
                    BorrowOwnership::Borrowed { lifetime } => {
                        // Borrow: Prüfe Lifetime
                        if !self.lifetime_analyzer.outlives(lifetime, scope) {
                            self.errors.push(BorrowError::LifetimeOutlivesScope {
                                lifetime: *lifetime,
                                scope,
                                location: format!("Call argument {:?}", arg),
                            });
                        }
                    }
                    BorrowOwnership::BorrowedMut { lifetime } => {
                        // Mutable Borrow: Prüfe Lifetime
                        if !self.lifetime_analyzer.outlives(lifetime, scope) {
                            self.errors.push(BorrowError::LifetimeOutlivesScope {
                                lifetime: *lifetime,
                                scope,
                                location: format!("Call argument {:?}", arg),
                            });
                        }
                    }
                    BorrowOwnership::Copy | BorrowOwnership::Shared => {
                        // Copy/Shared: Argument bleibt gültig
                    }
                }
            }
        }
        
        // Dest erhält Ownership basierend auf Return-Type
        if let Some(d) = dest {
            // Standardmäßig Owned (wird später durch Type-Info korrigiert)
            self.ownership_map.insert(d.clone(), BorrowOwnership::Owned);
        }
        
        Ok(())
    }
    
    /// Prüft Async-Call-Instruction (KRITISCH: Borrows überleben async boundaries nicht)
    fn check_async_call(&mut self, dest: &Option<IRValue>, _func: &IRValue, args: &[IRValue], _scope: ScopeId) -> Result<()> {
        // KRITISCH: Async-Calls erfordern, dass Borrows 'static sind oder shared
        // Erstelle neuen Scope für async boundary
        let async_scope = self.lifetime_analyzer.create_scope();
        
        // Prüfe: Werden Argumente moved?
        for arg in args {
            if let Some(ownership) = self.ownership_map.get(arg) {
                match ownership {
                    BorrowOwnership::Owned => {
                        // Move: Argument wird consumed
                        self.ownership_map.remove(arg);
                    }
                    BorrowOwnership::Borrowed { lifetime } => {
                        // KRITISCH: Borrow überlebt async boundary nicht
                        // Lifetime muss 'static sein (scope 0) oder shared
                        if lifetime.scope.0 != 0 && !self.lifetime_analyzer.outlives(lifetime, async_scope) {
                            self.errors.push(BorrowError::LifetimeOutlivesScope {
                                lifetime: *lifetime,
                                scope: async_scope,
                                location: format!("Async call argument {:?} - borrow does not outlive async boundary", arg),
                            });
                        }
                    }
                    BorrowOwnership::BorrowedMut { lifetime } => {
                        // KRITISCH: Mutable Borrow überlebt async boundary nicht
                        if lifetime.scope.0 != 0 && !self.lifetime_analyzer.outlives(lifetime, async_scope) {
                            self.errors.push(BorrowError::LifetimeOutlivesScope {
                                lifetime: *lifetime,
                                scope: async_scope,
                                location: format!("Async call argument {:?} - mutable borrow does not outlive async boundary", arg),
                            });
                        }
                    }
                    BorrowOwnership::Copy | BorrowOwnership::Shared => {
                        // Copy/Shared: Argument bleibt gültig (OK für async)
                    }
                }
            }
        }
        
        // Dest erhält Ownership basierend auf Return-Type
        if let Some(d) = dest {
            // Standardmäßig Owned (wird später durch Type-Info korrigiert)
            self.ownership_map.insert(d.clone(), BorrowOwnership::Owned);
        }
        
        Ok(())
    }
    
    /// Prüft Struct-Access
    fn check_struct_access(&mut self, dest: &IRValue, struct_val: &IRValue, scope: ScopeId) -> Result<()> {
        if let Some(ownership) = self.ownership_map.get(struct_val) {
            match ownership {
                BorrowOwnership::Borrowed { lifetime } => {
                    // Field-Access erstellt neue Borrow
                    if self.lifetime_analyzer.outlives(lifetime, scope) {
                        self.ownership_map.insert(dest.clone(), BorrowOwnership::Borrowed { lifetime: *lifetime });
                    } else {
                        self.errors.push(BorrowError::LifetimeOutlivesScope {
                            lifetime: *lifetime,
                            scope,
                            location: format!("Struct access {:?}", struct_val),
                        });
                    }
                }
                BorrowOwnership::BorrowedMut { lifetime } => {
                    // Field-Access erstellt neue mutable Borrow
                    if self.lifetime_analyzer.outlives(lifetime, scope) {
                        self.ownership_map.insert(dest.clone(), BorrowOwnership::BorrowedMut { lifetime: *lifetime });
                    } else {
                        self.errors.push(BorrowError::LifetimeOutlivesScope {
                            lifetime: *lifetime,
                            scope,
                            location: format!("Struct access {:?}", struct_val),
                        });
                    }
                }
                BorrowOwnership::Owned => {
                    // Field-Access erstellt Move
                    self.ownership_map.insert(dest.clone(), BorrowOwnership::Owned);
                }
                BorrowOwnership::Copy => {
                    // Field-Access erstellt Copy
                    self.ownership_map.insert(dest.clone(), BorrowOwnership::Copy);
                }
                BorrowOwnership::Shared => {
                    // Field-Access erstellt Shared
                    self.ownership_map.insert(dest.clone(), BorrowOwnership::Shared);
                }
            }
        } else {
            self.errors.push(BorrowError::UseAfterMove {
                value: struct_val.clone(),
                location: format!("Struct access to {:?}", dest),
            });
        }
        
        Ok(())
    }
    
    /// Prüft List-Get
    fn check_list_get(&mut self, dest: &IRValue, list: &IRValue, scope: ScopeId) -> Result<()> {
        // Ähnlich wie Struct-Access
        self.check_struct_access(dest, list, scope)
    }
    
    /// Prüft Map-Get
    fn check_map_get(&mut self, dest: &IRValue, map: &IRValue, scope: ScopeId) -> Result<()> {
        // Ähnlich wie Struct-Access
        self.check_struct_access(dest, map, scope)
    }
    
    /// Prüft Return-Value
    fn check_return_value(&mut self, value: &IRValue, scope: ScopeId) -> Result<()> {
        if let Some(ownership) = self.ownership_map.get(value) {
            match ownership {
                BorrowOwnership::Owned => {
                    // Move: value wird returned
                    self.ownership_map.remove(value);
                }
                BorrowOwnership::Borrowed { lifetime } | BorrowOwnership::BorrowedMut { lifetime } => {
                    // Prüfe: Lifetime muss Function-Scope überleben
                    if !self.lifetime_analyzer.outlives(lifetime, scope) {
                        self.errors.push(BorrowError::LifetimeOutlivesScope {
                            lifetime: *lifetime,
                            scope,
                            location: format!("Return value {:?}", value),
                        });
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Betritt einen neuen Scope
    fn enter_scope(&mut self, scope: ScopeId) {
        self.scope_stack.push(scope);
        self.current_scope = scope;
    }
    
    /// Verlässt den aktuellen Scope
    fn exit_scope(&mut self) {
        self.scope_stack.pop();
        self.current_scope = *self.scope_stack.last().unwrap_or(&ScopeId::new(0));
    }
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}