use crate::parser::ast::*;
use serde_json::json;

pub struct AutoDocGenerator {
    pub output_format: String, // "json" or "html" (future)
}

impl AutoDocGenerator {
    pub fn new(output_format: &str) -> Self {
        AutoDocGenerator {
            output_format: output_format.to_string(),
        }
    }

    pub fn generate(&self, program: &Program) -> String {
        let mut docs = Vec::new();

        for item in &program.items {
            if let Some(doc_item) = self.generate_item(item) {
                docs.push(doc_item);
            }
        }

        let output = json!({
            "project": "VelinProject",
            "version": "1.0.0",
            "items": docs,
            "knowledge_base": self.generate_knowledge_base(&docs)
        });

        serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
    }

    fn generate_knowledge_base(&self, docs: &[serde_json::Value]) -> serde_json::Value {
        // Transform the raw docs into a Q&A style knowledge base for LLMs/RAG
        let mut entries = Vec::new();

        for doc in docs {
            if let Some(name) = doc.get("name").and_then(|n| n.as_str()) {
                if let Some(kind) = doc.get("kind").and_then(|k| k.as_str()) {
                    let description = doc
                        .get("documentation")
                        .and_then(|d| d.as_str())
                        .unwrap_or("No description provided.");

                    entries.push(json!({
                        "question": format!("How do I use the {} {}?", kind, name),
                        "answer": format!("The {} '{}' is defined as follows:\n\nDescription: {}\n\nSignature: {}", kind, name, description, doc.get("signature").and_then(|s| s.as_str()).unwrap_or("")),
                        "tags": [kind, name]
                    }));
                }
            }
        }

        json!(entries)
    }

    fn generate_item(&self, item: &Item) -> Option<serde_json::Value> {
        match item {
            Item::Function(f) => self.generate_function(f),
            Item::Struct(s) => self.generate_struct(s),
            Item::Module(m) => self.generate_module(m),
            _ => None,
        }
    }

    fn generate_function(&self, func: &Function) -> Option<serde_json::Value> {
        // Only document if @VelinAutoDoc is present or it has doc comments
        let has_autodoc = func.decorators.iter().any(|d| d.name == "VelinAutoDoc");

        if !has_autodoc && func.documentation.is_none() {
            return None;
        }

        let params = func
            .params
            .iter()
            .map(|p| {
                json!({
                    "name": p.name,
                    "type": p.param_type.to_string(),
                    "default": p.default.is_some()
                })
            })
            .collect::<Vec<_>>();

        let return_type = func
            .return_type
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or("void".to_string());

        let decorators = func
            .decorators
            .iter()
            .map(|d| d.name.clone())
            .collect::<Vec<_>>();

        // AI-Hint: This structure is ready to be fed into an LLM for "explanation generation"
        Some(json!({
            "kind": "function",
            "name": func.name,
            "signature": format!("fn {}({}) -> {}", func.name, params.iter().map(|p| format!("{}: {}", p["name"].as_str().unwrap(), p["type"].as_str().unwrap())).collect::<Vec<_>>().join(", "), return_type),
            "documentation": func.documentation,
            "params": params,
            "return_type": return_type,
            "decorators": decorators,
            "is_async": func.is_async,
            "is_public": func.visibility == Visibility::Public,
            "llm_prompt_context": {
                "description": "Generate a user-friendly explanation for this function.",
                "input_explanation": "Explain what inputs are required.",
                "output_explanation": "Explain what this function returns.",
                "error_scenarios": "Infer potential error scenarios based on types (e.g. Result<T, E>)."
            }
        }))
    }

    fn generate_struct(&self, struc: &Struct) -> Option<serde_json::Value> {
        let has_autodoc = struc.decorators.iter().any(|d| d.name == "VelinAutoDoc");

        if !has_autodoc && struc.documentation.is_none() {
            return None;
        }

        let fields = struc
            .fields
            .iter()
            .map(|f| {
                json!({
                    "name": f.name,
                    "type": f.field_type.to_string(),
                    "is_public": f.visibility == Visibility::Public
                })
            })
            .collect::<Vec<_>>();

        Some(json!({
            "kind": "struct",
            "name": struc.name,
            "documentation": struc.documentation,
            "fields": fields,
            "is_public": struc.visibility == Visibility::Public
        }))
    }

    fn generate_module(&self, module: &Module) -> Option<serde_json::Value> {
        let mut items = Vec::new();
        for item in &module.items {
            if let Some(doc) = self.generate_item(item) {
                items.push(doc);
            }
        }

        if items.is_empty() && module.documentation.is_none() {
            return None;
        }

        Some(json!({
            "kind": "module",
            "name": module.name,
            "documentation": module.documentation,
            "items": items
        }))
    }
}
