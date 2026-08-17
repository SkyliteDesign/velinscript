use super::{Template, TemplateConfig};
use anyhow::Result;

/// Rate Limiting Template
/// 
/// Generiert Redis-basiertes Rate Limiting
/// Mit verschiedenen Strategien
pub struct RateLimitTemplate;

impl Template for RateLimitTemplate {
    fn generate(&self, config: &TemplateConfig) -> Result<String, String> {
        let strategy = config.options
            .get("strategy")
            .and_then(|v| v.as_str())
            .unwrap_or("fixed-window");

        match strategy {
            "fixed-window" => Ok(self.generate_fixed_window()),
            "sliding-window" => Ok(self.generate_sliding_window()),
            "token-bucket" => Ok(self.generate_token_bucket()),
            _ => Ok(self.generate_fixed_window()),
        }
    }
}

impl RateLimitTemplate {
    fn generate_fixed_window(&self) -> String {
        r#"use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, (u64, SystemTime)>>>,
    max_requests: u64,
    window_seconds: u64,
}

impl RateLimiter {
    fn new(max_requests: u64, window_seconds: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_seconds,
        }
    }

    fn check(&self, key: &str) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = SystemTime::now();
        
        if let Some((count, window_start)) = requests.get(key) {
            if now.duration_since(*window_start).unwrap().as_secs() < self.window_seconds {
                if *count >= self.max_requests {
                    return false;
                }
                *requests.get_mut(key).unwrap() = (*count + 1, *window_start);
            } else {
                requests.insert(key.to_string(), (1, now));
            }
        } else {
            requests.insert(key.to_string(), (1, now));
        }
        
        true
    }
}

pub async fn rate_limit_middleware(request: Request, next: Next) -> Response {
    // Fixed Window Rate Limiting
    let limiter = RateLimiter::new(100, 60); // 100 requests per 60 seconds
    let client_ip = request.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    
    if limiter.check(client_ip) {
        next.run(request).await
    } else {
        Response::builder()
            .status(429)
            .body("Rate limit exceeded".into())
            .unwrap()
    }
}
"#.to_string()
    }

    fn generate_sliding_window(&self) -> String {
        r#"use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

struct SlidingWindowLimiter {
    requests: VecDeque<SystemTime>,
    max_requests: u64,
    window_seconds: u64,
}

impl SlidingWindowLimiter {
    fn new(max_requests: u64, window_seconds: u64) -> Self {
        Self {
            requests: VecDeque::new(),
            max_requests,
            window_seconds,
        }
    }

    fn check(&mut self) -> bool {
        let now = SystemTime::now();
        let window_start = now - Duration::from_secs(self.window_seconds);
        
        // Entferne alte Requests außerhalb des Fensters
        while let Some(&oldest) = self.requests.front() {
            if oldest < window_start {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        
        if self.requests.len() < self.max_requests as usize {
            self.requests.push_back(now);
            true
        } else {
            false
        }
    }
}
"#.to_string()
    }

    fn generate_token_bucket(&self) -> String {
        r#"use std::time::{Duration, SystemTime};

struct TokenBucket {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: SystemTime,
}

impl TokenBucket {
    fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: SystemTime::now(),
        }
    }

    fn check(&mut self, tokens_needed: f64) -> bool {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.last_refill)
            .unwrap_or(Duration::from_secs(0))
            .as_secs_f64();
        
        // Refill tokens
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;
        
        if self.tokens >= tokens_needed {
            self.tokens -= tokens_needed;
            true
        } else {
            false
        }
    }
}
"#.to_string()
    }
}
