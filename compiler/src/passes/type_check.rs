use crate::borrow::checker::BorrowChecker;
use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::ir::builder::IRBuilder;
use crate::type_checker::TypeChecker;
use anyhow::Result;

pub struct TypeCheckPass {
    enabled: bool,
}

impl TypeCheckPass {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Pass for TypeCheckPass {
    fn name(&self) -> &str {
        "TypeCheck"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(program) = &context.program {
            let mut checker = TypeChecker::new();
            // Note: TypeChecker processes the entire merged AST from ParserPass,
            // so it sees all definitions across modules.

            match checker.check_program(program) {
                Ok(_) => {
                    // Type Checking erfolgreich
                }
                Err(errors) => {
                    for error in errors {
                        context.errors.push(error.into());
                    }
                }
            }

            // Borrow Checking (auf IR)
            if context.errors.is_empty() {
                let mut builder = IRBuilder::new();
                let ir_module = builder.build_module(program);

                let mut borrow_checker = BorrowChecker::new();
                if let Err(borrow_errors) = borrow_checker.check(&ir_module) {
                    for borrow_error in borrow_errors {
                        context
                            .errors
                            .push(crate::error::CompilerError::parse_error(
                                borrow_error.to_string(),
                                crate::error::ErrorLocation::new(0, 0),
                            ));
                    }
                }
            }
        }

        Ok(())
    }
}
