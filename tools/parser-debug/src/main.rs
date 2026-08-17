use clap::Parser;
use std::fs;
use velin_compiler::parser::lexer::Lexer;
use velin_compiler::parser::parser::Parser as VelinParser;
use velin_compiler::parser::ast::Program;

#[derive(Parser)]
#[command(name = "parser-debug")]
#[command(about = "Debug-Tool für VelinScript Parser - Visualisiert Context-Stack und Parsing-Fluss")]
struct Args {
    /// Eingabedatei zum Parsen
    #[arg(short, long)]
    file: String,
    
    /// Zeige detaillierte Token-Informationen
    #[arg(short, long)]
    tokens: bool,
    
    /// Zeige Context-Stack bei jedem Schritt
    #[arg(short, long)]
    context: bool,
    
    /// Zeige AST-Struktur
    #[arg(short, long)]
    ast: bool,
}

fn main() {
    let args = Args::parse();
    
    println!("🔍 VelinScript Parser Debug Tool");
    println!("================================\n");
    
    // Lade Datei
    let source = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Fehler beim Lesen der Datei '{}': {}", args.file, e);
            std::process::exit(1);
        }
    };
    
    println!("📁 Datei: {}\n", args.file);
    
    // Tokenisierung
    if args.tokens {
        println!("🔤 Tokenisierung:");
        println!("{}", "─".repeat(60));
        let lexer = Lexer::new(&source);
        match lexer.tokenize() {
            Ok(tokens) => {
                for (i, token) in tokens.iter().enumerate() {
                    println!("  {:3}: {:?}", i, token);
                }
                println!();
            }
            Err(e) => {
                eprintln!("❌ Lexer-Fehler: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    // Parsing mit Context-Tracking
    println!("🌳 Parsing:");
    println!("{}", "─".repeat(60));
    
    let lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("❌ Lexer-Fehler: {}", e);
            std::process::exit(1);
        }
    };
    
    let mut parser = VelinParser::new(tokens, source.clone());
    
    // Parse mit Fehlerbehandlung
    match parser.parse_program() {
        Ok(program) => {
            println!("✅ Parsing erfolgreich!\n");
            
            if args.ast {
                println!("📊 AST-Struktur:");
                println!("{}", "─".repeat(60));
                print_ast(&program, 0);
                println!();
            }
            
            // Analysiere Struct-Literale
            analyze_struct_literals(&program);
        }
        Err(e) => {
            eprintln!("❌ Parse-Fehler:");
            eprintln!("{}", "─".repeat(60));
            eprintln!("{}", e);
            eprintln!();
            
            // Zeige Kontext um Fehler
            if let Some(line) = e.line {
                show_error_context(&source, line, e.column);
            }
            
            std::process::exit(1);
        }
    }
}

fn print_ast(program: &Program, indent: usize) {
    let prefix = "  ".repeat(indent);
    println!("{}Program ({} items)", prefix, program.items.len());
    
    for (i, item) in program.items.iter().enumerate() {
        println!("{}├─ Item {}: {:?}", prefix, i, item);
    }
}

fn analyze_struct_literals(program: &Program) {
    println!("🔍 Struct-Literal-Analyse:");
    println!("{}", "─".repeat(60));
    
    // TODO: Durchsuche AST nach Struct-Literalen
    // Dies erfordert eine Traversierung des AST
    println!("  (Struct-Literal-Analyse noch nicht implementiert)");
    println!();
}

fn show_error_context(source: &str, line: usize, column: usize) {
    let lines: Vec<&str> = source.lines().collect();
    
    println!("📍 Kontext um Fehler (Zeile {}, Spalte {}):", line, column);
    println!("{}", "─".repeat(60));
    
    let start = if line > 3 { line - 3 } else { 0 };
    let end = if line + 2 < lines.len() { line + 2 } else { lines.len() };
    
    for i in start..end {
        let line_num = i + 1;
        let marker = if line_num == line { ">>>" } else { "   " };
        println!("{} {:3} │ {}", marker, line_num, lines[i]);
        
        if line_num == line && column > 0 {
            let spaces = " ".repeat(column.min(60));
            println!("      │ {}{}", spaces, "^");
        }
    }
    println!();
}
