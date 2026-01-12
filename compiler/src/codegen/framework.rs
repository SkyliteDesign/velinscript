// Framework-Selektor für Axum und Actix-Web
// Erkennt und generiert Code für das gewählte HTTP-Framework

use crate::parser::ast::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framework {
    Axum,
    Actix,
}

pub struct FrameworkSelector;

impl FrameworkSelector {
    /// Erkennt das zu verwendende Framework aus dem Programm
    pub fn detect_framework(program: &Program, config_framework: Option<&str>) -> Framework {
        // 1. Prüfe Config
        if let Some(fw) = config_framework {
            match fw.to_lowercase().as_str() {
                "axum" => return Framework::Axum,
                "actix" | "actix-web" => return Framework::Actix,
                _ => {}
            }
        }

        // 2. Prüfe Decorators
        for item in &program.items {
            if let Item::Function(f) = item {
                for decorator in &f.decorators {
                    match decorator.name.as_str() {
                        "Axum" | "@Axum" => return Framework::Axum,
                        "Actix" | "@Actix" | "ActixWeb" | "@ActixWeb" => return Framework::Actix,
                        _ => {}
                    }
                }
            }
        }

        // 3. Default: Axum (empfohlen für 2026)
        Framework::Axum
    }

    /// Generiert Framework-spezifische Imports
    pub fn generate_imports(framework: Framework) -> String {
        match framework {
            Framework::Axum => {
                "use axum::{\n    Router, extract::{Path, Query, Json, State}, routing::{get, post, put, delete}, response::Response, http::StatusCode\n};\nuse axum::response::IntoResponse;\nuse serde::{Deserialize, Serialize};\n".to_string()
            }
            Framework::Actix => {
                "use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};\nuse serde::{Deserialize, Serialize};\n".to_string()
            }
        }
    }

    /// Generiert Router/App-Initialisierung
    pub fn generate_app_init(framework: Framework, routes: Vec<(String, String, String)>) -> String {
        match framework {
            Framework::Axum => {
                let mut code = "pub fn create_router() -> Router {\n    Router::new()\n".to_string();
                for (method, path, handler) in routes {
                    let method_lower = method.to_lowercase();
                    code.push_str(&format!("        .route(\"{}\", {}({}_handler))\n", path, method_lower, handler));
                }
                code.push_str("}\n");
                code
            }
            Framework::Actix => {
                let mut code = "pub fn create_app() -> actix_web::App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest>> {\n    actix_web::App::new()\n".to_string();
                for (method, path, handler) in routes {
                    let method_lower = method.to_lowercase();
                    code.push_str(&format!("        .route(\"{}\", web::{}().to({}_handler))\n", path, method_lower, handler));
                }
                code.push_str("}\n");
                code
            }
        }
    }

    /// Generiert Handler-Signatur für Axum
    pub fn generate_axum_handler_signature(
        function_name: &str,
        params: &[crate::parser::ast::Parameter],
        return_type: &Option<Type>,
    ) -> String {
        let mut sig = format!("async fn {}_handler(", function_name);
        
        // Axum Extractors
        let mut extractors = Vec::new();
        for param in params {
            match param.param_type {
                Type::String | Type::Number | Type::Boolean => {
                    // Path parameter
                    extractors.push(format!("Path({}): Path<{}>", param.name, Self::velin_to_rust_type(&param.param_type)));
                }
                Type::Named(ref name) if name == "HttpRequest" => {
                    extractors.push("request: HttpRequest".to_string());
                }
                _ => {
                    // JSON Body
                    extractors.push(format!("Json(payload): Json<{}>", Self::velin_to_rust_type(&param.param_type)));
                }
            }
        }

        sig.push_str(&extractors.join(", "));
        sig.push_str(") -> impl IntoResponse");

        // Return type
        if let Some(ref _ret_type) = return_type {
            sig.push_str(&format!(" {{\n    // Handler implementation\n    let result = {}().await;\n    (StatusCode::OK, Json(result)).into_response()\n}}", function_name));
        } else {
            sig.push_str(" {\n    // Handler implementation\n    StatusCode::OK.into_response()\n}");
        }

        sig
    }

    /// Generiert Handler-Signatur für Actix
    pub fn generate_actix_handler_signature(
        function_name: &str,
        params: &[crate::parser::ast::Parameter],
        return_type: &Option<Type>,
    ) -> String {
        let mut sig = format!("async fn {}_handler(", function_name);
        
        // Actix Extractors
        let mut extractors = Vec::new();
        for param in params {
            match param.param_type {
                Type::String | Type::Number | Type::Boolean => {
                    extractors.push(format!("{}: {}", param.name, Self::velin_to_rust_type(&param.param_type)));
                }
                Type::Named(ref name) if name == "HttpRequest" => {
                    extractors.push("req: HttpRequest".to_string());
                }
                _ => {
                    extractors.push(format!("payload: web::Json<{}>", Self::velin_to_rust_type(&param.param_type)));
                }
            }
        }

        sig.push_str(&extractors.join(", "));
        sig.push_str(") -> impl Responder");

        // Return type
        if let Some(ref _ret_type) = return_type {
            sig.push_str(&format!(" {{\n    // Handler implementation\n    let result = {}().await;\n    HttpResponse::Ok().json(result)\n}}", function_name));
        } else {
            sig.push_str(" {\n    // Handler implementation\n    HttpResponse::Ok().finish()\n}");
        }

        sig
    }

