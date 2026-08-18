//! Attribute lowering for IR → target code (3.5.0 Säule 1).
//!
//! Converts IR attributes (from Velin decorators) into framework-specific
//! Rust/Axum surface: routes, auth markers, router wiring.

use crate::ir::ir::{IRAttribute, IRAttributeArg, IRFunction, IRModule, IRType};

/// HTTP method + path extracted from IR attributes
#[derive(Debug, Clone)]
pub struct HttpRoute {
    pub method: String,
    pub path: String,
    pub handler: String,
    pub requires_auth: bool,
    pub roles: Vec<String>,
}

/// Collect HTTP routes from an IR module
pub fn collect_routes(module: &IRModule) -> Vec<HttpRoute> {
    let mut routes = Vec::new();
    for func in &module.functions {
        if let Some(route) = route_from_function(func) {
            routes.push(route);
        }
    }
    routes
}

pub fn route_from_function(func: &IRFunction) -> Option<HttpRoute> {
    let mut method_path: Option<(String, String)> = None;
    let mut requires_auth = false;
    let mut roles = Vec::new();

    for attr in &func.attributes {
        match attr.name.as_str() {
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" => {
                if let Some(path) = first_string_arg(&attr.args) {
                    method_path = Some((attr.name.clone(), path));
                }
            }
            "Auth" => {
                requires_auth = true;
            }
            "Role" => {
                let found = all_string_args(&attr.args);
                if found.is_empty() {
                    roles.push("user".to_string());
                } else {
                    roles.extend(found);
                }
                requires_auth = true;
            }
            _ => {}
        }
    }

    method_path.map(|(method, path)| HttpRoute {
        method,
        path,
        handler: to_snake_case(&func.name),
        requires_auth,
        roles,
    })
}

fn first_string_arg(args: &[IRAttributeArg]) -> Option<String> {
    all_string_args(args).into_iter().next()
}

fn all_string_args(args: &[IRAttributeArg]) -> Vec<String> {
    let mut out = Vec::new();
    for arg in args {
        match arg {
            IRAttributeArg::String(s) => out.push(s.clone()),
            IRAttributeArg::Named { value, .. } => {
                if let IRAttributeArg::String(s) = value.as_ref() {
                    out.push(s.clone());
                }
            }
            _ => {}
        }
    }
    out
}

/// Whether the module needs Axum HTTP scaffolding
pub fn needs_axum(module: &IRModule) -> bool {
    !collect_routes(module).is_empty()
}

/// Axum imports for HTTP APIs
pub fn axum_imports() -> &'static str {
    "use axum::{\n\
     \x20   Router,\n\
     \x20   extract::{Path, Json, Query},\n\
     \x20   routing::{get, post, put, delete, patch},\n\
     \x20   response::IntoResponse,\n\
     \x20   http::StatusCode,\n\
     \x20   middleware,\n\
     };\n\
     use serde::{Serialize, Deserialize};\n\
     use std::collections::HashMap;\n\n"
}

/// Emit `create_router()` for collected routes.
/// Auth/Role use per-route `MethodRouter::layer` so unmatched paths are 404, not 401.
pub fn generate_axum_router(routes: &[HttpRoute]) -> String {
    if routes.is_empty() {
        return String::from("pub fn create_router() -> Router {\n    Router::new()\n}\n\n");
    }

    let mut groups: std::collections::BTreeMap<(String, bool, Vec<String>), Vec<&HttpRoute>> =
        std::collections::BTreeMap::new();
    for r in routes {
        let mut roles = r.roles.clone();
        roles.sort();
        roles.dedup();
        groups
            .entry((normalize_path(&r.path), r.requires_auth, roles))
            .or_default()
            .push(r);
    }

    let mut code = String::from("pub fn create_router() -> Router {\n    Router::new()\n");
    for ((path, requires_auth, roles), group) in &groups {
        let mut method_router = String::new();
        for (i, route) in group.iter().enumerate() {
            let method_fn = match route.method.to_uppercase().as_str() {
                "GET" => "get",
                "POST" => "post",
                "PUT" => "put",
                "DELETE" => "delete",
                "PATCH" => "patch",
                _ => "get",
            };
            if i == 0 {
                method_router = format!("{}({})", method_fn, route.handler);
            } else {
                method_router.push_str(&format!(".{}({})", method_fn, route.handler));
            }
        }
        if *requires_auth {
            let mw = if roles.is_empty() {
                "velin_auth_middleware".to_string()
            } else {
                role_mw_ident(roles)
            };
            method_router.push_str(&format!(".layer(middleware::from_fn({mw}))"));
        }
        code.push_str(&format!(
            "        .route(\"{}\", {method_router})\n",
            path.replace('{', ":").replace('}', "")
        ));
    }
    code.push_str("}\n\n");
    code
}

