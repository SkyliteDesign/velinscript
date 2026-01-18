// VelinScript Profiler
// CPU- und Memory-Profiling mit Flame Graphs und Allocation Tracking

mod cpu;
mod memory;
mod flamegraph;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "velin-profile")]
#[command(about = "VelinScript Profiler - CPU- und Memory-Profiling", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// CPU-Profiling
    Cpu {
        /// Zu profilende Datei
        file: PathBuf,
        
        /// Generiert Flame Graph
        #[arg(long)]
        flamegraph: bool,
        
        /// Output-Verzeichnis
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Memory-Profiling
    Memory {
        /// Zu profilende Datei
        file: PathBuf,
        
        /// Output-Verzeichnis
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cpu { file, flamegraph, output } => {
            cpu_profiling(file, flamegraph, output)
        }
        Commands::Memory { file, output } => {
            memory_profiling(file, output)
        }
    }
}

fn cpu_profiling(file: PathBuf, flamegraph: bool, output: Option<PathBuf>) -> Result<()> {
    println!("⚡ CPU-Profiling für: {}\n", file.display());
    
    let profiler = cpu::CpuProfiler::new();
    let profile_data = profiler.profile(&file)?;
    
    println!("📊 CPU-Profiling-Ergebnisse:");
    println!("  Gesamt-Zeit: {:.2}ms", profile_data.total_time);
    println!("  Funktionen: {}", profile_data.functions.len());
    
    if flamegraph {
        println!("\n🔥 Generiere Flame Graph...");
        let flame_gen = flamegraph::FlameGraphGenerator::new();
        let output_path = output.unwrap_or_else(|| PathBuf::from("flamegraph.svg"));
        flame_gen.generate(&profile_data, &output_path)?;
        println!("✓ Flame Graph gespeichert: {}", output_path.display());
    }
    
    Ok(())
}

fn memory_profiling(file: PathBuf, output: Option<PathBuf>) -> Result<()> {
    println!("💾 Memory-Profiling für: {}\n", file.display());
    
    let profiler = memory::MemoryProfiler::new();
    let profile_data = profiler.profile(&file)?;
    
    println!("📊 Memory-Profiling-Ergebnisse:");
    println!("  Gesamt-Allokationen: {} bytes", profile_data.total_allocations);
    println!("  Peak-Memory: {} bytes", profile_data.peak_memory);
    println!("  Allokationen: {}", profile_data.allocation_count);
    
    if let Some(output_path) = output {
        let json = serde_json::to_string_pretty(&profile_data)?;
        std::fs::write(&output_path, json)?;
        println!("\n✓ Report gespeichert: {}", output_path.display());
    }
    
    Ok(())
}
