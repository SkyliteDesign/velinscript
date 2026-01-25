// Standard Library für Security-Funktionalität
// Security Decorators und Middleware

use crate::parser::ast::Decorator;

/// Security Standard Library
pub struct SecurityStdlib;

impl SecurityStdlib {
    /// Prüft ob ein Decorator ein Security-Decorator ist
    pub fn is_security_decorator(decorator: &Decorator) -> bool {
        matches!(
            decorator.name.as_str(),
            "Auth" | "Role" | "JWT" | "OAuth2" | "APIKey"
        )
    }

    /// Generiert Rust-Code für @Auth Decorator
    pub fn generate_auth_middleware() -> String {
        r#"#[derive(Clone)]
pub struct AuthMiddleware {
    secret: String,
}

impl AuthMiddleware {
    pub fn new(secret: String) -> Self {
        AuthMiddleware { secret }
    }
}

impl actix_web::dev::Transform<actix_web::dev::ServiceRequest> for AuthMiddleware {
    type Response = actix_web::dev::ServiceResponse;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddlewareService;
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, _service: actix_web::dev::Service) -> Self::Future {
        futures::future::ok(AuthMiddlewareService {
            secret: self.secret.clone(),
        })
    }
}

pub struct AuthMiddlewareService {
    secret: String,
}

impl actix_web::dev::Service<actix_web::dev::ServiceRequest> for AuthMiddlewareService {
    type Response = actix_web::dev::ServiceResponse;
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        use actix_web::http::header::HeaderValue;
        use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
        use serde_json::Value;
        
        // Extract Authorization header
        let auth_header = req.headers().get("Authorization");
        if auth_header.is_none() {
            return futures::future::ok(
                req.into_response(actix_web::HttpResponse::Unauthorized()
                    .json(serde_json::json!({"error": "Missing Authorization header"}))
                    .finish())
            );
        }
        
        let auth_value = auth_header.unwrap().to_str().unwrap_or("");
        if !auth_value.starts_with("Bearer ") {
            return futures::future::ok(
                req.into_response(actix_web::HttpResponse::Unauthorized()
                    .json(serde_json::json!({"error": "Invalid Authorization header format"}))
                    .finish())
            );
        }
        
        let token = &auth_value[7..]; // Skip "Bearer "
        
        // Validate JWT
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["sub", "exp"]);
        
        match decode::<Value>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                // Store user info in request extensions for use in handlers
                req.extensions_mut().insert(token_data.claims);
                futures::future::ok(req.into_response(actix_web::HttpResponse::Ok().finish()))
            }
            Err(e) => {
                futures::future::ok(
                    req.into_response(actix_web::HttpResponse::Unauthorized()
                        .json(serde_json::json!({"error": format!("JWT validation failed: {}", e)}))
                        .finish())
                )
            }
        }
    }
}"#
        .to_string()
    }

    /// Generiert Rust-Code für @Role Decorator
    #[allow(unused_variables)]
    pub fn generate_role_middleware(role: &str) -> String {
        format!(
            r#"#[derive(Clone)]
pub struct RoleMiddleware {{
    required_role: String,
}}

impl RoleMiddleware {{
    pub fn new(role: &str) -> Self {{
        RoleMiddleware {{
            required_role: role.to_string(),
        }}
    }}
}}

impl actix_web::dev::Transform<actix_web::dev::ServiceRequest> for RoleMiddleware {{
    type Response = actix_web::dev::ServiceResponse;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = RoleMiddlewareService;
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, _service: actix_web::dev::Service) -> Self::Future {{
        futures::future::ok(RoleMiddlewareService {{
            required_role: self.required_role.clone(),
        }})
    }}
}}

pub struct RoleMiddlewareService {{
    required_role: String,
}}

impl actix_web::dev::Service<actix_web::dev::ServiceRequest> for RoleMiddlewareService {{
    type Response = actix_web::dev::ServiceResponse;
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {{
        use actix_web::http::header::HeaderValue;
        use jsonwebtoken::{{decode, DecodingKey, Validation, Algorithm}};
        use serde_json::Value;
        
        // Extract Authorization header
        let auth_header = req.headers().get("Authorization");
        if auth_header.is_none() {{
            return futures::future::ok(
                req.into_response(actix_web::HttpResponse::Unauthorized()
                    .json(serde_json::json!({{"error": "Missing Authorization header"}}))
                    .finish())
            );
        }}
        
        let auth_value = auth_header.unwrap().to_str().unwrap_or("");
        if !auth_value.starts_with("Bearer ") {{
            return futures::future::ok(
                req.into_response(actix_web::HttpResponse::Unauthorized()
                    .json(serde_json::json!({{"error": "Invalid Authorization header format"}}))
                    .finish())
            );
        }}
        
        let token = &auth_value[7..]; // Skip "Bearer "
        
        // Validate JWT and extract claims
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret".to_string());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["sub", "exp"]);
        
        match decode::<Value>(token, &decoding_key, &validation) {{
            Ok(token_data) => {{
                // Check if user has required role
                let claims = &token_data.claims;
                let user_roles = claims.get("roles")
                    .and_then(|r| r.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                    .unwrap_or_default();
                
                if !user_roles.contains(&self.required_role.as_str()) {{
                    let error_msg = format!("Required role: {{}}", self.required_role);
                    return futures::future::ok(
                        req.into_response(actix_web::HttpResponse::Forbidden()
                            .json(serde_json::json!({{
                                "error": error_msg
                            }}))
                            .finish())
                    );
                }}
                
                // Store user info in request extensions
                req.extensions_mut().insert(claims.clone());
                futures::future::ok(req.into_response(actix_web::HttpResponse::Ok().finish()))
            }}
            Err(e) => {{
                let error_msg = format!("JWT validation failed: {{}}", e);
                futures::future::ok(
                    req.into_response(actix_web::HttpResponse::Unauthorized()
                        .json(serde_json::json!({{
                            "error": error_msg
                        }}))
                        .finish())
                )
            }}
        }}
    }}
}}"#
        )
    }

    /// Generiert JWT Validation Code
    pub fn generate_jwt_validation() -> String {
        r#"// JWT Validation Helper
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde_json::Value;

pub fn validate_jwt(token: &str, secret: &str) -> Result<Value, String> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["sub", "exp"]);
    
    match decode::<Value>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(format!("JWT validation failed: {}", e)),
    }
}

pub fn validate_jwt_with_public_key(token: &str, public_key: &str) -> Result<Value, String> {
    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())
        .map_err(|e| format!("Failed to parse public key: {}", e))?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["sub", "exp"]);
    
    match decode::<Value>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(format!("JWT validation failed: {}", e)),
    }
}"#
        .to_string()
    }
}
