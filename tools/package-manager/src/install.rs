// Package Installation - Installiert Packages in vendor/ Verzeichnis

use anyhow::Result;
use git2::Repository;
use std::fs;
use std::path::{Path, PathBuf};
use crate::registry::Registry;
use crate::resolver::VelinToml;
use crate::lockfile::LockFile;

/// Initialisiert ein neues Projekt
pub fn init_project(name: Option<&str>) -> Result<()> {
    let project_name = name.unwrap_or("my-project");
    
    // Erstelle velin.toml
    let toml_content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"

[dependencies]
"#,
        project_name
    );
    
    fs::write("velin.toml", toml_content)?;
    
    // Erstelle vendor/ Verzeichnis
    fs::create_dir_all("vendor")?;
    
    // Erstelle .gitignore
    if !Path::new(".gitignore").exists() {
        fs::write(".gitignore", "vendor/\nvelin.lock\n")?;
    }
    
    Ok(())
}

/// Installiert alle Dependencies
pub async fn install_dependencies() -> Result<()> {
    let toml_path = Path::new("velin.toml");
    
    if !toml_path.exists() {
        anyhow::bail!("velin.toml nicht gefunden. Führe 'velin-pkg init' aus.");
    }

    let content = fs::read_to_string(toml_path)?;
    let config: VelinToml = toml::from_str(&content)?;

    // Erstelle vendor/ Verzeichnis
    fs::create_dir_all("vendor")?;

    let mut lock_file = LockFile::load()?;
    let mut locked_packages = std::collections::HashMap::new();

    // Installiere jede Dependency
    for (package_name, version_req) in &config.dependencies {
        println!("Installing {}...", package_name);
        
        // Parse package name (z.B. "github.com/user/repo")
        let parts: Vec<&str> = package_name.split('/').collect();
        if parts.len() < 3 {
            anyhow::bail!("Ungültiges Package-Format: {}. Erwartet: github.com/user/repo", package_name);
        }

        let owner = parts[1];
        let repo = parts[2];

        // Hole Package-Metadaten
        let metadata = Registry::fetch_package(owner, repo, Some(version_req)).await?;

        // Klone Repository in vendor/
        let vendor_path = PathBuf::from("vendor").join(package_name.replace('/', "_"));
        let url = format!("https://github.com/{}/{}.git", owner, repo);
        
        if vendor_path.exists() {
            // Update existing
            let repo = Repository::open(&vendor_path)?;
            // Use repo to check for updates
            let _head = repo.head()?;
            // In einer echten Implementierung würde man hier einen Pull machen
        } else {
            // Clone new
            Repository::clone(&url, &vendor_path)?;
        }

        locked_packages.insert(
            package_name.clone(),
            crate::lockfile::LockedPackage {
                version: metadata.version,
                source: url,
                checksum: None,
            },
        );
    }

    // Aktualisiere Lock File
    lock_file.update(locked_packages);
    lock_file.save()?;

    Ok(())
}
