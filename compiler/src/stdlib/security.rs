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
pub struct AuthMiddleware;

impl actix_web::dev::Transform<actix_web::dev::ServiceRequest> for AuthMiddleware {
    type Response = actix_web::dev::ServiceResponse;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddlewareService;
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, _service: actix_web::dev::Service) -> Self::Future {
        futures::future::ok(AuthMiddlewareService)
    }
}

pub struct AuthMiddlewareService;

impl actix_web::dev::Service<actix_web::dev::ServiceRequest> for AuthMiddlewareService {
    type Response = actix_web::dev::ServiceResponse;
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        // Authentication check implementation
        // Check for Authorization header, validate JWT, etc.
        futures::future::ok(req.into_response(actix_web::HttpResponse::Ok().finish()))
    }
}"#.to_string()
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
        // Check user role from JWT or session
        // if user.role != self.required_role {{
        //     return Err(actix_web::Error::from(actix_web::HttpResponse::Forbidden()));
        // }}
        futures::future::ok(req.into_response(actix_web::HttpResponse::Ok().finish()))
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
}"#.to_string()
    }
}
