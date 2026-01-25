use crate::codegen::framework::{Framework, FrameworkSelector};
use crate::codegen::traits::{CodeGenerator, CodegenConfig, TargetLanguage};
use crate::compiler::language::VELISCH_FINGERPRINT;
use crate::parser::ast::*;
#[cfg(feature = "oauth2")]
use crate::stdlib::oauth2::OAuth2Stdlib;
#[cfg(feature = "privacy")]
use crate::stdlib::privacy::PrivacyStdlib;
#[cfg(feature = "sea-orm")]
use crate::stdlib::seaorm::SeaORMStdlib;

pub struct RustCodeGenerator {
    output: String,
    indent_level: usize,
    use_seaorm: bool,
    framework: Framework,
    has_validation: bool,
}

impl CodeGenerator for RustCodeGenerator {
    fn generate(&mut self, program: &Program, config: &CodegenConfig) -> anyhow::Result<String> {
        let config_framework = config.framework.as_deref();
        let config_orm = config.orm.as_deref();
        Ok(self.generate_internal(program, config_framework, config_orm))
    }

    fn get_target_language(&self) -> TargetLanguage {
        TargetLanguage::Rust
    }
}

impl RustCodeGenerator {
    pub fn new() -> Self {
        RustCodeGenerator {
            output: String::new(),
            indent_level: 0,
            use_seaorm: false,
            framework: Framework::Axum,
            has_validation: false,
        }
    }

    // Legacy method for backward compatibility, now calls internal implementation
    pub fn generate(
        &mut self,
        program: &Program,
        config_framework: Option<&str>,
        config_orm: Option<&str>,
    ) -> String {
        self.generate_internal(program, config_framework, config_orm)
    }

    fn generate_internal(
        &mut self,
        program: &Program,
        config_framework: Option<&str>,
        config_orm: Option<&str>,
    ) -> String {
        self.output.clear();

        // Velisch Fingerabdruck - darf nicht entfernt werden
        self.writeln(VELISCH_FINGERPRINT);
        self.writeln("#![allow(unused_imports, unused_variables, dead_code)]");
        self.writeln("");

        // Detect framework
        let framework = FrameworkSelector::detect_framework(program, config_framework);
        self.framework = framework;
        let framework_imports = FrameworkSelector::generate_imports(framework);
        self.writeln(&framework_imports);

        // Check for validation usage
        self.has_validation = self.has_validation_decorators(program);
        if self.has_validation {
            use crate::stdlib::validation::ValidationStdlib;
            self.writeln("use regex::Regex;");
            let validator_code = ValidationStdlib::generate_validator_struct();
            self.writeln(&validator_code);
            self.writeln("");
        }

        // Add necessary imports
        // self.writeln("use serde::{Serialize, Deserialize};"); // Included in framework imports
        self.writeln("use serde_json::Value as Any;");
        self.writeln("use anyhow::Result;");
        self.writeln("use itertools::Itertools;");
        self.writeln("use rayon::prelude::*;");
        self.writeln("use tracing::{info, error, warn, debug};");
        self.writeln("use once_cell::sync::Lazy;");
        self.writeln("");

        // Inject AppError for Axum
        if matches!(self.framework, Framework::Axum) {
            self.writeln(
                r#"
// Global Error Handler
struct AppError(anyhow::Error);

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
        .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
"#,
            );
        }

        // Check ORM
        let use_seaorm =
            config_orm.map(|s| s == "seaorm").unwrap_or(false) || self.has_seaorm_usage(program);
        self.use_seaorm = use_seaorm;
        if self.use_seaorm {
            #[cfg(feature = "sea-orm")]
            {
                self.writeln("use sea_orm::entity::prelude::*;");
                self.writeln("use sea_orm::{Database, DatabaseConnection};");
            }
        }
        self.writeln("");

        // Check OAuth2/OIDC
        let has_oauth2 = self.has_oauth2_decorators(program);
        if has_oauth2 {
            #[cfg(feature = "oauth2")]
            {
                self.writeln("use oauth2::{Client, AuthUrl, TokenUrl};");
                self.writeln("use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};");
            }
        }
        self.writeln("");

        // Inject Stdlib Runtime Code
        self.writeln("pub mod stdlib {");

        // ML Runtime
        let has_ml = self.has_ml_usage(program);
        if has_ml {
            use crate::stdlib::ml::MLStdlib;
            self.writeln("    pub mod ml {");
            self.writeln("        use super::super::*;");
            self.writeln(&MLStdlib::generate_ml_runtime_code());
            self.writeln("    }");
        }

        // Flow Runtime
        let has_flow = self.has_flow_usage(program);
        if has_flow {
            use crate::stdlib::flow::FlowStdlib;
            self.writeln("    pub mod flow {");
            self.writeln("        use super::super::*;");
            self.writeln(&FlowStdlib::generate_flow_runtime_code());
            self.writeln("    }");
        }

        self.writeln("}");
        self.writeln("");

        // Check Privacy
        let has_privacy = self.has_privacy_decorators(program);
        if has_privacy {
            #[cfg(feature = "privacy")]
            {
                self.writeln("use zeroize::Zeroize;");
                self.writeln("use secrecy::Secret;");
                let privacy_code = PrivacyStdlib::generate_privacy_wrapper_type();
                self.writeln(&privacy_code);
            }
        }
        self.writeln("");

        // Check if security decorators are used
        let has_security = self.has_security_decorators(program);
        if has_security {
            self.writeln("// Security Middleware");
            if has_oauth2 {
                #[cfg(feature = "oauth2")]
                {
                    match self.framework {
                        Framework::Axum => {
                            let middleware = OAuth2Stdlib::generate_axum_oauth2_middleware();
                            self.writeln(&middleware);
                        }
                        Framework::Actix => {
                            let middleware = OAuth2Stdlib::generate_actix_oauth2_middleware();
                            self.writeln(&middleware);
                        }
                    }
                }
            } else {
                use crate::stdlib::auth::AuthStdlib;
                self.writeln(&AuthStdlib::generate_mfa_runtime_code());
                self.writeln(&AuthStdlib::generate_auth_middleware_code());
            }
            self.writeln("");
        }

        // Check if Result type is used
        let has_result = self.has_result_usage(program);

        // Generate all items
        for item in &program.items {
            self.generate_item(item, &framework, self.use_seaorm);
            self.writeln("");
        }

        // Add Result methods if Result is used
        if has_result {
            use crate::stdlib::result::ResultStdlib;
            let result_methods = ResultStdlib::generate_result_methods();
            self.writeln("");
            self.writeln("// Result Extension Methods");
            self.writeln(&result_methods);
        }

        self.output.clone()
    }

