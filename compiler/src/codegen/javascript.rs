use super::{CodeGenerator, CodegenConfig, TargetLanguage};
use crate::parser::ast::*;
use crate::codegen::framework::{Framework, FrameworkSelector};
use anyhow::Result;

/// JavaScript Code Generator
/// 
/// Generiert modernen JavaScript Code (ES2020+) ohne TypeScript-Typen
/// Unterstützt Express und NestJS Frameworks
pub struct JavaScriptCodeGenerator {
    buffer: String,
    indent_level: usize,
    framework: Option<Framework>,
    routes: Vec<(String, String, String)>, // (method, path, handler_name)
}

impl JavaScriptCodeGenerator {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            indent_level: 0,
            framework: None,
            routes: Vec::new(),
        }
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    fn writeln(&mut self, s: &str) {
        for _ in 0..self.indent_level {
            self.buffer.push_str("    ");
        }
        self.buffer.push_str(s);
        self.buffer.push('\n');
    }

    fn generate_type(&self, ty: &Type) -> String {
        match ty {
            Type::String => "string".to_string(),
            Type::Number => "number".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Void => "void".to_string(),
            Type::List(inner) => format!("Array<{}>", self.generate_type(inner)),
            Type::Map { key, value } => format!("Map<{}, {}>", self.generate_type(key), self.generate_type(value)),
            Type::Result { ok, err } => format!("Promise<{} | {}>", self.generate_type(ok), self.generate_type(err)),
            Type::Optional(inner) => format!("{} | null", self.generate_type(inner)),
            Type::Named(name) => name.clone(),
            Type::Generic { name, params } => {
                if params.is_empty() {
                    name.clone()
                } else {
                    format!("{}<{}>", name, params.iter().map(|p| self.generate_type(p)).collect::<Vec<_>>().join(", "))
                }
            }
            _ => "any".to_string(),
        }
    }

    fn generate_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::String(s) => {
                    let escaped = s.replace("\"", "\\\"").replace("\n", "\\n");
                    self.write(&format!("\"{}\"", escaped));
                }
                Literal::Number(n) => self.write(&n.to_string()),
                Literal::Boolean(b) => self.write(if *b { "true" } else { "false" }),
                Literal::Null => self.write("null"),
            }
            Expression::Identifier(name) => self.write(name),
            Expression::Call { callee, args } => {
                self.generate_expression(callee);
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.generate_expression(arg);
                }
                self.write(")");
            }
            Expression::Member { object, member } => {
                self.generate_expression(object);
                self.write(&format!(".{}", member));
            }
            Expression::BinaryOp { left, op, right } => {
                self.write("(");
                self.generate_expression(left);
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Modulo => "%",
                    BinaryOperator::Eq => "===",
                    BinaryOperator::NotEq => "!==",
                    BinaryOperator::Lt => "<",
                    BinaryOperator::LtEq => "<=",
                    BinaryOperator::Gt => ">",
                    BinaryOperator::GtEq => ">=",
                    BinaryOperator::And => "&&",
                    BinaryOperator::Or => "||",
                };
                self.write(&format!(" {} ", op_str));
                self.generate_expression(right);
                self.write(")");
            }
            Expression::UnaryOp { op, expr } => {
                let op_str = match op {
                    UnaryOperator::Not => "!",
                    UnaryOperator::Minus => "-",
                };
                self.write(op_str);
                self.generate_expression(expr);
            }
            Expression::Await { expr } => {
                self.write("await ");
                self.generate_expression(expr);
            }
            Expression::ListLiteral(elements) => {
                self.write("[");
                for (i, e) in elements.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.generate_expression(e);
                }
                self.write("]");
            }
            Expression::MapLiteral(fields) => {
                self.write("{ ");
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.write(&format!("{}: ", k));
                    self.generate_expression(v);
                }
                self.write(" }");
            }
            _ => self.write("/* unsupported expression */"),
        }
    }


    fn generate_struct(&mut self, s: &Struct) {
        self.writeln(&format!("class {} {{", s.name));
        self.indent();
        
        // Constructor
        let params: Vec<String> = s.fields.iter().map(|f| {
            format!("{}", f.name)
        }).collect();
        self.writeln(&format!("constructor({}) {{", params.join(", ")));
        self.indent();
        for field in &s.fields {
            self.writeln(&format!("this.{} = {};", field.name, field.name));
        }
        self.dedent();
        self.writeln("}");
        
        self.dedent();
        self.writeln("}");
    }

    fn add_route(&mut self, method: &str, decorator: &Decorator, function_name: &str) {
        if let Some(arg) = decorator.args.first() {
            if let DecoratorArg::String(path) = arg {
                self.routes.push((method.to_string(), path.clone(), function_name.to_string()));
            }
        }
    }

    fn generate_function(&mut self, f: &Function) {
        // Check for route decorators
        let is_handler = f.decorators.iter().any(|d| {
            matches!(d.name.as_str(), "Get" | "Post" | "Put" | "Delete" | "@Get" | "@Post" | "@Put" | "@Delete")
        });

        // Collect routes
        for decorator in &f.decorators {
            let name = if decorator.name.starts_with('@') {
                &decorator.name[1..]
            } else {
                &decorator.name[..]
            };
            match name {
                "Get" => self.add_route("GET", decorator, &f.name),
                "Post" => self.add_route("POST", decorator, &f.name),
                "Put" => self.add_route("PUT", decorator, &f.name),
                "Delete" => self.add_route("DELETE", decorator, &f.name),
                _ => {}
            }
        }

        // Generate function signature
        let async_keyword = if f.is_async { "async " } else { "" };
        let params: Vec<String> = f.params.iter().map(|p| {
            format!("{}", p.name)
        }).collect();

        // For Express handlers, add req, res if not present
        let mut handler_params = params;
        if is_handler && self.framework == Some(Framework::Express) {
            if !handler_params.contains(&"req".to_string()) {
                handler_params.insert(0, "req".to_string());
            }
            if !handler_params.contains(&"res".to_string()) {
                handler_params.insert(1, "res".to_string());
            }
        }

        self.writeln(&format!("{}function {}({}) {{", async_keyword, f.name, handler_params.join(", ")));
        self.indent();

        // Generate body
        if f.body.statements.is_empty() {
            // Leerer Body - generiere pass statement oder return
            if let Some(ref return_type) = f.return_type {
                match return_type {
                    crate::parser::ast::Type::Void => {
                        // Void return - kein return statement
                    }
                    _ => {
                        self.writeln("return null;");
                    }
                }
            }
        } else {
            for stmt in &f.body.statements {
                self.generate_statement(stmt);
            }
        }

        self.dedent();
        self.writeln("}");
    }

    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Return(ret_stmt) => {
                if let Some(e) = &ret_stmt.value {
                    if self.framework == Some(Framework::Express) {
                        self.writeln("res.json(");
                        self.indent();
                        self.generate_expression(e);
                        self.dedent();
                        self.writeln(");");
                        self.writeln("return;");
                    } else {
                        self.write("return ");
                        self.generate_expression(e);
                        self.writeln(";");
                    }
                } else {
                    if self.framework == Some(Framework::Express) {
                        self.writeln("res.status(200).send();");
                        self.writeln("return;");
                    } else {
                        self.writeln("return;");
                    }
                }
            }
            Statement::Let(let_stmt) => {
                let kw = if let_stmt.mutable { "let" } else { "const" };
                self.write(&format!("{} {} = ", kw, let_stmt.name));
                self.generate_expression(&let_stmt.value);
                self.writeln(";");
            }
            Statement::If(if_stmt) => {
                self.write("if (");
                self.generate_expression(&if_stmt.condition);
                self.writeln(") {");
                self.indent();
                for s in &if_stmt.then_block.statements {
                    self.generate_statement(s);
                }
                self.dedent();
                self.writeln("}");
                if let Some(else_block) = &if_stmt.else_block {
                    self.writeln("else {");
                    self.indent();
                    for s in &else_block.statements {
                        self.generate_statement(s);
                    }
                    self.dedent();
                    self.writeln("}");
                }
            }
            Statement::Expression(expr_stmt) => {
                self.generate_expression(&expr_stmt.expression);
                self.writeln(";");
            }
            Statement::For(for_stmt) => {
                // ForStatement hat variable, iterable, body (nicht init, condition, update)
                self.write("for (const ");
                self.write(&for_stmt.variable);
                self.write(" of ");
                self.generate_expression(&for_stmt.iterable);
                self.writeln(") {");
                self.indent();
                for s in &for_stmt.body.statements {
                    self.generate_statement(s);
                }
                self.dedent();
                self.writeln("}");
            }
            Statement::While(while_stmt) => {
                self.write("while (");
                self.generate_expression(&while_stmt.condition);
                self.writeln(") {");
                self.indent();
                for s in &while_stmt.body.statements {
                    self.generate_statement(s);
                }
                self.dedent();
                self.writeln("}");
            }
            _ => {
                self.writeln(&format!("// Unimplemented statement: {:?}", stmt));
            }
        }
    }

    fn write(&mut self, s: &str) {
        for _ in 0..self.indent_level {
            self.buffer.push_str("    ");
        }
        self.buffer.push_str(s);
    }
}

