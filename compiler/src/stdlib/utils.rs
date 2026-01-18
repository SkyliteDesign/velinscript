
pub struct UtilsStdlib;

impl UtilsStdlib {
    pub fn generate_uuid_code() -> String {
        "uuid::Uuid::new_v4().to_string()".to_string()
    }

    pub fn generate_sleep_code(ms: &str) -> String {
        format!("tokio::time::sleep(std::time::Duration::from_millis({} as u64)).await", ms)
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
        // Debounce is hard to implement as a pure function returning a function in Rust without Box<dyn Fn...>
        // We'll return the function itself for now (no-op)
        format!("{}", func)
    }

    pub fn generate_throttle_code(func: &str, ms: &str) -> String {
        format!("{}", func)
    }

    pub fn generate_memoize_code(func: &str) -> String {
        format!("{}", func)
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
        // Simple global cache check?
        // For now, just execute
        format!("{}()", func)
    }
}
