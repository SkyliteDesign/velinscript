// Security Analyzer - Analysiert AST auf Security-Vulnerabilities

use crate::rules::{SecurityFinding, SecurityRules};
use velin_compiler::parser::ast::*;
use std::fs;
use walkdir::WalkDir;
use anyhow::Result;

/// Scannt ein Verzeichnis auf Security-Vulnerabilities
pub fn scan_directory(path: &str) -> Result<Vec<SecurityFinding>> {
    let mut findings = Vec::new();
    let rules = SecurityRules::all_rules();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "velin")
                .unwrap_or(false)
        })
    {
        let file_path = entry.path();
        let content = fs::read_to_string(file_path)?;

        match velin_compiler::parser::parser::Parser::parse(&content) {
            Ok(program) => {
                let file_findings = analyze_program(&program, &rules);
                for mut finding in file_findings {
                    finding.location = format!("{}:{}", file_path.display(), "unknown");
                    findings.push(finding);
                }
            }
            Err(_) => {
                // Parse errors werden ignoriert (werden vom Compiler behandelt)
            }
        }
    }

    Ok(findings)
}

fn analyze_program(program: &Program, rules: &[crate::rules::SecurityRule]) -> Vec<SecurityFinding> {
    let mut findings = Vec::new();

    for item in &program.items {
        match item {
            Item::Function(f) => {
                findings.extend(analyze_block(&f.body, rules));
            }
            Item::Module(m) => {
                for item in &m.items {
                    if let Item::Function(f) = item {
                        findings.extend(analyze_block(&f.body, rules));
                    }
                }
            }
            _ => {}
        }
    }

    findings
}

fn analyze_block(block: &Block, rules: &[crate::rules::SecurityRule]) -> Vec<SecurityFinding> {
    let mut findings = Vec::new();

    for statement in &block.statements {
        match statement {
            Statement::Let(let_stmt) => {
                findings.extend(SecurityRules::check_expression(&let_stmt.value, rules));
            }
            Statement::Return(ret_stmt) => {
                if let Some(ref value) = ret_stmt.value {
                    findings.extend(SecurityRules::check_expression(value, rules));
                }
            }
            Statement::Expression(expr_stmt) => {
                findings.extend(SecurityRules::check_expression(&expr_stmt.expression, rules));
            }
            Statement::If(if_stmt) => {
                findings.extend(SecurityRules::check_expression(&if_stmt.condition, rules));
                findings.extend(analyze_block(&if_stmt.then_block, rules));
                if let Some(ref else_block) = if_stmt.else_block {
                    findings.extend(analyze_block(else_block, rules));
                }
            }
            Statement::For(for_stmt) => {
                findings.extend(SecurityRules::check_expression(&for_stmt.iterable, rules));
                findings.extend(analyze_block(&for_stmt.body, rules));
            }
            Statement::While(while_stmt) => {
                findings.extend(SecurityRules::check_expression(&while_stmt.condition, rules));
                findings.extend(analyze_block(&while_stmt.body, rules));
            }
            Statement::Match(match_stmt) => {
                findings.extend(SecurityRules::check_expression(&match_stmt.expression, rules));
                for arm in &match_stmt.arms {
                    findings.extend(analyze_block(&arm.body, rules));
                }
            }
            Statement::Throw(throw_stmt) => {
                findings.extend(SecurityRules::check_expression(&throw_stmt.expression, rules));
            }
            Statement::Break(_) => {
            }
        }
    }

    findings
}

