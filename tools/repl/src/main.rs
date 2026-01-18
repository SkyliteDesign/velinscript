// VelinScript REPL
// Interaktive Shell zum Testen von VelinScript-Code in Echtzeit

mod interpreter;
mod compiler;
mod evaluator;

use clap::Parser;
use anyhow::Result;
use std::path::PathBuf;
use interpreter::Interpreter;
use compiler::ReplCompiler;
use evaluator::Evaluator;

#[derive(Parser)]
#[command(name = "velin-repl")]
#[command(about = "VelinScript REPL - Interaktive Shell", long_about = None)]
struct Cli {
    /// Lädt Datei in REPL
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    println!("VelinScript REPL");
    println!("Tippe 'exit' oder 'quit' zum Beenden\n");
    
    let interpreter = Interpreter::new();
    let compiler = ReplCompiler::new();
    let evaluator = Evaluator::new();
    
    let mut repl = Repl::new(interpreter, compiler, evaluator);
    
    if let Some(file) = cli.file {
        repl.load_file(&file)?;
    }
    
    repl.run()?;
    
    Ok(())
}

struct Repl {
    interpreter: Interpreter,
    compiler: ReplCompiler,
    evaluator: Evaluator,
    history: Vec<String>,
}

impl Repl {
    fn new(interpreter: Interpreter, compiler: ReplCompiler, evaluator: Evaluator) -> Self {
        Self {
            interpreter,
            compiler,
            evaluator,
            history: Vec::new(),
        }
    }
    
    fn load_file(&mut self, path: &PathBuf) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        println!("📁 Datei geladen: {}\n", path.display());
        
        // Führe Datei-Inhalt aus
        self.evaluate(&content)?;
        
        Ok(())
    }
    
    fn run(&mut self) -> Result<()> {
        use rustyline::DefaultEditor;
        
        let mut rl = DefaultEditor::new()?;
        
        loop {
            let readline = rl.readline("velin> ");
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    
                    if line.is_empty() {
                        continue;
                    }
                    
                    if line == "exit" || line == "quit" {
                        println!("Auf Wiedersehen!");
                        break;
                    }
                    
                    if line == "clear" {
                        print!("\x1B[2J\x1B[1;1H");
                        continue;
                    }
                    
                    if line.starts_with(":") {
                        self.handle_command(line)?;
                        continue;
                    }
                    
                    self.history.push(line.to_string());
                    
                    match self.evaluate(line) {
                        Ok(result) => {
                            if !result.is_empty() {
                                println!("{}", result);
                            }
                        }
                        Err(e) => {
                            eprintln!("Fehler: {}", e);
                        }
                    }
                }
                Err(rustyline::error::ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(rustyline::error::ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(e) => {
                    eprintln!("Fehler: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn evaluate(&mut self, code: &str) -> Result<String> {
        // Versuche zuerst einfache Expression-Interpretation
        if let Ok(result) = self.interpreter.evaluate_expression(code) {
            return Ok(result);
        }
        
        // Falls das fehlschlägt, kompiliere und führe aus
        if let Ok(result) = self.compiler.compile_and_run(code) {
            return Ok(result);
        }
        
        // Falls beides fehlschlägt, versuche Evaluator
        self.evaluator.evaluate(code)
    }
    
    fn handle_command(&self, cmd: &str) -> Result<()> {
        match cmd {
            ":help" | ":h" => {
                println!("Verfügbare Befehle:");
                println!("  :help, :h     - Zeigt diese Hilfe");
                println!("  :history       - Zeigt Command-History");
                println!("  :clear         - Löscht Bildschirm");
                println!("  exit, quit     - Beendet REPL");
            }
            ":history" => {
                for (i, cmd) in self.history.iter().enumerate() {
                    println!("  {}: {}", i + 1, cmd);
                }
            }
            _ => {
                println!("Unbekannter Befehl: {}. Tippe :help für Hilfe.", cmd);
            }
        }
        Ok(())
    }
}
