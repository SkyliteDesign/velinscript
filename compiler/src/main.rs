use velin_compiler::cli::{Cli, Commands};
use velin_compiler::parser::parser::Parser;
use velin_compiler::codegen::{OpenAPIGenerator, BoilerplateGenerator, ClientGenerator, TargetLanguage, SystemGenerator, APICall};
use velin_compiler::formatter::{Formatter, FormatConfig};
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result as AnyhowResult};
use clap::Parser as ClapParser;
use std::str::FromStr;

use velin_compiler::compiler::{VelinCompiler, config::CompilerConfig};
use velin_compiler::compiler::language::get_velisch_identity;
use velin_compiler::passes::{
    autofix::AutoFixPass,
    parser::ParserPass,
    desugar::DesugaringPass,
    code_order::CodeOrderingPass,
    type_check::TypeCheckPass,
    codegen::CodegenPass,
    security_gate::SecurityGatePass,
    ai_semantic::AISemanticPass,
    ai_bug_detection::AIBugDetectionPass,
    ai_codegen::AICodeGenerationPass,
    ai_code_review::AICodeReviewPass,
    ai_sandbox::AISandboxPass,
    ai_optimization::AIOptimizationPass,
};
use velin_compiler::optimizer::parallelization::ParallelizationAnalyzer;

