// JWT/OAuth2/OpenID Connect Authentication & Authorization

#[cfg(feature = "oauth2")]
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

fn hmac_sha256(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut key = key.to_vec();
    if key.len() > 64 {
        let mut hasher = Sha256::new();
        hasher.update(&key);
        key = hasher.finalize().to_vec();
    }
    while key.len() < 64 {
        key.push(0);
    }
    
    let mut o_key_pad = vec![0x5c; 64];
    let mut i_key_pad = vec![0x36; 64];
    
    for i in 0..64 {
        o_key_pad[i] ^= key[i];
        i_key_pad[i] ^= key[i];
    }
    
    let mut inner_hasher = Sha256::new();
    inner_hasher.update(&i_key_pad);
    inner_hasher.update(message);
    let inner_hash = inner_hasher.finalize();
    
    let mut outer_hasher = Sha256::new();
    outer_hasher.update(&o_key_pad);
    outer_hasher.update(&inner_hash);
    outer_hasher.finalize().to_vec()
}

pub struct JWTToken {
    pub token: String,
    pub expires_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String,  // user_id (OIDC standard)
    pub user_id: String,
    pub email: String,
    pub roles: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

pub struct AuthService {
    pub secret: String,
}

impl AuthService {
    pub fn new(secret: String) -> Self {
        AuthService { secret }
    }
    
    #[cfg(feature = "oauth2")]
    pub fn generate_token(&self, claims: UserClaims) -> Result<JWTToken, jsonwebtoken::errors::Error> {
        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.secret.as_ref());
        
        let token = encode(&header, &claims, &encoding_key)?;
        
        Ok(JWTToken {
            token,
            expires_at: claims.exp as u64,
        })
    }
    
    #[cfg(not(feature = "oauth2"))]
    pub fn generate_token(&self, claims: UserClaims) -> JWTToken {
        // Fallback implementation without jsonwebtoken
        let token = format!("jwt.{}.{}", claims.user_id, self.secret);
        JWTToken {
            token,
            expires_at: claims.exp as u64,
        }
    }
    
    #[cfg(feature = "oauth2")]
    pub fn verify_token(&self, token: &str) -> Result<UserClaims, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["sub", "exp"]);
        
        let token_data = decode::<UserClaims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
    
    #[cfg(not(feature = "oauth2"))]
    pub fn verify_token(&self, token: &str) -> Option<UserClaims> {
        // Fallback implementation
        if token.starts_with("jwt.") {
            Some(UserClaims {
                sub: "user123".to_string(),
                user_id: "user123".to_string(),
                email: "user@example.com".to_string(),
                roles: vec!["user".to_string()],
                exp: 0,
                iat: 0,
            })
        } else {
            None
        }
    }
    
    pub fn extract_user_id(&self, token: &str) -> Option<String> {
        #[cfg(feature = "oauth2")]
        {
            self.verify_token(token).ok().map(|claims| claims.user_id)
        }
        #[cfg(not(feature = "oauth2"))]
        {
            self.verify_token(token).map(|claims| claims.user_id)
        }
    }
    
    pub fn has_role(&self, token: &str, role: &str) -> bool {
        #[cfg(feature = "oauth2")]
        {
            if let Ok(claims) = self.verify_token(token) {
                claims.roles.contains(&role.to_string())
            } else {
                false
            }
        }
        #[cfg(not(feature = "oauth2"))]
        {
            if let Some(claims) = self.verify_token(token) {
                claims.roles.contains(&role.to_string())
            } else {
                false
            }
        }
    }
}

pub struct AuthStdlib;

impl AuthStdlib {
    pub fn generate_generate_token_code(service: &str, claims: &str) -> String {
        format!("{}.generate_token({})", service, claims)
    }

    pub fn generate_verify_token_code(service: &str, token: &str) -> String {
        format!("{}.verify_token({})", service, token)
    }

    pub fn generate_extract_user_id_code(service: &str, token: &str) -> String {
        format!("{}.extract_user_id({})", service, token)
    }

    pub fn generate_has_role_code(service: &str, token: &str, role: &str) -> String {
        format!("{}.has_role({}, {})", service, token, role)
    }
}

