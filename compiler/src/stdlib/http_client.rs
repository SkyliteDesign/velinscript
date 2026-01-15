// HTTP Client Standard Library
// Client-seitige HTTP-Funktionalität für API-Aufrufe

pub struct HttpClientStdlib;

impl HttpClientStdlib {
    /// Generiert Rust-Code für HttpClient.new()
    pub fn generate_http_client_new() -> String {
        "reqwest::Client::new()".to_string()
    }
    
    /// Generiert Rust-Code für client.get(url)
    pub fn generate_get(url: &str, headers: Option<&str>) -> String {
        if let Some(headers_expr) = headers {
            format!(
                "reqwest::Client::new().get({}).headers({}).send().await",
                url, headers_expr
            )
        } else {
            format!(
                "reqwest::Client::new().get({}).send().await",
                url
            )
        }
    }
    
    /// Generiert Rust-Code für client.post(url, body)
    pub fn generate_post(url: &str, body: Option<&str>, headers: Option<&str>) -> String {
        let mut code = format!("reqwest::Client::new().post({})", url);
        
        if let Some(body_expr) = body {
            code.push_str(&format!(".json(&{})", body_expr));
        }
        
        if let Some(headers_expr) = headers {
            code.push_str(&format!(".headers({})", headers_expr));
        }
        
        code.push_str(".send().await");
        code
    }
    
    /// Generiert Rust-Code für client.put(url, body)
    pub fn generate_put(url: &str, body: Option<&str>, headers: Option<&str>) -> String {
        let mut code = format!("reqwest::Client::new().put({})", url);
        
        if let Some(body_expr) = body {
            code.push_str(&format!(".json(&{})", body_expr));
        }
        
        if let Some(headers_expr) = headers {
            code.push_str(&format!(".headers({})", headers_expr));
        }
        
        code.push_str(".send().await");
        code
    }
    
    /// Generiert Rust-Code für client.delete(url)
    pub fn generate_delete(url: &str, headers: Option<&str>) -> String {
        if let Some(headers_expr) = headers {
            format!(
                "reqwest::Client::new().delete({}).headers({}).send().await",
                url, headers_expr
            )
        } else {
            format!(
                "reqwest::Client::new().delete({}).send().await",
                url
            )
        }
    }
    
    /// Generiert Rust-Code für client.patch(url, body)
    pub fn generate_patch(url: &str, body: Option<&str>, headers: Option<&str>) -> String {
        let mut code = format!("reqwest::Client::new().patch({})", url);
        
        if let Some(body_expr) = body {
            code.push_str(&format!(".json(&{})", body_expr));
        }
        
        if let Some(headers_expr) = headers {
            code.push_str(&format!(".headers({})", headers_expr));
        }
        
        code.push_str(".send().await");
        code
    }
    
    /// Generiert Rust-Code für response.json()
    pub fn generate_response_json(response: &str) -> String {
        format!("{}.json::<serde_json::Value>().await", response)
    }
    
    /// Generiert Rust-Code für response.text()
    pub fn generate_response_text(response: &str) -> String {
        format!("{}.text().await", response)
    }
    
    /// Generiert Rust-Code für response.status()
    pub fn generate_response_status(response: &str) -> String {
        format!("{}.status()", response)
    }
    
    /// Generiert Rust-Code für Error Handling mit Retry
    pub fn generate_error_handling_with_retry(
        request: &str,
        max_retries: usize,
        retry_delay_ms: usize,
    ) -> String {
        format!(
            r#"
            {{
                let mut retries = 0;
                loop {{
                    match {}.await {{
                        Ok(response) => break Ok(response),
                        Err(e) => {{
                            if retries >= {} {{
                                break Err(anyhow::anyhow!("Request failed after {} retries: {{:?}}", e));
                            }}
                            retries += 1;
                            tokio::time::sleep(tokio::time::Duration::from_millis({})).await;
                        }}
                    }}
                }}
            }}"#,
            request, max_retries, max_retries, retry_delay_ms
        )
    }
    
    /// Generiert Rust-Code für Request Headers
    pub fn generate_headers(headers_map: &str) -> String {
        format!(
            r#"
            {{
                let mut header_map = reqwest::header::HeaderMap::new();
                for (key, value) in {}.iter() {{
                    if let (Ok(header_name), Ok(header_value)) = (
                        reqwest::header::HeaderName::from_bytes(key.as_bytes()),
                        reqwest::header::HeaderValue::from_str(value)
                    ) {{
                        header_map.insert(header_name, header_value);
                    }}
                }}
                header_map
            }}"#,
            headers_map
        )
    }
}

/// Prüft ob ein HTTP-Client-Methoden-Aufruf vorliegt
pub fn is_http_client_method(method: &str) -> bool {
    matches!(
        method,
        "get" | "post" | "put" | "delete" | "patch" | "json" | "text" | "status"
    )
}