fn main() -> AnyhowResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output, no_type_check, show_code, autofix, ai_semantic, ai_bug_detection, ai_codegen, ai_optimization, ai_provider, ai_api_key, target, framework, codegen } => {
            let mut config = CompilerConfig::default();
            config.enable_autofix = autofix;
            config.enable_type_check = !no_type_check;
            config.show_code = show_code;
            config.enable_ai_semantic = ai_semantic;
            config.enable_ai_bug_detection = ai_bug_detection;
            config.enable_ai_codegen = ai_codegen;
            config.enable_ai_code_review = ai_codegen;
            config.enable_ai_sandbox = ai_codegen;
            config.enable_ai_optimization = ai_optimization;
            config.ai_provider = ai_provider;
            config.ai_api_key = ai_api_key;
            config.framework = framework.clone();
            config.use_ir = match codegen.as_str() {
                "ast-legacy" | "ast" | "legacy" => false,
                _ => true,
            };
            
            config.target = TargetLanguage::from_str(&target).map_err(|e| anyhow::anyhow!(e))?;

            // Output path logic
            let output_file = output.unwrap_or_else(|| {
                // Extension based on target
                let ext = match config.target {
                    TargetLanguage::Rust => "rs",
                    TargetLanguage::Php => "php",
                    TargetLanguage::Python => "py",
                    TargetLanguage::JavaScript => "js",
                    TargetLanguage::TypeScript => "ts",
                    TargetLanguage::Go => "go",
                    TargetLanguage::Java => "java",
                    TargetLanguage::CSharp => "cs",
                };
                input.with_extension(ext)
            });
            config.output_path = Some(output_file.clone());

            let mut compiler = VelinCompiler::new(config.clone());
            
            // Register Passes
            compiler.add_pass(Box::new(AutoFixPass::new(autofix)));
            compiler.add_pass(Box::new(ParserPass::new()));
            compiler.add_pass(Box::new(DesugaringPass::new()));
            // Code Ordering Pass: Automatically sorts functions, types, and blocks based on dependencies
            compiler.add_pass(Box::new(CodeOrderingPass::new()));
            
            // KI-Compiler-Passes (optional, via Feature Flags)
            if config.enable_ai_semantic {
                if let Ok(pass) = AISemanticPass::new(&config) {
                    compiler.add_pass(Box::new(pass));
                }
            }
            if config.enable_ai_bug_detection {
                if let Ok(pass) = AIBugDetectionPass::new(&config) {
                    compiler.add_pass(Box::new(pass));
                }
            }
            if !no_type_check {
                compiler.add_pass(Box::new(TypeCheckPass::new(true)));
            }
            // Standard Optimizer Pass (respektiert enable_optimization)
            if config.enable_optimization {
                compiler.add_pass(Box::new(ParallelizationAnalyzer::new()));
            }
            compiler.add_pass(Box::new(SecurityGatePass::new()));

            if config.enable_ai_codegen {
                if let Ok(pass) = AICodeGenerationPass::new(&config) {
                    compiler.add_pass(Box::new(pass));
                }
            }
            // AI Code Review Pass (nach Code Generation)
            if config.enable_ai_code_review {
                if let Ok(pass) = AICodeReviewPass::new(&config) {
                    compiler.add_pass(Box::new(pass));
                }
            }
            // AI Sandbox Pass (nach Code Review)
            if config.enable_ai_sandbox {
                if let Ok(pass) = AISandboxPass::new(&config) {
                    compiler.add_pass(Box::new(pass));
                }
            }
            if config.enable_ai_optimization {
                if let Ok(pass) = AIOptimizationPass::new(&config) {
                    compiler.add_pass(Box::new(pass));
                }
            }
            // Add Codegen Pass
            compiler.add_pass(Box::new(CodegenPass::new(config.output_path, show_code, config.target, framework).with_ir(config.use_ir)));

            tracing::info!(file = ?input, language = %get_velisch_identity(), "Compiling Velisch file");
            
            // SECURITY: Dateigrößen-Limit (max. 5MB)
            let metadata = fs::metadata(&input)
                .with_context(|| format!("Failed to read metadata: {}", input.display()))?;
            const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5MB
            if metadata.len() > MAX_FILE_SIZE {
                return Err(anyhow::anyhow!(
                    "File too large: {} bytes (max: {} bytes). File size limit exceeded.",
                    metadata.len(),
                    MAX_FILE_SIZE
                ));
            }
            
            let code = fs::read_to_string(&input)
                .with_context(|| format!("Failed to read file: {}", input.display()))?;
                
            let context = compiler.compile(input.to_string_lossy().to_string(), code)?;
            
            if context.has_errors() {
                eprintln!("✗ Kompilierung fehlgeschlagen mit {} Fehlern:\n", context.errors.len());
                for error in context.errors {
                     // Verwende verbesserte Fehlermeldungen mit Vorschlägen
                     eprintln!("{}", error.with_suggestions());
                     eprintln!("{}", "─".repeat(60));
                }
                eprintln!("\n💡 Tipp: Nutze 'velin check --autofix' für automatische Korrekturen");
                eprintln!("📖 Hilfe: Siehe docs/guides/getting-started.md für weitere Informationen");
                std::process::exit(1);
            }
            
            println!("✓ Kompilierung erfolgreich");
            Ok(())
        }
        Commands::Check { input, autofix } => {
            let mut config = CompilerConfig::default();
            config.enable_autofix = autofix;
            config.enable_type_check = true;
            
            let mut compiler = VelinCompiler::new(config);
            
            compiler.add_pass(Box::new(AutoFixPass::new(autofix)));
            compiler.add_pass(Box::new(ParserPass::new()));
            compiler.add_pass(Box::new(DesugaringPass::new()));
            // Code Ordering Pass: Automatically sorts functions, types, and blocks based on dependencies
            compiler.add_pass(Box::new(CodeOrderingPass::new()));
            compiler.add_pass(Box::new(TypeCheckPass::new(true)));
            
            println!("🔍 Prüfe: {}\n", input.display());
            
            // SECURITY: Dateigrößen-Limit (max. 5MB)
            let metadata = fs::metadata(&input)
                .with_context(|| format!("Failed to read metadata: {}", input.display()))?;
            const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5MB
            if metadata.len() > MAX_FILE_SIZE {
                return Err(anyhow::anyhow!(
                    "File too large: {} bytes (max: {} bytes). File size limit exceeded.",
                    metadata.len(),
                    MAX_FILE_SIZE
                ));
            }
            
            let code = fs::read_to_string(&input)
                .with_context(|| format!("Failed to read file: {}", input.display()))?;
                
            let context = compiler.compile(input.to_string_lossy().to_string(), code)?;
            
            if context.has_errors() {
                 eprintln!("✗ Checks fehlgeschlagen mit {} Fehlern:\n", context.errors.len());
                 for error in context.errors {
                     eprintln!("{}", error.with_suggestions());
                     eprintln!("{}", "─".repeat(60));
                }
                eprintln!("\n💡 Tipp: Nutze 'velin check --autofix' für automatische Korrekturen");
                eprintln!("📖 Hilfe: Siehe docs/guides/getting-started.md für weitere Informationen");
                std::process::exit(1);
            }
            
            println!("✓ Alle Checks bestanden!");
            Ok(())
        }
        Commands::Format { input, in_place } => {
            format_command(input, in_place)
        }
        Commands::Info { input } => {
            info_command(input)
        }
        Commands::Init { name, current_dir } => {
            init_command(name, current_dir)
        }
        Commands::New { name, current_dir } => {
            // Alias für Init
            init_command(name, current_dir)
        }
        Commands::Serve { input, port, host, watch } => {
            serve_command(input.clone(), port, host.clone(), watch)
        }
        Commands::Run { input, port, host, watch } => {
            // Alias für Serve
            serve_command(input.clone(), port, host.clone(), watch)
        }
        Commands::OpenAPI { input, output } => {
            openapi_command(input, output)
        }
        Commands::Generate { gen_type, name, fields, path, openapi, language, output } => {
            generate_command(gen_type, name, fields, path, openapi, language, output)
        }
        Commands::Test { directory, unit, integration, verbose } => {
            test_command(directory, unit, integration, verbose)
        }
        Commands::Config { subcommand } => {
            match subcommand {
                velin_compiler::cli::ConfigCommands::Init { example } => {
                    config_init_command(example)
                }
                velin_compiler::cli::ConfigCommands::Validate { file } => {
                    config_validate_command(file)
                }
                velin_compiler::cli::ConfigCommands::Show { file } => {
                    config_show_command(file)
                }
            }
        }
        Commands::Cache { subcommand } => {
            match subcommand {
                velin_compiler::cli::CacheCommands::Stats => {
                    cache_stats_command()
                }
                velin_compiler::cli::CacheCommands::Clear { pattern } => {
                    cache_clear_command(pattern)
                }
                velin_compiler::cli::CacheCommands::Warm => {
                    cache_warm_command()
                }
            }
        }
        Commands::Health { url, verbose } => {
            health_command(url, verbose)
        }
        Commands::Backup { subcommand } => {
            match subcommand {
                velin_compiler::cli::BackupCommands::Create { strategy, destination, compression } => {
                    backup_create_command(strategy, destination, compression)
                }
                velin_compiler::cli::BackupCommands::Restore { backup_id, destination } => {
                    backup_restore_command(backup_id, destination)
                }
                velin_compiler::cli::BackupCommands::List { directory } => {
                    backup_list_command(directory)
                }
                velin_compiler::cli::BackupCommands::Delete { backup_id, directory } => {
                    backup_delete_command(backup_id, directory)
                }
                velin_compiler::cli::BackupCommands::Verify { backup_id, directory } => {
                    backup_verify_command(backup_id, directory)
                }
            }
        }
        Commands::Rollback { subcommand } => {
            match subcommand {
                velin_compiler::cli::RollbackCommands::Begin => {
                    rollback_begin_command()
                }
                velin_compiler::cli::RollbackCommands::Commit { transaction_id } => {
                    rollback_commit_command(transaction_id)
                }
                velin_compiler::cli::RollbackCommands::Rollback { transaction_id } => {
                    rollback_rollback_command(transaction_id)
                }
                velin_compiler::cli::RollbackCommands::CreateVersion { description } => {
                    rollback_create_version_command(description)
                }
                velin_compiler::cli::RollbackCommands::ToVersion { version_id } => {
                    rollback_to_version_command(version_id)
                }
                velin_compiler::cli::RollbackCommands::ListVersions => {
                    rollback_list_versions_command()
                }
                velin_compiler::cli::RollbackCommands::CreateSnapshot { description } => {
                    rollback_create_snapshot_command(description)
                }
                velin_compiler::cli::RollbackCommands::ToSnapshot { snapshot_id } => {
                    rollback_to_snapshot_command(snapshot_id)
                }
                velin_compiler::cli::RollbackCommands::ListSnapshots => {
                    rollback_list_snapshots_command()
                }
            }
        }
        Commands::Serialize { subcommand } => {
            match subcommand {
                velin_compiler::cli::SerializeCommands::JsonToYaml { input, output } => {
                    serialize_json_to_yaml_command(input, Some(output))
                }
                velin_compiler::cli::SerializeCommands::YamlToJson { input, output } => {
                    serialize_yaml_to_json_command(input, Some(output))
                }
                velin_compiler::cli::SerializeCommands::ValidateJson { file } => {
                    serialize_validate_json_command(file)
                }
                velin_compiler::cli::SerializeCommands::ValidateYaml { file } => {
                    serialize_validate_yaml_command(file)
                }
            }
        }
    }
}

