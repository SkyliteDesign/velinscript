
pub struct TracingStdlib;

impl TracingStdlib {
    pub fn generate_start_span_code(name: &str) -> String {
        format!(
            "tracing::span!(tracing::Level::INFO, \"{}\")",
            name
        )
    }

    pub fn generate_set_attribute_code(span: &str, key: &str, value: &str) -> String {
        format!(
            "{}.record(\"{}\", {});",
            span, key, value
        )
    }

    pub fn generate_child_span_code(parent: &str, name: &str) -> String {
        format!(
            "tracing::span!(parent: {}, tracing::Level::INFO, \"{}\")",
            parent, name
        )
    }

    pub fn generate_end_span_code(span: &str) -> String {
        format!(
            "drop({})",
            span
        )
    }

    pub fn generate_export_code(format: &str) -> String {
        format!(
            "{{
                // Export tracing data in {:?} format
                // This would typically be handled by a tracing subscriber
                // For now, this is a placeholder
                Ok(())
            }}",
            format
        )
    }

    pub fn generate_span_enter_code(span: &str) -> String {
        format!(
            "let _guard = {}.enter();",
            span
        )
    }
}
