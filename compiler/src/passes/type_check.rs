use crate::compiler::pass::Pass;
use crate::compiler::context::CompilationContext;
use crate::type_checker::TypeChecker;
use crate::error::CompilerError;
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
            // TODO: TypeChecker currently doesn't support multiple files context directly,
            // but since we merged ASTs in ParserPass, it should see everything in 'program'.
            
            match checker.check_program(program) {
                Ok(_) => {
                    // Success
                }
                Err(errors) => {
                    for error in errors {
                        context.errors.push(error.into());
                    }
                }
            }
        }

        Ok(())
    }
}
