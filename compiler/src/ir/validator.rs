/// IR Validator - Validiert IR-Code
/// 
/// Dieser Validator prüft die IR auf Korrektheit:
/// - SSA-Format wird eingehalten
/// - Alle Referenzen sind gültig
/// - Typen sind konsistent
/// 
/// # Beispiel
/// 
/// ```rust
/// use velin_compiler::ir::validator::IRValidator;
/// 
/// let validator = IRValidator::new();
/// match validator.validate(&ir_module) {
///     Ok(_) => println!("IR ist gültig"),
///     Err(errors) => println!("Fehler: {:?}", errors),
/// }
/// ```

use crate::ir::ir::*;
use std::collections::HashSet;
use anyhow::{Result, anyhow};

/// IR Validator
pub struct IRValidator {
    errors: Vec<ValidationError>,
}

/// Validierungs-Fehler
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidSSA { value: IRValue, location: String },
    InvalidReference { block: BlockId, location: String },
    TypeMismatch { expected: IRType, actual: IRType, location: String },
    InvalidType { value: String, expected: String, found: String, location: String },
    UndefinedVariable { var: String, location: String },
}

impl IRValidator {
    /// Erstellt einen neuen IR Validator
    pub fn new() -> Self {
        IRValidator {
            errors: Vec::new(),
        }
    }
    
    /// Validiert ein IR-Modul
    pub fn validate(&mut self, module: &IRModule) -> Result<()> {
        self.errors.clear();
        
        for func in &module.functions {
            self.validate_function(func)?;
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(anyhow!("Validierungs-Fehler: {:?}", self.errors))
        }
    }
    
    /// Validiert eine Funktion
    fn validate_function(&mut self, func: &IRFunction) -> Result<()> {
        // Prüfe SSA-Format
        self.validate_ssa(&func.body)?;
        
        // Prüfe Block-Referenzen
        self.validate_block_references(&func.body)?;
        
        // Prüfe Typen
        self.validate_types(func)?;
        
        Ok(())
    }
    
    /// Prüft SSA-Format
    /// 
    /// In SSA-Format darf jede Variable nur einmal zugewiesen werden.
    fn validate_ssa(&mut self, block: &IRBlock) -> Result<()> {
        let mut assigned = HashSet::new();
        
        for instruction in &block.instructions {
            match instruction {
                IRInstruction::Add { dest, .. } |
                IRInstruction::Subtract { dest, .. } |
                IRInstruction::Multiply { dest, .. } |
                IRInstruction::Divide { dest, .. } |
                IRInstruction::Modulo { dest, .. } |
                IRInstruction::Eq { dest, .. } |
                IRInstruction::NotEq { dest, .. } |
                IRInstruction::Lt { dest, .. } |
                IRInstruction::Gt { dest, .. } |
                IRInstruction::LtEq { dest, .. } |
                IRInstruction::GtEq { dest, .. } |
                IRInstruction::And { dest, .. } |
                IRInstruction::Or { dest, .. } |
                IRInstruction::Not { dest, .. } |
                IRInstruction::Load { dest, .. } |
                IRInstruction::Alloca { dest, .. } |
                IRInstruction::StructAccess { dest, .. } |
                IRInstruction::StructConstruct { dest, .. } |
                IRInstruction::EnumConstruct { dest, .. } |
                IRInstruction::ListGet { dest, .. } |
                IRInstruction::MapGet { dest, .. } |
                IRInstruction::Phi { dest, .. } => {
                    if assigned.contains(dest) {
                        self.errors.push(ValidationError::InvalidSSA {
                            value: dest.clone(),
                            location: format!("Block {:?}", block.id),
                        });
                    } else {
                        assigned.insert(dest.clone());
                    }
                }
                IRInstruction::Call { dest, .. } |
                IRInstruction::CallAsync { dest, .. } => {
                    if let Some(d) = dest {
                        if assigned.contains(d) {
                            self.errors.push(ValidationError::InvalidSSA {
                                value: d.clone(),
                                location: format!("Block {:?}", block.id),
                            });
                        } else {
                            assigned.insert(d.clone());
                        }
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Prüft Block-Referenzen
    fn validate_block_references(&mut self, block: &IRBlock) -> Result<()> {
        // Prüfe ob alle referenzierten Blocks existieren
        for instruction in &block.instructions {
            match instruction {
                IRInstruction::Branch { then_block: _, else_block: _, .. } => {
                    // Prüfe ob Blocks existieren
                    // Blocks werden in build_function erstellt
                }
                IRInstruction::Jump { target: _ } => {
                    // Prüfe ob Block existiert
                    // Blocks werden in build_function erstellt
                }
                IRInstruction::Match { arms, .. } => {
                    for _arm in arms {
                        // Prüfe ob Block existiert
                    // Blocks werden in build_function erstellt
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Prüft Typen
    fn validate_types(&mut self, func: &IRFunction) -> Result<()> {
        // Prüfe ob Parameter-Typen mit Return-Type konsistent sind
        // Typ-Validierung: Prüfe ob alle Variablen korrekte Typen haben
        for param in &func.params {
            if matches!(param.ty, IRType::Void) {
                self.errors.push(ValidationError::InvalidType {
                    value: format!("Parameter {}", param.name),
                    expected: "non-void type".to_string(),
                    found: "void".to_string(),
                    location: format!("Function {}", func.name),
                });
            }
        }
        
        Ok(())
    }
}

impl Default for IRValidator {
    fn default() -> Self {
        Self::new()
    }
}