/// Generiert HTML Report
pub fn generate_html_report(findings: &[SecurityFinding], output_path: &str) -> Result<()> {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>VelinScript Security Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .finding { border: 1px solid #ccc; padding: 10px; margin: 10px 0; }
        .critical { background-color: #ffcccc; }
        .high { background-color: #ffe6cc; }
        .medium { background-color: #ffffcc; }
        .low { background-color: #e6f3ff; }
    </style>
</head>
<body>
    <h1>VelinScript Security Report</h1>
    <p>Gefundene Vulnerabilities: <strong>"#,
    );

    html.push_str(&findings.len().to_string());
    html.push_str(r#"</strong></p>
    <hr>
"#);

    for finding in findings {
        let severity_class = match finding.severity {
            crate::rules::Severity::Critical => "critical",
            crate::rules::Severity::High => "high",
            crate::rules::Severity::Medium => "medium",
            crate::rules::Severity::Low => "low",
        };

        html.push_str(&format!(
            r#"    <div class="finding {}">
        <h3>{} - {}</h3>
        <p><strong>Location:</strong> {}</p>
        <p><strong>Description:</strong> {}</p>
        <p><strong>Recommendation:</strong> {}</p>
    </div>
"#,
            severity_class,
            finding.rule,
            format!("{:?}", finding.severity),
            finding.location,
            finding.message,
            finding.recommendation
        ));
    }

    html.push_str(
        r#"</body>
</html>"#,
    );

    fs::write(output_path, html)?;
    Ok(())
}

/// Druckt Text Report
pub fn print_text_report(findings: &[SecurityFinding]) {
    println!("VelinScript Security Report");
    println!("============================");
    println!("Gefundene Vulnerabilities: {}\n", findings.len());

    for finding in findings {
        println!("[{}] {}", format!("{:?}", finding.severity), finding.rule);
        println!("  Location: {}", finding.location);
        println!("  Description: {}", finding.message);
        println!("  Recommendation: {}", finding.recommendation);
        println!();
    }
}

/// Prüft Dependencies auf bekannte Security-Vulnerabilities
/// 
/// Liest velin.toml, extrahiert Dependencies und prüft sie gegen die OSV (Open Source Vulnerabilities) Datenbank.
pub async fn audit_dependencies(config_path: &str) -> Result<Vec<SecurityFinding>> {
    use std::fs;
    use std::path::Path;
    use std::collections::HashMap;
    use serde::{Deserialize, Serialize};
    
    let config_file = Path::new(config_path);
    if !config_file.exists() {
        anyhow::bail!("Konfigurationsdatei nicht gefunden: {}", config_path);
    }
    
    // Lese velin.toml
    let content = fs::read_to_string(config_file)?;
    
    #[derive(Deserialize)]
    struct VelinConfig {
        #[serde(default)]
        dependencies: HashMap<String, String>,
    }
    
    let config: VelinConfig = toml::de::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Fehler beim Parsen von {}: {}", config_path, e))?;
    
    if config.dependencies.is_empty() {
        println!("Keine Dependencies gefunden in {}", config_path);
        return Ok(vec![]);
    }
    
    println!("Prüfe {} Dependencies auf Vulnerabilities...", config.dependencies.len());
    
    let mut findings = Vec::new();
    let client = reqwest::Client::new();
    
    // OSV API Query
    #[derive(Serialize)]
    struct OSVQuery {
        version: String,
        package: OSVPackage,
    }
    
    #[derive(Serialize)]
    struct OSVPackage {
        name: String,
        ecosystem: String,
    }
    
    #[derive(Deserialize)]
    struct OSVResponse {
        vulns: Vec<OSVVuln>,
    }
    
    #[derive(Deserialize)]
    struct OSVVuln {
        id: String,
        summary: String,
        details: String, // Use details field for more information
        severity: Vec<OSVSeverity>,
    }
    
    #[derive(Deserialize)]
    struct OSVSeverity {
        #[serde(rename = "type")]
        severity_type: String, // Use severity_type field
        score: String,
    }
    
    // Prüfe jede Dependency
    for (package_name, version_req) in &config.dependencies {
        // Vereinfachte Version-Extraktion (nimmt erste Version)
        let version = version_req.split(',').next().unwrap_or(version_req).trim();
        
        // 1. Query OSV API (Open Source Vulnerabilities)
        let query = OSVQuery {
            version: version.to_string(),
            package: OSVPackage {
                name: package_name.clone(),
                ecosystem: "GitHub".to_string(),
            },
        };
        
        if let Ok(response) = client
            .post("https://api.osv.dev/v1/query")
            .json(&query)
            .send()
            .await
        {
            if response.status().is_success() {
                if let Ok(osv_response) = response.json::<OSVResponse>().await {
                    for vuln in osv_response.vulns {
                        // Use details and severity_type fields
                        let _vuln_details = &vuln.details;
                        let severity_str = vuln.severity
                            .first()
                            .map(|s| {
                                let _type = &s.severity_type; // Use severity_type
                                s.score.clone()
                            })
                            .unwrap_or_else(|| "unknown".to_string());
                        
                        findings.push(SecurityFinding {
                            rule: format!("OSV: {}", vuln.id),
                            location: format!("{}:{}", config_path, package_name),
                            message: format!("{} - {}", vuln.summary, vuln.details), // Use details
                            recommendation: format!("Update {} auf eine sichere Version", package_name),
                            severity: if severity_str.contains("CRITICAL") || severity_str.contains("HIGH") {
                                crate::rules::Severity::High
                            } else {
                                crate::rules::Severity::Medium
                            },
                        });
                    }
                }
            }
        }
        
        // 2. Query NVD API (National Vulnerability Database)
        // NVD API erfordert API Key für Rate Limits, aber wir versuchen es ohne
        if let Ok(response) = client
            .get(&format!("https://services.nvd.nist.gov/rest/json/cves/2.0?keywordSearch={}", package_name))
            .header("User-Agent", "VelinScript-Security-Scanner/1.0")
            .send()
            .await
        {
            if response.status().is_success() {
                #[derive(Deserialize)]
                struct NVDResponse {
                    #[serde(default)]
                    vulnerabilities: Vec<NVDVuln>,
                }
                
                #[derive(Deserialize)]
                struct NVDVuln {
                    cve: NVDCVE,
                }
                
                #[derive(Deserialize)]
                struct NVDCVE {
                    id: String,
                    #[serde(default)]
                    descriptions: Vec<NVDescription>,
                    #[serde(default)]
                    metrics: Option<NVCMetrics>,
                }
                
                #[derive(Deserialize)]
                struct NVDescription {
                    lang: String,
                    value: String,
                }
                
                #[derive(Deserialize)]
                struct NVCMetrics {
                    #[serde(default, rename = "cvssMetricV31")]
                    cvss_metric_v31: Vec<NVSScore>, // Use snake_case
                }
                
                #[derive(Deserialize)]
                struct NVSScore {
                    #[serde(default, rename = "cvssData")]
                    cvss_data: Option<NVSSData>, // Use snake_case
                }
                
                #[derive(Deserialize)]
                struct NVSSData {
                    #[serde(default, rename = "baseScore")]
                    base_score: Option<f64>, // Use snake_case
                }
                
                if let Ok(nvd_response) = response.json::<NVDResponse>().await {
                    for vuln in nvd_response.vulnerabilities {
                        let description = vuln.cve.descriptions
                            .iter()
                            .find(|d| d.lang == "en")
                            .map(|d| d.value.clone())
                            .unwrap_or_else(|| "No description available".to_string());
                        
                        let base_score = vuln.cve.metrics
                            .as_ref()
                            .and_then(|m| m.cvss_metric_v31.first())
                            .and_then(|v| v.cvss_data.as_ref())
                            .and_then(|d| d.base_score);
                        
                        let severity = if let Some(score) = base_score {
                            if score >= 9.0 {
                                crate::rules::Severity::Critical
                            } else if score >= 7.0 {
                                crate::rules::Severity::High
                            } else if score >= 4.0 {
                                crate::rules::Severity::Medium
                            } else {
                                crate::rules::Severity::Low
                            }
                        } else {
                            crate::rules::Severity::Medium
                        };
                        
                        findings.push(SecurityFinding {
                            rule: format!("CVE: {}", vuln.cve.id),
                            location: format!("{}:{}", config_path, package_name),
                            message: description,
                            recommendation: format!("Update {} auf eine sichere Version", package_name),
                            severity,
                        });
                    }
                }
            }
        }
        
        // 3. Query GitHub Security Advisories (für GitHub-Packages)
        if package_name.contains('/') {
            // Vermutlich ein GitHub-Package (user/repo Format)
            if let Ok(response) = client
                .get(&format!("https://api.github.com/repos/{}/security-advisories", package_name))
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await
            {
                if response.status().is_success() {
                    #[derive(Deserialize)]
                    struct GHAdvisory {
                        ghsa_id: String,
                        summary: String,
                        severity: String,
                        #[serde(default)]
                        cve_id: Option<String>,
                    }
                    
                    if let Ok(advisories) = response.json::<Vec<GHAdvisory>>().await {
                        for advisory in advisories {
                            let severity = match advisory.severity.as_str() {
                                "CRITICAL" => crate::rules::Severity::Critical,
                                "HIGH" => crate::rules::Severity::High,
                                "MODERATE" => crate::rules::Severity::Medium,
                                _ => crate::rules::Severity::Low,
                            };
                            
                            let rule_id = advisory.cve_id
                                .as_ref()
                                .map(|cve| format!("CVE: {}", cve))
                                .unwrap_or_else(|| format!("GHSA: {}", advisory.ghsa_id));
                            
                            findings.push(SecurityFinding {
                                rule: rule_id,
                                location: format!("{}:{}", config_path, package_name),
                                message: advisory.summary,
                                recommendation: format!("Update {} auf eine sichere Version", package_name),
                                severity,
                            });
                        }
                    }
                }
            }
        }
    }
    
    if findings.is_empty() {
        println!("✓ Keine bekannten Vulnerabilities gefunden");
    } else {
        println!("⚠️  {} Vulnerabilities gefunden", findings.len());
    }
    
    Ok(findings)
}
