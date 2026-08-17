use crate::compiler::pass::Pass;
use crate::compiler::context::CompilationContext;
use crate::autofix::AutoFixer;
use anyhow::Result;

pub struct AutoFixPass {
    enabled: bool,
}

impl AutoFixPass {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Pass for AutoFixPass {
    fn name(&self) -> &str {
        "AutoFix"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let root_file = &context.root_file;
        if let Some(source) = context.source_map.get_mut(root_file) {
            println!("🔧 AutoFix Engine aktiv...");
            let fixer = AutoFixer::new(root_file.clone());
            let result = fixer.fix(source);

            if result.fixed {
                println!("✨ {} Fehler automatisch repariert:", result.reports.len());
                for report in &result.reports {
                    println!("  - {}: {} -> {}", report.rule, report.original, report.fixed);
                }
                *source = result.code;
            } else {
                println!("  Keine automatischen Korrekturen notwendig.");
            }
        }

        Ok(())
    }
}