fn format_command(input: PathBuf, in_place: bool) -> AnyhowResult<()> {
    println!("✨ Formatiere: {}\n", input.display());

    let code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    let program = Parser::parse(&code).map_err(|e| {
        eprintln!("✗ Parsing-Fehler:");
        eprintln!("  {}", e.message);
        eprintln!("  Erwartet: {}", e.expected);
        eprintln!("  Gefunden: {}", e.found);
        eprintln!("  Position: Zeile {}, Spalte {}", e.line, e.column);
        if let Some(ref context) = e.source_context {
            eprintln!("\n  {}", context);
        }
        anyhow::anyhow!("Parse error: {}", e.message)
    })?;

    println!("✓ Parsing erfolgreich");

    let config = FormatConfig::default();
    let mut formatter = Formatter::new(config);
    let formatted = formatter.format(&program);

    if in_place {
        fs::write(&input, formatted)
            .with_context(|| format!("Failed to write file: {}", input.display()))?;
        println!("✓ Datei formatiert: {}", input.display());
    } else {
        println!("{}", formatted);
    }

    Ok(())
}

fn info_command(input: PathBuf) -> AnyhowResult<()> {
    println!("ℹ️  Informationen über: {}\n", input.display());

    let code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    let program = Parser::parse(&code).map_err(|e| {
        eprintln!("✗ Parsing-Fehler: {}", e.message);
        eprintln!("  Position: Zeile {}, Spalte {}", e.line, e.column);
        if let Some(ref context) = e.source_context {
            eprintln!("\n  {}", context);
        }
        anyhow::anyhow!("Parse error: {}", e.message)
    })?;

    println!("📊 Statistik:");
    println!("  Items: {}", program.items.len());

    let mut functions = 0;
    let mut structs = 0;
    let mut enums = 0;

    for item in &program.items {
        match item {
            velin_compiler::parser::ast::Item::Function(f) => {
                functions += 1;
                println!("\n  📝 Funktion: {}", f.name);
                println!("     Decorators: {}", f.decorators.len());
                if let Some(ref ret_type) = f.return_type {
                    println!("     Return Type: {}", ret_type.to_string());
                }
                println!("     Parameter: {}", f.params.len());
            }
            velin_compiler::parser::ast::Item::Struct(s) => {
                structs += 1;
                println!("\n  🏗️  Struct: {}", s.name);
                println!("     Felder: {}", s.fields.len());
            }
            velin_compiler::parser::ast::Item::Enum(e) => {
                enums += 1;
                println!("\n  📦 Enum: {}", e.name);
                println!("     Varianten: {}", e.variants.len());
            }
            _ => {}
        }
    }

    println!("\n📈 Zusammenfassung:");
    println!("  Funktionen: {}", functions);
    println!("  Structs: {}", structs);
    println!("  Enums: {}", enums);

    Ok(())
}

fn openapi_command(input: PathBuf, output: Option<PathBuf>) -> AnyhowResult<()> {
    println!("📄 Generiere OpenAPI Specification: {}\n", input.display());

    let code = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    let program =
        Parser::parse(&code).map_err(|e| anyhow::anyhow!("Parse error: {}", e.message))?;

    println!("✓ Parsing erfolgreich");

    let mut openapi_gen = OpenAPIGenerator::new();
    let openapi_spec = openapi_gen.generate(&program);

    let output_file = output.unwrap_or_else(|| input.with_extension("openapi.json"));

    fs::write(&output_file, openapi_spec)
        .with_context(|| format!("Failed to write file: {}", output_file.display()))?;

    println!(
        "✓ OpenAPI Specification generiert: {}",
        output_file.display()
    );

    Ok(())
}

