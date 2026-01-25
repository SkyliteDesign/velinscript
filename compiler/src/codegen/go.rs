use crate::codegen::framework::{Framework, FrameworkSelector};
use crate::codegen::traits::{CodeGenerator, CodegenConfig, TargetLanguage};
use crate::compiler::language::VELISCH_FINGERPRINT;
use crate::parser::ast::*;

pub struct GoCodeGenerator {
    output: String,
    indent_level: usize,
    framework: Framework,
    routes: Vec<(String, String, String)>, // Method, Path, HandlerName
}

impl GoCodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
            framework: Framework::Axum,
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
        let indent = "\t".repeat(self.indent_level);
        self.output.push_str(&format!("{}{}\n", indent, s));
    }

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn generate_struct(&mut self, s: &Struct) {
        self.writeln(&format!("type {} struct {{", s.name));
        self.indent();
        for field in &s.fields {
            let go_type = self.map_type(&field.field_type);
            // Add JSON tag automatically
            let json_tag = format!("`json:\"{}\"`", field.name);
            self.writeln(&format!("{} {} {}", field.name, go_type, json_tag));
        }
        self.dedent();
        self.writeln("}");
        self.writeln("");
    }

    fn generate_enum(&mut self, e: &Enum) {
        self.writeln(&format!("type {} int", e.name));
        self.writeln("const (");
        self.indent();
        for (i, variant) in e.variants.iter().enumerate() {
            if i == 0 {
                self.writeln(&format!("{}_{} {} = iota", e.name, variant.name, e.name));
            } else {
                self.writeln(&format!("{}_{}", e.name, variant.name));
            }
        }
        self.dedent();
        self.writeln(")");
        self.writeln("");
    }

    fn generate_function(&mut self, f: &Function) {
        let mut route_info = None;
        for decorator in &f.decorators {
            match decorator.name.as_str() {
                "Get" | "@Get" => route_info = Some(("GET", decorator)),
                "Post" | "@Post" => route_info = Some(("POST", decorator)),
                "Put" | "@Put" => route_info = Some(("PUT", decorator)),
                "Delete" | "@Delete" => route_info = Some(("DELETE", decorator)),
                _ => {}
            }
        }

        // Generate the business logic function
        let go_return_type = if let Some(ret) = &f.return_type {
            self.map_type(ret)
        } else {
            "".to_string()
        };

        let params: Vec<String> = f
            .params
            .iter()
            .map(|p| format!("{} {}", p.name, self.map_type(&p.param_type)))
            .collect();

        self.writeln(&format!(
            "func {}({}) {} {{",
            f.name,
            params.join(", "),
            go_return_type
        ));
        self.indent();
        // Body (simplified)
        // Check for return statement in body to determine zero value if needed
        let has_return = f
            .body
            .statements
            .iter()
            .any(|stmt| matches!(stmt, Statement::Return(_)));

        if !f.body.statements.is_empty() {
            for stmt in &f.body.statements {
                self.generate_statement(stmt);
            }
        }

        if !has_return && !go_return_type.is_empty() {
            // Return zero value basierend auf Typ
            let zero_value = match go_return_type.as_str() {
                "string" => "\"\"",
                "int" | "int64" | "int32" => "0",
                "float64" | "float32" => "0.0",
                "bool" => "false",
                _ => "nil",
            };
            self.writeln(&format!("return {}", zero_value));
        }

        self.dedent();
        self.writeln("}");
        self.writeln("");

        // If it has a route, generate a Gin Handler wrapper
        if let Some((method, decorator)) = route_info {
            let path_arg = if let Some(first_arg) = decorator.args.first() {
                match first_arg {
                    DecoratorArg::String(s) => s.clone(),
                    _ => "/".to_string(),
                }
            } else {
                "/".to_string()
            };

            let handler_name = format!("{}Handler", f.name);

            self.routes
                .push((method.to_string(), path_arg.clone(), handler_name.clone()));

            self.writeln(&format!("func {}(c *gin.Context) {{", handler_name));
            self.indent();

            // Call the function
            // Note: Argument binding is complex here. For now, assume no args or manual binding.
            if f.params.is_empty() {
                if go_return_type.is_empty() {
                    self.writeln(&format!("{}()", f.name));
                    self.writeln("c.Status(http.StatusOK)");
                } else {
                    self.writeln(&format!("result := {}()", f.name));
                    self.writeln("c.JSON(http.StatusOK, result)");
                }
            } else {
                // Argument binding
                let mut call_args = Vec::new();
                for param in &f.params {
                    let go_type = self.map_type(&param.param_type);
                    self.writeln(&format!("var {} {}", param.name, go_type));

                    let is_primitive = matches!(
                        param.param_type,
                        Type::String | Type::Number | Type::Boolean
                    );
                    let param_in_path = path_arg.contains(&format!(":{}", param.name));

                    if !is_primitive {
                        // Bind JSON Body
                        self.writeln(&format!(
                            "if err := c.ShouldBindJSON(&{}); err != nil {{",
                            param.name
                        ));
                        self.indent();
                        self.writeln(
                            "c.JSON(http.StatusBadRequest, gin.H{\"error\": err.Error()})",
                        );
                        self.writeln("return");
                        self.dedent();
                        self.writeln("}");
                    } else {
                        // Bind Query or Path
                        if param_in_path {
                            self.writeln(&format!(
                                "{}Str := c.Param(\"{}\")",
                                param.name, param.name
                            ));
                        } else {
                            self.writeln(&format!(
                                "{}Str := c.Query(\"{}\")",
                                param.name, param.name
                            ));
                        }

                        // Convert if needed
                        match param.param_type {
                            Type::String => {
                                self.writeln(&format!("{} = {}Str", param.name, param.name));
                            }
                            Type::Number => {
                                self.writeln(&format!(
                                    "if v, err := strconv.ParseFloat({}Str, 64); err == nil {{",
                                    param.name
                                ));
                                self.indent();
                                self.writeln(&format!("{} = v", param.name));
                                self.dedent();
                                self.writeln("}");
                            }
                            Type::Boolean => {
                                self.writeln(&format!(
                                    "if v, err := strconv.ParseBool({}Str); err == nil {{",
                                    param.name
                                ));
                                self.indent();
                                self.writeln(&format!("{} = v", param.name));
                                self.dedent();
                                self.writeln("}");
                            }
                            _ => {}
                        }
                    }
                    call_args.push(param.name.clone());
                }

                // Call
                let args_str = call_args.join(", ");
                if go_return_type.is_empty() {
                    self.writeln(&format!("{}({})", f.name, args_str));
                    self.writeln("c.Status(http.StatusOK)");
                } else {
                    self.writeln(&format!("result := {}({})", f.name, args_str));
                    self.writeln("c.JSON(http.StatusOK, result)");
                }
            }

            self.dedent();
            self.writeln("}");
            self.writeln("");
        }
    }

    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Return(ret_stmt) => {
                if let Some(e) = &ret_stmt.value {
                    self.write("\treturn ");
                    self.generate_expression(e);
                    self.write("\n");
                } else {
                    self.writeln("return");
                }
            }
            Statement::Expression(expr_stmt) => {
                self.write(&"\t".repeat(self.indent_level));
                self.generate_expression(&expr_stmt.expression);
                self.write("\n");
            }
            _ => self.writeln("// Statement not implemented"),
        }
    }

    fn generate_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::String(s) => self.write(&format!("\"{}\"", s)),
                Literal::Number(n) => self.write(&n.to_string()),
                Literal::Boolean(b) => self.write(&b.to_string()),
                _ => self.write("nil"),
            },
            Expression::Identifier(id) => self.write(id),
            Expression::StructLiteral { name, fields } => {
                self.write(&format!("{} {{", name));
                for (key, value) in fields {
                    self.write(&format!("{}: ", key));
                    self.generate_expression(value);
                    self.write(", ");
                }
                self.write("}");
            }
            _ => self.write("/* expr */"),
        }
    }

    fn map_type(&self, t: &Type) -> String {
        match t {
            Type::String => "string".to_string(),
            Type::Number => "float64".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::Void => "".to_string(),
            Type::Any => "interface{}".to_string(),
            Type::List(inner) => format!("[]{}", self.map_type(inner)),
            Type::Map { key, value } => {
                format!("map[{}]{}", self.map_type(key), self.map_type(value))
            }
            Type::Named(n) => n.clone(),
            Type::Optional(inner) => format!("*{}", self.map_type(inner)),
            Type::Result { ok: _, err: _ } => "interface{}".to_string(), // Go handles errors differently
            _ => "interface{}".to_string(),
        }
    }
}

impl CodeGenerator for GoCodeGenerator {
    fn generate(&mut self, program: &Program, config: &CodegenConfig) -> anyhow::Result<String> {
        self.output.clear();
        self.indent_level = 0;
        self.routes.clear();

        let framework = FrameworkSelector::detect_framework(program, config.framework.as_deref());
        self.framework = framework;

        self.writeln("package main");
        self.writeln("");
        self.writeln(&format!("// {}", VELISCH_FINGERPRINT));
        self.writeln("");

        // Imports
        self.writeln(&FrameworkSelector::generate_imports(framework));

        for item in &program.items {
            match item {
                Item::Struct(s) => self.generate_struct(s),
                Item::Enum(e) => self.generate_enum(e),
                Item::Function(f) => self.generate_function(f),
                _ => {}
            }
        }

        // Generate Main
        if !self.routes.is_empty() {
            self.writeln("");
            self.write(&FrameworkSelector::generate_go_main(
                self.framework,
                self.routes.clone(),
            ));
        }

        Ok(self.output.clone())
    }

    fn get_target_language(&self) -> TargetLanguage {
        TargetLanguage::Go
    }
}
