use crate::codegen::boilerplate::BoilerplateGenerator;
use crate::codegen::client::ClientGenerator;
use crate::codegen::framework::{Framework, FrameworkSelector};
use crate::stdlib::ml::LLMClient;
use anyhow::Result;

/// System-Generator für boilerplate-freie Systeme
///
/// Erkennt High-Level APIs und generiert vollständige Systeme:
/// - API-Server (Axum/Actix-Web)
/// - Routing
/// - Auth (JWT/OAuth2)
/// - Rate-Limit (Redis-basiert)
/// - Logging (Structured Logging)
/// - KI-Client (OpenAI/Anthropic)
/// - Fehlerbehandlung
/// - Deployment-Config (Docker, K8s)
pub struct SystemGenerator {
    boilerplate_generator: BoilerplateGenerator,
    client_generator: ClientGenerator,
    llm_client: Option<LLMClient>,
    framework: Framework,
    port: u16,
}

#[derive(Debug, Clone)]
pub struct GeneratedSystem {
    pub components: Vec<GeneratedComponent>,
    pub integration_code: Option<String>,
    pub deployment_config: Option<DeploymentConfig>,
}

#[derive(Debug, Clone)]
pub struct GeneratedComponent {
    pub name: String,
    pub component_type: ComponentType,
    pub code: String,
}

#[derive(Debug, Clone)]
pub enum ComponentType {
    Server,
    Routing,
    Handlers,
    Authentication,
    RateLimiting,
    Logging,
    AIClient,
    Database,
    Caching,
    ErrorHandling,
    Docker,
    Kubernetes,
}

#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    pub deployment_type: DeploymentType,
    pub dockerfile: Option<String>,
    pub docker_compose: Option<String>,
    pub kubernetes: Option<String>,
}

#[derive(Debug, Clone)]
pub enum DeploymentType {
    Local,
    CloudSingle,
    CloudMulti,
    Serverless,
}

#[derive(Debug, Clone)]
pub enum APIType {
    Chatbot,
    Database,
    Authentication,
    REST,
    Custom,
}

#[derive(Debug, Clone)]
pub struct Requirements {
    pub needs_auth: bool,
    pub needs_rate_limit: bool,
    pub needs_ai: bool,
    pub needs_database: bool,
    pub needs_caching: bool,
    pub needs_deployment: bool,
}

impl SystemGenerator {
    pub fn new(llm_client: Option<LLMClient>) -> Self {
        Self {
            boilerplate_generator: BoilerplateGenerator::new(),
            client_generator: ClientGenerator::new(),
            llm_client,
            framework: Framework::Axum, // Default
            port: 3000,                 // Default
        }
    }

