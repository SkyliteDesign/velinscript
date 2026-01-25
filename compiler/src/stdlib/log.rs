pub struct LogStdlib;

impl LogStdlib {
    pub fn generate_info_code(message: &str) -> String {
        format!("tracing::info!(\"{}\", {})", "{}", message)
    }

    pub fn generate_warn_code(message: &str) -> String {
        format!("tracing::warn!(\"{}\", {})", "{}", message)
    }

    pub fn generate_error_code(message: &str) -> String {
        format!("tracing::error!(\"{}\", {})", "{}", message)
    }

    pub fn generate_debug_code(message: &str) -> String {
        format!("tracing::debug!(\"{}\", {})", "{}", message)
    }

    pub fn generate_trace_code(message: &str) -> String {
        format!("tracing::trace!(\"{}\", {})", "{}", message)
    }

    pub fn generate_set_level_code(level: &str) -> String {
        // Changing log level at runtime might require a reloadable subscriber
        // For now, log that we want to change it
        format!(
            "tracing::info!(\"Setting log level to {}\", {})",
            "{}", level
        )
    }

    pub fn generate_with_context_code(key: &str, value: &str) -> String {
        format!(
            "{{
                let span = tracing::span!(tracing::Level::INFO, \"context\", {} = {}, {} = {});
                span.enter();
                crate::stdlib::logging::Logger::new()
            }}",
            key, value, key, value
        )
    }

    pub fn generate_to_file_code(path: &str) -> String {
        format!(
            "{{
                use tracing_subscriber::fmt;
                use tracing_subscriber::prelude::*;
                use std::fs::OpenOptions;
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open({})
                    .map_err(|e| e.to_string())?;
                let file_layer = fmt::layer()
                    .with_writer(file)
                    .with_ansi(false);
                tracing_subscriber::registry()
                    .with(file_layer)
                    .init();
                Ok(())
            }}",
            path
        )
    }

    pub fn generate_json_code(message: &str, data: &str) -> String {
        format!("tracing::info!(data = ?{}, \"{}\")", data, message)
    }
}