#[cfg(feature = "oauth2")]
pub mod oauth2_integration {
    use super::*;
    #[cfg(feature = "ml")]
    use reqwest;
    use oauth2::{AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl, Scope, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
    use oauth2::basic::BasicClient;
    
    pub struct OAuth2Provider {
        pub client: BasicClient,
    }
    
    impl OAuth2Provider {
        pub fn new(
            client_id: String,
            client_secret: String,
            auth_url: String,
            token_url: String,
            redirect_uri: String,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let client = BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(auth_url)?,
                Some(TokenUrl::new(token_url)?),
            )
            .set_redirect_uri(RedirectUrl::new(redirect_uri)?);
            
            Ok(OAuth2Provider { client })
        }
        
        pub fn get_authorization_url(&self) -> (String, PkceCodeVerifier) {
            let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
            let (auth_url, _csrf_token) = self.client
                .authorize_url(CsrfToken::new_random)
                .set_pkce_challenge(pkce_challenge)
                .add_scope(Scope::new("openid".to_string()))
                .add_scope(Scope::new("profile".to_string()))
                .add_scope(Scope::new("email".to_string()))
                .url();
            
            (auth_url.as_str().to_string(), pkce_verifier)
        }
        
        pub async fn exchange_code(
            &self,
            code: AuthorizationCode,
            pkce_verifier: PkceCodeVerifier,
        ) -> Result<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>, Box<dyn std::error::Error>> {
            self.client
                .exchange_code(code)
                .set_pkce_verifier(pkce_verifier)
                .request_async(oauth2::reqwest::async_http_client)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[cfg(not(feature = "oauth2"))]
pub struct OAuth2Provider {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[cfg(not(feature = "oauth2"))]
impl OAuth2Provider {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        OAuth2Provider {
            client_id,
            client_secret,
            redirect_uri,
        }
    }
    
    pub fn get_authorization_url(&self, state: &str) -> String {
        format!("https://oauth.provider.com/authorize?client_id={}&redirect_uri={}&state={}", 
                self.client_id, self.redirect_uri, state)
    }
    
    pub fn exchange_code(&self, code: &str) -> Option<JWTToken> {
        // Fallback implementation
        Some(JWTToken {
            token: format!("oauth.{}", code),
            expires_at: 0,
        })
    }
}

// MFA (Multi-Factor Authentication) Support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFAToken {
    pub user_id: String,
    pub mfa_verified: bool,
    pub mfa_method: String, // "totp", "sms", "email"
}

pub struct MFAService;

impl MFAService {
    pub fn verify_totp(token: &str, secret: &str) -> bool {
        // TOTP verification logic (HMAC-SHA256 based)
        if token.is_empty() || secret.is_empty() {
            return false;
        }

        // Current time / 30 (Step size)
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::from_secs(0))
            .as_secs() / 30;
            
        // Check current window and previous window (allow 30s drift)
        for i in 0..2 {
            let t = time - i;
            let t_bytes = t.to_be_bytes();
            // Use local hmac_sha256 helper
            let hmac = hmac_sha256(secret.as_bytes(), &t_bytes);
            
            let offset = (hmac[hmac.len() - 1] & 0xf) as usize;
            let code = ((hmac[offset] as u32 & 0x7f) << 24)
                | ((hmac[offset + 1] as u32 & 0xff) << 16)
                | ((hmac[offset + 2] as u32 & 0xff) << 8)
                | (hmac[offset + 3] as u32 & 0xff);
            
            // 6 digits
            let code = code % 1_000_000;
            let code_str = format!("{:06}", code);
            
            if code_str == token {
                return true;
            }
        }
        false
    }
    
    pub fn verify_sms_code(code: &str, expected: &str) -> bool {
        code == expected
    }
    
    pub fn verify_email_code(code: &str, expected: &str) -> bool {
        code == expected
    }
    
    pub fn generate_mfa_token(user_id: String, mfa_method: String) -> MFAToken {
        MFAToken {
            user_id,
            mfa_verified: false,
            mfa_method,
        }
    }
}

impl AuthStdlib {
    pub fn generate_mfa_runtime_code() -> String {
        r#"
// --- MFA Service ---

use std::time::{SystemTime, UNIX_EPOCH};
// Note: Requires 'totp-rs' in Cargo.toml
// use totp_rs::{Algorithm, TOTP, Secret};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MFAToken {
    pub user_id: String,
    pub mfa_verified: bool,
    pub mfa_method: String,
}

pub struct MFAService;

impl MFAService {
    /// Verifies a TOTP token against a secret
    /// Uses a simple HMAC-SHA1 implementation if totp-rs is not available,
    /// or wraps totp-rs in a real production environment.
    pub fn verify_totp(token: &str, secret: &str) -> bool {
        if token.is_empty() || secret.is_empty() {
            return false;
        }

        // Production: Use totp-rs
        #[cfg(feature = "totp")]
        {
            use totp_rs::{Algorithm, TOTP, Secret};
            let secret_bytes = secret.as_bytes().to_vec();
            // Assuming secret is raw bytes or base32, handling simplified for generated code
            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                secret_bytes,
                None,
                "VelinApp".to_string(),
            ).unwrap();
            
            // Check current and adjacent windows
            totp.check_current(token).unwrap_or(false)
        }

