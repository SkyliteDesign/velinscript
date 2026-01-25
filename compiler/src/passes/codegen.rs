use crate::codegen::{
    CSharpCodeGenerator, CodeGenerator, CodegenConfig, GoCodeGenerator, IRCodeGenerator,
    JavaCodeGenerator, JavaScriptCodeGenerator, PhpCodeGenerator, PythonCodeGenerator,
    RustCodeGenerator, TargetLanguage, TypeScriptCodeGenerator,
};
use crate::compiler::context::CompilationContext;
use crate::compiler::language::VELISCH_LANGUAGE_NAME;
use crate::compiler::pass::Pass;
use crate::ir::{IRBuilder, IROptimizer, IRValidator};
use crate::optimizer::profiling::ProfilingCollector;
use anyhow::Result;
use std::fs;
use std::time::Instant;

pub struct CodegenPass {
    output_path: Option<std::path::PathBuf>,
    show_code: bool,
    target: TargetLanguage,
    framework: Option<String>,
    use_ir: bool, // Neu: Nutze IR statt direkter AST → Code
}

impl CodegenPass {
    pub fn new(
        output_path: Option<std::path::PathBuf>,
        show_code: bool,
        target: TargetLanguage,
        framework: Option<String>,
    ) -> Self {
        Self {
            output_path,
            show_code,
            target,
            framework,
            use_ir: true, // Standardmäßig IR verwenden
        }
    }

    pub fn with_ir(mut self, use_ir: bool) -> Self {
        self.use_ir = use_ir;
        self
    }
}

impl Pass for CodegenPass {
    fn name(&self) -> &str {
        "Codegen"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        // Velisch Identity Check - Fingerabdruck im Codegen Pass
        let _velisch_check = VELISCH_LANGUAGE_NAME;

        if context.has_errors() {
            return Ok(()); // Don't generate code if there are errors
        }

        if let Some(program) = &context.program {
            // Profiling: Track Codegen-Performance
            let mut profiler = ProfilingCollector::new();
            let start = Instant::now();

            let generated_code = if self.use_ir {
                // IR-basierte Code-Generierung
                // 1. AST → IR
                let ir_start = Instant::now();
                let mut builder = IRBuilder::new();
                let mut ir_module = builder.build_module(program);
                profiler
                    .record_function_call("ir_build".to_string(), ir_start.elapsed().as_secs_f64());

                // 2. IR-Optimierungen
                let opt_start = Instant::now();
                let optimizer = IROptimizer::new();
                optimizer.optimize(&mut ir_module);
                profiler.record_function_call(
                    "ir_optimize".to_string(),
                    opt_start.elapsed().as_secs_f64(),
                );

                // 3. IR-Validierung
                let val_start = Instant::now();
                let mut validator = IRValidator::new();
                if let Err(e) = validator.validate(&ir_module) {
                    eprintln!("⚠️  IR-Validierungs-Warnung: {}", e);
                    // Weiter mit Code-Generierung trotz Warnungen
                }
                profiler.record_function_call(
                    "ir_validate".to_string(),
                    val_start.elapsed().as_secs_f64(),
                );

                // 4. IR → Target Code
                let codegen_start = Instant::now();
                let ir_codegen = IRCodeGenerator::new(self.target);
                let code = match ir_codegen.generate(&ir_module) {
                    Ok(c) => c,
                    Err(e) => {
                        context.add_error(
                            crate::error::CompilerError::codegen_error_with_location(
                                format!("IR code generation failed: {}", e),
                                crate::error::ErrorLocation::with_file(
                                    0,
                                    0,
                                    context.root_file.clone(),
                                ),
                                Some(format!(
                                    "Target: {:?}, Module: {}",
                                    self.target, context.root_file
                                )),
                            ),
                        );
                        return Ok(()); // Stoppe Pass, aber nicht Compiler
                    }
                };
                profiler.record_function_call(
                    "ir_codegen".to_string(),
                    codegen_start.elapsed().as_secs_f64(),
                );
                code
            } else {
                // Direkte AST → Code Generierung (Legacy)
                let gen_start = Instant::now();
                let mut generator: Box<dyn CodeGenerator> = match self.target {
                    TargetLanguage::Rust => Box::new(RustCodeGenerator::new()),
                    TargetLanguage::Php => Box::new(PhpCodeGenerator::new()),
                    TargetLanguage::Python => Box::new(PythonCodeGenerator::new()),
                    TargetLanguage::Go => Box::new(GoCodeGenerator::new()),
                    TargetLanguage::TypeScript => Box::new(TypeScriptCodeGenerator::new()),
                    TargetLanguage::JavaScript => Box::new(JavaScriptCodeGenerator::new()),
                    TargetLanguage::Java => Box::new(JavaCodeGenerator::new()),
                    TargetLanguage::CSharp => Box::new(CSharpCodeGenerator::new()),
                };

                let config = CodegenConfig {
                    target: self.target,
                    framework: self.framework.clone(),
                    orm: None,
                    output_path: self.output_path.clone(),
                };

                let code = match generator.generate(program, &config) {
                    Ok(c) => c,
                    Err(e) => {
                        context.add_error(
                            crate::error::CompilerError::codegen_error_with_location(
                                format!("Code generation failed: {}", e),
                                crate::error::ErrorLocation::with_file(
                                    0,
                                    0,
                                    context.root_file.clone(),
                                ),
                                Some(format!(
                                    "Target: {:?}, Framework: {:?}",
                                    self.target, self.framework
                                )),
                            ),
                        );
                        return Ok(()); // Stoppe Pass, aber nicht Compiler
                    }
                };
                profiler.record_function_call(
                    format!("codegen_{:?}", self.target),
                    gen_start.elapsed().as_secs_f64(),
                );
                code
            };

            // Profiling: Track Gesamtzeit
            profiler
                .record_function_call("codegen_total".to_string(), start.elapsed().as_secs_f64());

            // Persistiere Profiling-Daten
            if let Err(e) = profiler.persist() {
                eprintln!("⚠️  Profiling-Persistierung fehlgeschlagen: {}", e);
            }

            if self.show_code {
                println!("\n--- Generierter Code ({}) ---\n", self.target);
                println!("{}", generated_code);
            }

            if let Some(path) = &self.output_path {
                match fs::write(path, generated_code) {
                    Ok(_) => {
                        println!("✓ Code generiert: {}", path.display());
                    }
                    Err(e) => {
                        context.add_error(crate::error::CompilerError::io_error(format!(
                            "Failed to write output file {}: {}",
                            path.display(),
                            e
                        )));
                        return Ok(()); // Stoppe Pass, aber nicht Compiler
                    }
                }
            }
        }

        Ok(())
    }
}
