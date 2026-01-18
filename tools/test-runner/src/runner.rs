// Test Runner
// Führt Tests aus und sammelt Ergebnisse

use crate::parser::TestParser;
use crate::coverage::CoverageCollector;
use crate::mocking::MockManager;
use crate::assertions::AssertionRunner;
use velin_compiler::compiler::{VelinCompiler, config::CompilerConfig};
use velin_compiler::passes::{parser::ParserPass, type_check::TypeCheckPass};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub failures: Vec<TestFailure>,
    pub coverage: Option<CoverageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFailure {
    pub test_name: String,
    pub message: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageData {
    pub line_coverage: f64,
    pub function_coverage: f64,
    pub covered_lines: usize,
    pub total_lines: usize,
    pub covered_functions: usize,
    pub total_functions: usize,
}

pub struct TestRunner {
    coverage_enabled: bool,
    mocking_enabled: bool,
    parser: TestParser,
    coverage_collector: Option<CoverageCollector>,
    mock_manager: Option<MockManager>,
}

impl TestRunner {
    pub fn new(coverage_enabled: bool, mocking_enabled: bool) -> Self {
        Self {
            coverage_enabled,
            mocking_enabled,
            parser: TestParser::new(),
            coverage_collector: if coverage_enabled {
                Some(CoverageCollector::new())
            } else {
                None
            },
            mock_manager: if mocking_enabled {
                Some(MockManager::new())
            } else {
                None
            },
        }
    }
    
    pub async fn run(
        &self,
        path: &Path,
        unit_only: bool,
        integration_only: bool,
        verbose: bool,
    ) -> Result<TestResults> {
        let files = self.collect_test_files(path, unit_only, integration_only)?;
        
        if files.is_empty() {
            return Ok(TestResults {
                passed: 0,
                failed: 0,
                skipped: 0,
                failures: Vec::new(),
                coverage: None,
            });
        }
        
        if verbose {
            println!("📁 Gefundene Test-Dateien: {}", files.len());
        }
        
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;
        let mut failures = Vec::new();
        
        // Führe auch Rust-Tests aus, falls vorhanden
        let rust_test_result = self.run_rust_tests(verbose).await;
        
        // Führe VelinScript-Tests aus
        for file in &files {
            if verbose {
                println!("🔍 Teste: {}", file.display());
            }
            
            match self.run_velin_test(file, verbose).await {
                Ok(test_result) => {
                    if test_result.passed {
                        passed += 1;
                    } else {
                        failed += 1;
                        failures.push(TestFailure {
                            test_name: file.to_string_lossy().to_string(),
                            message: test_result.message,
                            file: file.to_string_lossy().to_string(),
                            line: test_result.line,
                        });
                    }
                }
                Err(e) => {
                    failed += 1;
                    failures.push(TestFailure {
                        test_name: file.to_string_lossy().to_string(),
                        message: format!("Fehler: {}", e),
                        file: file.to_string_lossy().to_string(),
                        line: 0,
                    });
                }
            }
        }
        
        // Coverage-Report
        let coverage = if self.coverage_enabled {
            self.coverage_collector.as_ref()
                .map(|c| c.generate_report())
                .transpose()?
        } else {
            None
        };
        
        Ok(TestResults {
            passed,
            failed,
            skipped,
            failures,
            coverage,
        })
    }
    
    async fn run_rust_tests(&self, verbose: bool) -> Result<()> {
        if verbose {
            println!("🔧 Führe Rust-Tests aus...");
        }
        
        let output = Command::new("cargo")
            .args(&["test", "--quiet"])
            .output()?;
        
        if !output.status.success() {
            if verbose {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        
        Ok(())
    }
    
    async fn run_velin_test(
        &self,
        file: &Path,
        verbose: bool,
    ) -> Result<TestResult> {
        let content = fs::read_to_string(file)?;
        let tests = self.parser.parse_tests(&content, file)?;
        
        if tests.is_empty() {
            return Ok(TestResult {
                passed: true,
                message: "Keine Tests gefunden".to_string(),
                line: 0,
            });
        }
        
        // Kompiliere und führe Tests aus
        let mut config = CompilerConfig::default();
        config.enable_type_check = true;
        
        let mut compiler = VelinCompiler::new(config);
        compiler.add_pass(Box::new(ParserPass::new()));
        compiler.add_pass(Box::new(TypeCheckPass::new(true)));
        
        let context = compiler.compile(
            file.to_string_lossy().to_string(),
            content.clone(),
        )?;
        
        if context.has_errors() {
            return Ok(TestResult {
                passed: false,
                message: format!("Kompilierungsfehler: {:?}", context.errors),
                line: 0,
            });
        }
        
        // Führe Assertions aus
        let assertion_runner = AssertionRunner::new();
        for test in &tests {
            match assertion_runner.run_assertions(test, &content).await {
                Ok(true) => {
                    if verbose {
                        println!("  ✓ {}", test.name);
                    }
                }
                Ok(false) => {
                    return Ok(TestResult {
                        passed: false,
                        message: format!("Test '{}' fehlgeschlagen", test.name),
                        line: test.line,
                    });
                }
                Err(e) => {
                    return Ok(TestResult {
                        passed: false,
                        message: format!("Fehler in Test '{}': {}", test.name, e),
                        line: test.line,
                    });
                }
            }
        }
        
        Ok(TestResult {
            passed: true,
            message: "Alle Tests bestanden".to_string(),
            line: 0,
        })
    }
    
    fn collect_test_files(
        &self,
        path: &Path,
        unit_only: bool,
        integration_only: bool,
    ) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        if path.is_file() {
            if path.extension().and_then(|s| s.to_str()) == Some("velin") {
                files.push(path.to_path_buf());
            }
        } else {
            let test_dir = path.join("tests");
            if test_dir.exists() {
                if unit_only || (!unit_only && !integration_only) {
                    let unit_dir = test_dir.join("unit");
                    if unit_dir.exists() {
                        self.collect_files_recursive(&unit_dir, &mut files)?;
                    }
                }
                
                if integration_only || (!unit_only && !integration_only) {
                    let integration_dir = test_dir.join("integration");
                    if integration_dir.exists() {
                        self.collect_files_recursive(&integration_dir, &mut files)?;
                    }
                }
            }
            
            // Auch im Hauptverzeichnis nach Test-Dateien suchen
            for entry in WalkDir::new(path) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("velin") {
                        let content = fs::read_to_string(entry.path())?;
                        if content.contains("@test") {
                            files.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }
    
    fn collect_files_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("velin") {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
        Ok(())
    }
}

struct TestResult {
    passed: bool,
    message: String,
    line: usize,
}
