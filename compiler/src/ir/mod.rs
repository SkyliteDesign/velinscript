/// Intermediate Representation (IR) für VelinScript
/// 
/// Dieses Modul implementiert eine echte Intermediate Representation zwischen AST und Code-Generierung.
/// Die IR verwendet SSA (Single Static Assignment) Format für optimierte Code-Generierung.
/// 
/// # Architektur
/// 
/// ```
/// Source Code → AST → IR → Optimized IR → Target Code
/// ```
/// 
/// # Module
/// 
/// - `ir.rs` - IR-Strukturen (IRModule, IRFunction, IRInstruction, etc.)
/// - `builder.rs` - AST → IR Konvertierung
/// - `optimizer.rs` - IR-Optimierungen (Dead Code Elimination, Constant Folding, etc.)
/// - `validator.rs` - IR-Validierung
/// 
/// # Beispiel
/// 
/// ```rust
/// use velin_compiler::ir::builder::IRBuilder;
/// use velin_compiler::ir::optimizer::IROptimizer;
/// 
/// let mut builder = IRBuilder::new();
/// let mut ir_module = builder.build_module(&ast_program);
/// 
/// let optimizer = IROptimizer::new();
/// optimizer.optimize(&mut ir_module);
/// ```
pub mod ir;
pub mod builder;
pub mod optimizer;
pub mod validator;

pub use ir::*;
pub use builder::IRBuilder;
pub use optimizer::IROptimizer;
pub use validator::IRValidator;
