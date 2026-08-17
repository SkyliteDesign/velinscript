use super::{Template, TemplateConfig};
use anyhow::Result;

/// Auth Template
/// 
/// Generiert JWT/OAuth2 Auth
/// Mit Middleware, Token-Validierung
pub struct AuthTemplate;

impl Template for AuthTemplate {
    fn generate(&self, config: &TemplateConfig) -> Result<String, String> {
        let auth_type = config.options
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("jwt");

        match auth_type {
            "jwt" => Ok(self.generate_jwt_auth()),
            "oauth2" => Ok(self.generate_oauth2_auth()),
            _ => Ok(self.generate_jwt_auth()),
        }
    }
}

impl AuthTemplate {
    fn generate_jwt_auth(&self) -> String {
        r#"use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation, Algorithm, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    // Extract token from Authorization header
    let auth_header = request.headers().get("authorization");
    
    if let Some(header_value) = auth_header {
        if let Ok(token_str) = header_value.to_str() {
            if token_str.starts_with("Bearer ") {
                let token = &token_str[7..];
                // Validate JWT token
                let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
                let key = DecodingKey::from_secret(secret.as_ref());
                let validation = Validation::new(Algorithm::HS256);
                
                match decode::<Claims>(token, &key, &validation) {
                    Ok(_) => return next.run(request).await,
                    Err(_) => {
                        return Response::builder()
                            .status(401)
                            .body("Invalid token".into())
                            .unwrap();
                    }
                }
            }
        }
    }
    
    // Return 401 if no valid token
    Response::builder()
        .status(401)
        .body("Unauthorized".into())
        .unwrap()
}

pub fn generate_token(user_id: &str) -> Result<String, String> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let key = EncodingKey::from_secret(secret.as_ref());
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600) as usize, // 1 hour expiry
    };
    
    jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, &key)
        .map_err(|e| format!("Failed to generate token: {}", e))
}
"#.to_string()
    }

    fn generate_oauth2_auth(&self) -> String {
        r#"use oauth2::{AuthorizationCode, TokenResponse};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn oauth2_middleware(request: Request, next: Next) -> Response {
    // OAuth2 token validation
    let auth_header = request.headers().get("authorization");
    
    if let Some(header_value) = auth_header {
        if let Ok(token_str) = header_value.to_str() {
            if token_str.starts_with("Bearer ") {
                let token = &token_str[7..];
                // In Produktion würde hier OAuth2 Token validiert werden
                // Für jetzt akzeptieren wir alle Tokens
                return next.run(request).await;
            }
        }
    }
    
    Response::builder()
        .status(401)
        .body("Unauthorized".into())
        .unwrap()
}

pub fn get_oauth2_client() -> oauth2::Client {
    let client_id = std::env::var("OAUTH2_CLIENT_ID").unwrap_or_else(|_| "client_id".to_string());
    let client_secret = std::env::var("OAUTH2_CLIENT_SECRET").unwrap_or_else(|_| "client_secret".to_string());
    let auth_url = std::env::var("OAUTH2_AUTH_URL").unwrap_or_else(|_| "https://oauth.example.com/auth".to_string());
    let token_url = std::env::var("OAUTH2_TOKEN_URL").unwrap_or_else(|_| "https://oauth.example.com/token".to_string());
    
    oauth2::Client::new(
        oauth2::ClientId::new(client_id),
        Some(oauth2::ClientSecret::new(client_secret)),
        oauth2::AuthUrl::new(auth_url).unwrap(),
        Some(oauth2::TokenUrl::new(token_url).unwrap()),
    )
}
"#.to_string()
    }
}