/// Actix-Web imports for HTTP APIs
pub fn actix_imports() -> &'static str {
    "use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::from_fn};\n\
     use actix_web::dev::{ServiceRequest, ServiceResponse};\n\
     use actix_web::Error as ActixError;\n\
     use serde::{Serialize, Deserialize};\n\
     use std::collections::HashMap;\n\n"
}

/// Emit Actix `configure_routes` for collected routes (distinct from Axum `create_router`)
pub fn generate_actix_router(routes: &[HttpRoute]) -> String {
    let mut code = String::from(
        "pub fn configure_routes(cfg: &mut web::ServiceConfig) {\n",
    );
    for route in routes {
        let method_fn = match route.method.to_uppercase().as_str() {
            "GET" => "get",
            "POST" => "post",
            "PUT" => "put",
            "DELETE" => "delete",
            "PATCH" => "patch",
            _ => "get",
        };
        let path = normalize_path(&route.path);
        code.push_str(&format!(
            "    cfg.route(\"{}\", web::{}().to({}));\n",
            path, method_fn, route.handler
        ));
    }
    code.push_str("}\n\n");
    code.push_str(
        "pub fn create_app() -> App<impl actix_web::dev::ServiceFactory<\n\
         \x20   actix_web::dev::ServiceRequest,\n\
         \x20   Config = (),\n\
         \x20   Response = actix_web::dev::ServiceResponse,\n\
         \x20   Error = actix_web::Error,\n\
         \x20   InitError = (),\n\
         >> {\n\
         \x20   App::new().configure(configure_routes)\n\
         }\n\n",
    );
    code
}

/// Actix auth middleware stub — requires Authorization header
pub fn generate_actix_auth_middleware(routes: &[HttpRoute]) -> String {
    if !routes.iter().any(|r| r.requires_auth) {
        return String::new();
    }
    "// Actix auth: handlers should check Authorization (from @Auth)\n\
     // Marker: ACTIX_AUTH_REQUIRED\n\n"
        .to_string()
}

/// Resolve rust HTTP framework string to lowering choice
pub fn is_actix_framework(framework: &str) -> bool {
    matches!(
        framework.to_lowercase().as_str(),
        "actix" | "actix-web" | "actix_web"
    )
}

/// Auth middleware from `@Auth` / `@Role` — requires `Authorization` header.
/// Role sets get their own middleware so missing/wrong `X-Role` is 403 per route.
pub fn generate_auth_middleware(routes: &[HttpRoute]) -> String {
    if !routes.iter().any(|r| r.requires_auth) {
        return String::new();
    }

    let mut out = String::from(
        "/// Requires `Authorization` header (from `@Auth` / `@Role`).\n\
         async fn velin_auth_middleware(\n\
         \x20   req: axum::extract::Request,\n\
         \x20   next: middleware::Next,\n\
         ) -> axum::response::Response {\n\
         \x20   if req.headers().get(axum::http::header::AUTHORIZATION).is_none() {\n\
         \x20       return (StatusCode::UNAUTHORIZED, \"Unauthorized\").into_response();\n\
         \x20   }\n\
         \x20   next.run(req).await\n\
         }\n\n",
    );

    let mut seen: std::collections::BTreeSet<Vec<String>> = std::collections::BTreeSet::new();
    for r in routes.iter().filter(|r| r.requires_auth && !r.roles.is_empty()) {
        let mut roles = r.roles.clone();
        roles.sort();
        roles.dedup();
        if !seen.insert(roles.clone()) {
            continue;
        }
        let ident = role_mw_ident(&roles);
        let list = roles
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "async fn {ident}(\n\
             \x20   req: axum::extract::Request,\n\
             \x20   next: middleware::Next,\n\
             ) -> axum::response::Response {{\n\
             \x20   if req.headers().get(axum::http::header::AUTHORIZATION).is_none() {{\n\
             \x20       return (StatusCode::UNAUTHORIZED, \"Unauthorized\").into_response();\n\
             \x20   }}\n\
             \x20   let roles: &[&str] = &[{list}];\n\
             \x20   match req.headers().get(\"X-Role\").and_then(|v| v.to_str().ok()) {{\n\
             \x20       Some(role_hdr) if roles.iter().any(|r| *r == role_hdr) => {{}}\n\
             \x20       _ => return (StatusCode::FORBIDDEN, \"Forbidden\").into_response(),\n\
             \x20   }}\n\
             \x20   next.run(req).await\n\
             }}\n\n"
        ));
    }
    out
}