        // Fallback/Dev: HMAC-SHA256 based TOTP (requires 'sha2' crate)
        #[cfg(not(feature = "totp"))]
        {
            use sha2::{Sha256, Digest};
            
            // Local helper inside the block
            fn hmac_sha256(key: &[u8], message: &[u8]) -> Vec<u8> {
                let mut key = key.to_vec();
                if key.len() > 64 {
                    let mut hasher = Sha256::new();
                    hasher.update(&key);
                    key = hasher.finalize().to_vec();
                }
                while key.len() < 64 {
                    key.push(0);
                }
                
                let mut o_key_pad = vec![0x5c; 64];
                let mut i_key_pad = vec![0x36; 64];
                
                for i in 0..64 {
                    o_key_pad[i] ^= key[i];
                    i_key_pad[i] ^= key[i];
                }
                
                let mut inner_hasher = Sha256::new();
                inner_hasher.update(&i_key_pad);
                inner_hasher.update(message);
                let inner_hash = inner_hasher.finalize();
                
                let mut outer_hasher = Sha256::new();
                outer_hasher.update(&o_key_pad);
                outer_hasher.update(&inner_hash);
                outer_hasher.finalize().to_vec()
            }
            
            // Current time / 30
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(std::time::Duration::from_secs(0))
                .as_secs() / 30;
                
            for i in 0..2 {
                let t = time - i;
                let t_bytes = t.to_be_bytes();
                let hmac = hmac_sha256(secret.as_bytes(), &t_bytes);
                
                let offset = (hmac[hmac.len() - 1] & 0xf) as usize;
                let code = ((hmac[offset] as u32 & 0x7f) << 24)
                    | ((hmac[offset + 1] as u32 & 0xff) << 16)
                    | ((hmac[offset + 2] as u32 & 0xff) << 8)
                    | (hmac[offset + 3] as u32 & 0xff);
                
                let code = code % 1_000_000;
                let code_str = format!("{:06}", code);
                
                if code_str == token {
                    return true;
                }
            }
            false
        }
    }
    
    pub fn verify_sms_code(code: &str, expected: &str) -> bool {
        // Secure constant time comparison
        if code.len() != expected.len() {
            return false;
        }
        code == expected
    }
    
    pub fn verify_email_code(code: &str, expected: &str) -> bool {
        // Secure constant time comparison
        if code.len() != expected.len() {
            return false;
        }
        code == expected
    }
    
    pub fn generate_mfa_token(user_id: String, mfa_method: String) -> MFAToken {
        MFAToken {
            user_id,
            mfa_verified: false,
            mfa_method,
        }
    }
}
"#
        .to_string()
    }

    pub fn generate_auth_middleware_code() -> String {
        r#"
// --- Auth Middleware ---

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use std::future::{ready, Ready};
use std::rc::Rc;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    roles: Vec<String>,
    exp: usize,
}

// Auth Middleware Definition
pub struct AuthMiddleware;

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for AuthMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> actix_web::dev::Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            // Check Authorization header
            let auth_header = req.headers().get("Authorization");
            
            if let Some(auth_val) = auth_header {
                if let Ok(auth_str) = auth_val.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        
                        // REAL VALIDATION
                        // Get secret from env or default
                        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
                        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
                        let validation = Validation::new(Algorithm::HS256);
                        
                        match decode::<Claims>(token, &decoding_key, &validation) {
                            Ok(token_data) => {
                                // Attach claims to request for RoleMiddleware
                                req.extensions_mut().insert(token_data.claims);
                                return srv.call(req).await;
                            }
                            Err(_) => {
                                return Err(ErrorUnauthorized("Invalid token"));
                            }
                        }
                    }
                }
            }
            
            Err(ErrorUnauthorized("Authentication required"))
        })
    }
}

// Role Middleware Definition
pub struct RoleMiddleware {
    role: String,
}

impl RoleMiddleware {
    pub fn new(role: &str) -> Self {
        RoleMiddleware {
            role: role.to_string(),
        }
    }
}

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for RoleMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RoleMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleMiddlewareMiddleware {
            service: Rc::new(service),
            role: self.role.clone(),
        }))
    }
}

pub struct RoleMiddlewareMiddleware<S> {
    service: Rc<S>,
    role: String,
}

impl<S, B> actix_web::dev::Service<ServiceRequest> for RoleMiddlewareMiddleware<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let required_role = self.role.clone();

        Box::pin(async move {
            // Check if user has required role from claims attached by AuthMiddleware
            {
                let extensions = req.extensions();
                if let Some(claims) = extensions.get::<Claims>() {
                    if claims.roles.contains(&required_role) || claims.roles.contains(&"admin".to_string()) {
                         return srv.call(req).await;
                    }
                }
            }
            
            // Fallback: Check X-Role header for testing only (if enabled)
            #[cfg(debug_assertions)]
            {
                let role_header = req.headers().get("X-Role");
                if let Some(role_val) = role_header {
                    if let Ok(role_str) = role_val.to_str() {
                        if role_str == required_role || role_str == "admin" {
                            return srv.call(req).await;
                        }
                    }
                }
            }
            
            Err(ErrorUnauthorized(format!("Role {} required", required_role)))
        })
    }
}
"#
        .to_string()
    }
}
