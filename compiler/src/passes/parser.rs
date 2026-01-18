use crate::compiler::pass::Pass;
use crate::compiler::context::CompilationContext;
use crate::parser::parser::Parser;
use crate::parser::ast::{Program, Item};
use anyhow::Result;
use std::path::Path;
use std::fs;

pub struct ParserPass;

impl ParserPass {
    pub fn new() -> Self {
        Self
    }

    fn resolve_imports(&self, program: &mut Program, base_path: &Path, context: &mut CompilationContext) -> Result<()> {
        let mut new_items = Vec::new();
        let mut modules_to_load = Vec::new();

        // 1. Collect all `use` statements that refer to local modules
        for item in &program.items {
            if let Item::Use(use_stmt) = item {
                if let Some(first_segment) = use_stmt.path.first() {
                    let module_path = base_path.join(format!("{}.velin", first_segment));
                    if module_path.exists() {
                        modules_to_load.push((first_segment.clone(), module_path));
                    }
                }
            }
        }

        // 2. Load and parse these modules
        for (mod_name, mod_path) in modules_to_load {
            if context.source_map.contains_key(&mod_path.to_string_lossy().to_string()) {
                continue;
            }

            let source = match fs::read_to_string(&mod_path) {
                Ok(s) => s,
                Err(e) => {
                     eprintln!("Failed to read module {}: {}", mod_path.display(), e);
                     continue;
                }
            };

            context.add_source(mod_path.to_string_lossy().to_string(), source.clone());
            
            match Parser::parse(&source) {
                Ok(mut mod_program) => {
                    let mod_dir = mod_path.parent().unwrap();
                    self.resolve_imports(&mut mod_program, mod_dir, context)?;

                    // Flatten imports: add all items from the module to the current program
                    // This mimics #include behavior which seems to be what the examples expect
                    new_items.extend(mod_program.items);
                    
                    /*
                    let mod_item = Item::Module(crate::parser::ast::Module {
                        name: mod_name.clone(),
                        items: mod_program.items,
                        visibility: crate::parser::ast::Visibility::Public, 
                        documentation: None,
                    });
                    
                    new_items.push(mod_item);
                    */
                }
                Err(e) => {
                     eprintln!("Failed to parse module {}: {}", mod_name, e.message);
                     eprintln!("  at line {}, column {}", e.line, e.column);
                     eprintln!("  found: {}", e.found);
                     if let Some(ctx) = e.source_context {
                         eprintln!("  Context:\n{}", ctx);
                     }
                }
            }
        }
        
        program.items.extend(new_items);

        Ok(())
    }
}

impl Pass for ParserPass {
    fn name(&self) -> &str {
        "Parser"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        let root_source = context.source_map.get(&context.root_file).unwrap().clone();
        let root_path_buf = Path::new(&context.root_file).parent().unwrap().to_path_buf();
        
        match Parser::parse(&root_source) {
            Ok(mut program) => {
                // Resolve imports
                self.resolve_imports(&mut program, &root_path_buf, context)?;
                
                context.program = Some(program);
                Ok(())
            }
            Err(e) => {
                context.errors.push(e.into());
                Ok(())
            }
        }
    }
}
