use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use dialoguer::{Input, Confirm, Select};
use heck::{ToSnakeCase, ToPascalCase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryConfig {
    pub name: String,
    pub description: String,
    pub category: Option<String>,
    pub functions: Vec<FunctionDef>,
    #[serde(default)]
    pub types: Vec<TypeDef>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: Option<String>,
    pub params: Vec<ParamDef>,
    pub return_type: Option<String>,
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDef {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: Option<String>,
    pub optional: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub description: Option<String>,
    pub optional: Option<bool>,
}

impl LibraryConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Konnte Datei nicht lesen: {:?}", path.as_ref()))?;
        
        let config: LibraryConfig = serde_yaml::from_str(&content)
            .with_context(|| "Fehler beim Parsen der YAML-Datei")?;
        
        Ok(config)
    }
    
    pub fn from_params(name: String, description: Option<String>) -> Result<Self> {
        Ok(LibraryConfig {
            name,
            description: description.unwrap_or_else(|| "Keine Beschreibung".to_string()),
            category: None,
            functions: vec![],
            types: vec![],
            dependencies: vec![],
            features: vec![],
        })
    }
    
    pub fn interactive() -> Result<Self> {
        println!("🔧 VelinScript Bibliotheks-Generator - Interaktiver Modus\n");
        
        // Modul-Name
        let name: String = Input::new()
            .with_prompt("Modul-Name")
            .validate_with(|input: &String| {
                if input.is_empty() {
                    Err("Modul-Name darf nicht leer sein")
                } else if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    Err("Modul-Name darf nur alphanumerische Zeichen und Unterstriche enthalten")
                } else {
                    Ok(())
                }
            })
            .interact()?;
        
        // Beschreibung
        let description: String = Input::new()
            .with_prompt("Beschreibung")
            .with_initial_text("")
            .interact()?;
        
        // Kategorie
        let categories = vec!["core", "api", "database", "security", "ml", "utils", "other"];
        let category_idx = Select::new()
            .with_prompt("Kategorie")
            .items(&categories)
            .default(6)
            .interact()?;
        let category = Some(categories[category_idx].to_string());
        
        // Funktionen
        let mut functions = vec![];
        loop {
            let add_function = Confirm::new()
                .with_prompt("Funktion hinzufügen?")
                .default(true)
                .interact()?;
            
            if !add_function {
                break;
            }
            
            let function = Self::interactive_function()?;
            functions.push(function);
        }
        
        // Typen
        let mut types = vec![];
        loop {
            let add_type = Confirm::new()
                .with_prompt("Typ hinzufügen?")
                .default(false)
                .interact()?;
            
            if !add_type {
                break;
            }
            
            let type_def = Self::interactive_type()?;
            types.push(type_def);
        }
        
        Ok(LibraryConfig {
            name,
            description,
            category,
            functions,
            types,
            dependencies: vec![],
            features: vec![],
        })
    }
    
    fn interactive_function() -> Result<FunctionDef> {
        let name: String = Input::new()
            .with_prompt("  Funktion-Name")
            .interact()?;
        
        let description: String = Input::new()
            .with_prompt("  Beschreibung")
            .with_initial_text("")
            .interact()
            .unwrap_or_default();
        
        // Parameter
        let mut params = vec![];
        loop {
            let add_param = Confirm::new()
                .with_prompt("    Parameter hinzufügen?")
                .default(true)
                .interact()?;
            
            if !add_param {
                break;
            }
            
            let param_name: String = Input::new()
                .with_prompt("      Parameter-Name")
                .interact()?;
            
            let param_types = vec!["string", "number", "boolean", "List<string>", "Map<string, any>", "any"];
            let param_type_idx = Select::new()
                .with_prompt("      Parameter-Typ")
                .items(&param_types)
                .default(0)
                .interact()?;
            let param_type = param_types[param_type_idx].to_string();
            
            params.push(ParamDef {
                name: param_name,
                param_type,
                description: None,
                optional: Some(false),
            });
        }
        
        // Rückgabetyp
        let return_types = vec!["void", "string", "number", "boolean", "List<string>", "Map<string, any>", "any"];
        let return_type_idx = Select::new()
            .with_prompt("  Rückgabetyp")
            .items(&return_types)
            .default(0)
            .interact()?;
        let return_type = if return_type_idx == 0 {
            None
        } else {
            Some(return_types[return_type_idx].to_string())
        };
        
        Ok(FunctionDef {
            name,
            description: if description.is_empty() { None } else { Some(description) },
            params,
            return_type,
            example: None,
        })
    }
    
    fn interactive_type() -> Result<TypeDef> {
        let name: String = Input::new()
            .with_prompt("  Typ-Name")
            .interact()?;
        
        let description: String = Input::new()
            .with_prompt("  Beschreibung")
            .with_initial_text("")
            .interact()
            .unwrap_or_default();
        
        // Felder
        let mut fields = vec![];
        loop {
            let add_field = Confirm::new()
                .with_prompt("    Feld hinzufügen?")
                .default(true)
                .interact()?;
            
            if !add_field {
                break;
            }
            
            let field_name: String = Input::new()
                .with_prompt("      Feld-Name")
                .interact()?;
            
            let field_types = vec!["string", "number", "boolean", "List<string>", "Map<string, any>", "any"];
            let field_type_idx = Select::new()
                .with_prompt("      Feld-Typ")
                .items(&field_types)
                .default(0)
                .interact()?;
            let field_type = field_types[field_type_idx].to_string();
            
            fields.push(FieldDef {
                name: field_name,
                field_type,
                description: None,
                optional: Some(false),
            });
        }
        
        Ok(TypeDef {
            name,
            description: if description.is_empty() { None } else { Some(description) },
            fields,
        })
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("❌ Fehler: Modul-Name darf nicht leer sein");
        }
        
        if !self.name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            anyhow::bail!(
                "❌ Fehler: Modul-Name '{}' enthält ungültige Zeichen.\n\
                 💡 Tipp: Verwenden Sie nur alphanumerische Zeichen und Unterstriche (z.B. 'my_module')",
                self.name
            );
        }
        
        if self.functions.is_empty() {
            anyhow::bail!(
                "❌ Fehler: Modul '{}' hat keine Funktionen definiert.\n\
                 💡 Tipp: Fügen Sie mindestens eine Funktion hinzu.",
                self.name
            );
        }
        
        // Validiere Funktionsnamen
        for function in &self.functions {
            if function.name.is_empty() {
                anyhow::bail!("❌ Fehler: Funktions-Name darf nicht leer sein");
            }
            
            if !function.name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                anyhow::bail!(
                    "❌ Fehler: Funktions-Name '{}' enthält ungültige Zeichen.\n\
                     💡 Tipp: Verwenden Sie nur alphanumerische Zeichen, Unterstriche und Bindestriche",
                    function.name
                );
            }
        }
        
        Ok(())
    }
    
    pub fn snake_case_name(&self) -> String {
        self.name.to_snake_case()
    }
    
    pub fn pascal_case_name(&self) -> String {
        self.name.to_pascal_case()
    }
    
    pub fn module_type(&self) -> ModuleType {
        if !self.types.is_empty() {
            ModuleType::StructBased
        } else if self.functions.iter().any(|f| f.name.contains("Service") || f.name.contains("Client")) {
            ModuleType::ServiceBased
        } else {
            ModuleType::SimpleFunctions
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ModuleType {
    SimpleFunctions,
    StructBased,
    ServiceBased,
}
