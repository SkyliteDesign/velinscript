// Standard Library für Actix-Web-Integration
// Actix-basierte Web-Framework Code-Generierung

/// Actix Standard Library
pub struct ActixStdlib;

impl ActixStdlib {
    /// Generiert Actix Route-Code für eine Funktion
    pub fn generate_route_code(
        method: &str,
        path: &str,
        function_name: &str,
        is_async: bool,
    ) -> String {
        let method_lower = method.to_lowercase();
        let handler_name = format!("{}_handler", function_name);

        format!(
            ".route(\"{}\", web::{}().to({}))",
            path, method_lower, handler_name
        )
    }

    /// Generiert vollständigen Actix HttpServer
    pub fn generate_http_server(routes: Vec<(String, String, String, bool)>) -> String {
        let mut code = "use actix_web::{web, App, HttpServer};\n\n".to_string();
        code.push_str("pub fn create_server() -> HttpServer<impl Fn() -> App> {\n");
        code.push_str("    HttpServer::new(|| {\n");
        code.push_str("        App::new()\n");

        for (method, path, function_name, is_async) in routes {
            let route_code = self.generate_route_code(&method, &path, &function_name, is_async);
            code.push_str(&format!("            {}\n", route_code));
        }

        code.push_str("    })\n");
        code.push_str("}\n");
        code
    }
}