fn role_mw_ident(roles: &[String]) -> String {
    format!("velin_role_{}", roles.join("_").replace('-', "_"))
}

/// Minimal Cargo.toml for a generated Axum project
pub fn axum_cargo_toml(package_name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
tower = "0.5"
"#,
        package_name
    )
}

/// Wrap generated lib-style router code into a runnable main.rs
pub fn axum_main_wrapper(generated_body: &str) -> String {
    axum_main_wrapper_with_port(generated_body, 3000)
}

/// Same as [`axum_main_wrapper`], with explicit listen port (or `PORT` env at runtime).
pub fn axum_main_wrapper_with_port(generated_body: &str, port: u16) -> String {
    axum_main_wrapper_with_bind(generated_body, "0.0.0.0", port)
}

/// Bind host + port. `VELIN_HOST` / `PORT` override the compiled defaults at runtime.
pub fn axum_main_wrapper_with_bind(generated_body: &str, host: &str, port: u16) -> String {
    let host = host.replace('\\', "\\\\").replace('"', "\\\"");
    format!(
        "{}\n\
         #[tokio::main]\n\
         async fn main() {{\n\
         \x20   let port: u16 = std::env::var(\"PORT\")\n\
         \x20       .ok()\n\
         \x20       .and_then(|s| s.parse().ok())\n\
         \x20       .unwrap_or({});\n\
         \x20   let host = std::env::var(\"VELIN_HOST\")\n\
         \x20       .ok()\n\
         \x20       .filter(|s| !s.is_empty())\n\
         \x20       .unwrap_or_else(|| \"{}\".to_string());\n\
         \x20   let addr = format!(\"{{}}:{{}}\", host, port);\n\
         \x20   let app = create_router();\n\
         \x20   let listener = tokio::net::TcpListener::bind(&addr).await.expect(\"bind\");\n\
         \x20   println!(\"listening on http://{{}}\" , addr);\n\
         \x20   axum::serve(listener, app).await.expect(\"serve\");\n\
         }}\n",
        generated_body, port, host
    )
}

// Back-compat alias
pub fn generate_auth_middleware_stub(routes: &[HttpRoute]) -> String {
    generate_auth_middleware(routes)
}

pub fn normalize_path(path: &str) -> String {
    // `:id` → `{id}` for Axum
    let mut out = String::new();
    let chars: Vec<char> = path.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ':' {
            out.push('{');
            i += 1;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                out.push(chars[i]);
                i += 1;
            }
            out.push('}');
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

pub fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        } else {
            result.push(c);
        }
    }
    result
}

fn path_has_exact_segment(path: &str, name: &str) -> bool {
    let needle = format!("{{{}}}", name);
    if let Some(idx) = path.find(&needle) {
        let after = idx + needle.len();
        let ok_after = after >= path.len() || !path[after..].chars().next().map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false);
        return ok_after;
    }
    false
}

fn is_path_param(func_param_name: &str, path: &str) -> bool {
    let snake = to_snake_case(func_param_name);
    path_has_exact_segment(path, func_param_name) || path_has_exact_segment(path, &snake)
}

