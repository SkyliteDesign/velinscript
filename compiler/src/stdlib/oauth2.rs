// OAuth2/OpenID Connect Standard Library
// Generiert OAuth2/OIDC Code für Authentication & Authorization

use crate::parser::ast::*;

pub struct OAuth2Stdlib;

impl OAuth2Stdlib {
    /// Generiert OAuth2 Client Setup
    pub fn generate_oauth2_client(
        client_id: &str,
        client_secret: &str,
        auth_url: &str,
        token_url: &str,
    ) -> String {
        format!(
            r#"use oauth2::{{Client, AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl, Scope}};

pub fn create_oauth2_client() -> Client {{
    Client::new(
        ClientId::new("{}".to_string()),
        Some(ClientSecret::new("{}".to_string())),
        AuthUrl::new("{}".to_string()).unwrap(),
        Some(TokenUrl::new("{}".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080/callback".to_string()).unwrap())
}}"#,
            client_id, client_secret, auth_url, token_url
        )
    }

    /// Generiert OAuth2 Authorization Code Flow
    pub fn generate_authorization_code_flow() -> String {
        r#"use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope};

pub async fn start_oauth2_flow(client: &Client) -> (String, PkceCodeVerifier) {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();
    
    (auth_url.to_string(), pkce_verifier)
}

pub async fn exchange_code(
    client: &Client,
    code: AuthorizationCode,
    pkce_verifier: PkceCodeVerifier,
) -> Result<TokenResponse, oauth2::RequestTokenError> {
    client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await
}"#.to_string()
    }

    /// Generiert OpenID Connect Discovery
    pub fn generate_oidc_discovery(issuer_url: &str) -> String {
        format!(
            r#"use openidconnect::{IssuerUrl, ClientId, ClientSecret, RedirectUrl, reqwest::async_http_client};
use openidconnect::core::{{CoreClient, CoreProviderMetadata}};

pub async fn discover_oidc_provider() -> Result<CoreClient, Box<dyn std::error::Error>> {{
    let issuer = IssuerUrl::new("{}".to_string())?;
    let provider_metadata = CoreProviderMetadata::discover_async(issuer, async_http_client).await?;
    
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080/callback".to_string())?);
    
    Ok(client)
}}"#,
            issuer_url
        )
    }

    /// Generiert OAuth2 Middleware für Axum
    pub fn generate_axum_oauth2_middleware() -> String {
        r#"use axum::{extract::Request, middleware::Next, response::Response};
use axum::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

pub async fn oauth2_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let token = &auth_header[7..];
    
    // Validate JWT token
    let decoding_key = DecodingKey::from_secret(b"secret");
    let validation = Validation::new(Algorithm::HS256);
    
    match decode::<serde_json::Value>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            // Extract claims and add to request extensions
            req.extensions_mut().insert(token_data.claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}"#.to_string()
    }

    /// Generiert OAuth2 Middleware für Actix
    pub fn generate_actix_oauth2_middleware() -> String {
        r#"use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::dev::{Service, Transform};
use actix_web::web::Data;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

pub struct OAuth2Middleware;

impl<S, B> Transform<S, ServiceRequest> for OAuth2Middleware
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = OAuth2MiddlewareService<S>;
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(OAuth2MiddlewareService { service })
    }
}

pub struct OAuth2MiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for OAuth2MiddlewareService<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Future = futures::future::LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());
        
        if let Some(header) = auth_header {
            if header.starts_with("Bearer ") {
                let token = &header[7..];
                let decoding_key = DecodingKey::from_secret(b"secret");
                let validation = Validation::new(Algorithm::HS256);
                
                if let Ok(token_data) = decode::<serde_json::Value>(token, &decoding_key, &validation) {
                    req.extensions_mut().insert(token_data.claims);
                }
            }
        }
        
        let fut = self.service.call(req);
        Box::pin(async move {
            fut.await
        })
    }
}"#.to_string()
    }

    /// Generiert MFA (Multi-Factor Authentication) Support
    pub fn generate_mfa_support() -> String {
        r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MFAToken {
    pub user_id: String,
    pub mfa_verified: bool,
    pub mfa_method: String, // "totp", "sms", "email"
}

pub fn verify_totp(token: &str, secret: &str) -> bool {
    // TOTP verification logic
    // In production, use a TOTP library like totp-rs
    true
}

pub fn verify_sms_code(code: &str, expected: &str) -> bool {
    code == expected
}

pub fn verify_email_code(code: &str, expected: &str) -> bool {
    code == expected
}"#.to_string()
    }

    /// Generiert OAuth2 Token Validation
    pub fn generate_token_validation() -> String {
        r#"use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub roles: Vec<String>,
    pub exp: usize,
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["sub", "exp"]);
    
    decode::<Claims>(token, &decoding_key, &validation)
}

pub fn extract_user_id(token: &str, secret: &[u8]) -> Option<String> {
    validate_token(token, secret)
        .ok()
        .map(|data| data.claims.sub)
}"#.to_string()
    }

    /// Prüft ob ein Decorator ein OAuth2-Decorator ist
    pub fn is_oauth2_decorator(decorator: &Decorator) -> bool {
        matches!(
            decorator.name.as_str(),
            "OAuth2" | "@OAuth2" | "OIDC" | "@OIDC" | "MFA" | "@MFA"
        )
    }
}
