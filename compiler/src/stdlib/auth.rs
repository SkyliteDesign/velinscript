// JWT/OAuth2/OpenID Connect Authentication & Authorization

#[cfg(feature = "oauth2")]
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};

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

#[cfg(feature = "oauth2")]
pub mod oauth2_integration {
    use super::*;
    use oauth2::{Client, AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl, Scope, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
    
    pub struct OAuth2Provider {
        pub client: Client,
    }
    
    impl OAuth2Provider {
        pub fn new(
            client_id: String,
            client_secret: String,
            auth_url: String,
            token_url: String,
            redirect_uri: String,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            let client = Client::new(
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
            
            (auth_url.to_string(), pkce_verifier)
        }
        
        pub async fn exchange_code(
            &self,
            code: AuthorizationCode,
            pkce_verifier: PkceCodeVerifier,
        ) -> Result<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>, oauth2::RequestTokenError> {
            self.client
                .exchange_code(code)
                .set_pkce_verifier(pkce_verifier)
                .request_async(oauth2::reqwest::async_http_client)
                .await
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
        // TOTP verification logic
        // TODO: In production, use a TOTP library like totp-rs
        // Aktuell: Basis-Validierung (nur Länge prüfen)
        // Zukünftig: Vollständige TOTP-Verifizierung mit Zeitfenster
        !token.is_empty() && !secret.is_empty()
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