fn is_body_param(method: &str, ty: &IRType, is_path: bool) -> bool {
    matches!(method, "POST" | "PUT" | "PATCH")
        && matches!(
            ty,
            IRType::Struct(_) | IRType::List(_) | IRType::Map { .. } | IRType::Any
        )
        && !is_path
}

fn is_query_param(func_param_name: &str, ty: &IRType, route: &HttpRoute) -> bool {
    let path = normalize_path(&route.path);
    let method = route.method.to_uppercase();
    if is_path_param(func_param_name, &path) {
        return false;
    }
    if is_body_param(&method, ty, false) {
        return false;
    }
    matches!(method.as_str(), "GET" | "DELETE" | "HEAD")
        && matches!(ty, IRType::String | IRType::Int | IRType::Float | IRType::Bool | IRType::Optional(_))
}

/// Build Axum-style parameter list for a route handler
pub fn axum_params(func: &IRFunction, route: &HttpRoute) -> String {
    let path = normalize_path(&route.path);
    let method = route.method.to_uppercase();
    let mut path_parts: Vec<String> = Vec::new();
    let mut other: Vec<String> = Vec::new();
    let mut has_query = false;

    for p in &func.params {
        let snake = to_snake_case(&p.name);
        let ty = ir_type_to_rust_simple(&p.ty);
        let is_path = is_path_param(&p.name, &path);
        let is_body = is_body_param(&method, &p.ty, is_path);

        if is_path {
            path_parts.push(format!("{}: {}", snake, ty));
        } else if is_body {
            other.push(format!("Json({}): Json<{}>", snake, ty));
        } else if is_query_param(&p.name, &p.ty, route) {
            has_query = true;
        } else if matches!(method.as_str(), "GET" | "DELETE" | "HEAD")
            && matches!(p.ty, IRType::Struct(_) | IRType::Map { .. })
            && !is_path
        {
            other.push(format!("Query({}): Query<{}>", snake, ty));
        } else if matches!(method.as_str(), "POST" | "PUT" | "PATCH")
            && matches!(p.ty, IRType::String | IRType::Int | IRType::Float | IRType::Bool)
        {
            other.push(format!("Json({}): Json<{}>", snake, ty));
        } else {
            other.push(format!("{}: {}", snake, ty));
        }
    }

    let mut parts = Vec::new();
    if path_parts.len() == 1 {
        let name_ty = &path_parts[0];
        let name = name_ty.split(':').next().unwrap().trim();
        let ty = name_ty.split(':').nth(1).unwrap().trim();
        parts.push(format!("Path({}): Path<{}>", name, ty));
    } else if path_parts.len() > 1 {
        let names: Vec<&str> = path_parts
            .iter()
            .map(|s| s.split(':').next().unwrap().trim())
            .collect();
        let tys: Vec<&str> = path_parts
            .iter()
            .map(|s| s.split(':').nth(1).unwrap().trim())
            .collect();
        parts.push(format!(
            "Path(({})): Path<({})>",
            names.join(", "),
            tys.join(", ")
        ));
    }
    if has_query {
        parts.push("Query(q): Query<std::collections::HashMap<String, String>>".to_string());
    }
    parts.extend(other);
    parts.join(", ")
}