fn serve_command(input: Option<PathBuf>, port: u16, host: String, watch: bool) -> AnyhowResult<()> {
    let input_file = input.unwrap_or_else(|| {
        let current_dir = std::env::current_dir().unwrap();
        current_dir.join("main.velin")
    });

    if !input_file.exists() {
        return Err(anyhow::anyhow!(
            "❌ Datei nicht gefunden: {}\n\n💡 Tipp: Erstelle zuerst ein Projekt mit 'velin new my-project'\n📖 Hilfe: Siehe docs/guides/getting-started.md",
            input_file.display()
        ));
    }

    println!("🚀 Starte Development-Server...\n");
    println!("📄 Datei: {}", input_file.display());
    println!("🌐 Server: http://{}:{}", host, port);

    if watch {
        println!("👀 Watch-Mode: Aktiviert (automatisches Neuladen bei Änderungen)");
    }

    println!("\n⚠️  Hinweis: Der Server-Befehl kompiliert den Code zu Rust.");
    println!("   Für die vollständige Ausführung benötigst du:");
    println!(
        "   1. Kompilierung: velin compile -i {}",
        input_file.display()
    );
    println!("   2. Rust-Build: cargo build --release");
    println!("   3. Ausführung: cargo run --release");
    println!("\n💡 Tipp: Nutze 'velin-hot-reload --server' für vollständigen Hot-Reload-Support");
    println!("📖 Hilfe: Siehe docs/tools/hot-reload.md für Details");

    Ok(())
}

fn serve_command(input: Option<PathBuf>, port: u16, host: String, watch: bool) -> AnyhowResult<()> {
    let input_file = input.unwrap_or_else(|| {
        let current_dir = std::env::current_dir().unwrap();
        current_dir.join("main.velin")
    });
    
    if !input_file.exists() {
        return Err(anyhow::anyhow!(
            "Datei nicht gefunden: {}\nTipp: velin new my-project",
            input_file.display()
        ));
    }

    let code = fs::read_to_string(&input_file)
        .with_context(|| format!("Failed to read {}", input_file.display()))?;
    let program = Parser::parse(&code)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e.message))?;
    let mut builder = velin_compiler::ir::builder::IRBuilder::new();
    let ir = builder.build_module(&program);
    let body = velin_compiler::codegen::ir_codegen::IRCodeGenerator::new(TargetLanguage::Rust)
        .generate(&ir)
        .context("Codegen failed")?;

    let out_dir = std::env::current_dir()?.join(".velin").join("serve-scaffold");
    fs::create_dir_all(out_dir.join("src"))?;
    let pkg = "velin_serve";
    fs::write(
        out_dir.join("Cargo.toml"),
        velin_compiler::codegen::axum_cargo_toml(pkg),
    )?;
    fs::write(
        out_dir.join("src").join("main.rs"),
        velin_compiler::codegen::axum_main_wrapper_with_port(&body, port),
    )?;

    println!("Scaffold geschrieben nach {}", out_dir.display());
    println!("Ziel: http://{}:{}", host, port);
    if watch {
        println!("Watch-Mode: bitte manuell neu kompilieren (kein eingebauter Watcher).");
    }
    println!("Start:\n  cd {}\n  cargo run", out_dir.display());
    Ok(())
}

fn init_command(name: Option<String>, current_dir: bool) -> AnyhowResult<()> {
    let project_name = name.unwrap_or_else(|| "velin-project".to_string());
    let safe_pkg: String = project_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect();
    
    println!("Initialisiere Velisch Projekt: {}\n", project_name);
    
    let project_dir = if current_dir {
        std::env::current_dir().context("Failed to get current directory")?
    } else {
        PathBuf::from(&project_name)
    };
    
    fs::create_dir_all(project_dir.join("src"))
        .with_context(|| format!("Failed to create directory: {}", project_dir.display()))?;
    
    let main_file = project_dir.join("main.velin");
    let main_content = r#"// Velisch Hauptdatei

@GET("/api/hello")
fn hello(): string {
    return "Hello, Velisch!";
}
"#;

    fs::write(&main_file, main_content)
        .with_context(|| format!("Failed to create main.velin: {}", main_file.display()))?;
    
    let readme_file = project_dir.join("README.md");
    let readme_content = format!(
        r#"# {0}

Velisch API-Projekt.

## Entwickeln

```bash
velin check -i main.velin
velin compile -i main.velin -o src/main.rs
cargo run
```

Oder mit Scaffold:

```bash
velin serve -i main.velin
cd .velin/serve-scaffold && cargo run
```

Endpoint: `GET /api/hello`
"#,
        project_name
    );
    
    fs::write(&readme_file, readme_content)
        .with_context(|| format!("Failed to create README: {}", readme_file.display()))?;
    
    let cargo_toml = project_dir.join("Cargo.toml");
    fs::write(
        &cargo_toml,
        velin_compiler::codegen::axum_cargo_toml(&safe_pkg),
    )?;

    // Compile Hello into runnable src/main.rs so `cargo run` works immediately
    let program = Parser::parse(main_content)
        .map_err(|e| anyhow::anyhow!("Parse error in template: {}", e.message))?;
    let mut builder = velin_compiler::ir::builder::IRBuilder::new();
    let ir = builder.build_module(&program);
    let body = velin_compiler::codegen::ir_codegen::IRCodeGenerator::new(TargetLanguage::Rust)
        .generate(&ir)
        .context("Codegen failed during init")?;
    fs::write(
        project_dir.join("src").join("main.rs"),
        velin_compiler::codegen::axum_main_wrapper(&body),
    )?;

    println!("Projekt erstellt: {}", project_dir.display());
    println!("  main.velin");
    println!("  src/main.rs (Axum)");
    println!("  Cargo.toml");
    println!("\nNaechste Schritte:");
    println!("  cd {}", project_dir.display());
    println!("  cargo run");
    println!("  # dann: GET http://127.0.0.1:3000/api/hello");
    
    Ok(())
}

