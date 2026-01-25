// Standard Library für Axum-Integration
// Axum-basierte Web-Framework Code-Generierung

/// Axum Standard Library
pub struct AxumStdlib;

impl AxumStdlib {
    /// Generiert Axum Router-Code für eine Funktion
    pub fn generate_route_code(
        method: &str,
        path: &str,
        function_name: &str,
        is_async: bool,
    ) -> String {
        let method_lower = method.to_lowercase();
        let _handler_name = format!("{}_handler", function_name);

        if is_async {
            format!(
                ".route(\"{}\", {}(\"{}_handler\"))",
                path, method_lower, function_name
            )
        } else {
            format!(
                ".route(\"{}\", {}(\"{}_handler\"))",
                path, method_lower, function_name
            )
        }
    }

    /// Generiert vollständigen Axum Router
    pub fn generate_router(routes: Vec<(String, String, String, bool)>) -> String {
        let mut code =
            "use axum::{Router, routing::get, routing::post, routing::put, routing::delete};\n\n"
                .to_string();
        code.push_str("pub fn create_router() -> Router {\n");
        code.push_str("    Router::new()\n");

        for (method, path, function_name, is_async) in routes {
            let route_code = Self::generate_route_code(&method, &path, &function_name, is_async);
            code.push_str(&format!("        {}\n", route_code));
        }

        code.push_str("}\n");
        code
    }
}