/// Bindings for GET/DELETE query parameters
pub fn axum_query_lets(func: &IRFunction, route: &HttpRoute) -> String {
    let mut out = String::new();
    for p in &func.params {
        if !is_query_param(&p.name, &p.ty, route) {
            continue;
        }
        let snake = to_snake_case(&p.name);
        let optional = matches!(&p.ty, IRType::Optional(_));
        if optional {
            out.push_str(&format!(
                "    let {snake} = q.get(\"{name}\").cloned().or_else(|| q.get(\"{snake}\").cloned()).unwrap_or_default();\n",
                snake = snake,
                name = p.name
            ));
        } else {
            out.push_str(&format!(
                "    let {snake} = match q.get(\"{name}\").cloned().or_else(|| q.get(\"{snake}\").cloned()) {{\n\
                 \x20       Some(v) if !v.is_empty() => v,\n\
                 \x20       _ => return (StatusCode::BAD_REQUEST, \"missing query parameter: {name}\").into_response(),\n\
                 \x20   }};\n",
                snake = snake,
                name = p.name
            ));
        }
        match &p.ty {
            IRType::Int => out.push_str(&format!(
                "    let {snake}: i64 = match {snake}.parse() {{\n\
                 \x20       Ok(v) => v,\n\
                 \x20       Err(_) => return (StatusCode::BAD_REQUEST, \"invalid query parameter: {name}\").into_response(),\n\
                 \x20   }};\n",
                snake = snake,
                name = p.name
            )),
            IRType::Float => out.push_str(&format!(
                "    let {snake}: f64 = match {snake}.parse() {{\n\
                 \x20       Ok(v) => v,\n\
                 \x20       Err(_) => return (StatusCode::BAD_REQUEST, \"invalid query parameter: {name}\").into_response(),\n\
                 \x20   }};\n",
                snake = snake,
                name = p.name
            )),
            IRType::Bool => out.push_str(&format!(
                "    let {snake}: bool = match {snake}.parse() {{\n\
                 \x20       Ok(v) => v,\n\
                 \x20       Err(_) => return (StatusCode::BAD_REQUEST, \"invalid query parameter: {name}\").into_response(),\n\
                 \x20   }};\n",
                snake = snake,
                name = p.name
            )),
            IRType::Optional(inner) if matches!(inner.as_ref(), IRType::Int) => out.push_str(&format!(
                "    let {snake}: i64 = {snake}.parse().unwrap_or(0);\n"
            )),
            IRType::Optional(inner) if matches!(inner.as_ref(), IRType::Float) => out.push_str(&format!(
                "    let {snake}: f64 = {snake}.parse().unwrap_or(0.0);\n"
            )),
            IRType::Optional(inner) if matches!(inner.as_ref(), IRType::Bool) => out.push_str(&format!(
                "    let {snake}: bool = {snake}.parse().unwrap_or(false);\n"
            )),
            _ => {}
        }
    }
    out
}

fn ir_type_to_rust_simple(ty: &IRType) -> String {
    match ty {
        IRType::Void | IRType::Null => "()".to_string(),
        IRType::Bool => "bool".to_string(),
        IRType::Int => "i64".to_string(),
        IRType::Float => "f64".to_string(),
        IRType::String => "String".to_string(),
        IRType::Any => "serde_json::Value".to_string(),
        IRType::Struct(n) | IRType::Enum(n) => n.clone(),
        IRType::List(inner) => format!("Vec<{}>", ir_type_to_rust_simple(inner)),
        IRType::Map { key, value } => format!(
            "HashMap<{}, {}>",
            ir_type_to_rust_simple(key),
            ir_type_to_rust_simple(value)
        ),
        IRType::Optional(inner) => format!("Option<{}>", ir_type_to_rust_simple(inner)),
        IRType::Result { ok, .. } => ir_type_to_rust_simple(ok),
        IRType::Pointer(inner) => format!("&{}", ir_type_to_rust_simple(inner)),
        IRType::Tuple(items) => {
            let parts: Vec<_> = items.iter().map(ir_type_to_rust_simple).collect();
            format!("({})", parts.join(", "))
        }
        IRType::Function { .. } => "Box<dyn Fn() + Send + Sync>".to_string(),
    }
}

/// Doc comment for HTTP attributes (visibility in generated code)
pub fn attribute_doc_comments(attrs: &[IRAttribute]) -> String {
    let mut out = String::new();
    for attr in attrs {
        match attr.name.as_str() {
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" => {
                if let Some(path) = first_string_arg(&attr.args) {
                    out.push_str(&format!(
                        "/// HTTP {} {}\n",
                        attr.name.to_uppercase(),
                        path
                    ));
                }
            }
            "Auth" => out.push_str("/// Requires authentication (`@Auth`)\n"),
            "Role" => {
                let role = first_string_arg(&attr.args).unwrap_or_else(|| "user".into());
                out.push_str(&format!("/// Requires role `{}` (`@Role`)\n", role));
            }
            other => {
                out.push_str(&format!("/// Attribute `@{}`\n", other));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_colon_path() {
        assert_eq!(normalize_path("/users/:id"), "/users/{id}");
    }

    #[test]
    fn snake_case_hello() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("hello"), "hello");
    }
}