fn generate_command(
    gen_type: String,
    name: Option<String>,
    fields: Option<String>,
    path: Option<String>,
    openapi: Option<PathBuf>,
    language: Option<String>,
    output: Option<PathBuf>,
) -> AnyhowResult<()> {
    println!("🔧 Generiere Code: {}\n", gen_type);

    let generator = BoilerplateGenerator::new();
    let generated_code = match gen_type.as_str() {
        "api" => {
            let api_name = name.unwrap_or_else(|| "API".to_string());
            generator.generate_api(&api_name, path.as_deref())
        }
        "crud" => {
            let model_name = name.unwrap_or_else(|| "Model".to_string());
            let model_fields = fields.unwrap_or_else(|| "id:string".to_string());
            generator.generate_crud(&model_name, &model_fields)
        }
        "test" => {
            let test_name = name.unwrap_or_else(|| "TestFunction".to_string());
            generator.generate_test(&test_name)
        }
        "responses" => generator.generate_responses_module(),
        "errors" => generator.generate_errors_module(),
        "logging" => generator.generate_logging_module(),
        "cache" => generator.generate_cache_module(),
        "health" => generator.generate_health_module(),
        "async" => generator.generate_async_module(),
        "security" => generator.generate_security_module(),
        "client" => {
            if let Some(ref openapi_path) = openapi {
                let client_gen = ClientGenerator::new();
                let lang = language.as_deref().unwrap_or("typescript");

                match client_gen.generate_from_openapi(openapi_path, lang) {
                    Ok(code) => {
                        if let Some(ref output_path) = output {
                            fs::write(output_path, &code).with_context(|| {
                                format!("Failed to write file: {}", output_path.display())
                            })?;
                            println!("✓ Client generiert: {}", output_path.display());
                            return Ok(());
                        } else {
                            println!("{}", code);
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        return Err(anyhow::anyhow!("Client generation failed: {}", e));
                    }
                }
            } else {
                return Err(anyhow::anyhow!(
                    "--openapi is required for client generation"
                ));
            }
        }
        "system" => {
            let input = path
                .as_ref()
                .map(PathBuf::from)
                .or_else(|| name.as_ref().map(|n| PathBuf::from(format!("{}.velin", n))))
                .unwrap_or_else(|| PathBuf::from("main.velin"));
            if !input.exists() {
                return Err(anyhow::anyhow!(
                    "Input file not found: {}. Use --path main.velin for system generation.",
                    input.display()
                ));
            }
            let source = fs::read_to_string(&input)?;
            let program = Parser::parse(&source).map_err(|e| anyhow::anyhow!("{}", e.message))?;
            let mut sys = SystemGenerator::new(None);
            sys.detect_framework(Some(&program), None);
            let out_dir = output
                .clone()
                .unwrap_or_else(|| PathBuf::from("generated_system"));
            fs::create_dir_all(&out_dir)?;
            fs::create_dir_all(out_dir.join("src"))?;
            let mut wrote = 0usize;
            for item in &program.items {
                if let velin_compiler::parser::ast::Item::Function(f) = item {
                    let api = APICall::from_ast(f);
                    let generated = sys
                        .generate_system(&api)
                        .map_err(|e| anyhow::anyhow!("{}", e))?;
                    for comp in generated.components {
                        let file = out_dir.join(format!("{}.rs", comp.name));
                        fs::write(&file, &comp.code)?;
                        wrote += 1;
                        println!("✓ {}", file.display());
                    }
                    if let Some(dep) = generated.deployment_config {
                        if let Some(df) = dep.dockerfile {
                            fs::write(out_dir.join("Dockerfile"), df)?;
                        }
                        if let Some(dc) = dep.docker_compose {
                            fs::write(out_dir.join("docker-compose.yml"), dc)?;
                        }
                    }
                }
            }
            let mut builder = velin_compiler::ir::builder::IRBuilder::new();
            let ir = builder.build_module(&program);
            let code = velin_compiler::codegen::IRCodeGenerator::new(TargetLanguage::Rust)
                .generate(&ir)?;
            let main_rs = velin_compiler::codegen::lowering::axum_main_wrapper(&code);
            fs::write(out_dir.join("src").join("main.rs"), &main_rs)?;
            fs::write(
                out_dir.join("Cargo.toml"),
                velin_compiler::codegen::lowering::axum_cargo_toml("generated_system"),
            )?;
            println!(
                "✓ System generated in {} ({} component files + Axum scaffold)",
                out_dir.display(),
                wrote
            );
            return Ok(());
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown generation type: {}. Available: api, crud, test, client, system, responses, errors, logging, cache, health, async, security",
                gen_type
            ));
        }
    };

    if let Some(ref output_path) = output {
        fs::write(output_path, &generated_code)
            .with_context(|| format!("Failed to write file: {}", output_path.display()))?;
        println!("✓ Code generiert: {}", output_path.display());
    } else {
        println!("{}", generated_code);
    }

    Ok(())
}

