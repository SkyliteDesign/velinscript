use crate::compiler::pass::Pass;
use crate::compiler::context::CompilationContext;
use crate::compiler::language::VELISCH_LANGUAGE_NAME;
use crate::codegen::{RustCodeGenerator, PhpCodeGenerator, PythonCodeGenerator, GoCodeGenerator, TypeScriptCodeGenerator, JavaScriptCodeGenerator, JavaCodeGenerator, CSharpCodeGenerator, CodeGenerator, CodegenConfig, TargetLanguage, IRCodeGenerator};
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
    use_ir: bool,
}

impl CodegenPass {
    pub fn new(output_path: Option<std::path::PathBuf>, show_code: bool, target: TargetLanguage, framework: Option<String>) -> Self {
        Self { 
            output_path, 
            show_code, 
            target, 
            framework,
            use_ir: true,
        }
    }
    
    pub fn with_ir(mut self, use_ir: bool) -> Self {
        self.use_ir = use_ir;
        self
    }

    pub fn effective_framework(&self, context: &CompilationContext) -> Option<String> {
        context
            .framework
            .clone()
            .or_else(|| self.framework.clone())
    }
}

impl Pass for CodegenPass {
    fn name(&self) -> &str {
        "Codegen"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        let _velisch_check = VELISCH_LANGUAGE_NAME;
        
        if context.has_errors() {
            return Ok(());
        }

        let framework = self.effective_framework(context);
        context.metadata.insert(
            "codegen_framework".to_string(),
            framework
                .clone()
                .unwrap_or_else(|| "axum".to_string()),
        );
        context.metadata.insert(
            "codegen_use_ir".to_string(),
            self.use_ir.to_string(),
        );

        if let Some(program) = &context.program {
            let mut profiler = ProfilingCollector::new();
            let start = Instant::now();
            
            let mut generated_code = if self.use_ir {
                let ir_start = Instant::now();
                let mut builder = IRBuilder::new();
                let mut ir_module = builder.build_module(program);
                profiler.record_function_call("ir_build".to_string(), ir_start.elapsed().as_secs_f64());
                
                let opt_start = Instant::now();
                let optimizer = IROptimizer::new();
                optimizer.optimize(&mut ir_module);
                profiler.record_function_call("ir_optimize".to_string(), opt_start.elapsed().as_secs_f64());
                
                let val_start = Instant::now();
                let mut validator = IRValidator::new();
                if let Err(e) = validator.validate(&ir_module) {
                    eprintln!("⚠️  IR-Validierungs-Warnung: {}", e);
                }
                profiler.record_function_call("ir_validate".to_string(), val_start.elapsed().as_secs_f64());
                
                let codegen_start = Instant::now();
                let ir_codegen = IRCodeGenerator::new(self.target)
                    .with_framework(framework.clone());
                let code = ir_codegen.generate(&ir_module)?;
                profiler.record_function_call("ir_codegen".to_string(), codegen_start.elapsed().as_secs_f64());
                code
            } else {
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
                    framework: framework.clone(),
                    orm: None,
                    output_path: self.output_path.clone(),
                };
                
                let code = generator.generate(program, &config)?;
                profiler.record_function_call(format!("codegen_{:?}", self.target), gen_start.elapsed().as_secs_f64());
                code
            };

            // Product path: writing Rust HTTP to main.rs → wrap with tokio main
            let write_main = self
                .output_path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                == Some("main.rs");
            if self.target == TargetLanguage::Rust
                && write_main
                && generated_code.contains("create_router")
                && !generated_code.contains("async fn main")
            {
                generated_code = crate::codegen::axum_main_wrapper(&generated_code);
            }
            
            profiler.record_function_call("codegen_total".to_string(), start.elapsed().as_secs_f64());
            
            if let Err(e) = profiler.persist() {
                eprintln!("⚠️  Profiling-Persistierung fehlgeschlagen: {}", e);
            }
            
            if self.show_code {
                println!("\n--- Generierter Code ({}) ---\n", self.target);
                println!("{}", generated_code);
            }
            
            if let Some(path) = &self.output_path {
                fs::write(path, &generated_code)
                    .map_err(|e| anyhow::anyhow!("Failed to write output file: {}", e))?;
                println!("✓ Code generiert: {}", path.display());
            }

            context.metadata.insert("generated_code".to_string(), generated_code);
        }

        Ok(())
    }
}
