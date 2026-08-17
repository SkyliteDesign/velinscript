// Benchmark Runner
// Führt Benchmarks aus und sammelt Ergebnisse

use crate::parser::BenchmarkParser;
use crate::stats::Statistics;
use velin_compiler::compiler::{VelinCompiler, config::CompilerConfig};
use velin_compiler::passes::{parser::ParserPass, type_check::TypeCheckPass};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub mean_time: f64,
    pub min_time: f64,
    pub max_time: f64,
    pub std_dev: f64,
    pub iterations: usize,
}

pub struct BenchmarkRunner {
    iterations: usize,
    compare: bool,
    parser: BenchmarkParser,
    stats: Statistics,
}

impl BenchmarkRunner {
    pub fn new(iterations: usize, compare: bool) -> Self {
        Self {
            iterations,
            compare,
            parser: BenchmarkParser::new(),
            stats: Statistics::new(),
        }
    }
    
    pub fn run(&self, path: &Path, verbose: bool) -> Result<Vec<BenchmarkResult>> {
        let files = self.collect_benchmark_files(path)?;
        
        if files.is_empty() {
            return Ok(Vec::new());
        }
        
        if verbose {
            println!("📁 Gefundene Benchmark-Dateien: {}", files.len());
        }
        
        let mut results = Vec::new();
        
        for file in &files {
            if verbose {
                println!("🔍 Benchmarke: {}", file.display());
            }
            
            let content = fs::read_to_string(file)?;
            let benchmarks = self.parser.parse_benchmarks(&content, file)?;
            
            for benchmark in &benchmarks {
                let result = self.run_benchmark(benchmark, verbose)?;
                results.push(result);
            }
        }
        
        Ok(results)
    }
    
    fn run_benchmark(&self, benchmark: &Benchmark, verbose: bool) -> Result<BenchmarkResult> {
        let mut times = Vec::new();
        
        // Führe Benchmark mehrfach aus
        for _ in 0..self.iterations {
            let start = Instant::now();
            
            // Kompiliere und führe Code aus
            let mut config = CompilerConfig::default();
            config.enable_type_check = true;
            
            let mut compiler = VelinCompiler::new(config);
            compiler.add_pass(Box::new(ParserPass::new()));
            compiler.add_pass(Box::new(TypeCheckPass::new(true)));
            
            let context = compiler.compile(
                benchmark.file.clone(),
                benchmark.code.clone(),
            )?;
            
            if context.has_errors() {
                return Err(anyhow::anyhow!("Kompilierungsfehler: {:?}", context.errors));
            }
            
            let elapsed = start.elapsed();
            times.push(elapsed.as_secs_f64() * 1000.0); // in ms
        }
        
        // Berechne Statistiken
        let mean = self.stats.mean(&times);
        let min = self.stats.min(&times);
        let max = self.stats.max(&times);
        let std_dev = self.stats.std_dev(&times);
        
        Ok(BenchmarkResult {
            name: benchmark.name.clone(),
            mean_time: mean,
            min_time: min,
            max_time: max,
            std_dev,
            iterations: self.iterations,
        })
    }
    
    fn collect_benchmark_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        if path.is_file() {
            if path.extension().and_then(|s| s.to_str()) == Some("velin") {
                files.push(path.to_path_buf());
            }
        } else {
            for entry in WalkDir::new(path) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("velin") {
                        let content = fs::read_to_string(entry.path())?;
                        if content.contains("@benchmark") {
                            files.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }
}

#[derive(Debug, Clone)]
pub struct Benchmark {
    pub name: String,
    pub code: String,
    pub file: String,
    pub line: usize,
}