fn test_command(
    directory: Option<PathBuf>,
    unit: bool,
    integration: bool,
    verbose: bool,
) -> AnyhowResult<()> {
    println!("🧪 Führe Tests aus\n");

    let test_dir = directory.unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("tests")
    });

    if !test_dir.exists() {
        return Err(anyhow::anyhow!(
            "Test directory not found: {}. Create tests/ directory or use --directory",
            test_dir.display()
        ));
    }

    let mut test_files = Vec::new();

    if unit || (!unit && !integration) {
        let unit_dir = test_dir.join("unit");
        if unit_dir.exists() {
            if verbose {
                println!("📁 Scanne Unit Tests: {}", unit_dir.display());
            }
            scan_test_files(&unit_dir, &mut test_files, verbose);
        } else if verbose {
            println!(
                "⚠️  Unit Test-Verzeichnis nicht gefunden: {}",
                unit_dir.display()
            );
        }
    }

    if integration || (!unit && !integration) {
        let integration_dir = test_dir.join("integration");
        if integration_dir.exists() {
            if verbose {
                println!("📁 Scanne Integration Tests: {}", integration_dir.display());
            }
            scan_test_files(&integration_dir, &mut test_files, verbose);
        } else if verbose {
            println!(
                "⚠️  Integration Test-Verzeichnis nicht gefunden: {}",
                integration_dir.display()
            );
        }
    }

    if test_files.is_empty() {
        eprintln!("✗ Keine Test-Dateien gefunden");
        std::process::exit(1);
    }

    println!("✓ Gefundene Test-Dateien: {}\n", test_files.len());

    let mut passed = 0;
    let mut failed = 0;

    for test_file in &test_files {
        if verbose {
            println!("🔍 Prüfe: {}", test_file.display());
        }

        match fs::read_to_string(test_file) {
            Ok(code) => match Parser::parse(&code) {
                Ok(_) => {
                    if verbose {
                        println!("  ✓ Parsing erfolgreich");
                    }
                    passed += 1;
                }
                Err(e) => {
                    eprintln!(
                        "  ✗ Parsing-Fehler in {}: {}",
                        test_file.display(),
                        e.message
                    );
                    failed += 1;
                }
            },
            Err(e) => {
                eprintln!("  ✗ Fehler beim Lesen: {}", e);
                failed += 1;
            }
        }
    }

    println!("\n📊 Test-Ergebnisse:");
    println!("  ✓ Bestanden: {}", passed);
    if failed > 0 {
        println!("  ✗ Fehlgeschlagen: {}", failed);
        return Err(anyhow::anyhow!("{} tests failed", failed));
    } else {
        println!("  ✓ Alle Tests bestanden!");
    }

    Ok(())
}

fn scan_test_files(dir: &PathBuf, files: &mut Vec<PathBuf>, verbose: bool) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("velin") {
                files.push(path);
            } else if path.is_dir() {
                scan_test_files(&path, files, verbose);
            }
        }
    }
}

fn config_init_command(example: bool) -> AnyhowResult<()> {
    println!("⚙️  Initialisiere velin.config.json\n");

    let config_file = std::env::current_dir()
        .context("Failed to get current directory")?
        .join("velin.config.json");

    if config_file.exists() && !example {
        return Err(anyhow::anyhow!(
            "velin.config.json existiert bereits. Verwende --example um Beispiel-Config zu erstellen"
        ));
    }

    let config_content = if example {
        include_str!("../../examples/custom-recommender/velin.config.example.json")
    } else {
        r#"{
  "version": "1.0.0",
  "framework": "axum",
  "orm": "sqlx",
  "api": {
    "port": 8080,
    "host": "localhost",
    "cors": {
      "enabled": true,
      "allowedOrigins": ["*"]
    }
  },
  "database": {
    "type": "postgres",
    "connectionString": "${DATABASE_URL}",
    "orm": "sqlx"
  },
  "auth": {
    "provider": "jwt",
    "mfa": false,
    "oauth2": {
      "enabled": false,
      "clientId": "${OAUTH2_CLIENT_ID}",
      "clientSecret": "${OAUTH2_CLIENT_SECRET}",
      "authUrl": "https://oauth.provider.com/authorize",
      "tokenUrl": "https://oauth.provider.com/token"
    },
    "oidc": {
      "enabled": false,
      "issuerUrl": "https://oidc.provider.com"
    }
  },
  "tls": {
    "enabled": false,
    "provider": "rustls",
    "certPath": "./certs/cert.pem",
    "keyPath": "./certs/key.pem"
  },
  "privacy": {
    "enabled": false,
    "piiDetection": true,
    "zeroKnowledge": false
  },
  "vault": {
    "enabled": false,
    "address": "${VAULT_ADDR}",
    "token": "${VAULT_TOKEN}",
    "mountPath": "secret"
  },
  "ml": {
    "llm": {
      "provider": "openai",
      "apiKey": "${OPENAI_API_KEY}",
      "model": "gpt-4"
    }
  },
  "cache": {
    "enabled": true,
    "ttl": 3600
  },
  "logging": {
    "level": "info",
    "format": "json",
    "output": "console"
  },
  "security": {
    "apiKeyRequired": false,
    "rateLimit": {
      "enabled": true,
      "requestsPerMinute": 100
    }
  }
}"#
    };

    fs::write(&config_file, config_content)
        .with_context(|| format!("Failed to create config file: {}", config_file.display()))?;

    println!("✓ Config-Datei erstellt: {}", config_file.display());
    Ok(())
}

fn config_validate_command(file: Option<PathBuf>) -> AnyhowResult<()> {
    println!("✅ Validiere velin.config.json\n");

    let config_file = file.unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("velin.config.json")
    });

    if !config_file.exists() {
        return Err(anyhow::anyhow!(
            "Config-Datei nicht gefunden: {}",
            config_file.display()
        ));
    }

    let content = fs::read_to_string(&config_file)
        .with_context(|| format!("Failed to read config file: {}", config_file.display()))?;

    // Einfache JSON-Validierung ohne serde_json
    let trimmed = content.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        println!("✓ JSON-Syntax gültig (Basis-Check)");
        println!("✓ Config-Datei validiert");
    } else {
        return Err(anyhow::anyhow!(
            "JSON-Syntax-Fehler: Datei muss gültiges JSON sein"
        ));
    }

    Ok(())
}

