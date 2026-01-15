// VelinScript API Documentation Generator
// Generiert OpenAPI/Swagger Dokumentation aus VelinScript Code

mod openapi;
mod jsdoc;
mod html;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::fs;
use velin_compiler::parser::parser::Parser as VelinParser;

#[derive(Parser)]
#[command(name = "velin-api-doc")]
#[command(about = "VelinScript API Documentation Generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generiert OpenAPI Dokumentation
    Generate {
        /// Input VelinScript Datei
        #[arg(short, long)]
        input: String,
        /// Output Datei
        #[arg(short, long, default_value = "openapi.json")]
        output: String,
            /// Output Format (json, yaml, markdown, html)
            #[arg(short, long, default_value = "json")]
            format: String,
            
            /// Generiere interaktive HTML-Dokumentation (Swagger UI)
            #[arg(long)]
            interactive: bool,
        /// API Titel
        #[arg(long, default_value = "VelinScript API")]
        title: String,
        /// API Version
        #[arg(long, default_value = "1.0.0")]
        version: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            input,
            output,
            format,
            title,
            version,
            interactive,
        } => {
            let content = fs::read_to_string(&input)?;
            
            // Parse JSDoc-Kommentare (für zukünftige Integration)
            let _jsdoc_docs = jsdoc::JSDocParser::parse(&content);
            
            let program = VelinParser::parse(&content)
                .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

            let spec = openapi::generate_openapi(&program, &title, &version);
            
            // Integriere JSDoc-Kommentare in OpenAPI Spec
            // (Dies würde die spec mit JSDoc-Informationen erweitern)
            // TODO: _jsdoc_docs verwenden, um OpenAPI Spec zu erweitern

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&spec)?;
                    fs::write(&output, json)?;
                    println!("✓ OpenAPI JSON generiert: {}", output);
                }
                "yaml" => {
                    let yaml = serde_yaml::to_string(&spec)?;
                    fs::write(&output, yaml)?;
                    println!("✓ OpenAPI YAML generiert: {}", output);
                }
                "markdown" => {
                    let markdown = openapi::generate_markdown(&spec);
                    fs::write(&output, markdown)?;
                    println!("✓ Markdown Dokumentation generiert: {}", output);
                }
                "html" => {
                    let html = if interactive {
                        html::HTMLGenerator::generate(&spec)
                    } else {
                        html::HTMLGenerator::generate_simple(&spec)
                    };
                    fs::write(&output, html)?;
                    println!("✓ HTML Dokumentation generiert: {}", output);
                }
                _ => {
                    anyhow::bail!("Unbekanntes Format: {}. Unterstützt: json, yaml, markdown, html", format);
                }
            }
        }
    }

    Ok(())
}