impl CodeGenerator for JavaScriptCodeGenerator {
    fn generate(&mut self, program: &Program, config: &CodegenConfig) -> Result<String> {
        self.buffer.clear();
        self.indent_level = 0;
        self.routes.clear();

        // Detect framework
        let framework = FrameworkSelector::detect_framework(program, config.framework.as_deref());
        self.framework = Some(framework);

        // Header
        self.writeln("// Auto-generated JavaScript Code");
        self.writeln("// Generated by VelinScript Compiler");
        self.writeln("");

        // Framework imports
        match framework {
            Framework::Express => {
                self.writeln("const express = require('express');");
                self.writeln("const app = express();");
                self.writeln("app.use(express.json());");
                self.writeln("");
            }
            Framework::NestJS => {
                self.writeln("// NestJS requires TypeScript - consider using --target typescript");
                self.writeln("");
            }
            _ => {}
        }

        // Structs/Classes
        for item in &program.items {
            if let Item::Struct(s) = item {
                self.generate_struct(s);
                self.writeln("");
            }
        }

        // Functions
        for item in &program.items {
            if let Item::Function(f) = item {
                self.generate_function(f);
                self.writeln("");
            }
        }

        // App initialization for Express
        if framework == Framework::Express && !self.routes.is_empty() {
            self.writeln("");
            self.writeln("// Routes");
            let routes_clone = self.routes.clone();
            for (method, path, handler) in &routes_clone {
                self.writeln(&format!("app.{}({}, {});", method.to_lowercase(), path, handler));
            }
            self.writeln("");
            self.writeln("const PORT = process.env.PORT || 3000;");
            self.writeln("app.listen(PORT, () => {");
            self.indent();
            self.writeln("console.log(`Server running on port ${PORT}`);");
            self.dedent();
            self.writeln("});");
        }

        Ok(self.buffer.clone())
    }

    fn get_target_language(&self) -> TargetLanguage {
        TargetLanguage::JavaScript
    }
}