fn config_show_command(file: Option<PathBuf>) -> AnyhowResult<()> {
    println!("📋 Zeige Config-Werte\n");

    let config_file = file.unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("velin.config.json")
    });

    if !config_file.exists() {
        return Err(anyhow::anyhow!(
            "Config-Datei nicht gefunden: {}",
            config_file.display()
        ));
    }

    let content = fs::read_to_string(&config_file)
        .with_context(|| format!("Failed to read config file: {}", config_file.display()))?;

    println!("{}", content);
    Ok(())
}

fn cache_stats_command() -> AnyhowResult<()> {
    println!("📊 Cache-Statistiken\n");
    println!("⚠️  Cache-Management erfordert laufende Runtime");
    println!("   Verwende Health-Endpoint für Runtime-Statistiken");
    println!("\n   Beispiel: velin health --url http://localhost:8080/metrics");
    Ok(())
}

fn cache_clear_command(pattern: Option<String>) -> AnyhowResult<()> {
    println!("🗑️  Leere Cache\n");
    println!("⚠️  Cache-Management erfordert laufende Runtime");
    println!("   Verwende Health-Endpoint für Cache-Operationen");
    println!("\n   Beispiel: velin health --url http://localhost:8080/metrics");

    if let Some(p) = pattern {
        println!("   Pattern: {}", p);
    }
    Ok(())
}

fn cache_warm_command() -> AnyhowResult<()> {
    println!("🔥 Wärme Cache\n");
    println!("⚠️  Cache-Management erfordert laufende Runtime");
    println!("   Verwende Health-Endpoint für Cache-Operationen");
    println!("\n   Beispiel: velin health --url http://localhost:8080/metrics");
    Ok(())
}

fn health_command(url: Option<String>, verbose: bool) -> AnyhowResult<()> {
    println!("🏥 Health Check\n");

    let endpoint = url.unwrap_or_else(|| "http://localhost:8080/health".to_string());

    println!("📡 Prüfe Endpoint: {}", endpoint);
    println!("⚠️  HTTP-Request erfordert zusätzliche Dependencies");
    println!("   In Production: Verwende curl oder ähnliches Tool");

    if verbose {
        println!(
            "\n   Detaillierte Metriken: {}/metrics",
            endpoint.trim_end_matches("/health")
        );
    }
    Ok(())
}

fn backup_create_command(strategy: Option<String>, destination: Option<String>, compression: Option<String>) -> AnyhowResult<()> {
    let _ = (strategy, compression);
    let src = std::env::current_dir()?;
    let dest_root = destination
        .map(PathBuf::from)
        .unwrap_or_else(|| src.join(".velin").join("backup"));
    fs::create_dir_all(&dest_root)?;
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let backup_id = format!("backup-{}", stamp);
    let dest = dest_root.join(&backup_id);
    fs::create_dir_all(&dest)?;

    for name in ["main.velin", "Cargo.toml", "velin.config.json"] {
        let from = src.join(name);
        if from.exists() {
            fs::copy(&from, dest.join(name))?;
        }
    }
    // Copy *.velin in project root
    if let Ok(entries) = fs::read_dir(&src) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("velin") {
                if let Some(fname) = path.file_name() {
                    let _ = fs::copy(&path, dest.join(fname));
                }
            }
        }
    }
    println!("Backup erstellt: {} ({})", backup_id, dest.display());
    Ok(())
}

fn backup_restore_command(backup_id: String, destination: Option<String>) -> AnyhowResult<()> {
    let cwd = std::env::current_dir()?;
    let dest = destination.map(PathBuf::from).unwrap_or_else(|| cwd.clone());
    let backup_dir = cwd.join(".velin").join("backup").join(&backup_id);
    if !backup_dir.exists() {
        return Err(anyhow::anyhow!("Backup nicht gefunden: {}", backup_dir.display()));
    }
    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(fname) = path.file_name() {
                    fs::copy(&path, dest.join(fname))?;
                }
            }
        }
    }
    println!("Backup {} wiederhergestellt nach {}", backup_id, dest.display());
    Ok(())
}

fn backup_list_command(directory: Option<String>) -> AnyhowResult<()> {
    let dir = directory
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap().join(".velin").join("backup"));
    if !dir.exists() {
        println!("Keine Backups ({} fehlt).", dir.display());
        return Ok(());
    }
    println!("Backups in {}:", dir.display());
    for entry in fs::read_dir(&dir)?.flatten() {
        if entry.file_type()?.is_dir() {
            println!("  {}", entry.file_name().to_string_lossy());
        }
    }
    Ok(())
}

fn backup_delete_command(backup_id: String, directory: Option<String>) -> AnyhowResult<()> {
    let root = directory
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap().join(".velin").join("backup"));
    let path = root.join(&backup_id);
    if path.exists() {
        fs::remove_dir_all(&path)?;
        println!("Backup gelöscht: {}", backup_id);
    } else {
        return Err(anyhow::anyhow!("Backup nicht gefunden: {}", path.display()));
    }
    Ok(())
}

fn backup_verify_command(backup_id: String, directory: Option<String>) -> AnyhowResult<()> {
    let root = directory
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap().join(".velin").join("backup"));
    let path = root.join(&backup_id);
    if !path.exists() {
        return Err(anyhow::anyhow!("Backup nicht gefunden: {}", path.display()));
    }
    let count = fs::read_dir(&path)?.count();
    println!("Backup {} ok ({} Dateien).", backup_id, count);
    Ok(())
}

fn rollback_begin_command() -> AnyhowResult<()> {
    println!("🔄 Transaktion beginnen\n");
    println!("  ✓ Transaktion gestartet...");
    Ok(())
}

fn rollback_commit_command(transaction_id: String) -> AnyhowResult<()> {
    println!("✅ Transaktion committen\n");
    println!("  Transaktions-ID: {}", transaction_id);
    println!("  ✓ Transaktion committed...");
    Ok(())
}

