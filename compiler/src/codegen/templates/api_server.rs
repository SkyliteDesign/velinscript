use super::{Template, TemplateConfig};
use anyhow::Result;

/// API Server Template
///
/// Generiert vollständigen Axum/Actix-Web Server
/// Mit Routing, Middleware, Error Handling
pub struct APIServerTemplate;

impl Template for APIServerTemplate {
    fn generate(&self, config: &TemplateConfig) -> Result<String, String> {
        let framework = config
            .options
            .get("framework")
            .and_then(|v| v.as_str())
            .unwrap_or("axum");

        match framework {
            "axum" => Ok(self.generate_axum_server(&config.name)),
            "actix" => Ok(self.generate_actix_server(&config.name)),
            _ => Ok(self.generate_axum_server(&config.name)),
        }
    }
}

impl APIServerTemplate {
    fn generate_axum_server(&self, name: &str) -> String {
        format!(
            r#"use axum::{{Router, routing::get, Json}};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub async fn create_{}_server() -> Router {{
    Router::new()
        .route("/health", get(health_check))
        .layer(ServiceBuilder::new()
            .layer(logging_middleware())
            .into_inner())
        .layer(CorsLayer::permissive())
}}

async fn health_check() -> Json<serde_json::Value> {{
    Json(serde_json::json!({{"status": "ok"}}))
}}
"#,
            name.to_lowercase()
        )
    }

    fn generate_actix_server(&self, name: &str) -> String {
        format!(
            r#"use actix_web::{{web, App, HttpServer, HttpResponse, Result}};

pub async fn create_{}_server() -> std::io::Result<()> {{
    HttpServer::new(|| {{
        App::new()
            .route("/health", web::get().to(health_check))
    }})
    .bind("127.0.0.1:3000")?
    .run()
    .await
}}

async fn health_check() -> Result<HttpResponse> {{
    Ok(HttpResponse::Ok().json(serde_json::json!({{"status": "ok"}})))
}}
"#,
            name.to_lowercase()
        )
    }
}