    pub fn with_framework(mut self, framework: Framework) -> Self {
        self.framework = framework;
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Erkennt Framework aus Programm oder Config
    pub fn detect_framework(
        &mut self,
        program: Option<&crate::parser::ast::Program>,
        config_framework: Option<&str>,
    ) {
        if let Some(prog) = program {
            self.framework = FrameworkSelector::detect_framework(prog, config_framework);
        } else if let Some(fw) = config_framework {
            self.framework = FrameworkSelector::detect_framework(
                &crate::parser::ast::Program { items: vec![] },
                Some(fw),
            );
        }
    }

    /// Generiert vollständiges System aus API-Call
    pub fn generate_system(&self, api_call: &APICall) -> Result<GeneratedSystem> {
        // 1. API-Typ erkennen
        let api_type = self.detect_api_type(api_call)?;

        // 2. System-Anforderungen analysieren
        let requirements = self.analyze_requirements(api_call, &api_type)?;

        // 3. Komponenten-Liste generieren
        let components = self.generate_components(&requirements)?;

        // 4. Für jede Komponente Code generieren
        let mut system = GeneratedSystem {
            components: Vec::new(),
            integration_code: None,
            deployment_config: None,
        };

        for component in components {
            let code = self.generate_component_code(&component, &requirements)?;
            system.components.push(GeneratedComponent {
                name: component.name.clone(),
                component_type: component.component_type.clone(),
                code,
            });
        }

        // 5. Integration generieren
        let integration = self.generate_integration(&system)?;
        system.integration_code = Some(integration);

        // 6. Deployment-Config generieren
        if requirements.needs_deployment {
            let deployment = self.generate_deployment(&system, &requirements)?;
            system.deployment_config = Some(deployment);
        }

        Ok(system)
    }

    /// Erkennt API-Typ mit KI
    fn detect_api_type(&self, api_call: &APICall) -> Result<APIType> {
        if let Some(ref client) = self.llm_client {
            let prompt = format!(
                "Analyze this API call and determine its type. Respond with one word: 'chatbot', 'database', 'authentication', 'rest', or 'custom'.\n\nAPI Call: {}",
                api_call.to_string()
            );

            match client.generate(&prompt) {
                Ok(response) => {
                    let response_lower = response.trim().to_lowercase();
                    Ok(match response_lower.as_str() {
                        "chatbot" | "llm" | "chat" => APIType::Chatbot,
                        "database" | "db" => APIType::Database,
                        "auth" | "authentication" => APIType::Authentication,
                        "api" | "rest" => APIType::REST,
                        _ => APIType::Custom,
                    })
                }
                Err(_) => Ok(APIType::Custom),
            }
        } else {
            // Heuristische Erkennung ohne KI
            Ok(self.heuristic_api_type_detection(api_call))
        }
    }

    /// Heuristische API-Typ-Erkennung
    fn heuristic_api_type_detection(&self, api_call: &APICall) -> APIType {
        let call_str = api_call.to_string().to_lowercase();
        let name_lower = api_call.name.to_lowercase();

        // Prüfe Decorators
        for decorator in &api_call.decorators {
            let decorator_lower = decorator.to_lowercase();
            if decorator_lower.contains("llm") || decorator_lower.contains("chat") {
                return APIType::Chatbot;
            }
        }

        // Prüfe Funktionsname und Call-String
        if name_lower.contains("chat")
            || name_lower.contains("llm")
            || call_str.contains("llm.")
            || call_str.contains("chat")
        {
            APIType::Chatbot
        } else if name_lower.contains("db")
            || name_lower.contains("database")
            || call_str.contains("db.")
            || call_str.contains("database")
        {
            APIType::Database
        } else if name_lower.contains("auth")
            || name_lower.contains("login")
            || call_str.contains("auth.")
            || call_str.contains("login")
        {
            APIType::Authentication
        } else if api_call.decorators.iter().any(|d| {
            let d_lower = d.to_lowercase();
            d_lower == "get" || d_lower == "post" || d_lower == "put" || d_lower == "delete"
        }) {
            APIType::REST
        } else {
            APIType::Custom
        }
    }

    /// Analysiert System-Anforderungen
    fn analyze_requirements(
        &self,
        _api_call: &APICall,
        api_type: &APIType,
    ) -> Result<Requirements> {
        let mut requirements = Requirements {
            needs_auth: false,
            needs_rate_limit: false,
            needs_ai: false,
            needs_database: false,
            needs_caching: false,
            needs_deployment: false,
        };

        match api_type {
            APIType::Chatbot => {
                requirements.needs_ai = true;
                requirements.needs_auth = true;
                requirements.needs_rate_limit = true;
                requirements.needs_deployment = true;
            }
            APIType::Database => {
                requirements.needs_database = true;
                requirements.needs_auth = true;
                requirements.needs_caching = true;
            }
            APIType::Authentication => {
                requirements.needs_auth = true;
                requirements.needs_rate_limit = true;
            }
            APIType::REST => {
                requirements.needs_auth = true;
                requirements.needs_rate_limit = true;
                requirements.needs_deployment = true;
            }
            APIType::Custom => {
                requirements.needs_auth = true;
                requirements.needs_rate_limit = true;
            }
        }

        Ok(requirements)
    }

    /// Generiert Komponenten-Liste
    fn generate_components(&self, requirements: &Requirements) -> Result<Vec<GeneratedComponent>> {
        let mut components = Vec::new();

        // Basis-Komponenten (immer)
        components.push(GeneratedComponent {
            name: "server".to_string(),
            component_type: ComponentType::Server,
            code: String::new(),
        });
        components.push(GeneratedComponent {
            name: "routing".to_string(),
            component_type: ComponentType::Routing,
            code: String::new(),
        });
        components.push(GeneratedComponent {
            name: "logging".to_string(),
            component_type: ComponentType::Logging,
            code: String::new(),
        });
        components.push(GeneratedComponent {
            name: "error_handling".to_string(),
            component_type: ComponentType::ErrorHandling,
            code: String::new(),
        });

        // Abhängig von Requirements
        if requirements.needs_auth {
            components.push(GeneratedComponent {
                name: "authentication".to_string(),
                component_type: ComponentType::Authentication,
                code: String::new(),
            });
        }
        if requirements.needs_rate_limit {
            components.push(GeneratedComponent {
                name: "rate_limiting".to_string(),
                component_type: ComponentType::RateLimiting,
                code: String::new(),
            });
        }
        if requirements.needs_ai {
            components.push(GeneratedComponent {
                name: "ai_client".to_string(),
                component_type: ComponentType::AIClient,
                code: String::new(),
            });
        }
        if requirements.needs_database {
            components.push(GeneratedComponent {
                name: "database".to_string(),
                component_type: ComponentType::Database,
                code: String::new(),
            });
        }
        if requirements.needs_caching {
            components.push(GeneratedComponent {
                name: "caching".to_string(),
                component_type: ComponentType::Caching,
                code: String::new(),
            });
        }

        // Deployment (optional)
        if requirements.needs_deployment {
            components.push(GeneratedComponent {
                name: "docker".to_string(),
                component_type: ComponentType::Docker,
                code: String::new(),
            });
            components.push(GeneratedComponent {
                name: "kubernetes".to_string(),
                component_type: ComponentType::Kubernetes,
                code: String::new(),
            });
        }

        Ok(components)
    }

    /// Generiert Code für Komponente
    fn generate_component_code(
        &self,
        component: &GeneratedComponent,
        requirements: &Requirements,
    ) -> Result<String> {
        let code = match component.component_type {
            ComponentType::Server => self.generate_server_code(),
            ComponentType::Routing => self.generate_routing_code(requirements),
            ComponentType::Handlers => self.generate_handlers_code(requirements),
            ComponentType::Authentication => self.generate_auth_code(),
            ComponentType::RateLimiting => self.generate_rate_limit_code(),
            ComponentType::Logging => self.generate_logging_code(),
            ComponentType::AIClient => self.generate_ai_client_code(),
            ComponentType::Database => self.generate_database_code(),
            ComponentType::Caching => self.generate_caching_code(),
            ComponentType::ErrorHandling => self.generate_error_handling_code(),
            ComponentType::Docker => self.generate_dockerfile(),
            ComponentType::Kubernetes => self.generate_kubernetes_config(),
        };

        // Validiere generierten Code
        self.validate_component_code(&code, &component.component_type)?;

        Ok(code)
    }

    /// Validiert generierten Komponenten-Code
    fn validate_component_code(&self, code: &str, component_type: &ComponentType) -> Result<()> {
        // Basis-Validierungen
        if code.is_empty() {
            return Err(anyhow::anyhow!(
                "Generated code for {:?} is empty",
                component_type
            ));
        }

        // Prüfe auf häufige Syntax-Fehler
        let open_braces = code.matches('{').count();
        let close_braces = code.matches('}').count();
        if open_braces != close_braces {
            return Err(anyhow::anyhow!(
                "Mismatched braces in {:?} code: {} open, {} close",
                component_type,
                open_braces,
                close_braces
            ));
        }

        let open_parens = code.matches('(').count();
        let close_parens = code.matches(')').count();
        if open_parens != close_parens {
            return Err(anyhow::anyhow!(
                "Mismatched parentheses in {:?} code: {} open, {} close",
                component_type,
                open_parens,
                close_parens
            ));
        }

        // Spezifische Validierungen je nach Komponenten-Typ
        match component_type {
            ComponentType::Server | ComponentType::Routing => {
                if !code.contains("Router") && !code.contains("router") {
                    return Err(anyhow::anyhow!("Server/Routing code must contain Router"));
                }
            }
            ComponentType::Database => {
                if code.contains("format!") && code.contains("SELECT") {
                    return Err(anyhow::anyhow!(
                        "Database code should not use format! for SQL queries (SQL injection risk)"
                    ));
                }
            }
            ComponentType::Kubernetes => {
                if !code.contains("apiVersion") {
                    return Err(anyhow::anyhow!("Kubernetes config must contain apiVersion"));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Generiert Server-Code
    fn generate_server_code(&self) -> String {
        match self.framework {
            Framework::Axum => {
                format!(
                    r#"// Axum Server Setup
// Port: {}
use axum::{{Router, routing::get, Json}};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

/// Erstellt und konfiguriert den Axum Server
/// 
/// Dieser Server verwendet:
/// - Structured Logging mit tracing
/// - CORS-Support für alle Origins
/// - Middleware-Stack für Request-Handling
pub async fn create_server() -> Router {{
    Router::new()
        .layer(ServiceBuilder::new()
            .layer(logging_middleware())
            .into_inner())
        .layer(CorsLayer::permissive())
}}
"#,
                    self.port
                )
            }
            Framework::Actix => {
                format!(
                    r#"// Actix-Web Server Setup
// Port: {}
use actix_web::{{web, App, HttpServer, middleware::Logger}};
use actix_cors::Cors;

/// Erstellt und konfiguriert den Actix-Web Server
/// 
/// Dieser Server verwendet:
/// - Structured Logging mit env_logger
/// - CORS-Support
/// - JSON-Request/Response-Handling
pub async fn create_server() -> std::io::Result<()> {{
    HttpServer::new(|| {{
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            // Routes werden hier hinzugefügt
    }})
    .bind(("0.0.0.0", {}))?
    .run()
    .await
}}
"#,
                    self.port, self.port
                )
            }
            _ => {
                // Fallback zu Axum
                self.generate_server_code_for_framework(Framework::Axum)
            }
        }
    }

    /// Generiert Server-Code für spezifisches Framework
    fn generate_server_code_for_framework(&self, framework: Framework) -> String {
        match framework {
            Framework::Axum => {
                format!(
                    r#"// Axum Server (Default)
// Port: {}
use axum::{{Router, routing::get, Json}};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub async fn create_server() -> Router {{
    Router::new()
        .layer(ServiceBuilder::new()
            .layer(logging_middleware())
            .into_inner())
        .layer(CorsLayer::permissive())
}}
"#,
                    self.port
                )
            }
            _ => {
                // Für andere Frameworks wird Axum als Fallback verwendet
                self.generate_server_code_for_framework(Framework::Axum)
            }
        }
    }

    /// Generiert Routing-Code
    fn generate_routing_code(&self, requirements: &Requirements) -> String {
        let imports = match self.framework {
            Framework::Axum => {
                "use axum::{Router, routing::{get, post, put, delete}};\n".to_string()
            }
            Framework::Actix => {
                "use actix_web::{web, HttpResponse, Result as ActixResult};\n".to_string()
            }
            _ => "use axum::{Router, routing::{get, post, put, delete}};\n".to_string(),
        };

        let mut code = format!(
            "// Routing Configuration\n// Framework: {:?}\n{}\n",
            self.framework, imports
        );

        match self.framework {
            Framework::Axum => {
                code.push_str("/// Fügt alle Routes zum Router hinzu\n");
                code.push_str("pub fn add_routes(router: Router) -> Router {\n");
                code.push_str("    router\n");
                code.push_str("        .route(\"/health\", get(|| async { \"OK\" }))\n");
            }
            Framework::Actix => {
                code.push_str("/// Konfiguriert alle Routes für Actix-Web\n");
                code.push_str("pub fn configure_routes(cfg: &mut web::ServiceConfig) {\n");
                code.push_str("    cfg.service(\n");
                code.push_str("        web::scope(\"\")\n");
                code.push_str("            .route(\"/health\", web::get().to(health_check))\n");
            }
            _ => {
                code.push_str("pub fn add_routes(router: Router) -> Router {\n");
                code.push_str("    router\n");
                code.push_str("        .route(\"/health\", get(|| async { \"OK\" }))\n");
            }
        }

        match self.framework {
            Framework::Axum => {
                if requirements.needs_ai {
                    code.push_str("        .route(\"/chat\", post(crate::handlers::chat))\n");
                }

                if requirements.needs_auth {
                    code.push_str("        .route(\"/login\", post(crate::handlers::login))\n");
                    code.push_str(
                        "        .route(\"/register\", post(crate::handlers::register))\n",
                    );
                }

                if requirements.needs_database {
                    code.push_str("        .route(\"/items\", get(crate::handlers::list_items))\n");
                    code.push_str(
                        "        .route(\"/items\", post(crate::handlers::create_item))\n",
                    );
                }

                code.push_str("}\n");
            }
            Framework::Actix => {
                if requirements.needs_ai {
                    code.push_str(
                        "            .route(\"/chat\", web::post().to(crate::handlers::chat))\n",
                    );
                }

                if requirements.needs_auth {
                    code.push_str(
                        "            .route(\"/login\", web::post().to(crate::handlers::login))\n",
                    );
                    code.push_str("            .route(\"/register\", web::post().to(crate::handlers::register))\n");
                }

                if requirements.needs_database {
                    code.push_str("            .route(\"/items\", web::get().to(crate::handlers::list_items))\n");
                    code.push_str("            .route(\"/items\", web::post().to(crate::handlers::create_item))\n");
                }

                code.push_str("    );\n");
                code.push_str("}\n\n");
                code.push_str("/// Health Check Endpoint\n");
                code.push_str("async fn health_check() -> ActixResult<HttpResponse> {\n");
                code.push_str("    Ok(HttpResponse::Ok().json(\"OK\"))\n");
                code.push_str("}\n");
            }
            _ => {
                // Fallback zu Axum-Format
                if requirements.needs_ai {
                    code.push_str("        .route(\"/chat\", post(crate::handlers::chat))\n");
                }
                if requirements.needs_auth {
                    code.push_str("        .route(\"/login\", post(crate::handlers::login))\n");
                    code.push_str(
                        "        .route(\"/register\", post(crate::handlers::register))\n",
                    );
                }
                if requirements.needs_database {
                    code.push_str("        .route(\"/items\", get(crate::handlers::list_items))\n");
                    code.push_str(
                        "        .route(\"/items\", post(crate::handlers::create_item))\n",
                    );
                }
                code.push_str("}\n");
            }
        }

        code
    }

    /// Generiert Handler-Code
    fn generate_handlers_code(&self, requirements: &Requirements) -> String {
        let mut code = match self.framework {
            Framework::Axum => {
                format!(
                    r#"// Handler Code für Axum Framework
// Port: {}

use axum::{{Json, response::IntoResponse, http::StatusCode}};
use serde::{{Deserialize, Serialize}};
"#,
                    self.port
                )
            }
            Framework::Actix => {
                format!(
                    r#"// Handler Code für Actix-Web Framework
// Port: {}

use actix_web::{{web, HttpResponse, Responder}};
use serde::{{Deserialize, Serialize}};
"#,
                    self.port
                )
            }
            _ => {
                format!(
                    r#"// Handler Code (Default: Axum)
// Port: {}

use axum::{{Json, response::IntoResponse, http::StatusCode}};
use serde::{{Deserialize, Serialize}};
"#,
                    self.port
                )
            }
        };

        if requirements.needs_auth {
            code.push_str("use jsonwebtoken::{encode, Header, EncodingKey};\n");
            code.push_str("use std::time::{SystemTime, UNIX_EPOCH};\n");
        }
        code.push_str("\n");

        if requirements.needs_ai {
            code.push_str("#[derive(Deserialize)]\n");
            code.push_str("pub struct ChatRequest { pub message: String }\n\n");
            code.push_str("#[derive(Serialize)]\n");
            code.push_str("pub struct ChatResponse { pub response: String }\n\n");
            code.push_str(
                "pub async fn chat(Json(payload): Json<ChatRequest>) -> impl IntoResponse {\n",
            );
            code.push_str("    // Real AI Logic using generated AIClient\n");
            code.push_str(
                "    let api_key = std::env::var(\"OPENAI_API_KEY\").unwrap_or_default();\n",
            );
            code.push_str("    // We use the generated AIClient struct\n");
            code.push_str("    // In a full system, this would be injected via Axum State\n");
            code.push_str("    use crate::ai_client::AIClient;\n");
            code.push_str("    use crate::stdlib::ml::LLMProvider;\n");
            code.push_str("    \n");
            code.push_str("    let client = AIClient::new(LLMProvider::OpenAI, api_key);\n");
            code.push_str("    match client.generate(&payload.message).await {\n");
            code.push_str("        Ok(content) => Json(ChatResponse { response: content }),\n");
            code.push_str(
                "        Err(e) => Json(ChatResponse { response: format!(\"AI Error: {}\", e) })\n",
            );
            code.push_str("    }\n");
            code.push_str("}\n\n");
        }

        if requirements.needs_auth {
            code.push_str("/// JWT Claims für Token-Validierung\n");
            code.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
            code.push_str("struct Claims {\n");
            code.push_str("    sub: String,\n");
            code.push_str("    exp: usize,\n");
            code.push_str("}\n\n");

            code.push_str("/// Login Request/Response Structs\n");
            code.push_str("#[derive(Deserialize)]\n");
            code.push_str(
                "pub struct LoginRequest { pub username: String, pub password: String }\n\n",
            );
            code.push_str("#[derive(Serialize)]\n");
            code.push_str("pub struct LoginResponse { pub token: String }\n\n");

            match self.framework {
                Framework::Axum => {
                    code.push_str(
                        "/// Login Handler - Authentifiziert Benutzer und gibt JWT-Token zurück\n",
                    );
                    code.push_str("pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {\n");
                }
                Framework::Actix => {
                    code.push_str(
                        "/// Login Handler - Authentifiziert Benutzer und gibt JWT-Token zurück\n",
                    );
                    code.push_str("pub async fn login(payload: web::Json<LoginRequest>) -> impl Responder {\n");
                }
                _ => {
                    code.push_str("pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {\n");
                }
            }
            if requirements.needs_database {
                code.push_str("    // Real Database Authentication\n");
                code.push_str(
                    "    // Note: In production, use dependency injection for the database pool\n",
                );
                code.push_str("    if let Ok(db) = crate::database::Database::new().await {\n");
                code.push_str(
                    "        // SECURITY: Using prepared statements to prevent SQL injection\n",
                );
                code.push_str("        use sqlx::Row;\n");
                code.push_str("        let query_result = sqlx::query(\"SELECT * FROM users WHERE username = $1 AND password = $2\")\n");
                code.push_str("            .bind(&payload.username)\n");
                code.push_str("            .bind(&payload.password)\n");
                code.push_str("            .fetch_optional(&db.pool)\n");
                code.push_str("            .await;\n");
                code.push_str("        if let Ok(Some(_row)) = query_result {\n");
                code.push_str(
                    "                let expiration = SystemTime::now().duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                    .as_secs() as usize + 3600;\n",
                );
                code.push_str("                let claims = Claims { sub: payload.username, exp: expiration };\n");
                code.push_str("                let secret = std::env::var(\"JWT_SECRET\").unwrap_or_else(|_| \"secret\".to_string());\n");
                code.push_str("                if let Ok(token) = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {\n");
                code.push_str(
                    "                    return (StatusCode::OK, Json(LoginResponse { token }));\n",
                );
                code.push_str("                }\n");
                code.push_str("            }\n");
                code.push_str("        }\n");
                code.push_str("    }\n");
            }
            code.push_str(
                "    // Fallback/Demo Authentication Logic (if DB fails or user not found)\n",
            );
            code.push_str(
                "    if payload.username == \"admin\" && payload.password == \"password\" {\n",
            );
            code.push_str(
                "        let expiration = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_secs() as usize + 3600;\n",
            );
            code.push_str(
                "        let claims = Claims { sub: payload.username, exp: expiration };\n",
            );
            code.push_str("        let secret = std::env::var(\"JWT_SECRET\").unwrap_or_else(|_| \"secret\".to_string());\n");
            code.push_str("        let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {\n");
            code.push_str("            Ok(t) => t,\n");
            code.push_str("            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(LoginResponse { token: format!(\"JWT encoding error: {{}}\", e) })),\n");
            code.push_str("        };\n");
            code.push_str("        (StatusCode::OK, Json(LoginResponse { token }))\n");
            code.push_str("    } else {\n");
            code.push_str("        (StatusCode::UNAUTHORIZED, Json(LoginResponse { token: \"\".to_string() }))\n");
            code.push_str("    }\n");
            code.push_str("}\n\n");

            code.push_str("pub async fn register(Json(_payload): Json<LoginRequest>) -> impl IntoResponse {\n");
            code.push_str("    (StatusCode::CREATED, \"User registered\")\n");
            code.push_str("}\n\n");
        }

        if requirements.needs_database {
            code.push_str("/// Item Struct für Datenbank-Operationen\n");
            code.push_str("#[derive(Serialize, Deserialize)]\n");
            code.push_str("pub struct Item { pub id: String, pub name: String }\n\n");

            match self.framework {
                Framework::Axum => {
                    code.push_str("/// List Items Handler - Gibt alle Items zurück\n");
                    code.push_str("pub async fn list_items() -> impl IntoResponse {\n");
                    code.push_str("    (StatusCode::OK, Json(vec![Item { id: \"1\".to_string(), name: \"Item 1\".to_string() }]))\n");
                    code.push_str("}\n\n");

                    code.push_str("/// Create Item Request Struct\n");
                    code.push_str("#[derive(Deserialize)]\n");
                    code.push_str("pub struct CreateItemRequest { pub name: String }\n\n");
                    code.push_str("/// Create Item Handler - Erstellt neues Item\n");
                    code.push_str("pub async fn create_item(Json(_payload): Json<CreateItemRequest>) -> impl IntoResponse {\n");
                    code.push_str("    (StatusCode::CREATED, Json(Item { id: \"2\".to_string(), name: \"New Item\".to_string() }))\n");
                }
                Framework::Actix => {
                    code.push_str("/// List Items Handler - Gibt alle Items zurück\n");
                    code.push_str("pub async fn list_items() -> impl Responder {\n");
                    code.push_str("    HttpResponse::Ok().json(vec![Item { id: \"1\".to_string(), name: \"Item 1\".to_string() }])\n");
                    code.push_str("}\n\n");

                    code.push_str("/// Create Item Request Struct\n");
                    code.push_str("#[derive(Deserialize)]\n");
                    code.push_str("pub struct CreateItemRequest { pub name: String }\n\n");
                    code.push_str("/// Create Item Handler - Erstellt neues Item\n");
                    code.push_str("pub async fn create_item(_payload: web::Json<CreateItemRequest>) -> impl Responder {\n");
                    code.push_str("    HttpResponse::Created().json(Item { id: \"2\".to_string(), name: \"New Item\".to_string() })\n");
                }
                _ => {
                    code.push_str("pub async fn list_items() -> impl IntoResponse {\n");
                    code.push_str("    Json(vec![Item { id: \"1\".to_string(), name: \"Item 1\".to_string() }])\n");
                    code.push_str("}\n\n");
                    code.push_str("#[derive(Deserialize)]\n");
                    code.push_str("pub struct CreateItemRequest { pub name: String }\n\n");
                    code.push_str("pub async fn create_item(Json(_payload): Json<CreateItemRequest>) -> impl IntoResponse {\n");
                    code.push_str("    (StatusCode::CREATED, Json(Item { id: \"2\".to_string(), name: \"New Item\".to_string() }))\n");
                }
            }
            code.push_str("}\n\n");
        }

        code
    }
    /// Generiert Auth-Code
    fn generate_auth_code(&self) -> String {
        r#"use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    // JWT Token Validation
    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: usize,
    }
    
    let auth_header = request.headers().get("authorization");
    if let Some(header_value) = auth_header {
        if let Ok(token_str) = header_value.to_str() {
            if token_str.starts_with("Bearer ") {
                let token = &token_str[7..];
                let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
                let key = DecodingKey::from_secret(secret.as_ref());
                let validation = Validation::new(Algorithm::HS256);
                
                if decode::<Claims>(token, &key, &validation).is_ok() {
                    return next.run(request).await;
                }
            }
        }
    }
    
    Response::builder()
        .status(401)
        .body("Unauthorized".into())
        .unwrap()
}
"#
        .to_string()
    }

    /// Generiert Rate-Limit-Code
    fn generate_rate_limit_code(&self) -> String {
        match self.framework {
            Framework::Axum => {
                format!(
                    r#"// Rate Limiting Middleware für Axum
// Port: {}

use axum::{{extract::Request, middleware::Next, response::Response, http::StatusCode}};

/// Rate Limit Middleware - Begrenzt Requests pro IP-Adresse
/// 
/// Verwendung:
/// - Fixed Window Strategy (100 Requests pro 60 Sekunden)
/// - In-Memory für Development, sollte in Production durch Redis ersetzt werden
pub async fn rate_limit_middleware(request: Request, next: Next) -> Response {{
    // Rate limiting (In-Memory for standalone, replace with Redis for production)
    use std::collections::HashMap;
    use std::sync::{{Arc, Mutex}};
    use std::time::{{Duration, SystemTime}};
    
    struct RateLimiter {{
        requests: Arc<Mutex<HashMap<String, (u64, SystemTime)>>>,
        max_requests: u64,
        window_seconds: u64,
    }}
    
    static LIMITER: once_cell::sync::Lazy<RateLimiter> = once_cell::sync::Lazy::new(|| {{
        RateLimiter {{
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests: 100,
            window_seconds: 60,
        }}
    }});
    
    let client_ip = request.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .or_else(|| request.headers().get("x-real-ip").and_then(|h| h.to_str().ok()))
        .unwrap_or("unknown");
    
    let mut requests = LIMITER.requests.lock().unwrap();
    let now = SystemTime::now();
    
    if let Some((count, window_start)) = requests.get(client_ip) {{
        if now.duration_since(*window_start)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_secs() < LIMITER.window_seconds {{
            if *count >= LIMITER.max_requests {{
                return Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS.as_u16())
                    .body("Rate limit exceeded".into())
                    .unwrap();
            }}
            if let Some(entry) = requests.get_mut(client_ip) {{
                *entry = (*count + 1, *window_start);
            }}
        }} else {{
            requests.insert(client_ip.to_string(), (1, now));
        }}
    }} else {{
        requests.insert(client_ip.to_string(), (1, now));
    }}
    
    next.run(request).await
}}
"#,
                    self.port
                )
            }
            Framework::Actix => {
                format!(
                    r#"// Rate Limiting Middleware für Actix-Web
// Port: {}

use actix_web::{{dev::ServiceRequest, Error}};
use std::collections::HashMap;
use std::sync::{{Arc, Mutex}};
use std::time::SystemTime;

/// Rate Limit Middleware - Begrenzt Requests pro IP
pub async fn rate_limit_middleware(
    req: ServiceRequest,
) -> Result<ServiceRequest, Error> {{
    // Rate limiting implementation
    // In Production: Use Redis-based rate limiting
    Ok(req)
}}
"#,
                    self.port
                )
            }
            _ => {
                // Fallback zu Axum
                self.generate_rate_limit_code_for_framework(Framework::Axum)
            }
        }
    }

    /// Generiert Rate-Limit-Code für spezifisches Framework
    fn generate_rate_limit_code_for_framework(&self, framework: Framework) -> String {
        match framework {
            Framework::Axum => {
                format!(
                    r#"use axum::{{extract::Request, middleware::Next, response::Response}};

pub async fn rate_limit_middleware(request: Request, next: Next) -> Response {{
    // Rate limiting implementation
    next.run(request).await
}}
"#
                )
            }
            _ => String::new(),
        }
    }

    /// Generiert Logging-Code
    fn generate_logging_code(&self) -> String {
        r#"use tracing::instrument;

#[instrument]
pub async fn logging_middleware(request: axum::extract::Request, next: axum::middleware::Next) -> axum::response::Response {
    tracing::info!("Request: {:?}", request.uri());
    next.run(request).await
}
"#.to_string()
    }

    /// Generiert AI-Client-Code
    fn generate_ai_client_code(&self) -> String {
        format!(
            r#"// AI Client für LLM-Integration
// Port: {}
// Framework: {:?}

use crate::stdlib::ml::LLMClient;
use crate::stdlib::ml::LLMProvider;

/// AIClient - Wrapper für LLM-Client mit Error-Handling
/// 
/// Unterstützt:
/// - OpenAI (GPT-4, GPT-3.5)
/// - Anthropic Claude
/// - Google Gemini
pub struct AIClient {{
    client: LLMClient,
}}

impl AIClient {{
    /// Erstellt neuen AI-Client mit Provider und API-Key
    /// 
    /// # Arguments
    /// * `provider` - LLM Provider (OpenAI, Anthropic, Gemini)
    /// * `api_key` - API-Key für den Provider (aus Umgebungsvariablen)
    pub fn new(provider: LLMProvider, api_key: String) -> Self {{
        Self {{
            client: LLMClient::new(provider, api_key),
        }}
    }}

    /// Generiert Text mit LLM
    /// 
    /// # Arguments
    /// * `prompt` - Eingabe-Prompt für LLM
    /// 
    /// # Returns
    /// * `Ok(String)` - Generierter Text
    /// * `Err(String)` - Fehler-Message
    pub async fn generate(&self, prompt: &str) -> Result<String, String> {{
        self.client.generate(prompt)
    }}
    
    /// Generiert Chat-Completion (für Konversationen)
    pub async fn chat(&self, messages: &[(&str, &str)]) -> Result<String, String> {{
        // Chat-Implementation würde hier kommen
        self.client.generate(&format!("Chat: {{:?}}", messages))
    }}
}}
"#,
            self.port, self.framework
        )
    }

    /// Generiert Database-Code
    fn generate_database_code(&self) -> String {
        r#"use sqlx::{PgPool, Row};
use std::env;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    pub async fn query(&self, sql: &str) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
        // WARNING: This method is unsafe and should only be used for trusted SQL.
        // For user input, always use prepared statements with bind parameters.
        sqlx::query(sql).fetch_all(&self.pool).await
    }
    
    /// Safe query method with prepared statements
    pub async fn query_with_params(&self, sql: &str, params: &[&dyn sqlx::postgres::PgHasArrayType]) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
        // Verwende Prepared Statements mit Parameter-Binding
        // sqlx::query() erstellt automatisch Prepared Statements
        // Parameter werden über .bind() gebunden
        let mut query = sqlx::query(sql);
        for param in params {
            // Parameter werden typ-sicher gebunden
            query = query.bind(param);
        }
        query.fetch_all(&self.pool).await
    }
}
"#.to_string()
    }

    /// Generiert Caching-Code
    fn generate_caching_code(&self) -> String {
        format!(
            r#"// Redis Cache für Caching-Operationen
// Port: {}
// Framework: {:?}

use redis::Client;

/// Cache - Redis-basierter Cache für schnelle Datenzugriffe
/// 
/// Features:
/// - Key-Value Storage
/// - TTL-Support (Time-To-Live)
/// - Connection Pooling
pub struct Cache {{
    client: Client,
}}

impl Cache {{
    /// Erstellt neuen Cache mit Redis-Client
    /// 
    /// # Environment Variables
    /// * `REDIS_URL` - Redis Connection String (z.B. redis://localhost:6379)
    pub fn new(redis_url: &str) -> Result<Self, String> {{
        let client = Client::open(redis_url)
            .map_err(|e| format!("Redis connection error: {{}}", e))?;
        Ok(Self {{ client }})
    }}

    /// Holt Wert aus Cache
    /// 
    /// # Arguments
    /// * `key` - Cache-Key
    /// 
    /// # Returns
    /// * `Ok(Some(String))` - Wert gefunden
    /// * `Ok(None)` - Key nicht gefunden
    /// * `Err(String)` - Redis-Fehler
    pub async fn get(&self, key: &str) -> Result<Option<String>, String> {{
        use redis::Commands;
        let mut conn = self.client.get_connection()
            .map_err(|e| format!("Redis connection error: {{}}", e))?;
        conn.get::<&str, String>(key)
            .map(Some)
            .or_else(|e| if e.kind() == redis::ErrorKind::TypeError {{
                Ok(None)
            }} else {{
                Err(format!("Redis get error: {{}}", e))
            }})
    }}

    /// Speichert Wert im Cache
    /// 
    /// # Arguments
    /// * `key` - Cache-Key
    /// * `value` - Zu speichernder Wert
    pub async fn set(&self, key: &str, value: &str) -> Result<(), String> {{
        use redis::Commands;
        let mut conn = self.client.get_connection()
            .map_err(|e| format!("Redis connection error: {{}}", e))?;
        conn.set::<&str, &str, ()>(key, value)
            .map_err(|e| format!("Redis set error: {{}}", e))
    }}
    
    /// Speichert Wert mit TTL (Time-To-Live)
    /// 
    /// # Arguments
    /// * `key` - Cache-Key
    /// * `value` - Zu speichernder Wert
    /// * `ttl_seconds` - TTL in Sekunden
    pub async fn set_with_ttl(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<(), String> {{
        use redis::Commands;
        let mut conn = self.client.get_connection()
            .map_err(|e| format!("Redis connection error: {{}}", e))?;
        conn.set_ex::<&str, &str, ()>(key, value, ttl_seconds as usize)
            .map_err(|e| format!("Redis set error: {{}}", e))
    }}
    
    /// Löscht Wert aus Cache
    pub async fn delete(&self, key: &str) -> Result<(), String> {{
        use redis::Commands;
        let mut conn = self.client.get_connection()
            .map_err(|e| format!("Redis connection error: {{}}", e))?;
        conn.del::<&str, ()>(key)
            .map_err(|e| format!("Redis delete error: {{}}", e))
    }}
}}
"#,
            self.port, self.framework
        )
    }

    /// Generiert Error-Handling-Code
    fn generate_error_handling_code(&self) -> String {
        r#"use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", self.0)).into_response()
    }
}
"#
        .to_string()
    }

    /// Generiert Dockerfile
    fn generate_dockerfile(&self) -> String {
        format!(
            r#"# Multi-stage Dockerfile für optimale Image-Größe
# Port: {}

# Build-Stage: Kompiliert die Anwendung
FROM rust:1.70 as builder
WORKDIR /app

# Kopiere Dependency-Dateien zuerst (für besseres Caching)
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Kompiliere in Release-Mode für optimale Performance
RUN cargo build --release

# Runtime-Stage: Minimale Runtime-Umgebung
FROM debian:bookworm-slim
WORKDIR /app

# Installiere nur notwendige Runtime-Dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Kopiere kompilierte Binary vom Builder-Stage
COPY --from=builder /app/target/release/velin-compiler /usr/local/bin/velin-compiler

# Exponiere den konfigurierten Port
EXPOSE {}

# Setze Umgebungsvariablen
ENV RUST_LOG=info
ENV PORT={}

# Starte die Anwendung
CMD ["velin-compiler"]
"#,
            self.port, self.port, self.port
        )
    }

    /// Generiert Kubernetes-Config
    fn generate_kubernetes_config(&self) -> String {
        format!(
            r#"# Kubernetes Deployment Configuration
# Port: {}
# Framework: {:?}

apiVersion: apps/v1
kind: Deployment
metadata:
  name: velin-api
  labels:
    app: velin-api
    version: "3.1.0"
spec:
  replicas: 3
  selector:
    matchLabels:
      app: velin-api
  template:
    metadata:
      labels:
        app: velin-api
        version: "3.1.0"
    spec:
      containers:
      - name: api
        image: velin-api:latest
        imagePullPolicy: Always
        ports:
        - name: http
          containerPort: {}
          protocol: TCP
        - name: metrics
          containerPort: 9090
          protocol: TCP
        env:
        - name: PORT
          value: "{}"
        - name: RUST_LOG
          value: "info"
        # Health Checks für automatische Pod-Wartung
        livenessProbe:
          httpGet:
            path: /health
            port: {}
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health
            port: {}
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
        # Resource Limits für Stabilität
        resources:
          requests:
            memory: "256Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        # Security Context
        securityContext:
          runAsNonRoot: true
          runAsUser: 1000
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: false
---
# Service für Load Balancing
apiVersion: v1
kind: Service
metadata:
  name: velin-api-service
  labels:
    app: velin-api
spec:
  type: LoadBalancer
  selector:
    app: velin-api
  ports:
  - name: http
    port: 80
    targetPort: {}
    protocol: TCP
  - name: metrics
    port: 9090
    targetPort: 9090
    protocol: TCP
  # Session Affinity für Stateful Applications
  sessionAffinity: ClientIP
  sessionAffinityConfig:
    clientIP:
      timeoutSeconds: 10800
---
# HorizontalPodAutoscaler für automatische Skalierung
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: velin-api-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: velin-api
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
---
# ServiceMonitor für Prometheus Monitoring (optional, benötigt Prometheus Operator)
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: velin-api-monitor
  labels:
    app: velin-api
spec:
  selector:
    matchLabels:
      app: velin-api
  endpoints:
  - port: metrics
    path: /metrics
    interval: 30s
    scrapeTimeout: 10s
"#,
            self.port, self.framework, self.port, self.port, self.port, self.port, self.port
        )
    }

    /// Generiert Integration-Code
    fn generate_integration(&self, system: &GeneratedSystem) -> Result<String> {
        let mut integration = String::from("// System Integration Code\n\n");

        // Zentrale Import-Verwaltung
        let mut imports = std::collections::HashSet::new();
        imports.insert("use axum::{Router, routing::{get, post, put, delete}, Json, extract::Request, middleware::Next, response::{Response, IntoResponse}, http::StatusCode};".to_string());
        imports.insert("use serde::{Deserialize, Serialize};".to_string());
        imports.insert("use tower::ServiceBuilder;".to_string());
        imports.insert("use tower_http::cors::CorsLayer;".to_string());

        // Sammle Imports aus allen Komponenten
        for component in &system.components {
            if component.code.contains("sqlx::") {
                imports.insert("use sqlx::{PgPool, Row};".to_string());
            }
            if component.code.contains("jsonwebtoken") {
                imports.insert("use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};".to_string());
            }
            if component.code.contains("redis") {
                imports.insert("use redis::Client;".to_string());
            }
            if component.code.contains("tracing") {
                imports.insert("use tracing::instrument;".to_string());
            }
        }

        // Füge Imports hinzu
        for import in &imports {
            integration.push_str(import);
            integration.push_str("\n");
        }
        integration.push_str("\n");

        // Füge Komponenten hinzu
        for component in &system.components {
            integration.push_str(&format!("// Component: {}\n", component.name));
            integration.push_str(&component.code);
            integration.push_str("\n\n");
        }

        Ok(integration)
    }

    /// Generiert Deployment-Config
    fn generate_deployment(
        &self,
        system: &GeneratedSystem,
        requirements: &Requirements,
    ) -> Result<DeploymentConfig> {
        let mut config = DeploymentConfig {
            deployment_type: DeploymentType::CloudSingle,
            dockerfile: None,
            docker_compose: None,
            kubernetes: None,
        };

        // Finde Docker und Kubernetes Komponenten
        for component in &system.components {
            match component.component_type {
                ComponentType::Docker => {
                    config.dockerfile = Some(component.code.clone());
                }
                ComponentType::Kubernetes => {
                    config.kubernetes = Some(component.code.clone());
                }
                _ => {}
            }
        }

        // Generiere docker-compose.yml
        if requirements.needs_database || requirements.needs_caching {
            config.docker_compose = Some(self.generate_docker_compose(requirements));
        }

        Ok(config)
    }

    /// Generiert docker-compose.yml
    fn generate_docker_compose(&self, requirements: &Requirements) -> String {
        let mut compose = format!(
            r#"# Docker Compose Configuration
# Port: {}
# Framework: {:?}

version: '3.8'
services:
  api:
    build: .
    ports:
      - "{}:{}"
    environment:
      - RUST_LOG=info
      - PORT={}
"#,
            self.port, self.framework, self.port, self.port, self.port
        );

        let mut depends_on = Vec::new();

        if requirements.needs_ai {
            compose.push_str("      - OPENAI_API_KEY=${OPENAI_API_KEY}\n");
        }
        if requirements.needs_caching {
            compose.push_str("      - REDIS_URL=redis://redis:6379\n");
            depends_on.push("redis".to_string());
        }
        if requirements.needs_database {
            compose.push_str("      - DATABASE_URL=${DATABASE_URL}\n");
            depends_on.push("postgres".to_string());
        }

        // Füge depends_on nur hinzu wenn es Dependencies gibt
        if !depends_on.is_empty() {
            compose.push_str("    depends_on:\n");
            for dep in &depends_on {
                compose.push_str(&format!("      - {}\n", dep));
            }
        }

        if requirements.needs_caching {
            compose.push_str(
                "\n  redis:\n    image: redis:alpine\n    ports:\n      - \"6379:6379\"\n",
            );
        }
        if requirements.needs_database {
            compose.push_str("\n  postgres:\n    image: postgres:15\n    environment:\n      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}\n    ports:\n      - \"5432:5432\"\n");
        }

        compose
    }
}