    /// Konvertiert VelinScript Type zu Rust Type
    fn velin_to_rust_type(velin_type: &Type) -> String {
        match velin_type {
            Type::String => "String".to_string(),
            Type::Number => "f64".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::Void => "()".to_string(),
            Type::List(ref inner) => format!("Vec<{}>", Self::velin_to_rust_type(inner)),
            Type::Named(ref name) => name.clone(),
            _ => "String".to_string(),
        }
    }

    /// Prüft ob ein Decorator ein Framework-Decorator ist
    pub fn is_framework_decorator(decorator: &Decorator) -> bool {
        matches!(
            decorator.name.as_str(),
            "Axum" | "@Axum" | "Actix" | "@Actix" | "ActixWeb" | "@ActixWeb"
        )
    }
    
    /// Extrahiert Path-Parameter aus einem Route-Path (z.B. "/api/users/:id" -> ["id"])
    pub fn extract_path_params(path: &str) -> Vec<String> {
        let mut params = Vec::new();
        for segment in path.split('/') {
            if segment.starts_with(':') {
                params.push(segment[1..].to_string());
            }
        }
        params
    }
    
    /// Generiert Error-Response-Code für Axum
    pub fn generate_axum_error_response(status_code: u16, message: &str) -> String {
        format!(
            r#"(StatusCode::from_u16({}).unwrap(), Json(serde_json::json!({{
    "error": "{}"
}}))).into_response()"#,
            status_code, message
        )
    }
    
    /// Generiert Error-Response-Code für Actix
    pub fn generate_actix_error_response(status_code: u16, message: &str) -> String {
        format!(
            r#"HttpResponse::build(actix_web::http::StatusCode::from_u16({}).unwrap())
    .json(serde_json::json!({{
        "error": "{}"
    }}))"#,
            status_code, message
        )
    }
    
    /// Generiert CORS-Middleware für Axum
    pub fn generate_axum_cors() -> String {
        r#"use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);"#.to_string()
    }
    
    /// Generiert CORS-Middleware für Actix
    pub fn generate_actix_cors() -> String {
        r#"use actix_cors::Cors;

let cors = Cors::default()
    .allow_any_origin()
    .allow_any_method()
    .allow_any_header();"#.to_string()
    }
    
    /// Generiert vollständigen Handler mit Path/Query-Parameter-Extraktion für Axum
    pub fn generate_axum_handler_complete(
        function_name: &str,
        path: &str,
        params: &[crate::parser::ast::Parameter],
        return_type: &Option<Type>,
    ) -> String {
        let path_params = Self::extract_path_params(path);
        let mut handler = format!("async fn {}_handler(", function_name);
        
        let mut extractors = Vec::new();
        
        // Path parameters
        for path_param in &path_params {
            if let Some(param) = params.iter().find(|p| p.name == *path_param) {
                extractors.push(format!("Path({}): Path<{}>", path_param, Self::velin_to_rust_type(&param.param_type)));
            }
        }
        
        // Query parameters
        for param in params {
            if !path_params.contains(&param.name) && matches!(param.param_type, Type::String | Type::Number | Type::Boolean) {
                extractors.push(format!("Query({}): Query<{}>", param.name, Self::velin_to_rust_type(&param.param_type)));
            }
        }
        
        // Body parameters
        for param in params {
            if !path_params.contains(&param.name) && !matches!(param.param_type, Type::String | Type::Number | Type::Boolean) {
                extractors.push(format!("Json(payload): Json<{}>", Self::velin_to_rust_type(&param.param_type)));
            }
        }
        
        handler.push_str(&extractors.join(", "));
        handler.push_str(") -> impl IntoResponse {\n");
        
        // Handler body
        if let Some(ref _ret_type) = return_type {
            handler.push_str(&format!("    let result = {}().await;\n", function_name));
            handler.push_str("    (StatusCode::OK, Json(result)).into_response()\n");
        } else {
            handler.push_str("    StatusCode::OK.into_response()\n");
        }
        
        handler.push_str("}");
        handler
    }
    
    /// Generiert vollständigen Handler mit Path/Query-Parameter-Extraktion für Actix
    pub fn generate_actix_handler_complete(
        function_name: &str,
        path: &str,
        params: &[crate::parser::ast::Parameter],
        return_type: &Option<Type>,
    ) -> String {
        let path_params = Self::extract_path_params(path);
        let mut handler = format!("async fn {}_handler(", function_name);
        
        let mut extractors = Vec::new();
        
        // Path parameters
        for path_param in &path_params {
            if let Some(param) = params.iter().find(|p| p.name == *path_param) {
                extractors.push(format!("{}: web::Path<{}>", path_param, Self::velin_to_rust_type(&param.param_type)));
            }
        }
        
        // Query parameters
        for param in params {
            if !path_params.contains(&param.name) && matches!(param.param_type, Type::String | Type::Number | Type::Boolean) {
                extractors.push(format!("{}: web::Query<{}>", param.name, Self::velin_to_rust_type(&param.param_type)));
            }
        }
        
        // Body parameters
        for param in params {
            if !path_params.contains(&param.name) && !matches!(param.param_type, Type::String | Type::Number | Type::Boolean) {
                extractors.push(format!("payload: web::Json<{}>", Self::velin_to_rust_type(&param.param_type)));
            }
        }
        
        handler.push_str(&extractors.join(", "));
        handler.push_str(") -> impl Responder {\n");
        
        // Handler body
        if let Some(ref _ret_type) = return_type {
            handler.push_str(&format!("    let result = {}().await;\n", function_name));
            handler.push_str("    HttpResponse::Ok().json(result)\n");
        } else {
            handler.push_str("    HttpResponse::Ok().finish()\n");
        }
        
        handler.push_str("}");
        handler
    }
}
