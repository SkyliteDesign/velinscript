use crate::parser::ast::*;
use crate::codegen::framework::{Framework, FrameworkSelector};
#[cfg(feature = "sea-orm")]
use crate::stdlib::seaorm::SeaORMStdlib;
#[cfg(feature = "oauth2")]
use crate::stdlib::oauth2::OAuth2Stdlib;
#[cfg(feature = "privacy")]
use crate::stdlib::privacy::PrivacyStdlib;

pub struct RustCodeGenerator {
    output: String,
    indent_level: usize,
    use_seaorm: bool,
    framework: Framework,
    has_validation: bool,
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
    
    pub fn generate(&mut self, program: &Program, config_framework: Option<&str>, config_orm: Option<&str>) -> String {
        self.output.clear();
        
        // Detect framework
        let framework = FrameworkSelector::detect_framework(program, config_framework);
        self.framework = framework;
        let framework_imports = FrameworkSelector::generate_imports(framework);
        self.writeln(&framework_imports);
        
        // Check for validation usage
        self.has_validation = self.has_validation_decorators(program);
        if self.has_validation {
            use crate::stdlib::validation::Validator;
            self.writeln("use regex::Regex;");
            let validator_code = Validator::generate_validator_struct();
            self.writeln(&validator_code);
            self.writeln("");
        }
        
        // Add necessary imports
        self.writeln("use serde::{Serialize, Deserialize};");
        self.writeln("use anyhow::Result;");
        self.writeln("use itertools::Itertools;");
        self.writeln("use rayon::prelude::*;");
        self.writeln("use tracing::{info, error, warn, debug};");
        self.writeln("");
        
        // Check ORM
        let use_seaorm = config_orm.map(|s| s == "seaorm").unwrap_or(false) || self.has_seaorm_usage(program);
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
                self.writeln("// Note: AuthMiddleware and RoleMiddleware need to be implemented");
            }
            self.writeln("");
        }
        
        // Generate all items
        for item in &program.items {
            self.generate_item(item, &framework, self.use_seaorm);
            self.writeln("");
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
                    if matches!(decorator.name.as_str(), "Validate" | "@Validate" | "Validation" | "@Validation") {
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
        
        if function.is_async {
            self.write("async ");
        }
        
        self.write("fn ");
        self.write(&self.to_snake_case(&function.name));
        self.write("(");
        
        // Generate parameters
        for (i, param) in function.params.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.write(&self.to_snake_case(&param.name));
            self.write(": ");
            self.generate_type(&param.param_type);
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
        
        // Erstelle tracing span für async Funktionen
        if function.is_async {
            self.write("    ");
            self.writeln(&format!("let _span = tracing::span!(tracing::Level::INFO, \"{}\");", function.name));
            self.write("    ");
            self.writeln("let _enter = _span.enter();");
        }
        self.indent();
        
        // Generate validation code if needed
        if self.has_validation {
            self.generate_validation_code(function, framework);
        }
        
        // Generate function body
        self.generate_block(&function.body);
        
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
                        self.writeln(&format!("#[actix_web::web::middleware(RoleMiddleware::new(\"{}\"))]", role));
                    } else {
                        self.writeln("#[actix_web::web::middleware(RoleMiddleware::new(\"user\"))]");
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
                self.writeln("#[test]");
            }
            _ => {
                // Generic decorator
                let mut args = Vec::new();
                for arg in &decorator.args {
                    args.push(self.decorator_arg_to_string(arg));
                }
                if !args.is_empty() {
                    self.writeln(&format!("#[{}({})]", decorator.name.to_lowercase(), args.join(", ")));
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
            self.writeln("#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Add, derive_more::Display, derive_more::From, derive_more::Into, derive_more::Deref)]");
        } else if !custom_derives.is_empty() {
            // Nur angegebene Derives
            let derive_list = custom_derives.iter()
                .map(|d| format!("derive_more::{}", d))
                .collect::<Vec<_>>()
                .join(", ");
            self.writeln(&format!("#[derive(Debug, Clone, Serialize, Deserialize, {})]", derive_list));
        } else {
            // Standard Derives
            self.writeln("#[derive(Debug, Clone, Serialize, Deserialize)]");
        }
        
        if struct_def.visibility == Visibility::Public {
            self.write("pub ");
        }
        self.write("struct ");
        self.write(&self.to_pascal_case(&struct_def.name));
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
            Type::Null => self.write("()"),
            Type::Named(name) => self.write(&self.to_pascal_case(name)),
            Type::Generic { name, params } => {
                match name.as_str() {
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
                }
            }
            Type::Function { params, return_type } => {
                self.write("fn(");
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.generate_type(param);
                }
                self.write(") -> ");
                self.generate_type(return_type);
            }
            Type::List(item_type) => {
                self.write("Vec<");
                self.generate_type(item_type);
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
                if types.len() == 1 {
                    self.write(",");
                }
                self.write(")");
            }
            Type::Optional(inner) => {
                self.write("Option<");
                self.generate_type(inner);
                self.write(">");
            }
        }
    }
    
    fn generate_block(&mut self, block: &Block) {
        for statement in &block.statements {
            self.generate_statement(statement);
        }
    }
    
    fn generate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_stmt) => {
                if let_stmt.mutable {
                    self.write("let mut ");
                } else {
                    self.write("let ");
                }
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
                    self.write(" => ");
                    self.generate_block(&arm.body);
                    self.writeln(",");
                }
                
                self.unindent();
                self.writeln("}");
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
                self.generate_expression(left);
                self.write(" ");
                self.generate_binary_operator(op);
                self.write(" ");
                self.generate_expression(right);
            }
            Expression::UnaryOp { op, expr } => {
                self.generate_unary_operator(op);
                self.generate_expression(expr);
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
                    }
                }
                
                // Check if this is a standard library function call
                if let Expression::Member { object, member } = callee.as_ref() {
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
                        }
                    } else if let Expression::Member { object: inner_obj, member: inner_member } = object.as_ref() {
                        // Nested member access wie list.groupBy
                        if let Expression::Identifier(inner_name) = inner_obj.as_ref() {
                            if inner_name == "list" {
                                self.generate_list_extension_call(inner_member, member, args, object);
                                return;
                            } else if inner_name == "string" {
                                self.generate_string_extension_call(inner_member, member, args, object);
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
            }
            Expression::Member { object, member } => {
                self.generate_expression(object);
                self.write(".");
                self.write(&self.to_snake_case(member));
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
            Expression::StructLiteral { name, fields } => {
                self.write(&self.to_pascal_case(name));
                self.write(" {");
                self.writeln("");
                self.indent();
                
                for (i, (field_name, field_expr)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.writeln(",");
                    }
                    self.write(&self.to_snake_case(field_name));
                    self.write(": ");
                    self.generate_expression(field_expr);
                }
                
                self.writeln("");
                self.unindent();
                self.write("}");
            }
            Expression::GenericConstructor { name, type_params, args: _args } => {
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
                self.write(&n.to_string());
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
        let pii_keywords = vec!["email", "phone", "ssn", "passport", "credit_card", "ip", "address"];
        pii_keywords.iter().any(|keyword| field_lower.contains(keyword))
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
                        if let Some(Expression::Literal(crate::parser::ast::Literal::String(query))) = args.first() {
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
                // db.query("SELECT * FROM users") -> sqlx::query("SELECT * FROM users")
                if let Some(Expression::Literal(crate::parser::ast::Literal::String(_))) = args.first() {
                    self.write("sqlx::query(");
                    self.generate_expression(&args[0]);
                    self.write(").execute(&db).await");
                } else {
                    self.write("sqlx::query(");
                    if let Some(arg) = args.first() {
                        self.generate_expression(arg);
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
    
    fn generate_list_extension_call(&mut self, _list_method: &str, method: &str, args: &[Expression], object: &Expression) {
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
    
    fn generate_string_extension_call(&mut self, _string_method: &str, method: &str, args: &[Expression], object: &Expression) {
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
            matches!(d.name.as_str(), "Validate" | "@Validate" | "Validation" | "@Validation")
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
                    self.writeln(&format!("validator.required(\"{}\", Some(&{}));", param_name, param_name));
                    // Auto-validate email if parameter name contains "email"
                    if param_name.to_lowercase().contains("email") {
                        self.write("    ");
                        self.writeln(&format!("validator.email(\"{}\", &{});", param_name, param_name));
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
                self.writeln("(axum::http::StatusCode::BAD_REQUEST, axum::Json(response)).into_response()");
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
                self.writeln("return Ok(actix_web::HttpResponse::BadRequest().json(serde_json::json!({");
                self.write("            ");
                self.writeln("\"error\": \"Validation failed\",");
                self.write("            ");
                self.writeln("\"errors\": errors");
                self.write("        ");
                self.writeln("})));");
            }
        }
        
        self.write("    ");
        self.writeln("}");
        self.writeln("");
    }
}

impl Default for RustCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}