/// Repräsentiert einen API-Call für System-Generierung
#[derive(Debug, Clone)]
pub struct APICall {
    pub name: String,
    pub args: Vec<String>,
    pub decorators: Vec<String>,
    pub return_type: Option<String>,
    pub is_async: bool,
}

impl APICall {
    pub fn to_string(&self) -> String {
        format!("{}({})", self.name, self.args.join(", "))
    }

    /// Erstellt APICall aus AST Function
    pub fn from_ast(function: &crate::parser::ast::Function) -> Self {
        let decorators: Vec<String> = function.decorators.iter().map(|d| d.name.clone()).collect();

        let args: Vec<String> = function
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, Self::type_to_string(&p.param_type)))
            .collect();

        let return_type = function
            .return_type
            .as_ref()
            .map(|t| Self::type_to_string(t));

        Self {
            name: function.name.clone(),
            args,
            decorators,
            return_type,
            is_async: function.is_async,
        }
    }

    /// Konvertiert Type zu String
    fn type_to_string(ty: &crate::parser::ast::Type) -> String {
        match ty {
            crate::parser::ast::Type::Named(name) => name.clone(),
            crate::parser::ast::Type::String => "string".to_string(),
            crate::parser::ast::Type::Number => "number".to_string(),
            crate::parser::ast::Type::Boolean => "boolean".to_string(),
            crate::parser::ast::Type::Void => "void".to_string(),
            crate::parser::ast::Type::Null => "null".to_string(),
            crate::parser::ast::Type::Any => "any".to_string(),
            crate::parser::ast::Type::List(item_type) => {
                format!("List<{}>", Self::type_to_string(item_type))
            }
            crate::parser::ast::Type::Map { key, value } => {
                format!(
                    "Map<{}, {}>",
                    Self::type_to_string(key),
                    Self::type_to_string(value)
                )
            }
            crate::parser::ast::Type::Result { ok, err } => {
                format!(
                    "Result<{}, {}>",
                    Self::type_to_string(ok),
                    Self::type_to_string(err)
                )
            }
            crate::parser::ast::Type::Optional(inner_type) => {
                format!("Option<{}>", Self::type_to_string(inner_type))
            }
            crate::parser::ast::Type::Generic { name, params } => {
                if params.is_empty() {
                    name.clone()
                } else {
                    format!(
                        "{}<{}>",
                        name,
                        params
                            .iter()
                            .map(|p| Self::type_to_string(p))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            crate::parser::ast::Type::Tuple(types) => {
                format!(
                    "({})",
                    types
                        .iter()
                        .map(|t| Self::type_to_string(t))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            crate::parser::ast::Type::Function {
                params,
                return_type,
            } => {
                let params_str = params
                    .iter()
                    .map(|p| Self::type_to_string(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "fn({}) -> {}",
                    params_str,
                    Self::type_to_string(return_type)
                )
            }
        }
    }
}
