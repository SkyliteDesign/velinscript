// API Client Generator aus OpenAPI

use std::fs;
use std::path::Path;

pub struct ClientGenerator;

impl ClientGenerator {
    pub fn new() -> Self {
        ClientGenerator
    }
    
    pub fn generate_from_openapi(&self, openapi_path: &Path, language: &str) -> Result<String, String> {
        // Lese OpenAPI Datei
        let content = fs::read_to_string(openapi_path)
            .map_err(|e| format!("Fehler beim Lesen der OpenAPI Datei: {}", e))?;
        
        // Parse JSON (einfache Implementierung)
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Fehler beim Parsen der OpenAPI JSON: {}", e))?;
        
        match language {
            "typescript" | "ts" => self.generate_typescript_client(&json),
            "javascript" | "js" => self.generate_javascript_client(&json),
            "rust" => self.generate_rust_client(&json),
            _ => Err(format!("Unbekannte Sprache: {}. Unterstützt: typescript, javascript, rust", language)),
        }
    }
    
    fn generate_typescript_client(&self, json: &serde_json::Value) -> Result<String, String> {
        let mut output = String::new();
        
        output.push_str("// Auto-generated TypeScript Client\n");
        output.push_str("// Generated from OpenAPI Specification\n\n");
        
        // Basis Client-Klasse
        output.push_str("export class ApiClient {\n");
        output.push_str("    private baseUrl: string;\n\n");
        output.push_str("    constructor(baseUrl: string = 'http://localhost:3000') {\n");
        output.push_str("        this.baseUrl = baseUrl;\n");
        output.push_str("    }\n\n");
        
        // Generiere Methoden aus Paths
        if let Some(paths) = json.get("paths").and_then(|p| p.as_object()) {
            for (path, path_item) in paths {
                self.generate_path_methods_ts(&mut output, path, path_item);
            }
        }
        
        output.push_str("}\n");
        
        Ok(output)
    }
    
    fn generate_path_methods_ts(&self, output: &mut String, path: &str, path_item: &serde_json::Value) {
        if let Some(obj) = path_item.as_object() {
            for (_method, operation) in obj {
                if let Some(op) = operation.as_object() {
                    let operation_id = op.get("operationId")
                        .and_then(|v| v.as_str())
                        .unwrap_or_else(|| "unknown");
                    
                    let summary = op.get("summary")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    
                    output.push_str(&format!("    // {}\n", summary));
                    output.push_str(&format!("    async {}() {{\n", operation_id));
                    output.push_str(&format!("        const response = await fetch(`${{this.baseUrl}}{}`);\n", path));
                    output.push_str("        return response.json();\n");
                    output.push_str("    }\n\n");
                }
            }
        }
    }
    
    fn generate_javascript_client(&self, json: &serde_json::Value) -> Result<String, String> {
        // Ähnlich wie TypeScript, aber ohne Types
        let mut output = String::new();
        
        output.push_str("// Auto-generated JavaScript Client\n");
        output.push_str("// Generated from OpenAPI Specification\n\n");
        
        output.push_str("class ApiClient {\n");
        output.push_str("    constructor(baseUrl = 'http://localhost:3000') {\n");
        output.push_str("        this.baseUrl = baseUrl;\n");
        output.push_str("    }\n\n");
        
        if let Some(paths) = json.get("paths").and_then(|p| p.as_object()) {
            for (path, path_item) in paths {
                self.generate_path_methods_js(&mut output, path, path_item);
            }
        }
        
        output.push_str("}\n\n");
        output.push_str("module.exports = ApiClient;\n");
        
        Ok(output)
    }
    
    fn generate_path_methods_js(&self, output: &mut String, path: &str, path_item: &serde_json::Value) {
        if let Some(obj) = path_item.as_object() {
            for (_method, operation) in obj {
                if let Some(op) = operation.as_object() {
                    let operation_id = op.get("operationId")
                        .and_then(|v| v.as_str())
                        .unwrap_or_else(|| "unknown");
                    
                    output.push_str(&format!("    async {}() {{\n", operation_id));
                    output.push_str(&format!("        const response = await fetch(`${{this.baseUrl}}{}`);\n", path));
                    output.push_str("        return response.json();\n");
                    output.push_str("    }\n\n");
                }
            }
        }
    }
    
    fn generate_rust_client(&self, json: &serde_json::Value) -> Result<String, String> {
        let mut output = String::new();
        
        output.push_str("// Auto-generated Rust Client\n");
        output.push_str("// Generated from OpenAPI Specification\n\n");
        
        output.push_str("use reqwest;\n");
        output.push_str("use serde_json;\n\n");
        
        output.push_str("pub struct ApiClient {\n");
        output.push_str("    base_url: String,\n");
        output.push_str("    client: reqwest::Client,\n");
        output.push_str("}\n\n");
        
        output.push_str("impl ApiClient {\n");
        output.push_str("    pub fn new(base_url: String) -> Self {\n");
        output.push_str("        ApiClient {\n");
        output.push_str("            base_url,\n");
        output.push_str("            client: reqwest::Client::new(),\n");
        output.push_str("        }\n");
        output.push_str("    }\n\n");
        
        if let Some(paths) = json.get("paths").and_then(|p| p.as_object()) {
            for (path, path_item) in paths {
                self.generate_path_methods_rust(&mut output, path, path_item);
            }
        }
        
        output.push_str("}\n");
        
        Ok(output)
    }
    
    fn generate_path_methods_rust(&self, output: &mut String, path: &str, path_item: &serde_json::Value) {
        if let Some(obj) = path_item.as_object() {
            for (method, operation) in obj {
                if let Some(op) = operation.as_object() {
                    let operation_id = op.get("operationId")
                        .and_then(|v| v.as_str())
                        .unwrap_or_else(|| "unknown");
                    
                    let method_upper = method.to_uppercase();
                    output.push_str(&format!("    pub async fn {}(&self) -> Result<serde_json::Value, reqwest::Error> {{\n", operation_id));
                    output.push_str(&format!("        let url = format!(\"{{}}{{}}\", self.base_url, \"{}\");\n", path));
                    output.push_str(&format!("        self.client.{}(&url).send().await?.json().await\n", method_upper));
                    output.push_str("    }\n\n");
                }
            }
        }
    }
}
