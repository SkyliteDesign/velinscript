pub struct UtilsStdlib;

impl UtilsStdlib {
    pub fn generate_uuid_code() -> String {
        "uuid::Uuid::new_v4().to_string()".to_string()
    }

    pub fn generate_sleep_code(ms: &str) -> String {
        format!(
            "tokio::time::sleep(std::time::Duration::from_millis({} as u64)).await",
            ms
        )
    }

    pub fn generate_retry_code(func: &str, times: &str) -> String {
        // Simple retry loop for async function
        // Note: func must be an async block or function pointer that returns a Future
        // This is tricky to generate generically without macros.
        // We'll assume func is a closure that returns a Future.
        format!(
            "{{
                let mut result = Err(\"Retries exhausted\".to_string());
                for _ in 0..{} {{
                    match {}().await {{
                        Ok(v) => {{ result = Ok(v); break; }},
                        Err(_) => tokio::time::sleep(std::time::Duration::from_millis(100)).await,
                    }}
                }}
                result
            }}",
            times, func
        )
    }

    pub fn generate_debounce_code(func: &str, ms: &str) -> String {
        format!(
            "{{
                use std::sync::Arc;
                use tokio::sync::Mutex;
                use tokio::time::Instant;
                let last_call = Arc::new(Mutex::new(Option::<Instant>::None));
                let func = Arc::new({});
                let delay_ms = {} as u64;
                move || {{
                    let last_call = last_call.clone();
                    let func = func.clone();
                    async move {{
                        let mut last = last_call.lock().await;
                        let now = Instant::now();
                        if let Some(prev) = *last {{
                            if now.duration_since(prev).as_millis() < delay_ms as u128 {{
                                tokio::time::sleep_until(prev + std::time::Duration::from_millis(delay_ms)).await;
                            }}
                        }}
                        *last = Some(now);
                        func().await
                    }}
                }}
            }}",
            func, ms
        )
    }

    pub fn generate_throttle_code(func: &str, ms: &str) -> String {
        format!(
            "{{
                use std::sync::Arc;
                use tokio::sync::Mutex;
                use tokio::time::Instant;
                let last_call = Arc::new(Mutex::new(Option::<Instant>::None));
                let func = Arc::new({});
                let interval_ms = {} as u64;
                move || {{
                    let last_call = last_call.clone();
                    let func = func.clone();
                    async move {{
                        let mut last = last_call.lock().await;
                        let now = Instant::now();
                        if let Some(prev) = *last {{
                            if now.duration_since(prev).as_millis() < interval_ms as u128 {{
                                return;
                            }}
                        }}
                        *last = Some(now);
                        func().await
                    }}
                }}
            }}",
            func, ms
        )
    }

    pub fn generate_memoize_code(func: &str) -> String {
        format!(
            "{{
                use std::collections::HashMap;
                use std::sync::Arc;
                use tokio::sync::Mutex;
                let cache: Arc<Mutex<HashMap<String, anyhow::Result<String>>>> = Arc::new(Mutex::new(HashMap::new()));
                let func = Arc::new({});
                move |key: String| {{
                    let cache = cache.clone();
                    let func = func.clone();
                    async move {{
                        let mut cache = cache.lock().await;
                        if let Some(result) = cache.get(&key) {{
                            return result.clone();
                        }}
                        let result = func().await;
                        cache.insert(key, result.clone());
                        result
                    }}
                }}
            }}",
            func
        )
    }

    pub fn generate_timeout_code(func: &str, ms: &str) -> String {
        format!(
            "tokio::time::timeout(std::time::Duration::from_millis({} as u64), {}).await.map_err(|_| \"Timeout\".to_string()).and_then(|r| r)",
            ms, func
        )
    }

    pub fn generate_parallel_code(tasks: &str) -> String {
        // tasks: Vec<fn() -> Future>
        format!(
            "futures_util::future::join_all({}.into_iter().map(|t| t())).await",
            tasks
        )
    }

    pub fn generate_cache_code(key: &str, func: &str) -> String {
        format!(
            "{{
                use std::collections::HashMap;
                use std::sync::Arc;
                use tokio::sync::Mutex;
                static CACHE: once_cell::sync::Lazy<Arc<Mutex<HashMap<String, String>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
                let cache_key = {}.to_string();
                let cache = CACHE.clone();
                async move {{
                    let mut cache = cache.lock().await;
                    if let Some(cached) = cache.get(&cache_key) {{
                        return cached.clone();
                    }}
                    let result = {}().await;
                    cache.insert(cache_key, result.clone());
                    result
                }}
            }}",
            key, func
        )
    }
}