    fn has_security_decorators(&self, program: &Program) -> bool {
        for item in &program.items {
            if let Item::Function(f) = item {
                for decorator in &f.decorators {
                    if matches!(decorator.name.as_str(), "Auth" | "Role") {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn has_oauth2_decorators(&self, program: &Program) -> bool {
        for item in &program.items {
            if let Item::Function(f) = item {
                for _decorator in &f.decorators {
                    #[cfg(feature = "oauth2")]
                    if OAuth2Stdlib::is_oauth2_decorator(_decorator) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn has_privacy_decorators(&self, program: &Program) -> bool {
        for item in &program.items {
            match item {
                Item::Struct(s) => {
                    for _field in &s.fields {
                        #[cfg(feature = "privacy")]
                        if PrivacyStdlib::is_privacy_decorator(&Decorator {
                            name: "Privacy".to_string(),
                            args: vec![],
                        }) {
                            return true;
                        }
                    }
                }
                Item::Function(f) => {
                    for _decorator in &f.decorators {
                        #[cfg(feature = "privacy")]
                        if PrivacyStdlib::is_privacy_decorator(_decorator) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn has_seaorm_usage(&self, _program: &Program) -> bool {
        // Check if db.* calls are used (would use SeaORM)
        // This is a simple check - in production, do more sophisticated analysis
        false
    }

    fn has_validation_decorators(&self, program: &Program) -> bool {
        for item in &program.items {
            if let Item::Function(f) = item {
                for decorator in &f.decorators {
                    if matches!(
                        decorator.name.as_str(),
                        "Validate" | "@Validate" | "Validation" | "@Validation"
                    ) {
                        return true;
                    }
                }
                // Check if function has string/number parameters that might need validation
                for param in &f.params {
                    match param.param_type {
                        Type::String | Type::Number => {
                            return true; // Auto-validate string/number params
                        }
                        _ => {}
                    }
                }
            }
        }
        false
    }

    fn has_result_usage(&self, program: &Program) -> bool {
        // Check if Result type is used anywhere in the program
        self.check_type_for_result(&program.items)
    }

    fn has_ml_usage(&self, _program: &Program) -> bool {
        true
    }

    fn has_flow_usage(&self, _program: &Program) -> bool {
        true
    }

    fn check_type_for_result(&self, items: &[Item]) -> bool {
        for item in items {
            match item {
                Item::Function(f) => {
                    // Check return type
                    if let Some(ref return_type) = f.return_type {
                        if self.is_result_type(return_type) {
                            return true;
                        }
                    }
                    // Check parameters
                    for param in &f.params {
                        if self.is_result_type(&param.param_type) {
                            return true;
                        }
                    }
                    // Check body for Result usage
                    if self.check_block_for_result(&f.body) {
                        return true;
                    }
                }
                Item::Struct(s) => {
                    for field in &s.fields {
                        if self.is_result_type(&field.field_type) {
                            return true;
                        }
                    }
                }
                Item::Enum(e) => {
                    for variant in &e.variants {
                        if let Some(ref types) = variant.data {
                            for t in types {
                                if self.is_result_type(t) {
                                    return true;
                                }
                            }
                        }
                    }
                }
                Item::TypeAlias(ta) => {
                    if self.is_result_type(&ta.aliased_type) {
                        return true;
                    }
                }
                Item::Trait(t) => {
                    for method in &t.methods {
                        for param in &method.params {
                            if self.is_result_type(&param.param_type) {
                                return true;
                            }
                        }
                        if let Some(ref return_type) = method.return_type {
                            if self.is_result_type(return_type) {
                                return true;
                            }
                        }
                    }
                }
                Item::Impl(i) => {
                    if self.is_result_type(&i.for_type) {
                        return true;
                    }
                    for method in &i.methods {
                        if let Some(ref return_type) = method.return_type {
                            if self.is_result_type(return_type) {
                                return true;
                            }
                        }
                        for param in &method.params {
                            if self.is_result_type(&param.param_type) {
                                return true;
                            }
                        }
                    }
                }
                Item::Module(m) => {
                    if self.check_type_for_result(&m.items) {
                        return true;
                    }
                }
                Item::Use(_) => {}
                Item::TopLevelCode(_) => {} // Top-level code doesn't affect Result type checking
            }
        }
        false
    }

    fn is_result_type(&self, t: &Type) -> bool {
        match t {
            Type::Result { .. } => true,
            Type::Generic { name, .. } => name == "Result",
            Type::List(inner) => self.is_result_type(inner),
            Type::Map { key, value } => self.is_result_type(key) || self.is_result_type(value),
            Type::Tuple(types) => types.iter().any(|t| self.is_result_type(t)),
            Type::Optional(inner) => self.is_result_type(inner),
            Type::Function {
                params,
                return_type,
            } => params.iter().any(|t| self.is_result_type(t)) || self.is_result_type(return_type),
            _ => false,
        }
    }

    fn check_block_for_result(&self, block: &Block) -> bool {
        for statement in &block.statements {
            match statement {
                Statement::Let(let_stmt) => {
                    if let Some(ref var_type) = let_stmt.var_type {
                        if self.is_result_type(var_type) {
                            return true;
                        }
                    }
                    if self.check_expression_for_result(&let_stmt.value) {
                        return true;
                    }
                }
                Statement::Return(ret_stmt) => {
                    if let Some(ref value) = ret_stmt.value {
                        if self.check_expression_for_result(value) {
                            return true;
                        }
                    }
                }
                Statement::Expression(expr_stmt) => {
                    if self.check_expression_for_result(&expr_stmt.expression) {
                        return true;
                    }
                }
                Statement::If(if_stmt) => {
                    if self.check_expression_for_result(&if_stmt.condition) {
                        return true;
                    }
                    if self.check_block_for_result(&if_stmt.then_block) {
                        return true;
                    }
                    if let Some(ref else_block) = if_stmt.else_block {
                        if self.check_block_for_result(else_block) {
                            return true;
                        }
                    }
                }
                Statement::For(for_stmt) => {
                    if self.check_expression_for_result(&for_stmt.iterable) {
                        return true;
                    }
                    if self.check_block_for_result(&for_stmt.body) {
                        return true;
                    }
                }
                Statement::While(while_stmt) => {
                    if self.check_expression_for_result(&while_stmt.condition) {
                        return true;
                    }
                    if self.check_block_for_result(&while_stmt.body) {
                        return true;
                    }
                }
                Statement::Match(match_stmt) => {
                    if self.check_expression_for_result(&match_stmt.expression) {
                        return true;
                    }
                    for arm in &match_stmt.arms {
                        if self.check_block_for_result(&arm.body) {
                            return true;
                        }
                    }
                }
                Statement::Throw(throw_stmt) => {
                    if self.check_expression_for_result(&throw_stmt.expression) {
                        return true;
                    }
                }
                Statement::Break(_) => {}
                Statement::Try(_) => {
                    // Try statements should be desugared before code generation
                    panic!("Try statement found after desugaring pass");
                }
            }
        }
        false
    }

    fn check_expression_for_result(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Literal(_) => false,
            Expression::FormatString { .. } => false,
            Expression::Identifier(name) => name == "Result",
            Expression::BinaryOp { left, right, .. } => {
                self.check_expression_for_result(left) || self.check_expression_for_result(right)
            }
            Expression::UnaryOp { expr, .. } => self.check_expression_for_result(expr),
            Expression::StructLiteral { name: _, fields } => fields
                .iter()
                .any(|(_, expr)| self.check_expression_for_result(expr)),
            Expression::MapLiteral(fields) => fields
                .iter()
                .any(|(_, expr)| self.check_expression_for_result(expr)),
            Expression::ListLiteral(elements) => elements
                .iter()
                .any(|expr| self.check_expression_for_result(expr)),
            Expression::Call { callee, args } => {
                self.check_expression_for_result(callee)
                    || args.iter().any(|arg| self.check_expression_for_result(arg))
            }
            Expression::Member { object, member: _ } => self.check_expression_for_result(object),
            Expression::Index { object, index } => {
                self.check_expression_for_result(object) || self.check_expression_for_result(index)
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                self.check_expression_for_result(condition)
                    || self.check_expression_for_result(then_expr)
                    || self.check_expression_for_result(else_expr)
            }
            Expression::Block(block) => self.check_block_for_result(block),
            Expression::Await { expr } => self.check_expression_for_result(expr),
            Expression::GenericConstructor { type_params, .. } => {
                type_params.iter().any(|t| self.is_result_type(t))
            }
            Expression::Lambda { body, .. } => self.check_expression_for_result(body),
            Expression::Assignment { target: _, value } => self.check_expression_for_result(value),
            Expression::LLMCall { method: _, args } => {
                args.iter().any(|arg| self.check_expression_for_result(arg))
            }
        }
    }

    fn generate_item(&mut self, item: &Item, framework: &Framework, use_seaorm: bool) {
        match item {
            Item::Function(f) => self.generate_function(f, framework, use_seaorm),
            Item::Struct(s) => self.generate_struct(s, use_seaorm),
            Item::Enum(e) => self.generate_enum(e),
            Item::TypeAlias(ta) => self.generate_type_alias(ta),
            Item::Module(m) => self.generate_module(m),
            Item::Use(_u) => {
                // Use statements are handled differently in Rust
                // For now, skip
            }
            Item::Trait(t) => self.generate_trait(t),
            Item::Impl(i) => self.generate_impl(i, framework, use_seaorm),
            Item::TopLevelCode(expr_stmt) => {
                // Generate top-level code (e.g., init(); startFileWatcher();)
                self.generate_expression(&expr_stmt.expression);
                self.output.push_str(";\n");
            }
        }
    }

    fn generate_function(&mut self, function: &Function, framework: &Framework, _use_seaorm: bool) {
        // Generate decorators as Rust attributes
        for decorator in &function.decorators {
            self.generate_decorator(decorator);
        }

        // Generate function signature
        if function.visibility == Visibility::Public {
            self.write("pub ");
        }

        if function.is_const {
            self.write("const ");
        }

        // Check for @Flow decorator
        let is_flow = function.decorators.iter().any(|d| d.name == "Flow");
        let is_main = function.name == "main";

        if is_main {
            self.writeln("#[tokio::main]");
            self.write("async ");
        } else if function.is_async || is_flow {
            self.writeln("#[tracing::instrument]");
            self.write("async ");
        }

        self.write("fn ");
        self.write(&self.to_snake_case(&function.name));

        // Generate generic type parameters with constraints
        if !function.type_params.is_empty() {
            self.write("<");
            for (i, param) in function.type_params.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(&param.name);

                // Generate constraints (T: Trait1 + Trait2)
                if !param.constraints.is_empty() {
                    self.write(": ");
                    for (j, constraint) in param.constraints.iter().enumerate() {
                        if j > 0 {
                            self.write(" + ");
                        }
                        match constraint {
                            GenericConstraint::Trait(trait_name) => {
                                self.write(&self.to_pascal_case(trait_name));
                            }
                            GenericConstraint::Multiple(trait_names) => {
                                for (k, trait_name) in trait_names.iter().enumerate() {
                                    if k > 0 {
                                        self.write(" + ");
                                    }
                                    self.write(&self.to_pascal_case(trait_name));
                                }
                            }
                        }
                    }
                }
            }
            self.write(">");
        }

        self.write("(");

        // Generate parameters
        let route_info = self.is_route_handler(&function.decorators);

        for (i, param) in function.params.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }

            let mut handled = false;
            if let Some((method, path)) = &route_info {
                // Check if it's a path parameter
                if path.contains(&format!(":{}", param.name))
                    || path.contains(&format!("{{{}}}", param.name))
                {
                    self.write(&format!(
                        "Path(mut {}): Path<",
                        self.to_snake_case(&param.name)
                    ));
                    self.generate_type(&param.param_type);
                    self.write(">");
                    handled = true;
                }
                // Check if it's a body parameter (Complex type in POST/PUT/PATCH)
                else if self.is_complex_type(&param.param_type)
                    && (method == "POST" || method == "PUT" || method == "PATCH")
                {
                    self.write(&format!(
                        "Json(mut {}): Json<",
                        self.to_snake_case(&param.name)
                    ));
                    self.generate_type(&param.param_type);
                    self.write(">");
                    handled = true;
                }
            }

            if !handled {
                self.write("mut ");
                self.write(&self.to_snake_case(&param.name));
                self.write(": ");
                self.generate_type(&param.param_type);
            }
        }

        self.write(")");

        // Generate return type
        if let Some(ref return_type) = function.return_type {
            self.write(" -> ");
            // Prüfe ob es ein Result-Type ist
            if let Type::Generic { name, params } = return_type {
                if name == "Result" {
                    // Generiere anyhow::Result
                    if let Some(success_type) = params.first() {
                        self.write("anyhow::Result<");
                        self.generate_type(success_type);
                        self.write(">");
                    } else {
                        self.generate_type(return_type);
                    }
                } else {
                    self.generate_type(return_type);
                }
            } else {
                self.generate_type(return_type);
            }
        }

        self.writeln(" {");
        self.indent();

        // Check for @Flow decorator
        let is_flow = function.decorators.iter().any(|d| d.name == "Flow");

        if is_flow {
            self.writeln("use crate::stdlib::flow::{FlowManager};");
            self.writeln(&format!(
                "let flow = FlowManager::new(\"{}\");",
                function.name
            ));
            self.writeln("flow.start();");
            self.writeln("");
            self.writeln("// Execute flow body");
            // For MVP, we wrap the body execution to capture result
            self.writeln("let result = async {");
            self.indent();
        }

        // Generate validation code if needed
        if self.has_validation {
            self.generate_validation_code(function, framework);
        }

        // Generate function body
        self.generate_block(&function.body);

        if is_flow {
            self.unindent();
            self.writeln("}.await;");
            self.writeln("");

            let returns_result = if let Some(ref ret) = function.return_type {
                self.is_result_type(ret)
            } else {
                false
            };

            if returns_result {
                self.writeln("match &result {");
                self.writeln("    Ok(_) => flow.commit(),");
                self.writeln("    Err(e) => flow.rollback(&e.to_string()),");
                self.writeln("}");
            } else {
                self.writeln("flow.commit();");
            }
            self.writeln("result");
        }

        self.unindent();
        self.writeln("}");
    }

    fn generate_decorator(&mut self, decorator: &Decorator) {
        match decorator.name.as_str() {
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" => {
                if let Some(arg) = decorator.args.first() {
                    if let DecoratorArg::String(path) = arg {
                        let method = decorator.name.to_lowercase();
                        self.writeln(&format!("#[{}(\"{}\")]", method, path));
                    }
                }
            }
            "Auth" => {
                self.writeln("#[actix_web::web::middleware(AuthMiddleware)]");
            }
            "Role" => {
                if let Some(arg) = decorator.args.first() {
                    if let DecoratorArg::String(role) = arg {
                        self.writeln(&format!(
                            "#[actix_web::web::middleware(RoleMiddleware::new(\"{}\"))]",
                            role
                        ));
                    } else {
                        self.writeln(
                            "#[actix_web::web::middleware(RoleMiddleware::new(\"user\"))]",
                        );
                    }
                } else {
                    self.writeln("#[actix_web::web::middleware(RoleMiddleware::new(\"user\"))]");
                }
            }
            "Cache" => {
                // Generate cache decorator
                let mut cache_args = Vec::new();
                for arg in &decorator.args {
                    match arg {
                        DecoratorArg::Named { name, value } => {
                            if let DecoratorArg::String(s) = value.as_ref() {
                                cache_args.push(format!("{} = \"{}\"", name, s));
                            }
                        }
                        _ => {}
                    }
                }
                if !cache_args.is_empty() {
                    self.writeln(&format!("#[cache({})]", cache_args.join(", ")));
                } else {
                    self.writeln("#[cache]");
                }
            }
            "SEO" => {
                // Generate SEO decorator
                let mut seo_args = Vec::new();
                for arg in &decorator.args {
                    match arg {
                        DecoratorArg::Named { name, value } => {
                            if let DecoratorArg::String(s) = value.as_ref() {
                                seo_args.push(format!("{} = \"{}\"", name, s));
                            }
                        }
                        _ => {}
                    }
                }
                if !seo_args.is_empty() {
                    self.writeln(&format!("#[seo({})]", seo_args.join(", ")));
                } else {
                    self.writeln("#[seo]");
                }
            }
            "AI" => {
                // Generate AI decorator
                if let Some(arg) = decorator.args.first() {
                    if let DecoratorArg::Named { name, value } = arg {
                        if name == "model" {
                            if let DecoratorArg::String(model) = value.as_ref() {
                                self.writeln(&format!("#[ai(model = \"{}\")]", model));
                            }
                        }
                    }
                }
            }
            "test" => {
                use crate::stdlib::testing::TestingStdlib;
                self.writeln(&TestingStdlib::generate_test_attribute());
            }
            "describe" => {
                use crate::stdlib::testing::TestingStdlib;
                let suite_name = decorator.args.first().and_then(|arg| {
                    if let DecoratorArg::String(s) = arg {
                        Some(s.as_str())
                    } else {
                        None
                    }
                });
                self.writeln(&TestingStdlib::generate_describe_attribute(suite_name));
            }
            "fixture" => {
                // Fixture wird als Setup/Teardown-Funktion generiert
                // Wird in der Funktion selbst behandelt
                let fixture_name = decorator
                    .args
                    .first()
                    .and_then(|arg| {
                        if let DecoratorArg::String(s) = arg {
                            Some(s.as_str())
                        } else {
                            None
                        }
                    })
                    .unwrap_or("default");
                // Fixture-Code wird später in der Funktion generiert
                self.writeln(&format!("// Fixture: {}", fixture_name));
            }
            "mock" => {
                // Mock wird als Trait-Implementierung generiert
                let trait_name = decorator
                    .args
                    .iter()
                    .find_map(|arg| {
                        if let DecoratorArg::Named { name, value } = arg {
                            if name == "trait" {
                                if let DecoratorArg::String(s) = value.as_ref() {
                                    return Some(s.as_str());
                                }
                            }
                        }
                        None
                    })
                    .unwrap_or("MockTrait");
                let struct_name = decorator
                    .args
                    .iter()
                    .find_map(|arg| {
                        if let DecoratorArg::Named { name, value } = arg {
                            if name == "struct" {
                                if let DecoratorArg::String(s) = value.as_ref() {
                                    return Some(s.as_str());
                                }
                            }
                        }
                        None
                    })
                    .unwrap_or("Mock");
                self.writeln(&format!("// Mock: {} for {}", struct_name, trait_name));
            }
            "Optimize" | "Flow" | "Generate" => {
                // Compiler directives or handled elsewhere - do not generate Rust attributes
            }
            _ => {
                // Generic decorator
                let mut args = Vec::new();
                for arg in &decorator.args {
                    args.push(self.decorator_arg_to_string(arg));
                }
                if !args.is_empty() {
                    self.writeln(&format!(
                        "#[{}({})]",
                        decorator.name.to_lowercase(),
                        args.join(", ")
                    ));
                } else {
                    self.writeln(&format!("#[{}]", decorator.name.to_lowercase()));
                }
            }
        }
    }

    fn decorator_arg_to_string(&self, arg: &DecoratorArg) -> String {
        match arg {
            DecoratorArg::String(s) => format!("\"{}\"", s),
            DecoratorArg::Number(n) => n.to_string(),
            DecoratorArg::Boolean(b) => b.to_string(),
            DecoratorArg::Identifier(id) => id.clone(),
            DecoratorArg::Named { name, value } => {
                format!("{} = {}", name, self.decorator_arg_to_string(value))
            }
        }
    }

    fn generate_struct(&mut self, struct_def: &Struct, _use_seaorm: bool) {
        // Prüfe auf @Derive oder @AutoDerive Decorators
        let mut has_auto_derive = false;
        let mut custom_derives = Vec::new();

        for decorator in &struct_def.decorators {
            if decorator.name == "AutoDerive" {
                has_auto_derive = true;
            } else if decorator.name == "Derive" {
                // Extrahiere Derive-Namen aus Args
                for arg in &decorator.args {
                    if let crate::parser::ast::DecoratorArg::Identifier(name) = arg {
                        custom_derives.push(name.clone());
                    }
                }
            }
        }

        // Generate struct with derives
        if has_auto_derive {
            // Alle sinnvollen Derives
            self.writeln("#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate, derive_more::Add, derive_more::Display, derive_more::From, derive_more::Into, derive_more::Deref)]");
        } else if !custom_derives.is_empty() {
            // Nur angegebene Derives
            let derive_list = custom_derives
                .iter()
                .map(|d| format!("derive_more::{}", d))
                .collect::<Vec<_>>()
                .join(", ");
            self.writeln(&format!(
                "#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate, {})]",
                derive_list
            ));
        } else {
            // Standard Derives
            self.writeln("#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]");
        }

        if struct_def.visibility == Visibility::Public {
            self.write("pub ");
        }
        self.write("struct ");
        self.write(&self.to_pascal_case(&struct_def.name));

        // Handle Generics
        if !struct_def.type_params.is_empty() {
            self.write("<");
            self.write(&struct_def.type_params.join(", "));
            self.write(">");
        }

        self.writeln(" {");
        self.indent();

        for field in &struct_def.fields {
            if field.visibility == Visibility::Public {
                self.write("    pub ");
            } else {
                self.write("    ");
            }

            // Prüfe auf Privacy Decorator
            let is_privacy = self.is_privacy_field(field);
            if is_privacy {
                #[cfg(feature = "privacy")]
                {
                    self.write("PrivacyWrapper<");
                }
            }

            self.write(&self.to_snake_case(&field.name));
            self.write(": ");
            self.generate_type(&field.field_type);

            if is_privacy {
                #[cfg(feature = "privacy")]
                {
                    self.write(">");
                }
            }

            self.writeln(",");
        }

        self.unindent();
        self.writeln("}");
    }

    fn generate_enum(&mut self, enum_def: &Enum) {
        self.writeln("#[derive(Debug, Clone, Serialize, Deserialize)]");
        if enum_def.visibility == Visibility::Public {
            self.write("pub ");
        }
        self.write("enum ");
        self.write(&self.to_pascal_case(&enum_def.name));
        self.writeln(" {");
        self.indent();

        for (i, variant) in enum_def.variants.iter().enumerate() {
            if i > 0 {
                self.writeln(",");
            }
            self.write(&self.to_pascal_case(&variant.name));

            if let Some(ref types) = variant.data {
                if types.len() == 1 {
                    self.write("(");
                    self.generate_type(&types[0]);
                    self.write(")");
                } else {
                    self.write("(");
                    for (j, t) in types.iter().enumerate() {
                        if j > 0 {
                            self.write(", ");
                        }
                        self.generate_type(t);
                    }
                    self.write(")");
                }
            }
        }

        self.writeln("");
        self.unindent();
        self.writeln("}");
    }

    fn generate_trait(&mut self, trait_def: &Trait) {
        if trait_def.visibility == Visibility::Public {
            self.write("pub ");
        }
        self.write("trait ");
        self.write(&self.to_pascal_case(&trait_def.name));

        // Generate generic type parameters
        if !trait_def.type_params.is_empty() {
            self.write("<");
            for (i, param) in trait_def.type_params.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(param);
            }
            self.write(">");
        }

        self.writeln(" {");
        self.indent();

        // Generate trait methods
        for method in &trait_def.methods {
            self.write("    fn ");
            self.write(&self.to_snake_case(&method.name));
            self.write("(");

            // Generate parameters
            for (i, param) in method.params.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(&self.to_snake_case(&param.name));
                self.write(": ");
                self.generate_type(&param.param_type);
            }

            self.write(")");

            // Generate return type
            if let Some(ref return_type) = method.return_type {
                self.write(" -> ");
                self.generate_type(return_type);
            }

            self.writeln(";");
        }

        self.unindent();
        self.writeln("}");
    }

    fn generate_impl(&mut self, impl_def: &Impl, framework: &Framework, use_seaorm: bool) {
        self.write("impl");

        // Generate generic type parameters
        if !impl_def.type_params.is_empty() {
            self.write("<");
            for (i, param) in impl_def.type_params.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.write(param);
            }
            self.write(">");
        }

        // Generate trait name (if not blank impl)
        if !impl_def.trait_name.is_empty() {
            self.write(" ");
            self.write(&self.to_pascal_case(&impl_def.trait_name));
            self.write(" for");
        }

        self.write(" ");

        // Generate the type being implemented
        self.generate_type(&impl_def.for_type);

        self.writeln(" {");
        self.indent();

        // Generate impl methods
        for method in &impl_def.methods {
            self.generate_function(method, framework, use_seaorm);
            self.writeln("");
        }

        self.unindent();
        self.writeln("}");
        if !impl_def.trait_name.is_empty() {
            self.write(" ");
            self.write(&self.to_pascal_case(&impl_def.trait_name));
            self.write(" for ");
        } else {
            self.write(" ");
        }

        // Generate type
        self.generate_type(&impl_def.for_type);

        self.writeln(" {");
        self.indent();

        // Generate impl methods
        for method in &impl_def.methods {
            self.generate_function(method, framework, use_seaorm);
            self.writeln("");
        }

        self.unindent();
        self.writeln("}");
    }

    fn generate_type_alias(&mut self, type_alias: &TypeAlias) {
        if type_alias.visibility == Visibility::Public {
            self.write("pub ");
        }
        self.write("type ");
        self.write(&self.to_pascal_case(&type_alias.name));
        self.write(" = ");
        self.generate_type(&type_alias.aliased_type);
        self.writeln(";");
    }

    fn generate_module(&mut self, module: &Module) {
        if module.visibility == Visibility::Public {
            self.write("pub ");
        }
        self.write("mod ");
        self.write(&self.to_snake_case(&module.name));
        self.writeln(" {");
        self.indent();

        let framework = self.framework.clone();
        let use_seaorm = self.use_seaorm;
        for item in &module.items {
            self.generate_item(item, &framework, use_seaorm);
        }

        self.unindent();
        self.writeln("}");
    }

    fn generate_type(&mut self, type_def: &Type) {
        match type_def {
            Type::String => self.write("String"),
            Type::Number => self.write("f64"),
            Type::Boolean => self.write("bool"),
            Type::Void => self.write("()"),
            Type::Null => self.write("Option<String>"),
            Type::Any => self.write("Box<dyn std::any::Any>"),
            Type::Named(name) => self.write(&self.to_pascal_case(name)),
            Type::List(inner) => {
                self.write("Vec<");
                self.generate_type(inner);
                self.write(">");
            }
            Type::Map { key, value } => {
                self.write("std::collections::HashMap<");
                self.generate_type(key);
                self.write(", ");
                self.generate_type(value);
                self.write(">");
            }
            Type::Tuple(types) => {
                self.write("(");
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_type(t);
                }
                self.write(")");
            }
            Type::Optional(inner) => {
                self.write("Option<");
                self.generate_type(inner);
                self.write(">");
            }
            Type::Result { ok, err } => {
                self.write("Result<");
                self.generate_type(ok);
                self.write(", ");
                self.generate_type(err);
                self.write(">");
            }
            Type::Function {
                params,
                return_type,
            } => {
                // Rust function type? e.g. fn(A) -> B
                // But typically used in closures: impl Fn(A) -> B
                // For now use Box<dyn Fn(...) -> ...>
                self.write("Box<dyn Fn(");
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_type(param);
                }
                self.write(") -> ");
                self.generate_type(return_type);
                self.write(">");
            }
            Type::Generic { name, params } => match name.as_str() {
                "List" => {
                    if let Some(param) = params.first() {
                        self.write("Vec<");
                        self.generate_type(param);
                        self.write(">");
                    } else {
                        self.write("Vec<()>");
                    }
                }
                "Map" => {
                    if params.len() >= 2 {
                        self.write("std::collections::HashMap<");
                        self.generate_type(&params[0]);
                        self.write(", ");
                        self.generate_type(&params[1]);
                        self.write(">");
                    } else {
                        self.write("std::collections::HashMap<String, ()>");
                    }
                }
                "Optional" => {
                    if let Some(param) = params.first() {
                        self.write("Option<");
                        self.generate_type(param);
                        self.write(">");
                    } else {
                        self.write("Option<()>");
                    }
                }
                _ => {
                    self.write(&self.to_pascal_case(name));
                    self.write("<");
                    for (i, param) in params.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.generate_type(param);
                    }
                    self.write(">");
                }
            },
        }
    }

    fn generate_block(&mut self, block: &Block) {
        // Check if we are inside a Pipeline function
        // Note: For full pipeline support, we should pass down context.
        // For MVP, we assume standard generation unless we implement the complex re-writer.
        // But wait, the user wants Pipeline support.
        // Let's integrate the PipelineOptimizer here!

        // This is a simplified integration. In a real compiler, the optimizer would
        // transform the AST before codegen. Here we do it on the fly or just use the optimizer
        // to guide generation.

        // Since we didn't transform the AST, we will generate standard code for now,
        // but we add a comment indicating where the optimization would happen.

        use crate::optimizer::pipeline::PipelineOptimizer;
        let optimizer = PipelineOptimizer::new();
        let parallel_groups = optimizer.identify_parallel_groups(block);

        if parallel_groups.iter().any(|g| g.len() > 1) {
            self.writeln("// VelinPipeline: Parallel execution block start");
            // Real implementation would re-order statements here.
            // For now, we just output them sequentially but grouped logically
        }

        for statement in &block.statements {
            self.generate_statement(statement);
        }
    }

    fn generate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_stmt) => {
                // Always make variables mutable to support VelinScript semantics where let is mutable
                self.write("let mut ");
                self.write(&self.to_snake_case(&let_stmt.name));

                if let Some(ref var_type) = let_stmt.var_type {
                    self.write(": ");
                    self.generate_type(var_type);
                }

                self.write(" = ");
                self.generate_expression(&let_stmt.value);
                self.writeln(";");
            }
            Statement::Return(ret_stmt) => {
                self.write("return");
                if let Some(ref value) = ret_stmt.value {
                    self.write(" ");
                    self.generate_expression(value);
                }
                self.writeln(";");
            }
            Statement::Expression(expr_stmt) => {
                self.generate_expression(&expr_stmt.expression);
                self.writeln(";");
            }
            Statement::If(if_stmt) => {
                self.write("if ");
                self.generate_expression(&if_stmt.condition);
                self.writeln(" {");
                self.indent();
                self.generate_block(&if_stmt.then_block);
                self.unindent();

                if let Some(ref else_block) = if_stmt.else_block {
                    self.writeln("} else {");
                    self.indent();
                    self.generate_block(else_block);
                    self.unindent();
                }

                self.writeln("}");
            }
            Statement::For(for_stmt) => {
                self.write("for ");
                self.write(&self.to_snake_case(&for_stmt.variable));
                self.write(" in ");
                self.generate_expression(&for_stmt.iterable);
                self.writeln(" {");
                self.indent();
                self.generate_block(&for_stmt.body);
                self.unindent();
                self.writeln("}");
            }
            Statement::While(while_stmt) => {
                self.write("while ");
                self.generate_expression(&while_stmt.condition);
                self.writeln(" {");
                self.indent();
                self.generate_block(&while_stmt.body);
                self.unindent();
                self.writeln("}");
            }
            Statement::Match(match_stmt) => {
                self.write("match ");
                self.generate_expression(&match_stmt.expression);
                self.writeln(" {");
                self.indent();

                for arm in &match_stmt.arms {
                    self.generate_pattern(&arm.pattern);

                    // Generate guard if present
                    if let Some(ref guard) = arm.guard {
                        self.write(" if ");
                        self.generate_expression(guard);
                    }

                    self.write(" => ");
                    self.generate_block(&arm.body);
                    self.writeln(",");
                }

                self.unindent();
                self.writeln("}");
            }
            Statement::Throw(throw_stmt) => {
                // In Rust we typically return Result::Err
                self.write("return Err(anyhow::anyhow!(");
                self.generate_expression(&throw_stmt.expression);
                self.writeln("));");
            }
            Statement::Break(_) => {
                self.writeln("break;");
            }
            Statement::Try(_) => {
                panic!("Try statement found after desugaring pass");
            }
        }
    }

