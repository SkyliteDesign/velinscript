use crate::compiler::context::CompilationContext;
use anyhow::Result;

pub trait Pass {
    fn name(&self) -> &str;
    fn run(&self, context: &mut CompilationContext) -> Result<()>;
}
