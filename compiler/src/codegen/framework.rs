// Framework-Selektor für Axum und Actix-Web
// Erkennt und generiert Code für das gewählte HTTP-Framework

use crate::parser::ast::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framework {
    Axum,
    Actix,
    Laravel,
    Symfony,
    FastAPI,
    Flask,
    Gin,
    Express,
    NestJS,
    Spring,
    AspNet,
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
                "laravel" => return Framework::Laravel,
                "symfony" => return Framework::Symfony,
                "fastapi" => return Framework::FastAPI,
                "flask" => return Framework::Flask,
                "gin" | "gogin" => return Framework::Gin,
                "express" => return Framework::Express,
                "nestjs" => return Framework::NestJS,
                "spring" | "springboot" => return Framework::Spring,
                "aspnet" | "aspnetcore" | "dotnet" => return Framework::AspNet,
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
                        "Laravel" | "@Laravel" => return Framework::Laravel,
                        "Symfony" | "@Symfony" => return Framework::Symfony,
                        "FastAPI" | "@FastAPI" => return Framework::FastAPI,
                        "Flask" | "@Flask" => return Framework::Flask,
                        "Gin" | "@Gin" => return Framework::Gin,
                        "Express" | "@Express" => return Framework::Express,
                        "NestJS" | "@NestJS" | "@Nest" => return Framework::NestJS,
                        "Spring" | "@Spring" | "SpringBoot" | "@SpringBoot" => return Framework::Spring,
                        "AspNet" | "@AspNet" | "AspNetCore" | "@AspNetCore" => return Framework::AspNet,
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
            Framework::Laravel => {
                "use Illuminate\\Http\\Request;\nuse Illuminate\\Support\\Facades\\Route;\nuse App\\Http\\Controllers\\Controller;\n".to_string()
            }
            Framework::Symfony => {
                "use Symfony\\Component\\HttpFoundation\\Response;\nuse Symfony\\Component\\HttpFoundation\\Request;\nuse Symfony\\Component\\Routing\\Annotation\\Route;\n".to_string()
            }
            Framework::FastAPI => {
                "from fastapi import FastAPI, HTTPException\nfrom pydantic import BaseModel\nimport uvicorn\n".to_string()
            }
            Framework::Flask => {
                "from flask import Flask, jsonify, request\n".to_string()
            }
            Framework::Gin => {
                "import (\n\t\"github.com/gin-gonic/gin\"\n\t\"net/http\"\n\t\"strconv\"\n)\n".to_string()
            }
            Framework::Express => {
                "import express, { Request, Response } from 'express';\n".to_string()
            }
            Framework::NestJS => {
                "import { Controller, Get, Post, Put, Delete, Body, Param, Query } from '@nestjs/common';\n".to_string()
            }
            Framework::Spring => {
                "import org.springframework.boot.SpringApplication;\nimport org.springframework.boot.autoconfigure.SpringBootApplication;\nimport org.springframework.web.bind.annotation.*;\nimport org.springframework.http.ResponseEntity;\n".to_string()
            }
            Framework::AspNet => {
                "using Microsoft.AspNetCore.Mvc;\nusing System.Collections.Generic;\n".to_string()
            }
        }
    }

    pub fn generate_node_main(framework: Framework, routes: Vec<(String, String, String)>) -> String {
        match framework {
            Framework::Express => {
                let mut code = "const app = express();\napp.use(express.json());\n\n".to_string();
                for (method, path, handler) in routes {
                    // app.get('/path', handler);
                    // convert /:id to :id (express format matches)
                    code.push_str(&format!("app.{}(\"{}\", {});\n", method.to_lowercase(), path, handler));
                }
                code.push_str("\nconst port = 3000;\napp.listen(port, () => {\n  console.log(`Server running on port ${port}`);\n});\n");
                code
            },
            _ => String::new()
        }
    }

    /// Generiert Python App-Initialisierung und Routing
    pub fn generate_python_app_init(framework: Framework, routes: Vec<(String, String, String)>) -> String {
        match framework {
            Framework::FastAPI => {
                let mut code = "app = FastAPI()\n\n".to_string();
                for (method, path, handler) in routes {
                    // app.add_api_route("/path", handler, methods=["GET"])
                    code.push_str(&format!("app.add_api_route(\"{}\", {}, methods=[\"{}\"])\n", path, handler, method.to_uppercase()));
                }
                code.push_str("\nif __name__ == \"__main__\":\n    uvicorn.run(app, host=\"0.0.0.0\", port=8000)\n");
                code
            },
            Framework::Flask => {
                let mut code = "app = Flask(__name__)\n\n".to_string();
                for (method, path, handler) in routes {
                    // app.add_url_rule('/path', view_func=handler, methods=['GET'])
                    code.push_str(&format!("app.add_url_rule(\"{}\", view_func={}, methods=[\"{}\"])\n", path, handler, method.to_uppercase()));
                }
                code.push_str("\nif __name__ == \"__main__\":\n    app.run(debug=True)\n");
                code
            },
            _ => String::new(),
        }
    }

    /// Generiert Go Main Function mit Router Setup
    pub fn generate_go_main(framework: Framework, routes: Vec<(String, String, String)>) -> String {
        match framework {
            Framework::Gin => {
                let mut code = "func main() {\n\tr := gin.Default()\n\n".to_string();
                for (method, path, handler) in routes {
                    // r.GET("/path", handler)
                    code.push_str(&format!("\tr.{}(\"{}\", {})\n", method.to_uppercase(), path, handler));
                }
                code.push_str("\n\tr.Run()\n}\n");
                code
            },
            _ => String::new(),
        }
    }

    /// Generiert PHP Routing Code (für Laravel/Symfony)
    pub fn generate_php_routes(framework: Framework, routes: Vec<(String, String, String)>, controller_name: &str) -> String {
        match framework {
            Framework::Laravel => {
                let mut code = String::new();
                if !routes.is_empty() {
                    code.push_str("// Routes\n");
                    for (method, path, func_name) in routes {
                        // Laravel: Route::get('/path', [AppController::class, 'methodName']);
                        code.push_str(&format!("Route::{}('{}', [{}::class, '{}']);\n", method.to_lowercase(), path, controller_name, func_name));
                    }
                }
                code
            },
            Framework::Symfony => {
                // Symfony typically uses Attributes on methods, so explicit route definition at the bottom is not needed
                // unless we want to support YAML/PHP config style. For now, we rely on Attributes.
                String::new()
            },
            _ => String::new(),
        }
    }

    /// Generiert Attribute/Decorators für Methoden (z.B. Symfony Route)
    pub fn generate_method_attributes(framework: Framework, method: &str, path: &str) -> Option<String> {
        match framework {
            Framework::Symfony => {
                Some(format!("#[Route('{}', methods: ['{}'])]", path, method.to_uppercase()))
            },
            _ => None,
        }
    }

    /// Generiert Router/App-Initialisierung (Rust)
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
            _ => String::new(),
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