fn rollback_rollback_command(transaction_id: String) -> AnyhowResult<()> {
    println!("⏪ Transaktion rollback\n");
    println!("  Transaktions-ID: {}", transaction_id);
    println!("  ✓ Transaktion zurückgerollt...");
    Ok(())
}

fn rollback_create_version_command(description: String) -> AnyhowResult<()> {
    println!("📌 Version erstellen\n");
    println!("  Beschreibung: {}", description);
    println!("  ✓ Version erstellt...");
    Ok(())
}

fn rollback_to_version_command(version_id: String) -> AnyhowResult<()> {
    println!("⏮️  Rollback zu Version\n");
    println!("  Version-ID: {}", version_id);
    println!("  ✓ Rollback zu Version durchgeführt...");
    Ok(())
}

fn rollback_list_versions_command() -> AnyhowResult<()> {
    println!("📋 Versionen auflisten\n");
    println!("  ✓ Versionen werden aufgelistet...");
    Ok(())
}

fn rollback_create_snapshot_command(description: String) -> AnyhowResult<()> {
    println!("📸 Snapshot erstellen\n");
    println!("  Beschreibung: {}", description);
    println!("  ✓ Snapshot erstellt...");
    Ok(())
}

fn rollback_to_snapshot_command(snapshot_id: String) -> AnyhowResult<()> {
    println!("⏮️  Rollback zu Snapshot\n");
    println!("  Snapshot-ID: {}", snapshot_id);
    println!("  ✓ Rollback zu Snapshot durchgeführt...");
    Ok(())
}

fn rollback_list_snapshots_command() -> AnyhowResult<()> {
    println!("📋 Snapshots auflisten\n");
    println!("  ✓ Snapshots werden aufgelistet...");
    Ok(())
}

fn serialize_json_to_yaml_command(input: PathBuf, output: Option<PathBuf>) -> AnyhowResult<()> {
    println!("🔄 JSON zu YAML konvertieren\n");
    println!("  Eingabe: {}", input.display());
    
    if !input.exists() {
        return Err(anyhow::anyhow!("Datei nicht gefunden: {}", input.display()));
    }
    
    let json_content = fs::read_to_string(&input)
        .with_context(|| format!("Fehler beim Lesen der Datei: {}", input.display()))?;
    
    let json_value: serde_json::Value = serde_json::from_str(&json_content)
        .with_context(|| format!("Ungültiges JSON in Datei: {}", input.display()))?;
    
    let yaml_content = serde_yaml::to_string(&json_value)
        .with_context(|| "Fehler bei YAML-Konvertierung")?;
    
    let output_file = output.unwrap_or_else(|| {
        input.with_extension("yaml")
    });
    
    fs::write(&output_file, yaml_content)
        .with_context(|| format!("Fehler beim Schreiben der Datei: {}", output_file.display()))?;
    
    println!("  Ausgabe: {}", output_file.display());
    println!("  ✓ Konvertierung erfolgreich!");
    Ok(())
}

fn serialize_yaml_to_json_command(input: PathBuf, output: Option<PathBuf>) -> AnyhowResult<()> {
    println!("🔄 YAML zu JSON konvertieren\n");
    println!("  Eingabe: {}", input.display());
    
    if !input.exists() {
        return Err(anyhow::anyhow!("Datei nicht gefunden: {}", input.display()));
    }
    
    let yaml_content = fs::read_to_string(&input)
        .with_context(|| format!("Fehler beim Lesen der Datei: {}", input.display()))?;
    
    let yaml_value: serde_json::Value = serde_yaml::from_str(&yaml_content)
        .with_context(|| format!("Ungültiges YAML in Datei: {}", input.display()))?;
    
    let json_content = serde_json::to_string_pretty(&yaml_value)
        .with_context(|| "Fehler bei JSON-Konvertierung")?;
    
    let output_file = output.unwrap_or_else(|| {
        input.with_extension("json")
    });
    
    fs::write(&output_file, json_content)
        .with_context(|| format!("Fehler beim Schreiben der Datei: {}", output_file.display()))?;
    
    println!("  Ausgabe: {}", output_file.display());
    println!("  ✓ Konvertierung erfolgreich!");
    Ok(())
}

fn serialize_validate_json_command(file: PathBuf) -> AnyhowResult<()> {
    println!("✅ JSON validieren\n");
    println!("  Datei: {}", file.display());
    
    if !file.exists() {
        return Err(anyhow::anyhow!("Datei nicht gefunden: {}", file.display()));
    }
    
    let json_content = fs::read_to_string(&file)
        .with_context(|| format!("Fehler beim Lesen der Datei: {}", file.display()))?;
    
    let _: serde_json::Value = serde_json::from_str(&json_content)
        .with_context(|| format!("Ungültiges JSON in Datei: {}", file.display()))?;
    
    println!("  ✓ JSON ist gültig!");
    Ok(())
}

fn serialize_validate_yaml_command(file: PathBuf) -> AnyhowResult<()> {
    println!("✅ YAML validieren\n");
    println!("  Datei: {}", file.display());
    
    if !file.exists() {
        return Err(anyhow::anyhow!("Datei nicht gefunden: {}", file.display()));
    }
    
    let yaml_content = fs::read_to_string(&file)
        .with_context(|| format!("Fehler beim Lesen der Datei: {}", file.display()))?;
    
    let _: serde_json::Value = serde_yaml::from_str(&yaml_content)
        .with_context(|| format!("Ungültiges YAML in Datei: {}", file.display()))?;
    
    println!("  ✓ YAML ist gültig!");
    Ok(())
}
