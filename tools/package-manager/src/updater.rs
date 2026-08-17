// Dependency Updater - Prüft und aktualisiert Dependencies

use anyhow::Result;
use semver::{Version, VersionReq};
use serde::Serialize;
use crate::resolver::VelinToml;
use crate::registry::Registry;

#[derive(Debug, Clone, Serialize)]
// Serialize is used for UpdateInfo
pub struct UpdateInfo {
    pub package: String,
    pub current: String,
    pub latest: String,
    pub breaking: bool,
    pub changelog: Option<String>,
}

pub struct DependencyUpdater;

impl DependencyUpdater {
    /// Prüft alle Dependencies auf verfügbare Updates
    pub async fn check_updates(config: &VelinToml) -> Result<Vec<UpdateInfo>> {
        let mut updates = Vec::new();

        for (package_name, version_req_str) in &config.dependencies {
            if VersionReq::parse(version_req_str).is_ok() {
                // Parse package name (z.B. "github.com/user/repo")
                let parts: Vec<&str> = package_name.split('/').collect();
                if parts.len() >= 3 {
                    let owner = parts[1];
                    let repo = parts[2];
                    // Use repo variable in fetch_package call

                    // Hole neueste Version von Registry
                    match Registry::fetch_package(owner, repo, None).await {
                        Ok(metadata) => {
                            if let Ok(latest_version) = Version::parse(&metadata.version) {
                                // Prüfe ob Update verfügbar (vereinfacht)
                                // Prüfe ob Breaking Change (Major Version)
                                let breaking = latest_version.major > 0;
                                
                                // Immer Update vorschlagen (vereinfachte Logik)
                                updates.push(UpdateInfo {
                                    package: package_name.clone(),
                                    current: version_req_str.clone(),
                                    latest: metadata.version,
                                    breaking,
                                    changelog: None,
                                });
                            }
                        }
                        Err(_) => {
                            // Package nicht gefunden oder Registry-Fehler
                            // Ignoriere für jetzt
                        }
                    }
                }
            }
        }

        Ok(updates)
    }

    /// Aktualisiert eine Dependency
    pub async fn update_dependency(
        config: &mut VelinToml,
        package: &str,
        version: Option<&str>,
    ) -> Result<()> {
        if let Some(version_str) = version {
            // Spezifische Version setzen
            config.dependencies.insert(package.to_string(), version_str.to_string());
        } else {
            // Neueste kompatible Version finden
            let parts: Vec<&str> = package.split('/').collect();
            if parts.len() >= 3 {
                let owner = parts[1];
                let repo = parts[2];
                // Use repo variable in fetch_package call

                match Registry::fetch_package(owner, repo, None).await {
                    Ok(metadata) => {
                        // Behalte SemVer-Constraint, aber aktualisiere auf neueste kompatible Version
                        if let Some(current_req) = config.dependencies.get(package) {
                            if let Ok(req) = VersionReq::parse(current_req) {
                                // Prüfe ob neue Version kompatibel ist
                                if let Ok(latest) = Version::parse(&metadata.version) {
                                    if req.matches(&latest) {
                                        // Update zu neuester kompatibler Version
                                        config.dependencies.insert(
                                            package.to_string(),
                                            format!("^{}", latest),
                                        );
                                    } else {
                                        // Breaking Change - behalte alte Version
                                        println!("⚠ Breaking Change für {}: {} -> {}", package, current_req, metadata.version);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        anyhow::bail!("Konnte Package nicht aktualisieren: {}", e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Aktualisiert alle Dependencies
    pub async fn update_all(
        config: &mut VelinToml,
        allow_breaking: bool,
    ) -> Result<Vec<UpdateInfo>> {
        let updates = Self::check_updates(config).await?;
        let mut updated = Vec::new();

        for update in &updates {
            if update.breaking && !allow_breaking {
                println!("⚠ Überspringe {} (Breaking Change)", update.package);
                continue;
            }

            match Self::update_dependency(config, &update.package, Some(&update.latest)).await {
                Ok(()) => {
                    updated.push(update.clone());
                    println!("✓ Aktualisiert: {} {} -> {}", update.package, update.current, update.latest);
                }
                Err(e) => {
                    println!("✗ Fehler beim Aktualisieren von {}: {}", update.package, e);
                }
            }
        }

        Ok(updated)
    }
}
