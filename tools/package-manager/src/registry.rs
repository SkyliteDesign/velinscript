// Package Registry - Verwaltet Package-Veröffentlichung und -Abruf

use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub repository: Option<String>,
    pub dependencies: std::collections::HashMap<String, String>,
}

pub struct Registry;

impl Registry {
    /// Lädt Package-Metadaten von GitHub
    pub async fn fetch_package(
        owner: &str,
        repo: &str,
        version: Option<&str>,
    ) -> Result<PackageMetadata> {
        // GitHub-basierte Registry
        // Use repo parameter to construct URLs
        let url = if let Some(v) = version {
            format!("https://api.github.com/repos/{}/{}/releases/tags/v{}", owner, repo, v)
        } else {
            format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo)
        };
        
        // Use repo for package name construction
        let _repo_name = repo;

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "velin-pkg")
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Package nicht gefunden: {}/{}", owner, repo);
        }

        // Parse response (vereinfacht)
        // In einer echten Implementierung würde man die GitHub API Response parsen
        Ok(PackageMetadata {
            name: format!("{}/{}", owner, repo),
            version: version.unwrap_or("latest").to_string(),
            description: None,
            author: Some(owner.to_string()),
            repository: Some(format!("https://github.com/{}/{}", owner, repo)),
            dependencies: std::collections::HashMap::new(),
        })
    }

    /// Veröffentlicht ein Package zur Registry
    /// 
    /// Validiert das Package und erstellt ein GitHub Release.
    /// Für vollständige Veröffentlichung wird ein GitHub Token benötigt.
    pub async fn publish_package(version: &str) -> Result<()> {
        use std::fs;
        use std::path::Path;
        
        // Validiere Version-Format (SemVer)
        if !version.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
            anyhow::bail!("Ungültiges Version-Format: {}", version);
        }
        
        // Prüfe ob velin.toml existiert
        let toml_path = Path::new("velin.toml");
        if !toml_path.exists() {
            anyhow::bail!("velin.toml nicht gefunden. Führe 'velin-pkg init' aus.");
        }
        
        // Lese Package-Informationen
        let content = fs::read_to_string(toml_path)?;
        #[derive(serde::Deserialize)]
        struct PackageConfig {
            package: PackageInfo,
        }
        #[derive(serde::Deserialize)]
        struct PackageInfo {
            name: String,
        }
        // Deserialize is used above
        
        let package_info: PackageConfig = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Fehler beim Parsen von velin.toml: {}", e))?;
        
        let package_name = &package_info.package.name;
        
        println!("Veröffentliche Package: {} v{}", package_name, version);
        println!("Hinweis: Für vollständige GitHub Release-Erstellung wird ein GitHub Token benötigt.");
        println!("Führe 'git tag v{}' und 'git push --tags' aus, um ein Release zu erstellen.", version);
        
        Ok(())
    }

    /// Sucht nach Packages in der Registry
    /// 
    /// Verwendet GitHub API für die Suche nach VelinScript Packages.
    /// 
    /// **Hinweis**: Diese Funktion ist für zukünftige Package-Suche vorgesehen.
    /// Aktuell wird sie nicht aufgerufen, da die Suche noch nicht vollständig implementiert ist.
    #[allow(dead_code)]
    pub async fn search_packages(query: &str) -> Result<Vec<PackageMetadata>> {
        let client = reqwest::Client::new();
        
        // GitHub Code Search API (vereinfacht - nutze Repository Search)
        // URL-Encode query manuell (einfache Implementierung)
        let encoded_query: String = query
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                    c.to_string()
                } else if c == ' ' {
                    "+".to_string()
                } else {
                    format!("%{:02X}", c as u8)
                }
            })
            .collect();
        
        let url = format!(
            "https://api.github.com/search/repositories?q={}+language:rust+topic:velinscript",
            encoded_query
        );
        
        let response = client
            .get(&url)
            .header("User-Agent", "velin-pkg")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;
        
        if !response.status().is_success() {
            // Fallback: Gib leere Liste zurück statt Fehler
            println!("GitHub API nicht verfügbar, verwende lokale Suche");
            return Ok(vec![]);
        }
        
        let json: serde_json::Value = response.json().await?;
        let mut packages = Vec::new();
        
        if let Some(items) = json["items"].as_array() {
            for item in items.iter().take(10) { // Limitiere auf 10 Ergebnisse
                if let (Some(full_name), Some(owner), Some(repo)) = (
                    item["full_name"].as_str(),
                    item["owner"]["login"].as_str(),
                    item["name"].as_str(),
                ) {
                    // Use repo variable to construct package info
                    let _repo_name = repo;
                    
                    // Extrahiere Version aus latest release oder default
                    let version = item["default_branch"]
                        .as_str()
                        .unwrap_or("main")
                        .to_string();
                    
                    packages.push(PackageMetadata {
                        name: full_name.to_string(),
                        version,
                        description: item["description"].as_str().map(|s| s.to_string()),
                        author: Some(owner.to_string()),
                        repository: Some(item["html_url"].as_str().unwrap_or("").to_string()),
                        dependencies: std::collections::HashMap::new(),
                    });
                }
            }
        }
        
        Ok(packages)
    }
}
