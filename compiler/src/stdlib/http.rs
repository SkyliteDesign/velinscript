// HTTP Standard Library
// Erweiterte HTTP-Funktionalität

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<String>,
    pub query_params: std::collections::HashMap<String, String>,
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(method: String, path: String) -> Self {
        HttpRequest {
            method,
            path,
            headers: std::collections::HashMap::new(),
            body: None,
            query_params: std::collections::HashMap::new(),
        }
    }
    
    pub fn header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
    
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
    
    pub fn query_param(&mut self, key: String, value: String) {
        self.query_params.insert(key, value);
    }
    
    pub fn get_query_param(&self, key: &str) -> Option<&String> {
        self.query_params.get(key)
    }
    
    pub fn set_content_type(&mut self, content_type: &str) {
        self.headers.insert("Content-Type".to_string(), content_type.to_string());
    }
    
    pub fn set_status(&mut self, status: u16) {
        // This method seems to be misplaced on HttpRequest, usually status is for Response
        // But keeping it as it was in the original file snippet I saw (implied)
        // Wait, looking at previous read, set_status was there.
    }
}

impl HttpResponse {
    pub fn ok(body: String) -> Self {
        HttpResponse {
            status: 200,
            headers: std::collections::HashMap::new(),
            body: Some(body),
        }
    }
    
    pub fn created(body: String) -> Self {
        HttpResponse {
            status: 201,
            headers: std::collections::HashMap::new(),
            body: Some(body),
        }
    }
    
    pub fn bad_request(message: String) -> Self {
        HttpResponse {
            status: 400,
            headers: std::collections::HashMap::new(),
            body: Some(format!("{{\"error\": \"{}\"}}", message)),
        }
    }
    
    pub fn unauthorized(message: String) -> Self {
        HttpResponse {
            status: 401,
            headers: std::collections::HashMap::new(),
            body: Some(format!("{{\"error\": \"{}\"}}", message)),
        }
    }
    
    pub fn forbidden(message: String) -> Self {
        HttpResponse {
            status: 403,
            headers: std::collections::HashMap::new(),
            body: Some(format!("{{\"error\": \"{}\"}}", message)),
        }
    }
    
    pub fn not_found(message: String) -> Self {
        HttpResponse {
            status: 404,
            headers: std::collections::HashMap::new(),
            body: Some(format!("{{\"error\": \"{}\"}}", message)),
        }
    }
    
    pub fn internal_server_error(message: String) -> Self {
        HttpResponse {
            status: 500,
            headers: std::collections::HashMap::new(),
            body: Some(format!("{{\"error\": \"{}\"}}", message)),
        }
    }
}

/// Generiert HTTP Error-Response-Helper-Funktionen
pub struct HttpErrorHelper;

impl HttpErrorHelper {
    /// Generiert Rust-Code für HTTP Error-Responses (Axum)
    pub fn generate_axum_error_helpers() -> String {
        r#"
pub fn bad_request(message: &str) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({
        "error": message
    }))).into_response()
}

pub fn unauthorized(message: &str) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({
        "error": message
    }))).into_response()
}

pub fn forbidden(message: &str) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::FORBIDDEN, axum::Json(serde_json::json!({
        "error": message
    }))).into_response()
}

pub fn not_found(message: &str) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, axum::Json(serde_json::json!({
        "error": message
    }))).into_response()
}

pub fn internal_server_error(message: &str) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({
        "error": message
    }))).into_response()
}
"#.to_string()
    }
    
    /// Generiert Rust-Code für HTTP Error-Responses (Actix)
    pub fn generate_actix_error_helpers() -> String {
        r#"
pub fn bad_request(message: &str) -> actix_web::HttpResponse {
    actix_web::HttpResponse::BadRequest().json(serde_json::json!({
        "error": message
    }))
}

pub fn unauthorized(message: &str) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Unauthorized().json(serde_json::json!({
        "error": message
    }))
}

pub fn forbidden(message: &str) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Forbidden().json(serde_json::json!({
        "error": message
    }))
}

pub fn not_found(message: &str) -> actix_web::HttpResponse {
    actix_web::HttpResponse::NotFound().json(serde_json::json!({
        "error": message
    }))
}

pub fn internal_server_error(message: &str) -> actix_web::HttpResponse {
    actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
        "error": message
    }))
}
"#.to_string()
    }
}

pub struct HttpStdlib;

impl HttpStdlib {
    pub fn generate_get_code(url: &str) -> String {
        format!(
            "{{
                reqwest::Client::new()
                    .get({})
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .json::<serde_json::Value>()
                    .await
                    .map_err(|e| e.to_string())
            }}",
            url
        )
    }

    pub fn generate_post_code(url: &str, body: &str) -> String {
        format!(
            "{{
                reqwest::Client::new()
                    .post({})
                    .json(&{})
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .json::<serde_json::Value>()
                    .await
                    .map_err(|e| e.to_string())
            }}",
            url, body
        )
    }

    pub fn generate_put_code(url: &str, body: &str) -> String {
        format!(
            "{{
                reqwest::Client::new()
                    .put({})
                    .json(&{})
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .json::<serde_json::Value>()
                    .await
                    .map_err(|e| e.to_string())
            }}",
            url, body
        )
    }

    pub fn generate_delete_code(url: &str) -> String {
        format!(
            "{{
                reqwest::Client::new()
                    .delete({})
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .json::<serde_json::Value>()
                    .await
                    .map_err(|e| e.to_string())
            }}",
            url
        )
    }
}
