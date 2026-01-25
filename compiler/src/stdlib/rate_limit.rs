// Rate Limiting Standard Library (erweitert)
// Verschiedene Rate-Limiting-Strategien und Distributed Rate Limiting

use crate::parser::ast::*;

pub enum RateLimitStrategy {
    FixedWindow,
    SlidingWindow,
    TokenBucket,
}

pub struct RateLimitConfig {
    pub requests: usize,
    pub window: String, // "1m", "1h", etc.
    pub strategy: RateLimitStrategy,
    pub key: Option<String>, // Custom key function
}

impl RateLimitConfig {
    pub fn parse_window(window: &str) -> u64 {
        // Parse window string like "1m", "1h", "60s" to seconds
        if window.ends_with('s') {
            window[..window.len() - 1].parse().unwrap_or(60)
        } else if window.ends_with('m') {
            window[..window.len() - 1].parse::<u64>().unwrap_or(1) * 60
        } else if window.ends_with('h') {
            window[..window.len() - 1].parse::<u64>().unwrap_or(1) * 3600
        } else {
            60 // Default: 60 seconds
        }
    }
}

pub struct RateLimitStdlib;

impl RateLimitStdlib {
    /// Generiert Rust-Code für Fixed Window Rate Limiting
    pub fn generate_fixed_window(requests: usize, window_seconds: u64, key: &str) -> String {
        format!(
            r#"
            {{
                use std::collections::HashMap;
                use std::sync::Arc;
                use tokio::sync::RwLock;
                use std::time::{{Duration, Instant}};
                
                static RATE_LIMIT_STORE: once_cell::sync::Lazy<Arc<RwLock<HashMap<String, (usize, Instant)>>>> = 
                    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));
                
                let key = {};
                let mut store = RATE_LIMIT_STORE.write().await;
                let now = Instant::now();
                
                if let Some((count, window_start)) = store.get_mut(&key) {{
                    if now.duration_since(*window_start) >= Duration::from_secs({}) {{
                        // Window expired, reset
                        *count = 1;
                        *window_start = now;
                        true
                    }} else if *count >= {} {{
                        false
                    }} else {{
                        *count += 1;
                        true
                    }}
                }} else {{
                    store.insert(key.clone(), (1, now));
                    true
                }}
            }}"#,
            key, window_seconds, requests
        )
    }

    /// Generiert Rust-Code für Sliding Window Rate Limiting
    pub fn generate_sliding_window(requests: usize, window_seconds: u64, key: &str) -> String {
        format!(
            r#"
            {{
                use std::collections::VecDeque;
                use std::sync::Arc;
                use tokio::sync::RwLock;
                use std::time::{{Duration, Instant}};
                
                static RATE_LIMIT_STORE: once_cell::sync::Lazy<Arc<RwLock<HashMap<String, VecDeque<Instant>>>>> = 
                    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));
                
                let key = {};
                let mut store = RATE_LIMIT_STORE.write().await;
                let now = Instant::now();
                let window = Duration::from_secs({});
                
                let timestamps = store.entry(key.clone()).or_insert_with(VecDeque::new);
                
                // Remove old timestamps outside the window
                while let Some(&oldest) = timestamps.front() {{
                    if now.duration_since(oldest) > window {{
                        timestamps.pop_front();
                    }} else {{
                        break;
                    }}
                }}
                
                if timestamps.len() >= {} {{
                    false
                }} else {{
                    timestamps.push_back(now);
                    true
                }}
            }}"#,
            key, window_seconds, requests
        )
    }

    /// Generiert Rust-Code für Token Bucket Rate Limiting
    pub fn generate_token_bucket(
        capacity: usize,
        refill_rate: f64, // tokens per second
        key: &str,
    ) -> String {
        format!(
            r#"
            {{
                use std::sync::Arc;
                use tokio::sync::RwLock;
                use std::time::{{Duration, Instant}};
                
                struct TokenBucket {{
                    tokens: f64,
                    capacity: usize,
                    refill_rate: f64,
                    last_refill: Instant,
                }}
                
                static RATE_LIMIT_STORE: once_cell::sync::Lazy<Arc<RwLock<HashMap<String, TokenBucket>>>> = 
                    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));
                
                let key = {};
                let mut store = RATE_LIMIT_STORE.write().await;
                let now = Instant::now();
                
                let bucket = store.entry(key.clone()).or_insert_with(|| TokenBucket {{
                    tokens: {} as f64,
                    capacity: {},
                    refill_rate: {},
                    last_refill: now,
                }});
                
                // Refill tokens
                let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
                bucket.tokens = (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.capacity as f64);
                bucket.last_refill = now;
                
                if bucket.tokens >= 1.0 {{
                    bucket.tokens -= 1.0;
                    true
                }} else {{
                    false
                }}
            }}"#,
            key, capacity, capacity, refill_rate
        )
    }

    /// Generiert Rust-Code für Distributed Rate Limiting (Redis-basiert)
    pub fn generate_distributed(requests: usize, window_seconds: u64, key: &str) -> String {
        format!(
            r#"
            {{
                use redis::Commands;
                
                let key = format!("rate_limit:{{}}", {});
                let client = redis::Client::open("redis://127.0.0.1/").unwrap();
                let mut con = client.get_connection().unwrap();
                
                let count: usize = con.incr(&key, 1).unwrap_or(0);
                
                if count == 1 {{
                    let _: () = con.expire(&key, {}).unwrap_or(());
                }}
                
                count <= {}
            }}"#,
            key, window_seconds, requests
        )
    }

    /// Generiert Rust-Code für Rate Limit Headers
    pub fn generate_rate_limit_headers(remaining: usize, reset: u64) -> String {
        format!(
            r#"
            headers.insert("X-RateLimit-Remaining", "{}".parse().unwrap());
            headers.insert("X-RateLimit-Reset", "{}".parse().unwrap());
            headers.insert("X-RateLimit-Limit", "{}".parse().unwrap());
            "#,
            remaining,
            reset,
            remaining + 1
        )
    }
}

/// Prüft ob ein Decorator ein RateLimit-Decorator ist
pub fn is_rate_limit_decorator(decorator_name: &str) -> bool {
    decorator_name == "RateLimit" || decorator_name == "rateLimit" || decorator_name == "rate_limit"
}

/// Extrahiert RateLimit-Konfiguration aus Decorator-Argumenten
pub fn parse_rate_limit_config(args: &[DecoratorArg]) -> Option<RateLimitConfig> {
    let mut requests = 100;
    let mut window = "1m".to_string();
    let mut strategy = RateLimitStrategy::FixedWindow;
    let mut key = None;

    for arg in args {
        match arg {
            DecoratorArg::Named { name, value } => match name.as_str() {
                "requests" => {
                    if let DecoratorArg::Number(n) = value.as_ref() {
                        requests = *n as usize;
                    }
                }
                "window" => {
                    if let DecoratorArg::String(s) = value.as_ref() {
                        window = s.clone();
                    }
                }
                "strategy" => {
                    if let DecoratorArg::String(s) = value.as_ref() {
                        strategy = match s.as_str() {
                            "sliding-window" | "slidingWindow" => RateLimitStrategy::SlidingWindow,
                            "token-bucket" | "tokenBucket" => RateLimitStrategy::TokenBucket,
                            _ => RateLimitStrategy::FixedWindow,
                        };
                    }
                }
                "key" => {
                    if let DecoratorArg::String(s) = value.as_ref() {
                        key = Some(s.clone());
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Some(RateLimitConfig {
        requests,
        window,
        strategy,
        key,
    })
}