    fn generate_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(lit) => {
                self.generate_literal(lit);
            }
            Expression::Identifier(name) => {
                self.write(&self.to_snake_case(name));
            }
            Expression::BinaryOp { left, op, right } => {
                // Special handling for 'in' operator
                match op {
                    BinaryOperator::In => {
                        self.generate_expression(right);
                        self.write(".contains(&");
                        self.generate_expression(left);
                        self.write(")");
                    }
                    _ => {
                        self.generate_expression(left);
                        self.write(" ");
                        self.generate_binary_operator(op);
                        self.write(" ");
                        self.generate_expression(right);
                    }
                }
            }
            Expression::UnaryOp { op, expr } => {
                self.generate_unary_operator(op);
                self.generate_expression(expr);
            }
            Expression::Assignment { target, value } => {
                self.generate_expression(target);
                self.write(" = ");
                self.generate_expression(value);
            }
            Expression::StructLiteral { name, fields } => {
                self.write(name);
                self.write(" {");
                for (i, (field_name, field_expr)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write(field_name);
                    self.write(": ");
                    if let Expression::Literal(Literal::String(s)) = field_expr {
                        let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                        self.write(&format!("\"{}\".to_string()", escaped));
                    } else {
                        self.generate_expression(field_expr);
                    }
                }
                self.write("}");
            }
            Expression::MapLiteral(fields) => {
                self.write("std::collections::HashMap::from([");
                for (i, (key, value)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write("(");
                    self.write(&format!("\"{}\".to_string()", key));
                    self.write(", ");
                    if let Expression::Literal(Literal::String(s)) = value {
                        let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                        self.write(&format!("\"{}\".to_string().into()", escaped));
                    } else {
                        self.generate_expression(value);
                        self.write(".into()");
                    }
                    self.write(")");
                }
                self.write("])");
            }
            Expression::ListLiteral(elements) => {
                self.write("vec![");
                for (i, expr) in elements.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    if let Expression::Literal(Literal::String(s)) = expr {
                        let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                        self.write(&format!("\"{}\".to_string()", escaped));
                    } else {
                        self.generate_expression(expr);
                    }
                }
                self.write("]");
            }
            Expression::Call { callee, args } => {
                // Check if this is assert() function
                if let Expression::Identifier(name) = callee.as_ref() {
                    if name == "assert" {
                        if let Some(arg) = args.first() {
                            // Check if it's a binary comparison (==, !=)
                            if let Expression::BinaryOp { left, op, right } = arg {
                                match op {
                                    BinaryOperator::Eq => {
                                        self.write("assert_eq!(");
                                        self.generate_expression(left);
                                        self.write(", ");
                                        self.generate_expression(right);
                                        self.write(")");
                                        return;
                                    }
                                    BinaryOperator::NotEq => {
                                        self.write("assert_ne!(");
                                        self.generate_expression(left);
                                        self.write(", ");
                                        self.generate_expression(right);
                                        self.write(")");
                                        return;
                                    }
                                    _ => {
                                        // Other binary ops -> assert!(condition)
                                        self.write("assert!(");
                                        self.generate_expression(arg);
                                        self.write(")");
                                        return;
                                    }
                                }
                            } else {
                                // Simple assert
                                self.write("assert!(");
                                self.generate_expression(arg);
                                self.write(")");
                                return;
                            }
                        }
                    } else if name == "print" {
                        self.write("println!(\"{:?}\", ");
                        if let Some(arg) = args.first() {
                            self.generate_expression(arg);
                        }
                        self.write(")");
                        return;
                    }
                }

                // Check if this is a standard library function call
                if let Expression::Member { object, member } = callee.as_ref() {
                    // Check for HTTP Client method calls
                    use crate::stdlib::http_client::is_http_client_method;

                    if is_http_client_method(member) {
                        self.generate_http_client_call(object, member, args);
                        return;
                    }

                    // Check for collections method calls: list.filter(), map.keys(), etc.

                    // Check if this is a List method
                    if self.is_list_method(member) {
                        self.generate_collections_call(object, member, args, "list");
                        return;
                    }

                    // Check if this is a Map method
                    if self.is_map_method(member) {
                        self.generate_collections_call(object, member, args, "map");
                        return;
                    }

                    // Check if this is a Set method
                    if self.is_set_method(member) {
                        self.generate_collections_call(object, member, args, "set");
                        return;
                    }

                    if let Expression::Identifier(obj_name) = object.as_ref() {
                        if obj_name == "db" {
                            self.generate_db_call(member, args);
                            return;
                        } else if obj_name == "backup" {
                            self.generate_backup_call(member, args);
                            return;
                        } else if obj_name == "rollback" {
                            self.generate_rollback_call(member, args);
                            return;
                        } else if obj_name == "file" {
                            self.generate_file_call(member, args);
                            return;
                        } else if obj_name == "list" {
                            self.generate_iterator_call(member, args, object);
                            return;
                        } else if obj_name == "string" {
                            self.generate_string_call(member, args);
                            return;
                        } else if obj_name == "math" {
                            self.generate_math_call(member, args);
                            return;
                        } else if obj_name == "date" {
                            self.generate_date_call(member, args);
                            return;
                        } else if obj_name == "fs" {
                            self.generate_fs_call(member, args);
                            return;
                        } else if obj_name == "ml" {
                            self.generate_ml_call(member, args);
                            return;
                        } else if obj_name == "alerting" {
                            self.generate_alerting_call(member, args);
                            return;
                        } else if obj_name == "csv" {
                            self.generate_csv_call(member, args);
                            return;
                        } else if obj_name == "redis" {
                            self.generate_redis_call(member, args);
                            return;
                        } else if obj_name == "flow" {
                            self.generate_flow_call(member, args);
                            return;
                        } else if obj_name == "llm" {
                            self.generate_llm_call(member, args);
                            return;
                        } else if obj_name == "embedding" {
                            self.generate_embedding_call(member, args);
                            return;
                        } else if obj_name == "agent" {
                            self.generate_agent_call(member, args);
                            return;
                        } else if obj_name == "process" {
                            self.generate_process_call(member, args);
                            return;
                        } else if obj_name == "sandbox" {
                            self.generate_sandbox_call(member, args);
                            return;
                        } else if obj_name == "websocket" {
                            self.generate_websocket_call(member, args);
                            return;
                        } else if obj_name == "utils" {
                            self.generate_utils_call(member, args);
                            return;
                        } else if obj_name == "log" {
                            self.generate_log_call(member, args);
                            return;
                        } else if obj_name == "queue" {
                            self.generate_queue_call(member, args);
                            return;
                        } else if obj_name == "mongodb" {
                            #[cfg(feature = "mongodb")]
                            self.generate_mongodb_call(member, args);
                            #[cfg(not(feature = "mongodb"))]
                            self.write("// MongoDB support not enabled");
                            return;
                        } else if obj_name == "event_bus" {
                            self.generate_event_bus_call(member, args);
                            return;
                        } else if obj_name == "encryption" {
                            #[cfg(feature = "security")]
                            self.generate_encryption_call(member, args);
                            #[cfg(not(feature = "security"))]
                            self.write("// Encryption support not enabled");
                            return;
                        } else if obj_name == "analytics" {
                            self.generate_analytics_call(member, args);
                            return;
                        } else if obj_name == "auth" {
                            self.generate_auth_call(member, args);
                            return;
                        } else if obj_name == "compression" {
                            self.generate_compression_call(member, args);
                            return;
                        } else if obj_name == "crypto" {
                            self.generate_crypto_call(member, args);
                            return;
                        } else if obj_name == "email" {
                            #[cfg(feature = "smtp")]
                            self.generate_email_call(member, args);
                            #[cfg(not(feature = "smtp"))]
                            self.write("// Email support not enabled");
                            return;
                        } else if obj_name == "encoding" {
                            self.generate_encoding_call(member, args);
                            return;
                        } else if obj_name == "file_storage" {
                            self.generate_file_call(member, args);
                            return;
                        } else if obj_name == "geolocation" {
                            self.generate_geolocation_call(member, args);
                            return;
                        } else if obj_name == "http" {
                            self.generate_http_call(member, args);
                            return;
                        } else if obj_name == "i18n" {
                            self.generate_i18n_call(member, args);
                            return;
                        } else if obj_name == "jwt" {
                            self.generate_jwt_call(member, args);
                            return;
                        } else if obj_name == "logger" {
                            self.generate_log_call(member, args);
                            return;
                        } else if obj_name == "pdf" {
                            self.generate_pdf_call(member, args);
                            return;
                        } else if obj_name == "regex" || obj_name == "regex_lib" {
                            self.generate_regex_lib_call(member, args);
                            return;
                        } else if obj_name == "search" {
                            self.generate_search_call(member, args);
                            return;
                        } else if obj_name == "string_utils" {
                            self.generate_string_utils_call(member, args);
                            return;
                        } else if obj_name == "system" {
                            self.generate_system_call(member, args);
                            return;
                        } else if obj_name == "time" || obj_name == "time_lib" {
                            self.generate_time_lib_call(member, args);
                            return;
                        } else if obj_name == "validation" {
                            self.generate_validation_call(member, args);
                            return;
                        } else if obj_name == "xml" {
                            self.generate_xml_call(member, args);
                            return;
                        }
                    } else if let Expression::Member {
                        object: inner_obj,
                        member: inner_member,
                    } = object.as_ref()
                    {
                        // Nested member access wie list.groupBy
                        if let Expression::Identifier(inner_name) = inner_obj.as_ref() {
                            if inner_name == "list" {
                                self.generate_list_extension_call(
                                    inner_member,
                                    member,
                                    args,
                                    object,
                                );
                                return;
                            } else if inner_name == "string" {
                                self.generate_string_extension_call(
                                    inner_member,
                                    member,
                                    args,
                                    object,
                                );
                                return;
                            } else if inner_name == "agent" {
                                self.generate_agent_nested_call(inner_member, member, args);
                                return;
                            }
                        }
                    }
                }

                self.generate_expression(callee);
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");

                // Add await for async functions
                if let Expression::Identifier(name) = callee.as_ref() {
                    if name == "process_workflow" {
                        self.write(".await");
                    }
                }
            }
            Expression::Member { object, member } => {
                // Check if this is a Result method call
                use crate::stdlib::result::ResultStdlib;
                if ResultStdlib::is_result_method(member) {
                    // This will be handled as a method call, so just generate the member access
                    // The actual method call will be generated in the Call expression
                    self.generate_expression(object);
                    self.write(".");
                    self.write(&self.to_snake_case(member));
                } else {
                    self.generate_expression(object);
                    self.write(".");
                    self.write(&self.to_snake_case(member));
                }
            }
            Expression::Index { object, index } => {
                self.generate_expression(object);
                self.write("[");
                self.generate_expression(index);
                self.write("]");
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                self.write("if ");
                self.generate_expression(condition);
                self.write(" { ");
                self.generate_expression(then_expr);
                self.write(" } else { ");
                self.generate_expression(else_expr);
                self.write(" }");
            }
            Expression::Block(block) => {
                self.writeln("{");
                self.indent();
                self.generate_block(block);
                self.unindent();
                self.write("}");
            }
            Expression::Await { expr } => {
                self.generate_expression(expr);
                self.write(".await");
            }
            Expression::Lambda {
                params,
                return_type: _,
                body,
            } => {
                // Generate Rust closure: |param1, param2| { body }
                self.write("|");
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write(&self.to_snake_case(&param.name));
                    self.write(": ");
                    self.generate_type(&param.param_type);
                }
                self.write("| ");

                // Generate body
                match body.as_ref() {
                    Expression::Block(block) => {
                        self.writeln("{");
                        self.indent();
                        self.generate_block(block);
                        self.unindent();
                        self.write("}");
                    }
                    _ => {
                        // Single expression - wrap in braces
                        self.write("{ ");
                        self.generate_expression(body);
                        self.write(" }");
                    }
                }
            }
            Expression::LLMCall { method, args } => {
                // Generiert: llm_client.analyze(text) mit Prompt-Optimierung
                self.write("llm_client.");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(").await");
            }
            Expression::FormatString { parts } => {
                // Generate Rust format! macro
                self.write("format!(");

                // Build format string and arguments
                let mut format_str = String::new();
                let mut args = Vec::new();
                let mut arg_index = 0;

                for part in parts {
                    match part {
                        FormatStringPart::Text(text) => {
                            // Escape special characters for format! macro
                            let escaped = text.replace('{', "{{").replace('}', "}}");
                            format_str.push_str(&escaped);
                        }
                        FormatStringPart::Expression(expr) => {
                            format_str.push_str(&format!("{{{}}}", arg_index));
                            args.push(expr);
                            arg_index += 1;
                        }
                    }
                }

                // Write format string
                let escaped_format = format_str.replace('\\', "\\\\").replace('"', "\\\"");
                self.write(&format!("\"{}\"", escaped_format));

                // Write arguments
                for arg in args {
                    self.write(", ");
                    self.generate_expression(arg);
                }

                self.write(")");
            }
            Expression::GenericConstructor {
                name,
                type_params,
                args: _args,
            } => {
                match name.as_str() {
                    "Map" => {
                        if type_params.len() == 2 {
                            self.write("std::collections::HashMap::<");
                            self.generate_type(&type_params[0]);
                            self.write(", ");
                            self.generate_type(&type_params[1]);
                            self.write(">::new()");
                        } else {
                            self.write("std::collections::HashMap::new()");
                        }
                    }
                    "List" => {
                        if type_params.len() == 1 {
                            self.write("Vec::<");
                            self.generate_type(&type_params[0]);
                            self.write(">::new()");
                        } else {
                            self.write("Vec::new()");
                        }
                    }
                    _ => {
                        // Generic struct constructor
                        self.write(&self.to_pascal_case(name));
                        if !type_params.is_empty() {
                            self.write("::<");
                            for (i, tp) in type_params.iter().enumerate() {
                                if i > 0 {
                                    self.write(", ");
                                }
                                self.generate_type(tp);
                            }
                            self.write(">");
                        }
                        self.write("::new()");
                    }
                }
            }
        }
    }

    fn generate_literal(&mut self, lit: &Literal) {
        match lit {
            Literal::String(s) => {
                // Escape string for Rust
                let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                self.write(&format!("\"{}\"", escaped));
            }
            Literal::Number(n) => {
                let s = n.to_string();
                if s.contains('.') {
                    self.write(&s);
                } else {
                    self.write(&format!("{}.0", s));
                }
            }
            Literal::Boolean(b) => {
                self.write(&b.to_string());
            }
            Literal::Null => {
                self.write("None");
            }
        }
    }

    fn generate_binary_operator(&mut self, op: &BinaryOperator) {
        match op {
            BinaryOperator::Add => self.write("+"),
            BinaryOperator::Subtract => self.write("-"),
            BinaryOperator::Multiply => self.write("*"),
            BinaryOperator::Divide => self.write("/"),
            BinaryOperator::Modulo => self.write("%"),
            BinaryOperator::Eq => self.write("=="),
            BinaryOperator::NotEq => self.write("!="),
            BinaryOperator::Lt => self.write("<"),
            BinaryOperator::Gt => self.write(">"),
            BinaryOperator::LtEq => self.write("<="),
            BinaryOperator::GtEq => self.write(">="),
            BinaryOperator::And => self.write("&&"),
            BinaryOperator::Or => self.write("||"),
            BinaryOperator::In => self.write(".contains(&"), // Will be handled specially in binary op generation
        }
    }

    fn generate_unary_operator(&mut self, op: &UnaryOperator) {
        match op {
            UnaryOperator::Not => self.write("!"),
            UnaryOperator::Minus => self.write("-"),
        }
    }

    fn generate_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Literal(lit) => {
                self.generate_literal(lit);
            }
            Pattern::Identifier(name) => {
                self.write(&self.to_snake_case(name));
            }
            Pattern::Wildcard => {
                self.write("_");
            }
            Pattern::Tuple(patterns) => {
                self.write("(");
                for (i, p) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_pattern(p);
                }
                self.write(")");
            }
            Pattern::Struct { name, fields } => {
                self.write(&self.to_pascal_case(name));
                self.write(" { ");
                for (i, (field_name, field_pattern)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write(&self.to_snake_case(field_name));
                    self.write(": ");
                    self.generate_pattern(field_pattern);
                }
                self.write(" }");
            }
            Pattern::EnumVariant { name, data } => {
                // Convert "Enum::Variant" to "Enum::Variant"
                let parts: Vec<&str> = name.split("::").collect();
                if parts.len() == 2 {
                    self.write(&self.to_pascal_case(parts[0]));
                    self.write("::");
                    self.write(&self.to_pascal_case(parts[1]));
                } else {
                    self.write(&self.to_pascal_case(name));
                }

                if let Some(ref patterns) = data {
                    self.write("(");
                    for (i, p) in patterns.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.generate_pattern(p);
                    }
                    self.write(")");
                }
            }
            Pattern::Range {
                start,
                end,
                inclusive,
            } => {
                self.generate_expression(start);
                if *inclusive {
                    self.write("..=");
                } else {
                    self.write("..");
                }
                self.generate_expression(end);
            }
            Pattern::Or(patterns) => {
                for (i, p) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.write(" | ");
                    }
                    self.generate_pattern(p);
                }
            }
        }
    }

    // Helper methods
    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn writeln(&mut self, s: &str) {
        self.write(s);
        self.write("\n");
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn unindent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_uppercase() && !result.is_empty() {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        }

        result
    }

    fn to_pascal_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize = true;

        for ch in s.chars() {
            if ch == '_' {
                capitalize = true;
            } else if capitalize {
                result.push(ch.to_uppercase().next().unwrap_or(ch));
                capitalize = false;
            } else {
                result.push(ch);
            }
        }

        result
    }

    #[allow(dead_code)]
    fn has_privacy_fields(&self, struct_def: &Struct) -> bool {
        struct_def.fields.iter().any(|f| self.is_privacy_field(f))
    }

    fn is_privacy_field(&self, field: &StructField) -> bool {
        // StructField has no decorators field
        // Privacy decorators would be on the struct itself, not individual fields

        // Prüfe Feldname auf PII-Keywords
        let field_lower = field.name.to_lowercase();
        let pii_keywords = vec![
            "email",
            "phone",
            "ssn",
            "passport",
            "credit_card",
            "ip",
            "address",
        ];
        pii_keywords
            .iter()
            .any(|keyword| field_lower.contains(keyword))
    }

    fn generate_db_call(&mut self, method: &str, args: &[Expression]) {
        if self.use_seaorm {
            #[cfg(feature = "sea-orm")]
            {
                match method {
                    "find" => {
                        if args.len() >= 2 {
                            if let Expression::Identifier(entity) = &args[0] {
                                let entity_name = &self.to_pascal_case(entity);
                                self.write(&format!("{}::Entity::find_by_id(", entity_name));
                                self.generate_expression(&args[1]);
                                self.write(").one(&db).await");
                            } else {
                                self.write("// SeaORM find requires entity type");
                            }
                        }
                    }
                    "findAll" | "find_all" => {
                        if let Some(Expression::Identifier(entity)) = args.first() {
                            let entity_name = &self.to_pascal_case(entity);
                            self.write(&format!("{}::Entity::find().all(&db).await", entity_name));
                        }
                    }
                    "save" => {
                        if let Some(arg) = args.first() {
                            self.write("{\n        let active_model = ");
                            self.generate_expression(arg);
                            self.write(".into();\n        active_model.insert(&db).await\n    }");
                        }
                    }
                    "delete" => {
                        if args.len() >= 2 {
                            if let Expression::Identifier(entity) = &args[0] {
                                let entity_name = &self.to_pascal_case(entity);
                                self.write(&format!("{}::Entity::delete_by_id(", entity_name));
                                self.generate_expression(&args[1]);
                                self.write(").exec(&db).await");
                            }
                        }
                    }
                    "update" => {
                        if args.len() >= 3 {
                            if let Expression::Identifier(entity) = &args[0] {
                                let entity_name = &self.to_pascal_case(entity);
                                self.write(&format!("{{\n        let mut active_model: {}::ActiveModel = {}::Entity::find_by_id(", entity_name, entity_name));
                                self.generate_expression(&args[1]);
                                self.write(")\n            .one(&db)\n            .await?\n            .ok_or_else(|| anyhow::anyhow!(\"Entity not found\"))?\n            .into();\n        // Update fields\n        active_model.update(&db).await\n    }}");
                            }
                        }
                    }
                    "query" => {
                        if let Some(Expression::Literal(crate::parser::ast::Literal::String(
                            query,
                        ))) = args.first()
                        {
                            self.write("sea_orm::Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, ");
                            self.generate_expression(&args[0]);
                            self.write(", vec![]).execute(&db).await");
                        } else {
                            self.write("// db.query() requires SQL string");
                        }
                    }
                    "transaction" => {
                        self.write("db.transaction(|txn| Box::pin(async move {\n        ");
                        if let Some(block_expr) = args.first() {
                            // This would need to handle a block expression
                            self.write("// Transaction block");
                        }
                        self.write("\n        Ok(())\n    })).await");
                    }
                    _ => {
                        self.write("db.");
                        self.write(&self.to_snake_case(method));
                        self.write("(");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                self.write(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.write(")");
                    }
                }
                return;
            }
        }

        // Fallback to sqlx/default
        match method {
            "find" => {
                // db.find(User, id) -> db.find::<User>(id).await
                if args.len() >= 2 {
                    if let Expression::Identifier(entity) = &args[0] {
                        self.write("db.find::<");
                        self.write(&self.to_pascal_case(entity));
                        self.write(">(");
                        self.generate_expression(&args[1]);
                        self.write(").await");
                    } else {
                        self.write("db.find(");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                self.write(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.write(").await");
                    }
                } else {
                    self.write("db.find(");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.generate_expression(arg);
                    }
                    self.write(").await");
                }
            }
            "findAll" | "find_all" => {
                // db.findAll(User) -> db.find_all::<User>().await
                if let Some(Expression::Identifier(entity)) = args.first() {
                    self.write("db.find_all::<");
                    self.write(&self.to_pascal_case(entity));
                    self.write(">().await");
                } else {
                    self.write("db.find_all().await");
                }
            }
            "save" => {
                // db.save(entity) -> db.save(entity).await
                self.write("db.save(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(").await");
            }
            "delete" => {
                // db.delete(User, id) -> db.delete::<User>(id).await
                if args.len() >= 2 {
                    if let Expression::Identifier(entity) = &args[0] {
                        self.write("db.delete::<");
                        self.write(&self.to_pascal_case(entity));
                        self.write(">(");
                        self.generate_expression(&args[1]);
                        self.write(").await");
                    } else {
                        self.write("db.delete(");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                self.write(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.write(").await");
                    }
                } else {
                    self.write("db.delete(");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.generate_expression(arg);
                    }
                    self.write(").await");
                }
            }
            "update" => {
                // db.update(User, id, entity) -> db.update(entity, id).await
                if args.len() >= 3 {
                    if let Expression::Identifier(entity) = &args[0] {
                        self.write("db.update::<");
                        self.write(&self.to_pascal_case(entity));
                        self.write(">(");
                        if args.len() >= 3 {
                            self.generate_expression(&args[2]); // entity
                            self.write(", ");
                            self.generate_expression(&args[1]); // id
                        }
                        self.write(").await");
                    } else {
                        self.write("db.update(");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                self.write(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.write(").await");
                    }
                } else {
                    self.write("db.update(");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.generate_expression(arg);
                    }
                    self.write(").await");
                }
            }
            "query" => {
                // SECURITY: SQL-Parameterisierung erzwingen
                // db.query("SELECT * FROM users WHERE id = $1", id) -> sqlx::query! mit Parametern
                if let Some(Expression::Literal(crate::parser::ast::Literal::String(_))) =
                    args.first()
                {
                    // Wenn nur String, warnen aber Prepared Statement verwenden
                    self.write("// SECURITY: Use parameterized queries for user input\n");
                    self.write("sqlx::query(");
                    self.generate_expression(&args[0]);
                    if args.len() > 1 {
                        // Parameter vorhanden - verwende query_as oder query_with
                        self.write(")");
                        for (i, param) in args.iter().skip(1).enumerate() {
                            if i == 0 {
                                self.write(".bind(");
                            } else {
                                self.write(".bind(");
                            }
                            self.generate_expression(param);
                            self.write(")");
                        }
                        self.write(".execute(&db).await");
                    } else {
                        // Keine Parameter - OK für statische Queries
                        self.write(").execute(&db).await");
                    }
                } else {
                    // Dynamische Query - muss parameterisiert sein
                    self.write("// SECURITY: Dynamic queries must use parameters\n");
                    self.write("sqlx::query(");
                    if let Some(arg) = args.first() {
                        self.generate_expression(arg);
                    }
                    if args.len() > 1 {
                        for param in args.iter().skip(1) {
                            self.write(".bind(");
                            self.generate_expression(param);
                            self.write(")");
                        }
                    }
                    self.write(").execute(&db).await");
                }
            }
            "transaction" => {
                // db.transaction(block) -> let mut tx = db.begin().await?; ... tx.commit().await?
                self.write("let mut tx = db.begin().await?;\n    ");
                if let Some(_block_expr) = args.first() {
                    // This would need to handle a block expression
                    self.write("// Transaction block - implement block execution here");
                }
                self.write("\n    tx.commit().await?");
            }
            _ => {
                // Generic method call
                self.write("db.");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_backup_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "create" => {
                self.write("backup::create_backup(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "restore" => {
                self.write("backup::restore_backup(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "list" => {
                self.write("backup::list_backups()");
            }
            "delete" => {
                self.write("backup::delete_backup(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "verify" => {
                self.write("backup::verify_backup(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            _ => {
                self.write("backup.");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_rollback_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "beginTransaction" | "begin_transaction" => {
                self.write("rollback::begin_transaction()");
            }
            "commit" | "commitTransaction" => {
                self.write("rollback::commit_transaction(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "rollback" | "rollbackTransaction" => {
                self.write("rollback::rollback_transaction(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "createVersion" | "create_version" => {
                self.write("rollback::create_version(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "rollbackToVersion" | "rollback_to_version" => {
                self.write("rollback::rollback_to_version(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "createSnapshot" | "create_snapshot" => {
                self.write("rollback::create_snapshot(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "rollbackToSnapshot" | "rollback_to_snapshot" => {
                self.write("rollback::rollback_to_snapshot(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            _ => {
                self.write("rollback.");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_file_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "read" | "readFile" => {
                self.write("fileio::read_file(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "write" | "writeFile" => {
                self.write("fileio::write_file(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            "asyncRead" | "asyncReadFile" => {
                self.write("async_read_file(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(").await");
            }
            "asyncWrite" | "asyncWriteFile" => {
                self.write("async_write_file(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(").await");
            }
            _ => {
                self.write("file.");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_string_utils_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "contains" => {
                if args.len() >= 2 {
                    self.generate_expression(&args[0]);
                    self.write(".contains(");
                    self.generate_expression(&args[1]);
                    self.write(")");
                }
            }
            "startsWith" | "starts_with" => {
                if args.len() >= 2 {
                    self.generate_expression(&args[0]);
                    self.write(".starts_with(");
                    self.generate_expression(&args[1]);
                    self.write(")");
                }
            }
            "endsWith" | "ends_with" => {
                if args.len() >= 2 {
                    self.generate_expression(&args[0]);
                    self.write(".ends_with(");
                    self.generate_expression(&args[1]);
                    self.write(")");
                }
            }
            "replace" => {
                if args.len() >= 3 {
                    self.generate_expression(&args[0]);
                    self.write(".replace(");
                    self.generate_expression(&args[1]);
                    self.write(", ");
                    self.generate_expression(&args[2]);
                    self.write(")");
                }
            }
            "trim" => {
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                    self.write(".trim()");
                }
            }
            "toLower" | "to_lower" | "toLowerCase" => {
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                    self.write(".to_lowercase()");
                }
            }
            "toUpper" | "to_upper" | "toUpperCase" => {
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                    self.write(".to_uppercase()");
                }
            }
            "split" => {
                if args.len() >= 2 {
                    self.generate_expression(&args[0]);
                    self.write(".split(");
                    self.generate_expression(&args[1]);
                    self.write(").collect::<Vec<&str>>()");
                }
            }
            _ => {
                self.write("string_utils::");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_system_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "exec" | "execute" => {
                self.write("std::process::Command::new(\"sh\").arg(\"-c\").arg(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(").output().expect(\"failed to execute process\")");
            }
            "env" | "getEnv" => {
                self.write("std::env::var(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(").unwrap_or_default()");
            }
            "setEnv" => {
                self.write("std::env::set_var(");
                if args.len() >= 2 {
                    self.generate_expression(&args[0]);
                    self.write(", ");
                    self.generate_expression(&args[1]);
                }
                self.write(")");
            }
            "exit" => {
                self.write("std::process::exit(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                } else {
                    self.write("0");
                }
                self.write(")");
            }
            "cwd" | "currentDir" => {
                self.write("std::env::current_dir().unwrap_or_default()");
            }
            "os" => {
                self.write("std::env::consts::OS");
            }
            "arch" => {
                self.write("std::env::consts::ARCH");
            }
            _ => {
                self.write("system::");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_time_lib_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "now" => {
                self.write("std::time::SystemTime::now()");
            }
            "now_millis" | "timestamp" => {
                self.write("std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()");
            }
            "sleep" => {
                self.write("std::thread::sleep(std::time::Duration::from_millis(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                    self.write(" as u64");
                }
                self.write("))");
            }
            "format" => {
                // Requires chrono, but we can try to use a simple formatter or assume chrono
                // For now, let's assume chrono is available or generate a placeholder that works with std
                // Actually, let's assume the user has chrono if they are doing time formatting
                self.write("chrono::Local::now().format(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(").to_string()");
            }
            _ => {
                self.write("time::");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_xml_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "parse" => {
                self.write("// XML Parsing requires 'quick-xml' crate\n");
                self.write("quick_xml::events::Event::Text(");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(".into())");
            }
            "stringify" => {
                self.write("// Simple XML serialization\n");
                self.write("format!(\"<root>{}</root>\", ");
                if let Some(arg) = args.first() {
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            _ => {
                self.write("xml::");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_iterator_call(&mut self, method: &str, args: &[Expression], object: &Expression) {
        match method {
            "groupBy" | "group_by" => {
                if let Some(key_fn) = args.first() {
                    self.write("itertools::Itertools::group_by(");
                    self.generate_expression(object);
                    self.write(".iter(), |x| ");
                    self.generate_expression(key_fn);
                    self.write(").into_iter().map(|(k, v)| (k, v.collect::<Vec<_>>())).collect()");
                } else {
                    self.write("itertools::Itertools::group_by(");
                    self.generate_expression(object);
                    self.write(".iter(), |x| x).collect()");
                }
            }
            "sorted" => {
                self.write("itertools::Itertools::sorted(");
                self.generate_expression(object);
                self.write(".iter()).collect()");
            }
            "chunks" => {
                if let Some(size) = args.first() {
                    self.generate_expression(object);
                    self.write(".chunks(");
                    self.generate_expression(size);
                    self.write(" as usize).map(|chunk| chunk.to_vec()).collect()");
                } else {
                    self.generate_expression(object);
                    self.write(".chunks(1).collect()");
                }
            }
            "multizip" => {
                if args.len() >= 2 {
                    self.write("itertools::multizip((");
                    self.generate_expression(object);
                    self.write(".iter(), ");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.generate_expression(arg);
                        self.write(".iter()");
                    }
                    self.write(")).collect()");
                } else {
                    self.generate_expression(object);
                    self.write(".iter().collect()");
                }
            }
            _ => {
                self.generate_expression(object);
                self.write(".");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_list_extension_call(
        &mut self,
        _list_method: &str,
        method: &str,
        args: &[Expression],
        object: &Expression,
    ) {
        // list.groupBy, list.sorted, etc.
        match method {
            "groupBy" | "group_by" => {
                if let Some(key_fn) = args.first() {
                    self.generate_expression(object);
                    self.write(".iter().group_by(|x| ");
                    self.generate_expression(key_fn);
                    self.write(").into_iter().map(|(k, v)| (k, v.collect())).collect()");
                }
            }
            "sorted" => {
                self.generate_expression(object);
                self.write(".iter().sorted().collect()");
            }
            "first" => {
                self.generate_expression(object);
                self.write(".first().cloned()");
            }
            "last" => {
                self.generate_expression(object);
                self.write(".last().cloned()");
            }
            _ => {
                self.generate_expression(object);
                self.write(".");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_string_extension_call(
        &mut self,
        _string_method: &str,
        method: &str,
        args: &[Expression],
        object: &Expression,
    ) {
        // string.camelCase, etc.
        match method {
            "camelCase" | "camel_case" => {
                self.write("camel_case(");
                self.generate_expression(object);
                self.write(")");
            }
            "snakeCase" | "snake_case" => {
                self.write("snake_case(");
                self.generate_expression(object);
                self.write(")");
            }
            _ => {
                self.generate_expression(object);
                self.write(".");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    fn generate_validation_code(&mut self, function: &Function, framework: &Framework) {
        // Check if function has @Validate decorator or needs auto-validation
        let has_validate_decorator = function.decorators.iter().any(|d| {
            matches!(
                d.name.as_str(),
                "Validate" | "@Validate" | "Validation" | "@Validation"
            )
        });

        if !has_validate_decorator && !self.has_validation {
            return;
        }

        // Generate validator initialization
        self.write("    ");
        self.writeln("let mut validator = Validator::new();");
        self.writeln("");

        // Generate validation for each parameter
        for param in &function.params {
            let param_name = &self.to_snake_case(&param.name);

            match param.param_type {
                Type::String => {
                    self.write("    ");
                    self.writeln(&format!(
                        "validator.required(\"{}\", Some(&{}));",
                        param_name, param_name
                    ));
                    // Auto-validate email if parameter name contains "email"
                    if param_name.to_lowercase().contains("email") {
                        self.write("    ");
                        self.writeln(&format!(
                            "validator.email(\"{}\", &{});",
                            param_name, param_name
                        ));
                    }
                }
                Type::Number => {
                    // Numbers are always present (not Option), so no required check
                    // But we could add min/max validation if decorators specify
                }
                _ => {
                    // For other types, check if it's an Option
                    if let Type::Generic { name, .. } = &param.param_type {
                        if name == "Option" {
                            // Option types don't need required validation
                        } else {
                            // Custom types - might need validation
                        }
                    }
                }
            }
        }

        self.writeln("");

        // Generate error handling
        self.write("    ");
        self.writeln("if !validator.is_valid() {");
        self.write("        ");

        match framework {
            Framework::Axum => {
                self.writeln("let errors: Vec<serde_json::Value> = validator.errors()");
                self.write("            ");
                self.writeln(".iter()");
                self.write("            ");
                self.writeln(".map(|e| serde_json::json!({");
                self.write("                ");
                self.writeln("\"field\": e.field.clone(),");
                self.write("                ");
                self.writeln("\"message\": e.message.clone()");
                self.write("            ");
                self.writeln("}))");
                self.write("            ");
                self.writeln(".collect();");
                self.write("        ");
                self.writeln("return Err(anyhow::anyhow!(\"Validation failed\")).map_err(|e| {");
                self.write("            ");
                self.writeln("let response = serde_json::json!({");
                self.write("                ");
                self.writeln("\"error\": \"Validation failed\",");
                self.write("                ");
                self.writeln("\"errors\": errors");
                self.write("            ");
                self.writeln("});");
                self.write("            ");
                self.writeln(
                    "(axum::http::StatusCode::BAD_REQUEST, axum::Json(response)).into_response()",
                );
                self.write("        ");
                self.writeln("});");
            }
            Framework::Actix => {
                self.writeln("let errors: Vec<serde_json::Value> = validator.errors()");
                self.write("            ");
                self.writeln(".iter()");
                self.write("            ");
                self.writeln(".map(|e| serde_json::json!({");
                self.write("                ");
                self.writeln("\"field\": e.field.clone(),");
                self.write("                ");
                self.writeln("\"message\": e.message.clone()");
                self.write("            ");
                self.writeln("}))");
                self.write("            ");
                self.writeln(".collect();");
                self.write("        ");
                self.writeln(
                    "return Ok(actix_web::HttpResponse::BadRequest().json(serde_json::json!({",
                );
                self.write("            ");
                self.writeln("\"error\": \"Validation failed\",");
                self.write("            ");
                self.writeln("\"errors\": errors");
                self.write("        ");
                self.writeln("})));");
            }
            _ => {
                // Should not happen for Rust target with unsupported framework
                self.writeln("compile_error!(\"Unsupported framework for Rust target\");");
            }
        }

        self.write("    ");
        self.writeln("}");
        self.writeln("");
    }

    /// Prüft ob eine Methode eine List-Methode ist
    fn is_list_method(&self, method: &str) -> bool {
        matches!(
            method,
            "filter"
                | "map"
                | "reduce"
                | "find"
                | "contains"
                | "indexOf"
                | "sort"
                | "reverse"
                | "chunk"
                | "slice"
                | "chunks"
                | "sorted"
                | "unique"
                | "flatten"
                | "join"
                | "groupBy"
                | "group_by"
                | "push"
        )
    }

    /// Prüft ob eine Methode eine Map-Methode ist
    fn is_map_method(&self, method: &str) -> bool {
        matches!(
            method,
            "keys"
                | "values"
                | "entries"
                | "get"
                | "set"
                | "delete"
                | "has"
                | "size"
                | "containsKey"
                | "contains_key"
        )
    }

    /// Prüft ob eine Methode eine Set-Methode ist
    fn is_set_method(&self, method: &str) -> bool {
        matches!(
            method,
            "add"
                | "remove"
                | "has"
                | "size"
                | "union"
                | "intersection"
                | "difference"
                | "contains"
        )
    }

    /// Generiert HTTP Client Methoden-Aufrufe
    fn generate_http_client_call(
        &mut self,
        object: &Expression,
        method: &str,
        args: &[Expression],
    ) {
        // Check if object is HttpClient or response
        let is_client = if let Expression::Identifier(name) = object {
            name == "client" || name == "httpClient" || name == "http_client"
        } else {
            false
        };

        // Extrahiere Client-Name für spätere Verwendung
        let client_name = if let Expression::Identifier(name) = object {
            name.clone()
        } else {
            "client".to_string()
        };

        match method {
            "get" => {
                if let Some(url) = args.first() {
                    let mut url_str = String::new();
                    let old_output = std::mem::replace(&mut self.output, url_str);
                    self.generate_expression(url);
                    url_str = std::mem::replace(&mut self.output, old_output);

                    let headers = args.get(1).map(|arg| {
                        let mut headers_str = String::new();
                        let old_output = std::mem::replace(&mut self.output, headers_str);
                        self.generate_expression(arg);
                        headers_str = std::mem::replace(&mut self.output, old_output);
                        headers_str
                    });

                    if is_client {
                        self.write(&format!("{}.get({})", client_name, url_str));
                        if let Some(h) = headers {
                            self.write(&format!(".headers({})", h));
                        }
                        self.write(".send().await");
                    } else {
                        use crate::stdlib::http_client::HttpClientStdlib;
                        self.write(&HttpClientStdlib::generate_get(
                            &url_str,
                            headers.as_deref(),
                        ));
                    }
                }
            }
            "post" => {
                if let Some(url) = args.first() {
                    let mut url_str = String::new();
                    let old_output = std::mem::replace(&mut self.output, url_str);
                    self.generate_expression(url);
                    url_str = std::mem::replace(&mut self.output, old_output);

                    let body = args.get(1).map(|arg| {
                        let mut body_str = String::new();
                        let old_output = std::mem::replace(&mut self.output, body_str);
                        self.generate_expression(arg);
                        body_str = std::mem::replace(&mut self.output, old_output);
                        body_str
                    });

                    let headers = args.get(2).map(|arg| {
                        let mut headers_str = String::new();
                        let old_output = std::mem::replace(&mut self.output, headers_str);
                        self.generate_expression(arg);
                        headers_str = std::mem::replace(&mut self.output, old_output);
                        headers_str
                    });

                    if is_client {
                        self.write(&format!("{}.post({})", client_name, url_str));
                        if let Some(b) = body {
                            self.write(&format!(".json(&{})", b));
                        }
                        if let Some(h) = headers {
                            self.write(&format!(".headers({})", h));
                        }
                        self.write(".send().await");
                    } else {
                        use crate::stdlib::http_client::HttpClientStdlib;
                        self.write(&HttpClientStdlib::generate_post(
                            &url_str,
                            body.as_deref(),
                            headers.as_deref(),
                        ));
                    }
                }
            }
            "put" => {
                if let Some(url) = args.first() {
                    let mut url_str = String::new();
                    let old_output = std::mem::replace(&mut self.output, url_str);
                    self.generate_expression(url);
                    url_str = std::mem::replace(&mut self.output, old_output);

                    let body = args.get(1).map(|arg| {
                        let mut body_str = String::new();
                        let old_output = std::mem::replace(&mut self.output, body_str);
                        self.generate_expression(arg);
                        body_str = std::mem::replace(&mut self.output, old_output);
                        body_str
                    });

                    let headers = args.get(2).map(|arg| {
                        let mut headers_str = String::new();
                        let old_output = std::mem::replace(&mut self.output, headers_str);
                        self.generate_expression(arg);
                        headers_str = std::mem::replace(&mut self.output, old_output);
                        headers_str
                    });

                    if is_client {
                        self.write(&format!("{}.put({})", client_name, url_str));
                        if let Some(b) = body {
                            self.write(&format!(".json(&{})", b));
                        }
                        if let Some(h) = headers {
                            self.write(&format!(".headers({})", h));
                        }
                        self.write(".send().await");
                    } else {
                        use crate::stdlib::http_client::HttpClientStdlib;
                        self.write(&HttpClientStdlib::generate_put(
                            &url_str,
                            body.as_deref(),
                            headers.as_deref(),
                        ));
                    }
                }
            }
            "delete" => {
                if let Some(url) = args.first() {
                    let mut url_str = String::new();
                    let old_output = std::mem::replace(&mut self.output, url_str);
                    self.generate_expression(url);
                    url_str = std::mem::replace(&mut self.output, old_output);

                    let headers = args.get(1).map(|arg| {
                        let mut headers_str = String::new();
                        let old_output = std::mem::replace(&mut self.output, headers_str);
                        self.generate_expression(arg);
                        headers_str = std::mem::replace(&mut self.output, old_output);
                        headers_str
                    });

                    if is_client {
                        self.write(&format!("{}.delete({})", client_name, url_str));
                        if let Some(h) = headers {
                            self.write(&format!(".headers({})", h));
                        }
                        self.write(".send().await");
                    } else {
                        use crate::stdlib::http_client::HttpClientStdlib;
                        self.write(&HttpClientStdlib::generate_delete(
                            &url_str,
                            headers.as_deref(),
                        ));
                    }
                }
            }
            "patch" => {
                if let Some(url) = args.first() {
                    let mut url_str = String::new();
                    let old_output = std::mem::replace(&mut self.output, url_str);
                    self.generate_expression(url);
                    url_str = std::mem::replace(&mut self.output, old_output);

                    let body = args.get(1).map(|arg| {
                        let mut body_str = String::new();
                        let old_output = std::mem::replace(&mut self.output, body_str);
                        self.generate_expression(arg);
                        body_str = std::mem::replace(&mut self.output, old_output);
                        body_str
                    });

                    let headers = args.get(2).map(|arg| {
                        let mut headers_str = String::new();
                        let old_output = std::mem::replace(&mut self.output, headers_str);
                        self.generate_expression(arg);
                        headers_str = std::mem::replace(&mut self.output, old_output);
                        headers_str
                    });

                    if is_client {
                        self.write(&format!("{}.patch({})", client_name, url_str));
                        if let Some(b) = body {
                            self.write(&format!(".json(&{})", b));
                        }
                        if let Some(h) = headers {
                            self.write(&format!(".headers({})", h));
                        }
                        self.write(".send().await");
                    } else {
                        use crate::stdlib::http_client::HttpClientStdlib;
                        self.write(&HttpClientStdlib::generate_patch(
                            &url_str,
                            body.as_deref(),
                            headers.as_deref(),
                        ));
                    }
                }
            }
            "json" => {
                // response.json()
                self.generate_expression(object);
                self.write(".json::<serde_json::Value>().await");
            }
            "text" => {
                // response.text()
                self.generate_expression(object);
                self.write(".text().await");
            }
            "status" => {
                // response.status()
                self.generate_expression(object);
                self.write(".status()");
            }
            _ => {
                // Fallback
                self.generate_expression(object);
                self.write(".");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }

    /// Generiert Collections-Methoden-Aufrufe
    fn generate_collections_call(
        &mut self,
        object: &Expression,
        method: &str,
        args: &[Expression],
        collection_type: &str,
    ) {
        use crate::stdlib::collections::CollectionsStdlib;

        // Generate object expression as string for collections methods
        let mut obj_str = String::new();
        let old_output = std::mem::replace(&mut self.output, obj_str);
        self.generate_expression(object);
        obj_str = std::mem::replace(&mut self.output, old_output);

        match collection_type {
            "list" => {
                match method {
                    "filter" => {
                        if let Some(predicate) = args.first() {
                            let mut pred_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, pred_str);
                            self.generate_expression(predicate);
                            pred_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_list_filter(
                                &obj_str, &pred_str,
                            ));
                        }
                    }
                    "map" => {
                        if let Some(mapper) = args.first() {
                            let mut map_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, map_str);
                            self.generate_expression(mapper);
                            map_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_list_map(&obj_str, &map_str));
                        }
                    }
                    "reduce" => {
                        if args.len() >= 2 {
                            let mut reducer_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, reducer_str);
                            self.generate_expression(&args[0]);
                            reducer_str = std::mem::replace(&mut self.output, old_output);

                            let mut initial_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, initial_str);
                            self.generate_expression(&args[1]);
                            initial_str = std::mem::replace(&mut self.output, old_output);

                            self.write(&CollectionsStdlib::generate_list_reduce(
                                &obj_str,
                                &reducer_str,
                                &initial_str,
                            ));
                        }
                    }
                    "find" => {
                        if let Some(predicate) = args.first() {
                            let mut pred_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, pred_str);
                            self.generate_expression(predicate);
                            pred_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_list_find(&obj_str, &pred_str));
                        }
                    }
                    "contains" => {
                        if let Some(item) = args.first() {
                            let mut item_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, item_str);
                            self.generate_expression(item);
                            item_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_list_contains(
                                &obj_str, &item_str,
                            ));
                        }
                    }
                    "indexOf" | "index_of" => {
                        if let Some(item) = args.first() {
                            let mut item_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, item_str);
                            self.generate_expression(item);
                            item_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_list_index_of(
                                &obj_str, &item_str,
                            ));
                        }
                    }
                    "sort" => {
                        let compare = args.first().map(|arg| {
                            let mut compare_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, compare_str);
                            self.generate_expression(arg);
                            compare_str = std::mem::replace(&mut self.output, old_output);
                            compare_str
                        });
                        self.write(&CollectionsStdlib::generate_list_sort(
                            &obj_str,
                            compare.as_deref(),
                        ));
                    }
                    "reverse" => {
                        self.write(&CollectionsStdlib::generate_list_reverse(&obj_str));
                    }
                    "chunk" | "chunks" => {
                        if let Some(size) = args.first() {
                            let mut size_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, size_str);
                            self.generate_expression(size);
                            size_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_list_chunk(
                                &obj_str, &size_str,
                            ));
                        }
                    }
                    "slice" => {
                        if args.len() >= 2 {
                            let mut start_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, start_str);
                            self.generate_expression(&args[0]);
                            start_str = std::mem::replace(&mut self.output, old_output);

                            let mut end_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, end_str);
                            self.generate_expression(&args[1]);
                            end_str = std::mem::replace(&mut self.output, old_output);

                            self.write(&CollectionsStdlib::generate_list_slice(
                                &obj_str, &start_str, &end_str,
                            ));
                        }
                    }
                    "push" => {
                        self.generate_expression(object);
                        self.write(".push(");
                        if let Some(arg) = args.first() {
                            if let Expression::Literal(Literal::String(s)) = arg {
                                let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                                self.write(&format!("\"{}\".to_string()", escaped));
                            } else {
                                self.generate_expression(arg);
                            }
                        }
                        self.write(")");
                    }
                    _ => {
                        // Fallback to standard method call
                        self.generate_expression(object);
                        self.write(".");
                        self.write(&self.to_snake_case(method));
                        self.write("(");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                self.write(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.write(")");
                    }
                }
            }
            "map" => {
                match method {
                    "keys" => {
                        self.write(&CollectionsStdlib::generate_map_keys(&obj_str));
                    }
                    "values" => {
                        self.write(&CollectionsStdlib::generate_map_values(&obj_str));
                    }
                    "entries" => {
                        self.write(&CollectionsStdlib::generate_map_entries(&obj_str));
                    }
                    "get" => {
                        if let Some(key) = args.first() {
                            let mut key_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, key_str);
                            self.generate_expression(key);
                            key_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_map_get(&obj_str, &key_str));
                        }
                    }
                    "set" => {
                        if args.len() >= 2 {
                            let mut key_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, key_str);
                            self.generate_expression(&args[0]);
                            key_str = std::mem::replace(&mut self.output, old_output);

                            let mut value_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, value_str);
                            self.generate_expression(&args[1]);
                            value_str = std::mem::replace(&mut self.output, old_output);

                            self.write(&CollectionsStdlib::generate_map_set(
                                &obj_str, &key_str, &value_str,
                            ));
                        }
                    }
                    "delete" | "remove" => {
                        if let Some(key) = args.first() {
                            let mut key_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, key_str);
                            self.generate_expression(key);
                            key_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_map_delete(&obj_str, &key_str));
                        }
                    }
                    "has" | "containsKey" | "contains_key" => {
                        if let Some(key) = args.first() {
                            let mut key_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, key_str);
                            self.generate_expression(key);
                            key_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_map_has(&obj_str, &key_str));
                        }
                    }
                    "size" | "len" => {
                        self.write(&CollectionsStdlib::generate_map_size(&obj_str));
                    }
                    _ => {
                        // Fallback
                        self.generate_expression(object);
                        self.write(".");
                        self.write(&self.to_snake_case(method));
                        self.write("(");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                self.write(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.write(")");
                    }
                }
            }
            "set" => {
                match method {
                    "add" => {
                        if let Some(item) = args.first() {
                            let mut item_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, item_str);
                            self.generate_expression(item);
                            item_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_set_add(&obj_str, &item_str));
                        }
                    }
                    "remove" => {
                        if let Some(item) = args.first() {
                            let mut item_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, item_str);
                            self.generate_expression(item);
                            item_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_set_remove(
                                &obj_str, &item_str,
                            ));
                        }
                    }
                    "has" | "contains" => {
                        if let Some(item) = args.first() {
                            let mut item_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, item_str);
                            self.generate_expression(item);
                            item_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_set_has(&obj_str, &item_str));
                        }
                    }
                    "size" | "len" => {
                        self.write(&CollectionsStdlib::generate_set_size(&obj_str));
                    }
                    "union" => {
                        if let Some(other) = args.first() {
                            let mut other_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, other_str);
                            self.generate_expression(other);
                            other_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_set_union(
                                &obj_str, &other_str,
                            ));
                        }
                    }
                    "intersection" => {
                        if let Some(other) = args.first() {
                            let mut other_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, other_str);
                            self.generate_expression(other);
                            other_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_set_intersection(
                                &obj_str, &other_str,
                            ));
                        }
                    }
                    "difference" => {
                        if let Some(other) = args.first() {
                            let mut other_str = String::new();
                            let old_output = std::mem::replace(&mut self.output, other_str);
                            self.generate_expression(other);
                            other_str = std::mem::replace(&mut self.output, old_output);
                            self.write(&CollectionsStdlib::generate_set_difference(
                                &obj_str, &other_str,
                            ));
                        }
                    }
                    _ => {
                        // Fallback
                        self.generate_expression(object);
                        self.write(".");
                        self.write(&self.to_snake_case(method));
                        self.write("(");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                self.write(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.write(")");
                    }
                }
            }
            _ => {
                // Fallback
                self.generate_expression(object);
                self.write(".");
                self.write(&self.to_snake_case(method));
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
        }
    }
    fn generate_string_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::string::StringStdlib;

        match method {
            "split" => {
                if args.len() >= 2 {
                    let text = self.capture_expression(&args[0]);
                    let delimiter = self.capture_expression(&args[1]);
                    self.write(&StringStdlib::generate_split_code(&text, &delimiter));
                }
            }
            "join" => {
                if args.len() >= 2 {
                    let list = self.capture_expression(&args[0]);
                    let delimiter = self.capture_expression(&args[1]);
                    self.write(&StringStdlib::generate_join_code(&list, &delimiter));
                }
            }
            "replace" => {
                if args.len() >= 3 {
                    let text = self.capture_expression(&args[0]);
                    let old = self.capture_expression(&args[1]);
                    let new = self.capture_expression(&args[2]);
                    self.write(&StringStdlib::generate_replace_code(&text, &old, &new));
                }
            }
            "trim" => {
                if let Some(text_arg) = args.first() {
                    let text = self.capture_expression(text_arg);
                    self.write(&StringStdlib::generate_trim_code(&text));
                }
            }
            "slugify" => {
                if let Some(text_arg) = args.first() {
                    let text = self.capture_expression(text_arg);
                    self.write(&StringStdlib::generate_slugify_code(&text));
                }
            }
            "to_int" | "toInt" => {
                if let Some(text_arg) = args.first() {
                    let text = self.capture_expression(text_arg);
                    self.write(&StringStdlib::generate_to_int_code(&text));
                }
            }
            "to_float" | "toFloat" => {
                if let Some(text_arg) = args.first() {
                    let text = self.capture_expression(text_arg);
                    self.write(&StringStdlib::generate_to_float_code(&text));
                }
            }
            "capitalize" => {
                if let Some(text_arg) = args.first() {
                    let text = self.capture_expression(text_arg);
                    self.write(&StringStdlib::generate_capitalize_code(&text));
                }
            }
            "lowercase" => {
                if let Some(text_arg) = args.first() {
                    let text = self.capture_expression(text_arg);
                    self.write(&StringStdlib::generate_lowercase_code(&text));
                }
            }
            "uppercase" => {
                if let Some(text_arg) = args.first() {
                    let text = self.capture_expression(text_arg);
                    self.write(&StringStdlib::generate_uppercase_code(&text));
                }
            }
            "starts_with" | "startsWith" => {
                if args.len() >= 2 {
                    let text = self.capture_expression(&args[0]);
                    let prefix = self.capture_expression(&args[1]);
                    self.write(&StringStdlib::generate_starts_with_code(&text, &prefix));
                }
            }
            "ends_with" | "endsWith" => {
                if args.len() >= 2 {
                    let text = self.capture_expression(&args[0]);
                    let suffix = self.capture_expression(&args[1]);
                    self.write(&StringStdlib::generate_ends_with_code(&text, &suffix));
                }
            }
            _ => self.write(&format!("// Unknown string method: {}", method)),
        }
    }

    fn generate_math_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::math::MathStdlib;

        match method {
            "clamp" => {
                if args.len() >= 3 {
                    let value = self.capture_expression(&args[0]);
                    let min = self.capture_expression(&args[1]);
                    let max = self.capture_expression(&args[2]);
                    self.write(&MathStdlib::generate_clamp_code(&value, &min, &max));
                }
            }
            "lerp" => {
                if args.len() >= 3 {
                    let a = self.capture_expression(&args[0]);
                    let b = self.capture_expression(&args[1]);
                    let t = self.capture_expression(&args[2]);
                    self.write(&MathStdlib::generate_lerp_code(&a, &b, &t));
                }
            }
            "round_to" | "roundTo" => {
                if args.len() >= 2 {
                    let value = self.capture_expression(&args[0]);
                    let decimals = self.capture_expression(&args[1]);
                    self.write(&MathStdlib::generate_round_to_code(&value, &decimals));
                }
            }
            "random_range" | "randomRange" => {
                if args.len() >= 2 {
                    let min = self.capture_expression(&args[0]);
                    let max = self.capture_expression(&args[1]);
                    self.write(&MathStdlib::generate_random_range_code(&min, &max));
                }
            }
            "min" => {
                if args.len() >= 2 {
                    let a = self.capture_expression(&args[0]);
                    let b = self.capture_expression(&args[1]);
                    self.write(&MathStdlib::generate_min_code(&a, &b));
                }
            }
            "max" => {
                if args.len() >= 2 {
                    let a = self.capture_expression(&args[0]);
                    let b = self.capture_expression(&args[1]);
                    self.write(&MathStdlib::generate_max_code(&a, &b));
                }
            }
            "abs" => {
                if let Some(val) = args.first() {
                    let value = self.capture_expression(val);
                    self.write(&MathStdlib::generate_abs_code(&value));
                }
            }
            "floor" => {
                if let Some(val) = args.first() {
                    let value = self.capture_expression(val);
                    self.write(&MathStdlib::generate_floor_code(&value));
                }
            }
            "ceil" => {
                if let Some(val) = args.first() {
                    let value = self.capture_expression(val);
                    self.write(&MathStdlib::generate_ceil_code(&value));
                }
            }
            _ => self.write(&format!("// Unknown math method: {}", method)),
        }
    }

    fn generate_date_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::date::DateStdlib;

        match method {
            "add_days" | "addDays" => {
                if args.len() >= 2 {
                    let ts = self.capture_expression(&args[0]);
                    let days = self.capture_expression(&args[1]);
                    self.write(&DateStdlib::generate_add_days_code(&ts, &days));
                }
            }
            "add_hours" | "addHours" => {
                if args.len() >= 2 {
                    let ts = self.capture_expression(&args[0]);
                    let hours = self.capture_expression(&args[1]);
                    self.write(&DateStdlib::generate_add_hours_code(&ts, &hours));
                }
            }
            "add_minutes" | "addMinutes" => {
                if args.len() >= 2 {
                    let ts = self.capture_expression(&args[0]);
                    let mins = self.capture_expression(&args[1]);
                    self.write(&DateStdlib::generate_add_minutes_code(&ts, &mins));
                }
            }
            "format_relative" | "formatRelative" => {
                if let Some(arg) = args.first() {
                    let ts = self.capture_expression(arg);
                    self.write(&DateStdlib::generate_format_relative_code(&ts));
                }
            }
            "is_weekend" | "isWeekend" => {
                if let Some(arg) = args.first() {
                    let ts = self.capture_expression(arg);
                    self.write(&DateStdlib::generate_is_weekend_code(&ts));
                }
            }
            "is_weekday" | "isWeekday" => {
                if let Some(arg) = args.first() {
                    let ts = self.capture_expression(arg);
                    self.write(&DateStdlib::generate_is_weekday_code(&ts));
                }
            }
            _ => self.write(&format!("// Unknown date method: {}", method)),
        }
    }

    fn generate_fs_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::fs::FsStdlib;

        match method {
            "read_json" | "readJson" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&FsStdlib::generate_read_json_code(&path));
                }
            }
            "write_json" | "writeJson" => {
                if args.len() >= 2 {
                    let path = self.capture_expression(&args[0]);
                    let value = self.capture_expression(&args[1]);
                    self.write(&FsStdlib::generate_write_json_code(&path, &value));
                }
            }
            "copy" => {
                if args.len() >= 2 {
                    let src = self.capture_expression(&args[0]);
                    let dest = self.capture_expression(&args[1]);
                    self.write(&FsStdlib::generate_copy_code(&src, &dest));
                }
            }
            "move_file" | "moveFile" => {
                if args.len() >= 2 {
                    let src = self.capture_expression(&args[0]);
                    let dest = self.capture_expression(&args[1]);
                    self.write(&FsStdlib::generate_move_file_code(&src, &dest));
                }
            }
            "get_size" | "getSize" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&FsStdlib::generate_get_size_code(&path));
                }
            }
            "is_empty" | "isEmpty" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&FsStdlib::generate_is_empty_code(&path));
                }
            }
            _ => self.write(&format!("// Unknown fs method: {}", method)),
        }
    }

    fn generate_llm_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::llm::LLMStdlib;

        match method {
            "summarize" => {
                if let Some(arg) = args.first() {
                    let text = self.capture_expression(arg);
                    self.write(&LLMStdlib::generate_summarize_code("llm_client", &text));
                }
            }
            "classify" => {
                if args.len() >= 2 {
                    let text = self.capture_expression(&args[0]);
                    let cats = self.capture_expression(&args[1]);
                    self.write(&LLMStdlib::generate_classify_code(
                        "llm_client",
                        &text,
                        &cats,
                    ));
                }
            }
            "extract_entities" | "extractEntities" => {
                if let Some(arg) = args.first() {
                    let text = self.capture_expression(arg);
                    self.write(&LLMStdlib::generate_extract_entities_code(
                        "llm_client",
                        &text,
                    ));
                }
            }
            "generate" => {
                if let Some(arg) = args.first() {
                    let title = self.capture_expression(arg);
                    let style = if args.len() >= 2 {
                        self.capture_expression(&args[1])
                    } else {
                        "\"default\"".to_string()
                    };
                    self.write(&LLMStdlib::generate_generate_code(
                        "llm_client",
                        &title,
                        Some(&style),
                    ));
                }
            }
            "translate" => {
                if args.len() >= 2 {
                    let text = self.capture_expression(&args[0]);
                    let lang = self.capture_expression(&args[1]);
                    self.write(&LLMStdlib::generate_translate_code(
                        "llm_client",
                        &text,
                        &lang,
                    ));
                }
            }
            "sentiment" => {
                if let Some(arg) = args.first() {
                    let text = self.capture_expression(arg);
                    self.write(&LLMStdlib::generate_sentiment_code("llm_client", &text));
                }
            }
            "complete" => {
                if let Some(arg) = args.first() {
                    let prompt = self.capture_expression(arg);
                    let max_tokens = if args.len() >= 2 {
                        self.capture_expression(&args[1])
                    } else {
                        "100".to_string()
                    };
                    self.write(&LLMStdlib::generate_complete_code(
                        "llm_client",
                        &prompt,
                        Some(&max_tokens),
                    ));
                }
            }
            "embed" => {
                if let Some(arg) = args.first() {
                    let text = self.capture_expression(arg);
                    self.write(&LLMStdlib::generate_embed_code("llm_client", &text));
                }
            }
            "chat" => {
                if let Some(arg) = args.first() {
                    let messages = self.capture_expression(arg);
                    self.write(&LLMStdlib::generate_chat_code("llm_client", &messages));
                }
            }
            _ => self.write(&format!("// Unknown llm method: {}", method)),
        }
    }

    fn generate_embedding_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::embedding::EmbeddingStdlib;

        match method {
            "compare" => {
                if args.len() >= 2 {
                    let a = self.capture_expression(&args[0]);
                    let b = self.capture_expression(&args[1]);
                    self.write(&EmbeddingStdlib::generate_compare_code(&a, &b));
                }
            }
            "similarity" => {
                if args.len() >= 2 {
                    let a = self.capture_expression(&args[0]);
                    let b = self.capture_expression(&args[1]);
                    self.write(&EmbeddingStdlib::generate_similarity_code(&a, &b));
                }
            }
            "cluster" => {
                if args.len() >= 2 {
                    let list = self.capture_expression(&args[0]);
                    let k = self.capture_expression(&args[1]);
                    self.write(&EmbeddingStdlib::generate_cluster_code(&list, &k));
                }
            }
            "normalize" => {
                if let Some(arg) = args.first() {
                    let emb = self.capture_expression(arg);
                    self.write(&EmbeddingStdlib::generate_normalize_code(&emb));
                }
            }
            "distance" => {
                if args.len() >= 2 {
                    let a = self.capture_expression(&args[0]);
                    let b = self.capture_expression(&args[1]);
                    self.write(&EmbeddingStdlib::generate_distance_code(&a, &b));
                }
            }
            "find_nearest" | "findNearest" => {
                if args.len() >= 3 {
                    let query = self.capture_expression(&args[0]);
                    let candidates = self.capture_expression(&args[1]);
                    let k = self.capture_expression(&args[2]);
                    self.write(&EmbeddingStdlib::generate_find_nearest_code(
                        &query,
                        &candidates,
                        &k,
                    ));
                }
            }
            "average" => {
                if let Some(arg) = args.first() {
                    let embs = self.capture_expression(arg);
                    self.write(&EmbeddingStdlib::generate_average_code(&embs));
                }
            }
            "dimension" => {
                if let Some(arg) = args.first() {
                    let emb = self.capture_expression(arg);
                    self.write(&EmbeddingStdlib::generate_dimension_code(&emb));
                }
            }
            _ => self.write(&format!("// Unknown embedding method: {}", method)),
        }
    }

    fn generate_agent_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::agent::AgentStdlib;

        match method {
            "create" => {
                if let Some(arg) = args.first() {
                    let name = self.capture_expression(arg);
                    self.write(&AgentStdlib::generate_agent_create_code(&name));
                }
            }
            "think" => {
                if let Some(arg) = args.first() {
                    let ctx = self.capture_expression(arg);
                    self.write(&AgentStdlib::generate_agent_think_code("agent", &ctx));
                }
            }
            _ => self.write(&format!("// Unknown agent method: {}", method)),
        }
    }

    fn generate_agent_nested_call(&mut self, module: &str, method: &str, args: &[Expression]) {
        use crate::stdlib::agent::AgentStdlib;

        match module {
            "memory" => match method {
                "store" => {
                    if args.len() >= 2 {
                        let key = self.capture_expression(&args[0]);
                        let value = self.capture_expression(&args[1]);
                        self.write(&AgentStdlib::generate_memory_store_code(&key, &value));
                    }
                }
                "search" => {
                    if let Some(arg) = args.first() {
                        let query = self.capture_expression(arg);
                        self.write(&AgentStdlib::generate_memory_search_code(&query));
                    }
                }
                "get" => {
                    if let Some(arg) = args.first() {
                        let key = self.capture_expression(arg);
                        self.write(&AgentStdlib::generate_memory_get_code(&key));
                    }
                }
                "delete" => {
                    if let Some(arg) = args.first() {
                        let key = self.capture_expression(arg);
                        self.write(&AgentStdlib::generate_memory_delete_code(&key));
                    }
                }
                _ => self.write(&format!("// Unknown agent.memory method: {}", method)),
            },
            "task" => match method {
                "run" => {
                    if let Some(arg) = args.first() {
                        let desc = self.capture_expression(arg);
                        self.write(&AgentStdlib::generate_task_run_code(&desc));
                    }
                }
                "plan" => {
                    if let Some(arg) = args.first() {
                        let goal = self.capture_expression(arg);
                        self.write(&AgentStdlib::generate_task_plan_code(&goal));
                    }
                }
                "execute" => {
                    if let Some(arg) = args.first() {
                        let plan = self.capture_expression(arg);
                        self.write(&AgentStdlib::generate_task_execute_code(&plan));
                    }
                }
                _ => self.write(&format!("// Unknown agent.task method: {}", method)),
            },
            _ => self.write(&format!("// Unknown agent module: {}", module)),
        }
    }

    fn generate_process_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::process::ProcessStdlib;

        match method {
            "spawn" => {
                if args.len() >= 2 {
                    let cmd = self.capture_expression(&args[0]);
                    let cmd_args = self.capture_expression(&args[1]);
                    self.write(&ProcessStdlib::generate_spawn_code(&cmd, &cmd_args));
                }
            }
            "kill" => {
                if let Some(arg) = args.first() {
                    let pid = self.capture_expression(arg);
                    self.write(&ProcessStdlib::generate_kill_code(&pid));
                }
            }
            "restart" => {
                if let Some(arg) = args.first() {
                    let pid = self.capture_expression(arg);
                    self.write(&ProcessStdlib::generate_restart_code(&pid));
                }
            }
            "status" => {
                if let Some(arg) = args.first() {
                    let pid = self.capture_expression(arg);
                    self.write(&ProcessStdlib::generate_status_code(&pid));
                }
            }
            "list" => {
                self.write(&ProcessStdlib::generate_list_code());
            }
            "wait" => {
                if let Some(arg) = args.first() {
                    let pid = self.capture_expression(arg);
                    self.write(&ProcessStdlib::generate_wait_code(&pid));
                }
            }
            "get_output" | "getOutput" => {
                if let Some(arg) = args.first() {
                    let pid = self.capture_expression(arg);
                    self.write(&ProcessStdlib::generate_get_output_code(&pid));
                }
            }
            "is_running" | "isRunning" => {
                if let Some(arg) = args.first() {
                    let pid = self.capture_expression(arg);
                    self.write(&ProcessStdlib::generate_is_running_code(&pid));
                }
            }
            "get_memory" | "getMemory" => {
                if let Some(arg) = args.first() {
                    let pid = self.capture_expression(arg);
                    self.write(&ProcessStdlib::generate_get_memory_code(&pid));
                }
            }
            _ => self.write(&format!("// Unknown process method: {}", method)),
        }
    }

    fn generate_sandbox_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::sandbox::SandboxStdlib;

        match method {
            "build" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&SandboxStdlib::generate_build_code(&path));
                }
            }
            "test" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&SandboxStdlib::generate_test_code(&path));
                }
            }
            "validate" => {
                if let Some(arg) = args.first() {
                    let code = self.capture_expression(arg);
                    self.write(&SandboxStdlib::generate_validate_code(&code));
                }
            }
            "run" => {
                if let Some(arg) = args.first() {
                    let code = self.capture_expression(arg);
                    self.write(&SandboxStdlib::generate_run_code(&code));
                }
            }
            "lint" => {
                if let Some(arg) = args.first() {
                    let code = self.capture_expression(arg);
                    self.write(&SandboxStdlib::generate_lint_code(&code));
                }
            }
            "format" => {
                if let Some(arg) = args.first() {
                    let code = self.capture_expression(arg);
                    self.write(&SandboxStdlib::generate_format_code(&code));
                }
            }
            "check_types" | "checkTypes" => {
                if let Some(arg) = args.first() {
                    let code = self.capture_expression(arg);
                    self.write(&SandboxStdlib::generate_check_types_code(&code));
                }
            }
            "optimize" => {
                if let Some(arg) = args.first() {
                    let code = self.capture_expression(arg);
                    self.write(&SandboxStdlib::generate_optimize_code(&code));
                }
            }
            _ => self.write(&format!("// Unknown sandbox method: {}", method)),
        }
    }

    fn generate_websocket_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::websocket::WebSocketStdlib;

        match method {
            "connect" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&WebSocketStdlib::generate_connect_code(&url));
                }
            }
            "send" => {
                if args.len() >= 2 {
                    let ws = self.capture_expression(&args[0]);
                    let msg = self.capture_expression(&args[1]);
                    self.write(&WebSocketStdlib::generate_send_code(&ws, &msg));
                }
            }
            "receive" => {
                if let Some(arg) = args.first() {
                    let ws = self.capture_expression(arg);
                    self.write(&WebSocketStdlib::generate_receive_code(&ws));
                }
            }
            "close" => {
                if let Some(arg) = args.first() {
                    let ws = self.capture_expression(arg);
                    self.write(&WebSocketStdlib::generate_close_code(&ws));
                }
            }
            "is_connected" | "isConnected" => {
                if let Some(arg) = args.first() {
                    let ws = self.capture_expression(arg);
                    self.write(&WebSocketStdlib::generate_is_connected_code(&ws));
                }
            }
            "ping" => {
                if let Some(arg) = args.first() {
                    let ws = self.capture_expression(arg);
                    self.write(&WebSocketStdlib::generate_ping_code(&ws));
                }
            }
            "subscribe" => {
                if args.len() >= 2 {
                    let ws = self.capture_expression(&args[0]);
                    let topic = self.capture_expression(&args[1]);
                    self.write(&WebSocketStdlib::generate_subscribe_code(&ws, &topic));
                }
            }
            "on_message" | "onMessage" => {
                if args.len() >= 2 {
                    let ws = self.capture_expression(&args[0]);
                    let cb = self.capture_expression(&args[1]);
                    self.write(&WebSocketStdlib::generate_on_message_code(&ws, &cb));
                }
            }
            _ => self.write(&format!("// Unknown websocket method: {}", method)),
        }
    }

    fn generate_utils_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::utils::UtilsStdlib;

        match method {
            "uuid" => {
                self.write(&UtilsStdlib::generate_uuid_code());
            }
            "sleep" => {
                if let Some(arg) = args.first() {
                    let ms = self.capture_expression(arg);
                    self.write(&UtilsStdlib::generate_sleep_code(&ms));
                }
            }
            "retry" => {
                if args.len() >= 2 {
                    let func = self.capture_expression(&args[0]);
                    let times = self.capture_expression(&args[1]);
                    self.write(&UtilsStdlib::generate_retry_code(&func, &times));
                }
            }
            "debounce" => {
                if args.len() >= 2 {
                    let func = self.capture_expression(&args[0]);
                    let ms = self.capture_expression(&args[1]);
                    self.write(&UtilsStdlib::generate_debounce_code(&func, &ms));
                }
            }
            "throttle" => {
                if args.len() >= 2 {
                    let func = self.capture_expression(&args[0]);
                    let ms = self.capture_expression(&args[1]);
                    self.write(&UtilsStdlib::generate_throttle_code(&func, &ms));
                }
            }
            "memoize" => {
                if let Some(arg) = args.first() {
                    let func = self.capture_expression(arg);
                    self.write(&UtilsStdlib::generate_memoize_code(&func));
                }
            }
            "timeout" => {
                if args.len() >= 2 {
                    let func = self.capture_expression(&args[0]);
                    let ms = self.capture_expression(&args[1]);
                    self.write(&UtilsStdlib::generate_timeout_code(&func, &ms));
                }
            }
            "parallel" => {
                if let Some(arg) = args.first() {
                    let tasks = self.capture_expression(arg);
                    self.write(&UtilsStdlib::generate_parallel_code(&tasks));
                }
            }
            "cache" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let func = self.capture_expression(&args[1]);
                    self.write(&UtilsStdlib::generate_cache_code(&key, &func));
                }
            }
            _ => self.write(&format!("// Unknown utils method: {}", method)),
        }
    }

    fn generate_log_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::log::LogStdlib;

        match method {
            "info" => {
                if let Some(arg) = args.first() {
                    let msg = self.capture_expression(arg);
                    self.write(&LogStdlib::generate_info_code(&msg));
                }
            }
            "warn" => {
                if let Some(arg) = args.first() {
                    let msg = self.capture_expression(arg);
                    self.write(&LogStdlib::generate_warn_code(&msg));
                }
            }
            "error" => {
                if let Some(arg) = args.first() {
                    let msg = self.capture_expression(arg);
                    self.write(&LogStdlib::generate_error_code(&msg));
                }
            }
            "debug" => {
                if let Some(arg) = args.first() {
                    let msg = self.capture_expression(arg);
                    self.write(&LogStdlib::generate_debug_code(&msg));
                }
            }
            "trace" => {
                if let Some(arg) = args.first() {
                    let msg = self.capture_expression(arg);
                    self.write(&LogStdlib::generate_trace_code(&msg));
                }
            }
            "set_level" | "setLevel" => {
                if let Some(arg) = args.first() {
                    let level = self.capture_expression(arg);
                    self.write(&LogStdlib::generate_set_level_code(&level));
                }
            }
            "with_context" | "withContext" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let val = self.capture_expression(&args[1]);
                    self.write(&LogStdlib::generate_with_context_code(&key, &val));
                }
            }
            "to_file" | "toFile" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&LogStdlib::generate_to_file_code(&path));
                }
            }
            "json" => {
                if args.len() >= 2 {
                    let msg = self.capture_expression(&args[0]);
                    let data = self.capture_expression(&args[1]);
                    self.write(&LogStdlib::generate_json_code(&msg, &data));
                }
            }
            _ => self.write(&format!("// Unknown log method: {}", method)),
        }
    }

    #[allow(dead_code)]
    fn generate_config_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::config::ConfigStdlib;

        match method {
            "get_env" | "getEnv" => {
                if let Some(arg) = args.first() {
                    let key = self.capture_expression(arg);
                    self.write(&ConfigStdlib::generate_get_env_code(&key));
                }
            }
            "get_or_default" | "getOrDefault" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let default = self.capture_expression(&args[1]);
                    self.write(&ConfigStdlib::generate_get_or_default_code(&key, &default));
                }
            }
            "load_dotenv" | "loadDotenv" => {
                self.write(&ConfigStdlib::generate_load_dotenv_code());
            }
            _ => self.write(&format!("// Unknown config method: {}", method)),
        }
    }

    #[allow(dead_code)]
    fn generate_path_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::path::PathStdlib;

        match method {
            "join" => {
                if let Some(arg) = args.first() {
                    let parts = self.capture_expression(arg);
                    self.write(&PathStdlib::generate_join_code(&parts));
                }
            }
            "dirname" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&PathStdlib::generate_dirname_code(&path));
                }
            }
            "basename" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&PathStdlib::generate_basename_code(&path));
                }
            }
            "extname" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&PathStdlib::generate_extname_code(&path));
                }
            }
            "normalize" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&PathStdlib::generate_normalize_code(&path));
                }
            }
            "resolve" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&PathStdlib::generate_resolve_code(&path));
                }
            }
            "relative" => {
                if args.len() >= 2 {
                    let from = self.capture_expression(&args[0]);
                    let to = self.capture_expression(&args[1]);
                    self.write(&PathStdlib::generate_relative_code(&from, &to));
                }
            }
            "is_absolute" | "isAbsolute" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&PathStdlib::generate_is_absolute_code(&path));
                }
            }
            "separator" => {
                self.write(&PathStdlib::generate_separator_code());
            }
            _ => self.write(&format!("// Unknown path method: {}", method)),
        }
    }

    #[allow(dead_code)]
    fn generate_url_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::url::UrlStdlib;

        match method {
            "parse" => {
                if let Some(arg) = args.first() {
                    let url_str = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_parse_code(&url_str));
                }
            }
            "protocol" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_protocol_code(&url));
                }
            }
            "hostname" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_hostname_code(&url));
                }
            }
            "port" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_port_code(&url));
                }
            }
            "pathname" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_pathname_code(&url));
                }
            }
            "search" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_search_code(&url));
                }
            }
            "hash" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_hash_code(&url));
                }
            }
            "format" => {
                if let Some(arg) = args.first() {
                    let components = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_format_code(&components));
                }
            }
            "parse_query" | "parseQuery" => {
                if let Some(arg) = args.first() {
                    let query_str = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_parse_query_code(&query_str));
                }
            }
            "stringify_query" | "stringifyQuery" => {
                if let Some(arg) = args.first() {
                    let params = self.capture_expression(arg);
                    self.write(&UrlStdlib::generate_stringify_query_code(&params));
                }
            }
            _ => self.write(&format!("// Unknown url method: {}", method)),
        }
    }

    #[allow(dead_code)]
    fn generate_stream_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::stream::StreamStdlib;

        match method {
            "create" => {
                self.write(&StreamStdlib::generate_create_code());
            }
            "map" => {
                if args.len() >= 2 {
                    let stream = self.capture_expression(&args[0]);
                    let mapper = self.capture_expression(&args[1]);
                    self.write(&StreamStdlib::generate_map_code(&stream, &mapper));
                }
            }
            "filter" => {
                if args.len() >= 2 {
                    let stream = self.capture_expression(&args[0]);
                    let predicate = self.capture_expression(&args[1]);
                    self.write(&StreamStdlib::generate_filter_code(&stream, &predicate));
                }
            }
            "reduce" => {
                if args.len() >= 3 {
                    let stream = self.capture_expression(&args[0]);
                    let reducer = self.capture_expression(&args[1]);
                    let initial = self.capture_expression(&args[2]);
                    self.write(&StreamStdlib::generate_reduce_code(
                        &stream, &reducer, &initial,
                    ));
                }
            }
            "batch" => {
                if args.len() >= 2 {
                    let stream = self.capture_expression(&args[0]);
                    let size = self.capture_expression(&args[1]);
                    self.write(&StreamStdlib::generate_batch_code(&stream, &size));
                }
            }
            "buffer" => {
                if args.len() >= 2 {
                    let stream = self.capture_expression(&args[0]);
                    let size = self.capture_expression(&args[1]);
                    self.write(&StreamStdlib::generate_buffer_code(&stream, &size));
                }
            }
            "merge" => {
                if args.len() >= 2 {
                    let stream1 = self.capture_expression(&args[0]);
                    let stream2 = self.capture_expression(&args[1]);
                    self.write(&StreamStdlib::generate_merge_code(&stream1, &stream2));
                }
            }
            "zip" => {
                if args.len() >= 2 {
                    let stream1 = self.capture_expression(&args[0]);
                    let stream2 = self.capture_expression(&args[1]);
                    self.write(&StreamStdlib::generate_zip_code(&stream1, &stream2));
                }
            }
            _ => self.write(&format!("// Unknown stream method: {}", method)),
        }
    }

    fn generate_redis_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::redis::RedisStdlib;

        match method {
            "connect" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&RedisStdlib::generate_connect_code(&url));
                }
            }
            "set" => {
                if args.len() >= 3 {
                    let client = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    if args.len() >= 4 {
                        let ttl = self.capture_expression(&args[3]);
                        self.write(&RedisStdlib::generate_set_code(
                            &client,
                            &key,
                            &value,
                            Some(&ttl),
                        ));
                    } else {
                        self.write(&RedisStdlib::generate_set_code(&client, &key, &value, None));
                    }
                }
            }
            "get" => {
                if args.len() >= 2 {
                    let client = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    self.write(&RedisStdlib::generate_get_code(&client, &key));
                }
            }
            "delete" => {
                if args.len() >= 2 {
                    let client = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    self.write(&RedisStdlib::generate_delete_code(&client, &key));
                }
            }
            "hset" => {
                if args.len() >= 4 {
                    let client = self.capture_expression(&args[0]);
                    let hash = self.capture_expression(&args[1]);
                    let field = self.capture_expression(&args[2]);
                    let value = self.capture_expression(&args[3]);
                    self.write(&RedisStdlib::generate_hset_code(
                        &client, &hash, &field, &value,
                    ));
                }
            }
            "hget" => {
                if args.len() >= 3 {
                    let client = self.capture_expression(&args[0]);
                    let hash = self.capture_expression(&args[1]);
                    let field = self.capture_expression(&args[2]);
                    self.write(&RedisStdlib::generate_hget_code(&client, &hash, &field));
                }
            }
            "hgetall" => {
                if args.len() >= 2 {
                    let client = self.capture_expression(&args[0]);
                    let hash = self.capture_expression(&args[1]);
                    self.write(&RedisStdlib::generate_hgetall_code(&client, &hash));
                }
            }
            "lpush" => {
                if args.len() >= 3 {
                    let client = self.capture_expression(&args[0]);
                    let list = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    self.write(&RedisStdlib::generate_lpush_code(&client, &list, &value));
                }
            }
            "rpush" => {
                if args.len() >= 3 {
                    let client = self.capture_expression(&args[0]);
                    let list = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    self.write(&RedisStdlib::generate_rpush_code(&client, &list, &value));
                }
            }
            "lpop" => {
                if args.len() >= 2 {
                    let client = self.capture_expression(&args[0]);
                    let list = self.capture_expression(&args[1]);
                    self.write(&RedisStdlib::generate_lpop_code(&client, &list));
                }
            }
            "llen" => {
                if args.len() >= 2 {
                    let client = self.capture_expression(&args[0]);
                    let list = self.capture_expression(&args[1]);
                    self.write(&RedisStdlib::generate_llen_code(&client, &list));
                }
            }
            "sadd" => {
                if args.len() >= 3 {
                    let client = self.capture_expression(&args[0]);
                    let set = self.capture_expression(&args[1]);
                    let member = self.capture_expression(&args[2]);
                    self.write(&RedisStdlib::generate_sadd_code(&client, &set, &member));
                }
            }
            "sismember" => {
                if args.len() >= 3 {
                    let client = self.capture_expression(&args[0]);
                    let set = self.capture_expression(&args[1]);
                    let member = self.capture_expression(&args[2]);
                    self.write(&RedisStdlib::generate_sismember_code(
                        &client, &set, &member,
                    ));
                }
            }
            "smembers" => {
                if args.len() >= 2 {
                    let client = self.capture_expression(&args[0]);
                    let set = self.capture_expression(&args[1]);
                    self.write(&RedisStdlib::generate_smembers_code(&client, &set));
                }
            }
            "publish" => {
                if args.len() >= 3 {
                    let client = self.capture_expression(&args[0]);
                    let channel = self.capture_expression(&args[1]);
                    let message = self.capture_expression(&args[2]);
                    self.write(&RedisStdlib::generate_publish_code(
                        &client, &channel, &message,
                    ));
                }
            }
            _ => self.write(&format!("// Unknown redis method: {}", method)),
        }
    }

    #[allow(dead_code)]
    fn generate_tracing_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::tracing::TracingStdlib;

        match method {
            "start_span" | "startSpan" => {
                if let Some(arg) = args.first() {
                    let name = self.capture_expression(arg);
                    self.write(&TracingStdlib::generate_start_span_code(&name));
                }
            }
            "set_attribute" | "setAttribute" => {
                if args.len() >= 3 {
                    let span = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    self.write(&TracingStdlib::generate_set_attribute_code(
                        &span, &key, &value,
                    ));
                }
            }
            "child_span" | "childSpan" => {
                if args.len() >= 2 {
                    let parent = self.capture_expression(&args[0]);
                    let name = self.capture_expression(&args[1]);
                    self.write(&TracingStdlib::generate_child_span_code(&parent, &name));
                }
            }
            "end_span" | "endSpan" => {
                if let Some(arg) = args.first() {
                    let span = self.capture_expression(arg);
                    self.write(&TracingStdlib::generate_end_span_code(&span));
                }
            }
            "export" => {
                if let Some(arg) = args.first() {
                    let format = self.capture_expression(arg);
                    self.write(&TracingStdlib::generate_export_code(&format));
                }
            }
            _ => self.write(&format!("// Unknown tracing method: {}", method)),
        }
    }

    fn generate_encoding_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::encoding::EncodingStdlib;
        match method {
            "base64_encode" | "base64Encode" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    self.write(&EncodingStdlib::generate_base64_encode_code(&input));
                }
            }
            "base64_decode" | "base64Decode" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    self.write(&EncodingStdlib::generate_base64_decode_code(&input));
                }
            }
            "url_encode" | "urlEncode" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    self.write(&EncodingStdlib::generate_url_encode_code(&input));
                }
            }
            "url_decode" | "urlDecode" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    self.write(&EncodingStdlib::generate_url_decode_code(&input));
                }
            }
            "hex_encode" | "hexEncode" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    self.write(&EncodingStdlib::generate_hex_encode_code(&input));
                }
            }
            "hex_decode" | "hexDecode" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    self.write(&EncodingStdlib::generate_hex_decode_code(&input));
                }
            }
            "is_valid_utf8" | "isValidUtf8" => {
                if let Some(arg) = args.first() {
                    let bytes = self.capture_expression(arg);
                    self.write(&EncodingStdlib::generate_is_valid_utf8_code(&bytes));
                }
            }
            "fix_utf8" | "fixUtf8" => {
                if let Some(arg) = args.first() {
                    let bytes = self.capture_expression(arg);
                    self.write(&EncodingStdlib::generate_fix_utf8_code(&bytes));
                }
            }
            _ => self.write(&format!("// Unknown encoding method: {}", method)),
        }
    }

    fn generate_queue_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::queue::QueueStdlib;
        match method {
            "create" => {
                let capacity = args.first().map(|a| self.capture_expression(a));
                self.write(&QueueStdlib::generate_create_code(capacity.as_deref()));
            }
            "enqueue" => {
                if args.len() >= 2 {
                    let queue = self.capture_expression(&args[0]);
                    let item = self.capture_expression(&args[1]);
                    self.write(&QueueStdlib::generate_enqueue_code(&queue, &item));
                }
            }
            "dequeue" => {
                if let Some(arg) = args.first() {
                    let queue = self.capture_expression(arg);
                    self.write(&QueueStdlib::generate_dequeue_code(&queue));
                }
            }
            "peek" => {
                if let Some(arg) = args.first() {
                    let queue = self.capture_expression(arg);
                    self.write(&QueueStdlib::generate_peek_code(&queue));
                }
            }
            "size" => {
                if let Some(arg) = args.first() {
                    let queue = self.capture_expression(arg);
                    self.write(&QueueStdlib::generate_size_code(&queue));
                }
            }
            "is_empty" | "isEmpty" => {
                if let Some(arg) = args.first() {
                    let queue = self.capture_expression(arg);
                    self.write(&QueueStdlib::generate_is_empty_code(&queue));
                }
            }
            "is_full" | "isFull" => {
                if let Some(arg) = args.first() {
                    let queue = self.capture_expression(arg);
                    self.write(&QueueStdlib::generate_is_full_code(&queue));
                }
            }
            "priority" => {
                if let Some(arg) = args.first() {
                    let compare = self.capture_expression(arg);
                    self.write(&QueueStdlib::generate_priority_create_code(&compare));
                }
            }
            "bounded" => {
                if let Some(arg) = args.first() {
                    let capacity = self.capture_expression(arg);
                    self.write(&QueueStdlib::generate_bounded_create_code(&capacity));
                }
            }
            _ => self.write(&format!("// Unknown queue method: {}", method)),
        }
    }

    #[cfg(feature = "mongodb")]
    fn generate_mongodb_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::mongodb::MongoDbStdlib;
        match method {
            "connect" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&MongoDbStdlib::generate_connect_code(&url));
                }
            }
            "database" => {
                if args.len() >= 2 {
                    let client = self.capture_expression(&args[0]);
                    let name = self.capture_expression(&args[1]);
                    self.write(&MongoDbStdlib::generate_database_code(&client, &name));
                }
            }
            "collection" => {
                if args.len() >= 2 {
                    let db = self.capture_expression(&args[0]);
                    let name = self.capture_expression(&args[1]);
                    self.write(&MongoDbStdlib::generate_collection_code(&db, &name));
                }
            }
            "insert_one" | "insertOne" => {
                if args.len() >= 2 {
                    let collection = self.capture_expression(&args[0]);
                    let doc = self.capture_expression(&args[1]);
                    self.write(&MongoDbStdlib::generate_insert_one_code(&collection, &doc));
                }
            }
            "find" => {
                if args.len() >= 2 {
                    let collection = self.capture_expression(&args[0]);
                    let filter = self.capture_expression(&args[1]);
                    self.write(&MongoDbStdlib::generate_find_code(&collection, &filter));
                }
            }
            "find_one" | "findOne" => {
                if args.len() >= 2 {
                    let collection = self.capture_expression(&args[0]);
                    let filter = self.capture_expression(&args[1]);
                    self.write(&MongoDbStdlib::generate_find_one_code(&collection, &filter));
                }
            }
            "update_one" | "updateOne" => {
                if args.len() >= 3 {
                    let collection = self.capture_expression(&args[0]);
                    let filter = self.capture_expression(&args[1]);
                    let update = self.capture_expression(&args[2]);
                    self.write(&MongoDbStdlib::generate_update_one_code(
                        &collection,
                        &filter,
                        &update,
                    ));
                }
            }
            "delete_one" | "deleteOne" => {
                if args.len() >= 2 {
                    let collection = self.capture_expression(&args[0]);
                    let filter = self.capture_expression(&args[1]);
                    self.write(&MongoDbStdlib::generate_delete_one_code(
                        &collection,
                        &filter,
                    ));
                }
            }
            "aggregate" => {
                if args.len() >= 2 {
                    let collection = self.capture_expression(&args[0]);
                    let pipeline = self.capture_expression(&args[1]);
                    self.write(&MongoDbStdlib::generate_aggregate_code(
                        &collection,
                        &pipeline,
                    ));
                }
            }
            "create_index" | "createIndex" => {
                if args.len() >= 3 {
                    let collection = self.capture_expression(&args[0]);
                    let keys = self.capture_expression(&args[1]);
                    let unique = if let Expression::Boolean(b) = &args[2] {
                        *b
                    } else {
                        false
                    };
                    self.write(&MongoDbStdlib::generate_create_index_code(
                        &collection,
                        &keys,
                        unique,
                    ));
                }
            }
            _ => self.write(&format!("// Unknown mongodb method: {}", method)),
        }
    }

    #[cfg(feature = "smtp")]
    fn generate_email_call(&mut self, method: &str, args: &[Expression]) {
        self.generate_smtp_call(method, args);
    }

    #[cfg(not(feature = "smtp"))]
    fn generate_email_call(&mut self, method: &str, _args: &[Expression]) {
        self.write(&format!("// Email support not enabled. Method: {}", method));
    }

    #[cfg(feature = "smtp")]
    fn generate_smtp_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::smtp::SmtpStdlib;
        match method {
            "connect" => {
                if let Some(arg) = args.first() {
                    let config = self.capture_expression(arg);
                    self.write(&SmtpStdlib::generate_connect_code(&config));
                }
            }
            "send" => {
                if args.len() >= 2 {
                    let mailer = self.capture_expression(&args[0]);
                    let email = self.capture_expression(&args[1]);
                    self.write(&SmtpStdlib::generate_send_code(&mailer, &email));
                }
            }
            "template" => {
                if args.len() >= 2 {
                    let template_path = self.capture_expression(&args[0]);
                    let data = self.capture_expression(&args[1]);
                    self.write(&SmtpStdlib::generate_template_code(&template_path, &data));
                }
            }
            _ => self.write(&format!("// Unknown smtp method: {}", method)),
        }
    }

    fn generate_csv_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::csv::CsvStdlib;
        match method {
            "read" => {
                if args.len() >= 2 {
                    let path = self.capture_expression(&args[0]);
                    let has_header = if let Expression::Literal(Literal::Boolean(b)) = &args[1] {
                        *b
                    } else {
                        false
                    };
                    self.write(&CsvStdlib::generate_read_code(&path, has_header));
                }
            }
            "write" => {
                if args.len() >= 2 {
                    let path = self.capture_expression(&args[0]);
                    let rows = self.capture_expression(&args[1]);
                    let headers = args.get(2).map(|a| self.capture_expression(a));
                    self.write(&CsvStdlib::generate_write_code(
                        &path,
                        &rows,
                        headers.as_deref(),
                    ));
                }
            }
            "parse" => {
                if let Some(arg) = args.first() {
                    let csv_string = self.capture_expression(arg);
                    self.write(&CsvStdlib::generate_parse_code(&csv_string));
                }
            }
            "stringify" => {
                if args.len() >= 2 {
                    let rows = self.capture_expression(&args[0]);
                    let headers = self.capture_expression(&args[1]);
                    self.write(&CsvStdlib::generate_stringify_code(&rows, &headers));
                }
            }
            "validate" => {
                if args.len() >= 2 {
                    let path = self.capture_expression(&args[0]);
                    let schema = self.capture_expression(&args[1]);
                    self.write(&CsvStdlib::generate_validate_code(&path, &schema));
                }
            }
            _ => self.write(&format!("// Unknown csv method: {}", method)),
        }
    }

    fn generate_yaml_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::yaml::YamlStdlib;
        match method {
            "parse" => {
                if let Some(arg) = args.first() {
                    let yaml_string = self.capture_expression(arg);
                    self.write(&YamlStdlib::generate_parse_code(&yaml_string));
                }
            }
            "parse_file" | "parseFile" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&YamlStdlib::generate_parse_file_code(&path));
                }
            }
            "stringify" => {
                if let Some(arg) = args.first() {
                    let value = self.capture_expression(arg);
                    self.write(&YamlStdlib::generate_stringify_code(&value));
                }
            }
            "write_file" | "writeFile" => {
                if args.len() >= 2 {
                    let path = self.capture_expression(&args[0]);
                    let value = self.capture_expression(&args[1]);
                    self.write(&YamlStdlib::generate_write_file_code(&path, &value));
                }
            }
            "validate" => {
                if args.len() >= 2 {
                    let path = self.capture_expression(&args[0]);
                    let schema = self.capture_expression(&args[1]);
                    self.write(&YamlStdlib::generate_validate_code(&path, &schema));
                }
            }
            _ => self.write(&format!("// Unknown yaml method: {}", method)),
        }
    }

    fn generate_audit_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::audit::AuditStdlib;
        match method {
            "log" => {
                if let Some(arg) = args.first() {
                    let log_data = self.capture_expression(arg);
                    self.write(&AuditStdlib::generate_log_code(&log_data));
                }
            }
            "query" => {
                if let Some(arg) = args.first() {
                    let filters = self.capture_expression(arg);
                    self.write(&AuditStdlib::generate_query_code(&filters));
                }
            }
            "export" => {
                if args.len() >= 2 {
                    let format = self.capture_expression(&args[0]);
                    let filters = self.capture_expression(&args[1]);
                    self.write(&AuditStdlib::generate_export_code(&format, &filters));
                }
            }
            _ => self.write(&format!("// Unknown audit method: {}", method)),
        }
    }

    #[cfg(feature = "security")]
    fn generate_encryption_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::encryption::EncryptionStdlib;
        match method {
            "aes_encrypt" | "aesEncrypt" => {
                if args.len() >= 2 {
                    let data = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    self.write(&EncryptionStdlib::generate_aes_encrypt_code(&data, &key));
                }
            }
            "aes_decrypt" | "aesDecrypt" => {
                if args.len() >= 2 {
                    let encrypted = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    self.write(&EncryptionStdlib::generate_aes_decrypt_code(
                        &encrypted, &key,
                    ));
                }
            }
            "rsa_generate_keypair" | "rsaGenerateKeypair" => {
                if let Some(arg) = args.first() {
                    let bits = self.capture_expression(arg);
                    self.write(&EncryptionStdlib::generate_rsa_generate_keypair_code(&bits));
                }
            }
            "rsa_encrypt" | "rsaEncrypt" => {
                if args.len() >= 2 {
                    let data = self.capture_expression(&args[0]);
                    let public_key = self.capture_expression(&args[1]);
                    self.write(&EncryptionStdlib::generate_rsa_encrypt_code(
                        &data,
                        &public_key,
                    ));
                }
            }
            "rsa_decrypt" | "rsaDecrypt" => {
                if args.len() >= 2 {
                    let encrypted = self.capture_expression(&args[0]);
                    let private_key = self.capture_expression(&args[1]);
                    self.write(&EncryptionStdlib::generate_rsa_decrypt_code(
                        &encrypted,
                        &private_key,
                    ));
                }
            }
            "fernet_generate_key" | "fernetGenerateKey" => {
                self.write(&EncryptionStdlib::generate_fernet_generate_key_code());
            }
            "fernet_encrypt" | "fernetEncrypt" => {
                if args.len() >= 2 {
                    let data = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    self.write(&EncryptionStdlib::generate_fernet_encrypt_code(&data, &key));
                }
            }
            "fernet_decrypt" | "fernetDecrypt" => {
                if args.len() >= 2 {
                    let encrypted = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    self.write(&EncryptionStdlib::generate_fernet_decrypt_code(
                        &encrypted, &key,
                    ));
                }
            }
            "generate_key" | "generateKey" => {
                if let Some(arg) = args.first() {
                    let algorithm = self.capture_expression(arg);
                    self.write(&EncryptionStdlib::generate_generate_key_code(&algorithm));
                }
            }
            "store_key" | "storeKey" => {
                if args.len() >= 3 {
                    let key_id = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    let vault = self.capture_expression(&args[2]);
                    self.write(&EncryptionStdlib::generate_store_key_code(
                        &key_id, &key, &vault,
                    ));
                }
            }
            "retrieve_key" | "retrieveKey" => {
                if let Some(arg) = args.first() {
                    let key_id = self.capture_expression(arg);
                    self.write(&EncryptionStdlib::generate_retrieve_key_code(&key_id));
                }
            }
            _ => self.write(&format!("// Unknown encryption method: {}", method)),
        }
    }

    fn generate_alerting_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::alerting::AlertingStdlib;
        match method {
            "create_rule" | "createRule" => {
                if let Some(arg) = args.first() {
                    let rule = self.capture_expression(arg);
                    self.write(&AlertingStdlib::generate_create_rule_code(&rule));
                }
            }
            "check" => {
                if args.len() >= 3 {
                    let metric = self.capture_expression(&args[0]);
                    let value = self.capture_expression(&args[1]);
                    let rules = self.capture_expression(&args[2]);
                    self.write(&AlertingStdlib::generate_check_code(
                        &metric, &value, &rules,
                    ));
                }
            }
            "trigger" => {
                if let Some(arg) = args.first() {
                    let alert = self.capture_expression(arg);
                    self.write(&AlertingStdlib::generate_trigger_code(&alert));
                }
            }
            "history" => {
                if let Some(arg) = args.first() {
                    let filters = self.capture_expression(arg);
                    self.write(&AlertingStdlib::generate_history_code(&filters));
                }
            }
            _ => self.write(&format!("// Unknown alerting method: {}", method)),
        }
    }

    fn generate_nlp_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::nlp::NlpStdlib;
        match method {
            "tokenize" => {
                if let Some(arg) = args.first() {
                    let text = self.capture_expression(arg);
                    self.write(&NlpStdlib::generate_tokenize_code(&text));
                }
            }
            "sentiment" => {
                if let Some(arg) = args.first() {
                    let text = self.capture_expression(arg);
                    self.write(&NlpStdlib::generate_sentiment_code(&text));
                }
            }
            "ner" => {
                if let Some(arg) = args.first() {
                    let text = self.capture_expression(arg);
                    self.write(&NlpStdlib::generate_ner_code(&text));
                }
            }
            "keywords" => {
                if args.len() >= 2 {
                    let text = self.capture_expression(&args[0]);
                    let count = self.capture_expression(&args[1]);
                    self.write(&NlpStdlib::generate_keywords_code(&text, &count));
                }
            }
            "similarity" => {
                if args.len() >= 2 {
                    let text1 = self.capture_expression(&args[0]);
                    let text2 = self.capture_expression(&args[1]);
                    self.write(&NlpStdlib::generate_similarity_code(&text1, &text2));
                }
            }
            "summarize" => {
                if args.len() >= 2 {
                    let text = self.capture_expression(&args[0]);
                    let sentences = self.capture_expression(&args[1]);
                    self.write(&NlpStdlib::generate_summarize_code(&text, &sentences));
                }
            }
            _ => self.write(&format!("// Unknown nlp method: {}", method)),
        }
    }

    fn generate_workflow_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::workflow::WorkflowStdlib;
        match method {
            "create" => {
                if let Some(arg) = args.first() {
                    let definition = self.capture_expression(arg);
                    self.write(&WorkflowStdlib::generate_create_code(&definition));
                }
            }
            "start" => {
                if let Some(arg) = args.first() {
                    let workflow = self.capture_expression(arg);
                    self.write(&WorkflowStdlib::generate_start_code(&workflow));
                }
            }
            "execute_step" | "executeStep" => {
                if args.len() >= 2 {
                    let workflow = self.capture_expression(&args[0]);
                    let step_id = self.capture_expression(&args[1]);
                    self.write(&WorkflowStdlib::generate_execute_step_code(
                        &workflow, &step_id,
                    ));
                }
            }
            "get_status" | "getStatus" => {
                if let Some(arg) = args.first() {
                    let workflow = self.capture_expression(arg);
                    self.write(&WorkflowStdlib::generate_get_status_code(&workflow));
                }
            }
            "get_history" | "getHistory" => {
                if let Some(arg) = args.first() {
                    let workflow = self.capture_expression(arg);
                    self.write(&WorkflowStdlib::generate_get_history_code(&workflow));
                }
            }
            "complete" => {
                if let Some(arg) = args.first() {
                    let workflow = self.capture_expression(arg);
                    self.write(&WorkflowStdlib::generate_complete_code(&workflow));
                }
            }
            "fail" => {
                if args.len() >= 2 {
                    let workflow = self.capture_expression(&args[0]);
                    let error = self.capture_expression(&args[1]);
                    self.write(&WorkflowStdlib::generate_fail_code(&workflow, &error));
                }
            }
            _ => self.write(&format!("// Unknown workflow method: {}", method)),
        }
    }

    fn generate_scheduler_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::scheduler::SchedulerStdlib;
        match method {
            "schedule" => {
                if args.len() >= 2 {
                    let task = self.capture_expression(&args[0]);
                    let cron = self.capture_expression(&args[1]);
                    self.write(&SchedulerStdlib::generate_schedule_code(&task, &cron));
                }
            }
            "schedule_interval" | "scheduleInterval" => {
                if args.len() >= 2 {
                    let task = self.capture_expression(&args[0]);
                    let interval = self.capture_expression(&args[1]);
                    self.write(&SchedulerStdlib::generate_schedule_interval_code(
                        &task, &interval,
                    ));
                }
            }
            "cancel" => {
                if let Some(arg) = args.first() {
                    let task_id = self.capture_expression(arg);
                    self.write(&SchedulerStdlib::generate_cancel_code(&task_id));
                }
            }
            "list" => {
                self.write(&SchedulerStdlib::generate_list_code());
            }
            "get" => {
                if let Some(arg) = args.first() {
                    let task_id = self.capture_expression(arg);
                    self.write(&SchedulerStdlib::generate_get_code(&task_id));
                }
            }
            "enable" => {
                if let Some(arg) = args.first() {
                    let task_id = self.capture_expression(arg);
                    self.write(&SchedulerStdlib::generate_enable_code(&task_id));
                }
            }
            "disable" => {
                if let Some(arg) = args.first() {
                    let task_id = self.capture_expression(arg);
                    self.write(&SchedulerStdlib::generate_disable_code(&task_id));
                }
            }
            _ => self.write(&format!("// Unknown scheduler method: {}", method)),
        }
    }

    fn generate_event_bus_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::event_bus::EventBusStdlib;
        match method {
            "create" => {
                self.write(&EventBusStdlib::generate_create_code());
            }
            "publish" => {
                if args.len() >= 3 {
                    let bus = self.capture_expression(&args[0]);
                    let topic = self.capture_expression(&args[1]);
                    let event = self.capture_expression(&args[2]);
                    self.write(&EventBusStdlib::generate_publish_code(&bus, &topic, &event));
                }
            }
            "subscribe" => {
                if args.len() >= 2 {
                    let bus = self.capture_expression(&args[0]);
                    let topic = self.capture_expression(&args[1]);
                    self.write(&EventBusStdlib::generate_subscribe_code(&bus, &topic));
                }
            }
            "unsubscribe" => {
                if let Some(arg) = args.first() {
                    let subscription = self.capture_expression(arg);
                    self.write(&EventBusStdlib::generate_unsubscribe_code(&subscription));
                }
            }
            "get_history" | "getHistory" => {
                if args.len() >= 3 {
                    let bus = self.capture_expression(&args[0]);
                    let topic = self.capture_expression(&args[1]);
                    let limit = self.capture_expression(&args[2]);
                    self.write(&EventBusStdlib::generate_get_history_code(
                        &bus, &topic, &limit,
                    ));
                }
            }
            _ => self.write(&format!("// Unknown event_bus method: {}", method)),
        }
    }

    fn generate_fixtures_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::fixtures::FixturesStdlib;
        match method {
            "create" => {
                if let Some(arg) = args.first() {
                    let template = self.capture_expression(arg);
                    self.write(&FixturesStdlib::generate_create_code(&template));
                }
            }
            "create_many" | "createMany" => {
                if args.len() >= 2 {
                    let template = self.capture_expression(&args[0]);
                    let count = self.capture_expression(&args[1]);
                    self.write(&FixturesStdlib::generate_create_many_code(
                        &template, &count,
                    ));
                }
            }
            "factory" => {
                if args.len() >= 2 {
                    let name = self.capture_expression(&args[0]);
                    let builder = self.capture_expression(&args[1]);
                    self.write(&FixturesStdlib::generate_factory_code(&name, &builder));
                }
            }
            "build" => {
                if args.len() >= 2 {
                    let factory = self.capture_expression(&args[0]);
                    let overrides = self.capture_expression(&args[1]);
                    self.write(&FixturesStdlib::generate_build_code(&factory, &overrides));
                }
            }
            _ => self.write(&format!("// Unknown fixtures method: {}", method)),
        }
    }

    fn generate_mocks_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::mocks::MocksStdlib;
        match method {
            "mock" => {
                if args.len() >= 2 {
                    let original = self.capture_expression(&args[0]);
                    let mock = self.capture_expression(&args[1]);
                    self.write(&MocksStdlib::generate_mock_code(&original, &mock));
                }
            }
            "spy" => {
                if let Some(arg) = args.first() {
                    let target = self.capture_expression(arg);
                    self.write(&MocksStdlib::generate_spy_code(&target));
                }
            }
            "verify" => {
                if args.len() >= 2 {
                    let spy = self.capture_expression(&args[0]);
                    let expected_calls = self.capture_expression(&args[1]);
                    self.write(&MocksStdlib::generate_verify_code(&spy, &expected_calls));
                }
            }
            "reset" => {
                if let Some(arg) = args.first() {
                    let spy = self.capture_expression(arg);
                    self.write(&MocksStdlib::generate_reset_code(&spy));
                }
            }
            "stub" => {
                if let Some(arg) = args.first() {
                    let return_value = self.capture_expression(arg);
                    self.write(&MocksStdlib::generate_stub_code(&return_value));
                }
            }
            _ => self.write(&format!("// Unknown mocks method: {}", method)),
        }
    }

    fn generate_template_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::template::TemplateStdlib;
        match method {
            "render" => {
                if args.len() >= 2 {
                    let template = self.capture_expression(&args[0]);
                    let data = self.capture_expression(&args[1]);
                    self.write(&TemplateStdlib::generate_render_code(&template, &data));
                }
            }
            "render_file" | "renderFile" => {
                if args.len() >= 2 {
                    let path = self.capture_expression(&args[0]);
                    let data = self.capture_expression(&args[1]);
                    self.write(&TemplateStdlib::generate_render_file_code(&path, &data));
                }
            }
            "partial" => {
                if args.len() >= 2 {
                    let partial_path = self.capture_expression(&args[0]);
                    let data = self.capture_expression(&args[1]);
                    self.write(&TemplateStdlib::generate_partial_code(&partial_path, &data));
                }
            }
            "cache" => {
                if args.len() >= 2 {
                    let template = self.capture_expression(&args[0]);
                    let cache_key = self.capture_expression(&args[1]);
                    self.write(&TemplateStdlib::generate_cache_code(&template, &cache_key));
                }
            }
            _ => self.write(&format!("// Unknown template method: {}", method)),
        }
    }

    fn generate_env_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::env::EnvStdlib;
        match method {
            "load" => {
                if let Some(arg) = args.first() {
                    let path = self.capture_expression(arg);
                    self.write(&EnvStdlib::generate_load_code(&path));
                }
            }
            "get" => {
                if args.len() >= 1 {
                    let key = self.capture_expression(&args[0]);
                    let default = args.get(1).map(|a| self.capture_expression(a));
                    self.write(&EnvStdlib::generate_get_code(&key, default.as_deref()));
                }
            }
            "get_number" | "getNumber" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let default = self.capture_expression(&args[1]);
                    self.write(&EnvStdlib::generate_get_number_code(&key, &default));
                }
            }
            "get_bool" | "getBool" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let default = if let Expression::Literal(Literal::Boolean(b)) = &args[1] {
                        if *b {
                            "true"
                        } else {
                            "false"
                        }
                    } else {
                        "false"
                    };
                    self.write(&EnvStdlib::generate_get_bool_code(&key, default));
                }
            }
            "set" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let value = self.capture_expression(&args[1]);
                    self.write(&EnvStdlib::generate_set_code(&key, &value));
                }
            }
            "validate" => {
                if let Some(arg) = args.first() {
                    let schema = self.capture_expression(arg);
                    self.write(&EnvStdlib::generate_validate_code(&schema));
                }
            }
            "get_secret" | "getSecret" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let vault = self.capture_expression(&args[1]);
                    self.write(&EnvStdlib::generate_get_secret_code(&key, &vault));
                }
            }
            _ => self.write(&format!("// Unknown env method: {}", method)),
        }
    }

    fn generate_test_module_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::test_module::TestModuleStdlib;

        match method {
            "process_data" => {
                if args.len() >= 1 {
                    let input = self.capture_expression(&args[0]);
                    let options = if args.len() > 1 {
                        self.capture_expression(&args[1])
                    } else {
                        String::new()
                    };
                    self.write(&TestModuleStdlib::generate_process_data_code(
                        &input, &options,
                    ));
                } else {
                    self.write("// Error: process_data requires at least 1 arguments");
                }
            }
            "validate_input" => {
                if args.len() >= 1 {
                    let data = self.capture_expression(&args[0]);
                    self.write(&TestModuleStdlib::generate_validate_input_code(&data));
                } else {
                    self.write("// Error: validate_input requires at least 1 arguments");
                }
            }
            "transform_format" => {
                if args.len() >= 2 {
                    let data = self.capture_expression(&args[0]);
                    let target_format = self.capture_expression(&args[1]);
                    self.write(&TestModuleStdlib::generate_transform_format_code(
                        &data,
                        &target_format,
                    ));
                } else {
                    self.write("// Error: transform_format requires at least 2 arguments");
                }
            }
            _ => self.write(&format!("// Unknown test_module method: {}", method)),
        }
    }

    fn generate_metrics_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::metrics::MetricsStdlib;
        match method {
            "increment" => {
                if args.len() >= 1 {
                    let name = self.capture_expression(&args[0]);
                    let labels = args.get(1).map(|a| self.capture_expression(a));
                    self.write(&MetricsStdlib::generate_increment_code(
                        &name,
                        labels.as_deref(),
                    ));
                }
            }
            "gauge" => {
                if args.len() >= 2 {
                    let name = self.capture_expression(&args[0]);
                    let value = self.capture_expression(&args[1]);
                    let labels = args.get(2).map(|a| self.capture_expression(a));
                    self.write(&MetricsStdlib::generate_gauge_code(
                        &name,
                        &value,
                        labels.as_deref(),
                    ));
                }
            }
            "histogram" => {
                if args.len() >= 2 {
                    let name = self.capture_expression(&args[0]);
                    let value = self.capture_expression(&args[1]);
                    let labels = args.get(2).map(|a| self.capture_expression(a));
                    self.write(&MetricsStdlib::generate_histogram_code(
                        &name,
                        &value,
                        labels.as_deref(),
                    ));
                }
            }
            _ => self.write(&format!("// Unknown metrics method: {}", method)),
        }
    }

    fn generate_cache_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::cache::CacheStdlib;
        match method {
            "set" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let value = self.capture_expression(&args[1]);
                    let ttl = args.get(2).map(|a| self.capture_expression(a));
                    self.write(&CacheStdlib::generate_set_code(
                        &key,
                        &value,
                        ttl.as_deref(),
                    ));
                }
            }
            "get" => {
                if let Some(arg) = args.first() {
                    let key = self.capture_expression(arg);
                    self.write(&CacheStdlib::generate_get_code(&key));
                }
            }
            "remove" => {
                if let Some(arg) = args.first() {
                    let key = self.capture_expression(arg);
                    self.write(&CacheStdlib::generate_remove_code(&key));
                }
            }
            "clear" => {
                self.write(&CacheStdlib::generate_clear_code());
            }
            "exists" => {
                if let Some(arg) = args.first() {
                    let key = self.capture_expression(arg);
                    self.write(&CacheStdlib::generate_exists_code(&key));
                }
            }
            "size" => {
                self.write(&CacheStdlib::generate_size_code());
            }
            _ => self.write(&format!("// Unknown cache method: {}", method)),
        }
    }

    fn generate_auth_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::auth::AuthStdlib;
        match method {
            "generate_token" | "generateToken" => {
                if args.len() >= 2 {
                    let service = self.capture_expression(&args[0]);
                    let claims = self.capture_expression(&args[1]);
                    self.write(&AuthStdlib::generate_generate_token_code(&service, &claims));
                }
            }
            "verify_token" | "verifyToken" => {
                if args.len() >= 2 {
                    let service = self.capture_expression(&args[0]);
                    let token = self.capture_expression(&args[1]);
                    self.write(&AuthStdlib::generate_verify_token_code(&service, &token));
                }
            }
            "extract_user_id" | "extractUserId" => {
                if args.len() >= 2 {
                    let service = self.capture_expression(&args[0]);
                    let token = self.capture_expression(&args[1]);
                    self.write(&AuthStdlib::generate_extract_user_id_code(&service, &token));
                }
            }
            "has_role" | "hasRole" => {
                if args.len() >= 3 {
                    let service = self.capture_expression(&args[0]);
                    let token = self.capture_expression(&args[1]);
                    let role = self.capture_expression(&args[2]);
                    self.write(&AuthStdlib::generate_has_role_code(&service, &token, &role));
                }
            }
            _ => self.write(&format!("// Unknown auth method: {}", method)),
        }
    }

    fn generate_validation_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::validation::ValidationStdlib;
        match method {
            "new" => {
                self.write(&ValidationStdlib::generate_new_code());
            }
            "required" => {
                if args.len() >= 3 {
                    let validator = self.capture_expression(&args[0]);
                    let field = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    self.write(&ValidationStdlib::generate_required_code(
                        &validator, &field, &value,
                    ));
                }
            }
            "min_length" | "minLength" => {
                if args.len() >= 4 {
                    let validator = self.capture_expression(&args[0]);
                    let field = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    let min = self.capture_expression(&args[3]);
                    self.write(&ValidationStdlib::generate_min_length_code(
                        &validator, &field, &value, &min,
                    ));
                }
            }
            "max_length" | "maxLength" => {
                if args.len() >= 4 {
                    let validator = self.capture_expression(&args[0]);
                    let field = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    let max = self.capture_expression(&args[3]);
                    self.write(&ValidationStdlib::generate_max_length_code(
                        &validator, &field, &value, &max,
                    ));
                }
            }
            "email" => {
                if args.len() >= 3 {
                    let validator = self.capture_expression(&args[0]);
                    let field = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    self.write(&ValidationStdlib::generate_email_code(
                        &validator, &field, &value,
                    ));
                }
            }
            "pattern" => {
                if args.len() >= 5 {
                    let validator = self.capture_expression(&args[0]);
                    let field = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    let pattern = self.capture_expression(&args[3]);
                    let message = self.capture_expression(&args[4]);
                    self.write(&ValidationStdlib::generate_pattern_code(
                        &validator, &field, &value, &pattern, &message,
                    ));
                }
            }
            "min" => {
                if args.len() >= 4 {
                    let validator = self.capture_expression(&args[0]);
                    let field = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    let min = self.capture_expression(&args[3]);
                    self.write(&ValidationStdlib::generate_min_code(
                        &validator, &field, &value, &min,
                    ));
                }
            }
            "max" => {
                if args.len() >= 4 {
                    let validator = self.capture_expression(&args[0]);
                    let field = self.capture_expression(&args[1]);
                    let value = self.capture_expression(&args[2]);
                    let max = self.capture_expression(&args[3]);
                    self.write(&ValidationStdlib::generate_max_code(
                        &validator, &field, &value, &max,
                    ));
                }
            }
            _ => self.write(&format!("// Unknown validation method: {}", method)),
        }
    }

    fn generate_logger_call(&mut self, method: &str, args: &[Expression]) {
        self.generate_log_call(method, args);
    }

    fn generate_pdf_call(&mut self, method: &str, _args: &[Expression]) {
        match method {
            "generate" => self.write("/* PDF Generation Logic (requires 'printpdf' crate) */"),
            _ => self.write(&format!("// Unknown pdf method: {}", method)),
        }
    }

    fn generate_regex_lib_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "match" => {
                if args.len() >= 2 {
                    let pattern = self.capture_expression(&args[0]);
                    let text = self.capture_expression(&args[1]);
                    self.write(&format!(
                        "regex::Regex::new({}).unwrap().is_match({})",
                        pattern, text
                    ));
                }
            }
            "replace" => {
                if args.len() >= 3 {
                    let pattern = self.capture_expression(&args[0]);
                    let text = self.capture_expression(&args[1]);
                    let replacement = self.capture_expression(&args[2]);
                    self.write(&format!(
                        "regex::Regex::new({}).unwrap().replace_all({}, {}).to_string()",
                        pattern, text, replacement
                    ));
                }
            }
            _ => self.write(&format!("// Unknown regex method: {}", method)),
        }
    }

    fn generate_search_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "contains" => {
                if args.len() >= 2 {
                    let haystack = self.capture_expression(&args[0]);
                    let needle = self.capture_expression(&args[1]);
                    self.write(&format!("{}.contains({})", haystack, needle));
                }
            }
            _ => self.write(&format!("// Unknown search method: {}", method)),
        }
    }

    fn generate_crypto_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::crypto::CryptoStdlib;
        match method {
            "sha256" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    self.write(&CryptoStdlib::generate_sha256_code(&input));
                }
            }
            "md5" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    self.write(&CryptoStdlib::generate_md5_code(&input));
                }
            }
            "uuid" => {
                self.write(&CryptoStdlib::generate_uuid_code());
            }
            "hash_password" | "hashPassword" => {
                if args.len() >= 2 {
                    let password = self.capture_expression(&args[0]);
                    let salt = self.capture_expression(&args[1]);
                    self.write(&CryptoStdlib::generate_hash_password_code(&password, &salt));
                }
            }
            _ => self.write(&format!("// Unknown crypto method: {}", method)),
        }
    }

    fn generate_http_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::http::HttpStdlib;
        match method {
            "get" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&HttpStdlib::generate_get_code(&url));
                }
            }
            "post" => {
                if args.len() >= 2 {
                    let url = self.capture_expression(&args[0]);
                    let body = self.capture_expression(&args[1]);
                    self.write(&HttpStdlib::generate_post_code(&url, &body));
                }
            }
            "put" => {
                if args.len() >= 2 {
                    let url = self.capture_expression(&args[0]);
                    let body = self.capture_expression(&args[1]);
                    self.write(&HttpStdlib::generate_put_code(&url, &body));
                }
            }
            "delete" => {
                if let Some(arg) = args.first() {
                    let url = self.capture_expression(arg);
                    self.write(&HttpStdlib::generate_delete_code(&url));
                }
            }
            _ => self.write(&format!("// Unknown http method: {}", method)),
        }
    }

    fn generate_json_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::json::JsonStdlib;
        match method {
            "parse" => {
                if let Some(arg) = args.first() {
                    let json_string = self.capture_expression(arg);
                    self.write(&JsonStdlib::generate_parse_code(&json_string));
                }
            }
            "stringify" => {
                if let Some(arg) = args.first() {
                    let value = self.capture_expression(arg);
                    self.write(&JsonStdlib::generate_stringify_code(&value));
                }
            }
            "get" => {
                if args.len() >= 2 {
                    let value = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    self.write(&JsonStdlib::generate_get_code(&value, &key));
                }
            }
            "set" => {
                if args.len() >= 3 {
                    let value = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    let new_value = self.capture_expression(&args[2]);
                    self.write(&JsonStdlib::generate_set_code(&value, &key, &new_value));
                }
            }
            "has_key" => {
                if args.len() >= 2 {
                    let value = self.capture_expression(&args[0]);
                    let key = self.capture_expression(&args[1]);
                    self.write(&JsonStdlib::generate_has_key_code(&value, &key));
                }
            }
            "keys" => {
                if let Some(arg) = args.first() {
                    let value = self.capture_expression(arg);
                    self.write(&JsonStdlib::generate_keys_code(&value));
                }
            }
            "length" => {
                if let Some(arg) = args.first() {
                    let value = self.capture_expression(arg);
                    self.write(&JsonStdlib::generate_length_code(&value));
                }
            }
            _ => self.write(&format!("// Unknown json method: {}", method)),
        }
    }

    fn capture_expression(&mut self, expr: &Expression) -> String {
        let mut captured = String::new();
        let old_output = std::mem::replace(&mut self.output, captured);
        self.generate_expression(expr);
        captured = std::mem::replace(&mut self.output, old_output);
        captured
    }
    fn is_route_handler(&self, decorators: &[Decorator]) -> Option<(String, String)> {
        for dec in decorators {
            match dec.name.as_str() {
                "GET" | "POST" | "PUT" | "DELETE" | "PATCH" => {
                    if let Some(arg) = dec.args.first() {
                        if let DecoratorArg::String(path) = arg {
                            return Some((dec.name.clone(), path.clone()));
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn is_complex_type(&self, t: &Type) -> bool {
        match t {
            Type::Named(n) => {
                n != "string" && n != "number" && n != "boolean" && n != "void" && n != "any"
            }
            Type::List(_) | Type::Map { .. } => true,
            _ => false,
        }
    }

    fn generate_analytics_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "track" => {
                if args.len() >= 2 {
                    let event = self.capture_expression(&args[0]);
                    let data = self.capture_expression(&args[1]);
                    self.write(&format!(
                        "tracing::info!(\"Analytics Event: {{}} - {{:?}}\", {}, {})",
                        event, data
                    ));
                }
            }
            _ => self.write(&format!("// Unknown analytics method: {}", method)),
        }
    }

    fn generate_compression_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "gzip" => {
                if let Some(arg) = args.first() {
                    let input = self.capture_expression(arg);
                    // Hypothetical helper usage
                    self.write(&format!("velin_runtime::compression::gzip({})", input));
                }
            }
            _ => self.write(&format!("// Unknown compression method: {}", method)),
        }
    }

    fn generate_geolocation_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "distance" => {
                if args.len() >= 4 {
                    let lat1 = self.capture_expression(&args[0]);
                    let lon1 = self.capture_expression(&args[1]);
                    let lat2 = self.capture_expression(&args[2]);
                    let lon2 = self.capture_expression(&args[3]);
                    // Haversine formula approximation or helper
                    self.write(&format!(
                        "velin_runtime::geo::distance({}, {}, {}, {})",
                        lat1, lon1, lat2, lon2
                    ));
                }
            }
            _ => self.write(&format!("// Unknown geolocation method: {}", method)),
        }
    }

    fn generate_i18n_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "translate" | "t" => {
                if args.len() >= 2 {
                    let key = self.capture_expression(&args[0]);
                    let lang = self.capture_expression(&args[1]);
                    self.write(&format!(
                        "velin_runtime::i18n::translate({}, {})",
                        key, lang
                    ));
                }
            }
            _ => self.write(&format!("// Unknown i18n method: {}", method)),
        }
    }

    fn generate_jwt_call(&mut self, method: &str, args: &[Expression]) {
        match method {
            "sign" => {
                if args.len() >= 2 {
                    let payload = self.capture_expression(&args[0]);
                    let secret = self.capture_expression(&args[1]);
                    self.write(&format!("jsonwebtoken::encode(&jsonwebtoken::Header::default(), &{}, &jsonwebtoken::EncodingKey::from_secret({}.as_bytes())).unwrap()", payload, secret));
                }
            }
            "verify" => {
                if args.len() >= 2 {
                    let token = self.capture_expression(&args[0]);
                    let secret = self.capture_expression(&args[1]);
                    self.write(&format!("jsonwebtoken::decode::<serde_json::Value>(&{}, &jsonwebtoken::DecodingKey::from_secret({}.as_bytes()), &jsonwebtoken::Validation::default()).is_ok()", token, secret));
                }
            }
            _ => self.write(&format!("// Unknown jwt method: {}", method)),
        }
    }

    fn generate_ml_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::ml::MlStdlib;

        match method {
            "load_model" | "loadModel" => {
                if args.len() >= 3 {
                    let name = self.capture_expression(&args[0]);
                    let model_type = self.capture_expression(&args[1]);
                    let path = self.capture_expression(&args[2]);
                    // Handle model type - assume string literal for now
                    let type_str = model_type.trim_matches('"');
                    self.write(&MlStdlib::generate_load_model_code(&name, type_str, &path));
                }
            }
            "predict" => {
                if args.len() >= 2 {
                    let name = self.capture_expression(&args[0]);
                    let input = self.capture_expression(&args[1]);
                    self.write(&MlStdlib::generate_predict_code(&name, &input));
                }
            }
            _ => self.write(&format!("// Unknown ml method: {}", method)),
        }
    }

    fn generate_flow_call(&mut self, method: &str, args: &[Expression]) {
        use crate::stdlib::flow::FlowStdlib;

        match method {
            "start" => {
                let name = if let Some(arg) = args.first() {
                    Some(self.capture_expression(arg))
                } else {
                    None
                };
                self.write(&FlowStdlib::generate_start_code(name.as_deref()));
            }
            "checkpoint" | "step" => {
                if let Some(arg) = args.first() {
                    let name = self.capture_expression(arg);
                    self.write(&FlowStdlib::generate_checkpoint_code(&name));
                }
            }
            "fail" => {
                if args.len() >= 2 {
                    let step = self.capture_expression(&args[0]);
                    let error = self.capture_expression(&args[1]);
                    self.write(&FlowStdlib::generate_fail_code(&step, &error));
                }
            }
            "commit" => {
                self.write(&FlowStdlib::generate_commit_code());
            }
            _ => self.write(&format!("// Unknown flow method: {}", method)),
        }
    }
}

impl Default for RustCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}
