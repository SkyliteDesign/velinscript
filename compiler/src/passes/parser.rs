use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::parser::ast::{Item, Program};
use crate::parser::parser::Parser;
use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct ParserPass;

impl ParserPass {
    pub fn new() -> Self {
        Self
    }

    fn resolve_imports(
        &self,
        program: &Program,
        base_path: &Path,
        context: &mut CompilationContext,
        visited_modules: &mut HashSet<String>,
        global_modules: &mut Vec<Item>,
    ) -> Result<()> {
        let mut modules_to_load = Vec::new();

        // 1. Collect all `use` statements that refer to local modules
        for item in &program.items {
            if let Item::Use(use_stmt) = item {
                if let Some(first_segment) = use_stmt.path.first() {
                    // SECURITY: Path-Traversal-Prüfung
                    if first_segment.contains("..")
                        || first_segment.contains("\\")
                        || first_segment.starts_with("/")
                    {
                        context.errors.push(crate::error::CompilerError::parse_error(
                            format!("Invalid module path: '{}'. Path traversal (../) and absolute paths are not allowed.", first_segment),
                            crate::error::ErrorLocation::new(0, 0),
                        ));
                        continue;
                    }

                    // SECURITY: Validierung von Modulnamen (nur alphanumerisch, underscore, hyphen)
                    if !first_segment
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                    {
                        context.errors.push(crate::error::CompilerError::parse_error(
                            format!("Invalid module name: '{}'. Only alphanumeric characters, underscore, and hyphen are allowed.", first_segment),
                            crate::error::ErrorLocation::new(0, 0),
                        ));
                        continue;
                    }

                    let module_path = base_path.join(format!("{}.velin", first_segment));
                    if module_path.exists() {
                        modules_to_load.push((first_segment.clone(), module_path));
                    } else {
                        // SECURITY: Fehler statt Warnung bei fehlenden Modulen
                        context
                            .errors
                            .push(crate::error::CompilerError::parse_error(
                                format!(
                                    "Module '{}' not found. Expected file: {}",
                                    first_segment,
                                    module_path.display()
                                ),
                                crate::error::ErrorLocation::new(0, 0),
                            ));
                    }
                }
            }
        }

        // 2. Load and parse these modules
        for (mod_name, mod_path) in modules_to_load {
            let mod_path_str = mod_path.to_string_lossy().to_string();

            // Check if already visited to prevent infinite recursion and diamonds
            if visited_modules.contains(&mod_path_str) {
                continue;
            }
            visited_modules.insert(mod_path_str.clone());

            // Get source: from context or read file
            let source = if let Some(src) = context.source_map.get(&mod_path_str) {
                src.clone()
            } else {
                match fs::read_to_string(&mod_path) {
                    Ok(s) => {
                        context.add_source(mod_path_str.clone(), s.clone());
                        s
                    }
                    Err(e) => {
                        eprintln!("Failed to read module {}: {}", mod_path.display(), e);
                        continue;
                    }
                }
            };

            match Parser::parse(&source) {
                Ok(mod_program) => {
                    let mod_dir = mod_path.parent().unwrap();

                    // Recurse to find more modules
                    self.resolve_imports(
                        &mod_program,
                        mod_dir,
                        context,
                        visited_modules,
                        global_modules,
                    )?;

                    // Wrap imported items in a Module item
                    // This enables namespacing (e.g., models.Item)
                    let mod_item = Item::Module(crate::parser::ast::Module {
                        name: mod_name.clone(),
                        items: mod_program.items,
                        visibility: crate::parser::ast::Visibility::Public,
                        documentation: None,
                    });

                    // Add to global modules list (Flattening)
                    global_modules.push(mod_item);
                }
                Err(e) => {
                    // SECURITY: Fehler statt nur Logging
                    context
                        .errors
                        .push(crate::error::CompilerError::parse_error(
                            format!(
                                "Failed to parse module {}: {} (at line {}, column {})",
                                mod_name, e.message, e.line, e.column
                            ),
                            crate::error::ErrorLocation::new(e.line, e.column),
                        ));
                    eprintln!("Failed to parse module {}: {}", mod_name, e.message);
                    eprintln!("  at line {}, column {}", e.line, e.column);
                    eprintln!("  found: {}", e.found);
                    if let Some(ctx) = e.source_context {
                        eprintln!("  Context:\n{}", ctx);
                    }
                }
            }
        }

        Ok(())
    }
}

impl Pass for ParserPass {
    fn name(&self) -> &str {
        "Parser"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        let root_source = context.source_map.get(&context.root_file).unwrap().clone();
        let root_path_buf = Path::new(&context.root_file)
            .parent()
            .unwrap()
            .to_path_buf();

        match Parser::parse(&root_source) {
            Ok(mut program) => {
                // Resolve imports
                let mut visited_modules = HashSet::new();
                visited_modules.insert(context.root_file.clone()); // Mark root as visited

                let mut global_modules = Vec::new();

                self.resolve_imports(
                    &program,
                    &root_path_buf,
                    context,
                    &mut visited_modules,
                    &mut global_modules,
                )?;

                // Add all resolved modules to the root program
                program.items.extend(global_modules);

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
