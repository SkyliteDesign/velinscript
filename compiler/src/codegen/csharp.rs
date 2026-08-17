use super::{CodeGenerator, CodegenConfig, TargetLanguage};
use crate::parser::ast::*;
use crate::codegen::framework::{Framework, FrameworkSelector};
use anyhow::Result;

pub struct CSharpCodeGenerator {
    buffer: String,
    indent_level: usize,
    framework: Option<Framework>,
    namespace: String,
}

impl CSharpCodeGenerator {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            indent_level: 0,
            framework: None,
            namespace: "VelinApp".to_string(),
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

    fn map_type(&self, t: &Type) -> String {
        match t {
            Type::String => "string".to_string(),
            Type::Number => "double".to_string(), 
            Type::Boolean => "bool".to_string(),
            Type::Void => "void".to_string(),
            Type::Any => "object".to_string(),
            Type::List(inner) => format!("List<{}>", self.map_type(inner)),
            Type::Optional(inner) => format!("{}?", self.map_type(inner)),
            Type::Named(name) => name.clone(),
            Type::Generic { name, params } => {
                let params_str = params
                    .iter()
                    .map(|p| self.map_type(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}<{}>", name, params_str)
            }
            Type::Map { key, value } => format!("Dictionary<{}, {}>", self.map_type(key), self.map_type(value)),
            _ => "object".to_string(),
        }
    }

    fn generate_struct(&mut self, s: &Struct) {
        self.writeln(&format!("public class {}", s.name));
        self.writeln("{");
        self.indent();
        for field in &s.fields {
            self.writeln(&format!("public {} {} {{ get; set; }}", self.map_type(&field.field_type), field.name));
        }
        self.dedent();
        self.writeln("}");
        self.writeln("");
    }

    fn generate_function(&mut self, f: &Function) {
        // Attributes
        if self.framework == Some(Framework::AspNet) {
            for decorator in &f.decorators {
                let name = decorator.name.trim_start_matches('@');
                match name {
                    "Get" | "Post" | "Put" | "Delete" => {
                        let method = match name {
                            "Get" => "HttpGet",
                            "Post" => "HttpPost",
                            "Put" => "HttpPut",
                            "Delete" => "HttpDelete",
                            _ => "",
                        };
                        let path = if let Some(arg) = decorator.args.first() {
                            match arg {
                                DecoratorArg::String(s) => s.clone(),
                                _ => "".to_string(),
                            }
                        } else {
                            "".to_string()
                        };
                        
                        if path.is_empty() {
                            self.writeln(&format!("[{}]", method));
                        } else {
                            self.writeln(&format!("[{}(\"{}\")]", method, path));
                        }
                    }
                    _ => {}
                }
            }
        }

        // Signature
        let params: Vec<String> = f.params.iter()
            .map(|p| {
                let mut prefix = String::new();
                 if self.framework == Some(Framework::AspNet) {
                     let is_primitive = matches!(p.param_type, Type::String | Type::Number | Type::Boolean);
                     if !is_primitive {
                         prefix = "[FromBody] ".to_string();
                     } else {
                         // [FromQuery] or [FromRoute] usually inferred, but can be explicit
                         // Simplified:
                         prefix = "[FromQuery] ".to_string(); 
                     }
                 }
                format!("{}{} {}", prefix, self.map_type(&p.param_type), p.name)
            })
            .collect();
        
        let ret_type = if let Some(rt) = &f.return_type {
            self.map_type(rt)
        } else {
            "void".to_string()
        };

        // ActionResult wrapper for Web API
        let final_ret = if self.framework == Some(Framework::AspNet) {
            if ret_type == "void" { "IActionResult".to_string() } else { format!("ActionResult<{}>", ret_type) }
        } else {
            ret_type
        };

        self.writeln(&format!("public {} {}({})", final_ret, f.name, params.join(", ")));
        self.writeln("{");
        self.indent();
        
        // Body
        for stmt in &f.body.statements {
            self.generate_statement(stmt);
        }

        self.dedent();
        self.writeln("}");
        self.writeln("");
    }

    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Return(ret) => {
                if let Some(val) = &ret.value {
                    self.buffer.push_str(&"    ".repeat(self.indent_level));
                    if self.framework == Some(Framework::AspNet) {
                        self.buffer.push_str("return Ok(");
                        self.generate_expression(val);
                        self.buffer.push_str(");\n");
                    } else {
                        self.buffer.push_str("return ");
                        self.generate_expression(val);
                        self.buffer.push_str(";\n");
                    }
                } else {
                    if self.framework == Some(Framework::AspNet) {
                        self.writeln("return Ok();");
                    } else {
                        self.writeln("return;");
                    }
                }
            }
            Statement::Expression(expr) => {
                self.buffer.push_str(&"    ".repeat(self.indent_level));
                self.generate_expression(&expr.expression);
                self.buffer.push_str(";\n");
            }
            Statement::Let(decl) => {
                 let _kw = if decl.mutable { "" } else { "const " }; // C# uses types, const ...for constants
                 self.buffer.push_str(&"    ".repeat(self.indent_level));
                 let type_ann = if let Some(t) = &decl.var_type {
                     self.map_type(t)
                 } else {
                     "var".to_string()
                 };
                 // C# const requires value at compile time, otherwise readonly, but for local vars 'const' or just type
                 // Simplified: use var/type
                 self.buffer.push_str(&format!("{} {} = ", type_ann, decl.name));
                 self.generate_expression(&decl.value);
                 self.buffer.push_str(";\n");
            }
            Statement::If(if_stmt) => {
                self.buffer.push_str(&"    ".repeat(self.indent_level));
                self.buffer.push_str("if (");
                self.generate_expression(&if_stmt.condition);
                self.buffer.push_str(") {\n");
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
            _ => {}
        }
    }

    fn generate_expression(&mut self, expr: &Expression) {
         match expr {
            Expression::Literal(lit) => match lit {
                Literal::String(s) => self.buffer.push_str(&format!("\"{}\"", s)),
                Literal::Number(n) => self.buffer.push_str(&n.to_string()),
                Literal::Boolean(b) => self.buffer.push_str(&b.to_string()),
                _ => self.buffer.push_str("null"),
            },
            Expression::Identifier(id) => self.buffer.push_str(id),
            Expression::BinaryOp { left, op, right } => {
                self.generate_expression(left);
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Eq => "==",
                    BinaryOperator::NotEq => "!=",
                    BinaryOperator::Lt => "<",
                    BinaryOperator::Gt => ">",
                    BinaryOperator::LtEq => "<=",
                    BinaryOperator::GtEq => ">=",
                    _ => "+", 
                };
                self.buffer.push_str(&format!(" {} ", op_str));
                self.generate_expression(right);
            }
            Expression::Call { callee, args } => {
                self.generate_expression(callee);
                self.buffer.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.generate_expression(arg);
                }
                self.buffer.push(')');
            }
            Expression::ListLiteral(items) => {
                // Using target-typed new for lists, assuming context provides type (e.g. return type or variable type)
                // Fallback to new List<object> if needed? new() is safer if context exists.
                self.buffer.push_str("new() { ");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.generate_expression(item);
                }
                self.buffer.push_str(" }");
            }
            Expression::MapLiteral(entries) => {
                self.buffer.push_str("new() { ");
                for (i, (key, value)) in entries.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.buffer.push_str(&format!("{{ \"{}\", ", key));
                    self.generate_expression(value);
                    self.buffer.push_str(" }");
                }
                self.buffer.push_str(" }");
            }
            Expression::StructLiteral { name, fields } => {
                self.buffer.push_str(&format!("new {} {{ ", name));
                for (i, (field_name, value)) in fields.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.buffer.push_str(&format!("{} = ", field_name));
                    self.generate_expression(value);
                }
                self.buffer.push_str(" }");
            }
            _ => self.buffer.push_str("/* expr */"),
        }
    }
}

impl CodeGenerator for CSharpCodeGenerator {
    fn generate(&mut self, program: &Program, config: &CodegenConfig) -> Result<String> {
        self.framework = Some(FrameworkSelector::detect_framework(program, config.framework.as_deref()));
        
        self.writeln(&format!("namespace {};", self.namespace));
        self.writeln("");
        if let Some(fw) = self.framework {
            self.buffer.push_str(&FrameworkSelector::generate_imports(fw));
        }

        if self.framework == Some(Framework::AspNet) {
            self.writeln("[ApiController]");
            self.writeln("[Route(\"[controller]\")]");
            self.writeln("public class AppController : ControllerBase");
            self.writeln("{");
            self.indent();
        } else {
             self.writeln("public class Program");
             self.writeln("{");
             self.indent();
        }

        // Structs need to be outside Controller in C# usually, or nested.
        // Let's put them outside. But for this simple generator, nested is easier to manage context.
        // Actually, C# nested classes are fine.
        
        for item in &program.items {
            if let Item::Struct(s) = item {
                self.generate_struct(s);
            }
        }

        for item in &program.items {
            if let Item::Function(f) = item {
                self.generate_function(f);
            }
        }

        self.dedent();
        self.writeln("}");

        Ok(self.buffer.clone())
    }

    fn get_target_language(&self) -> TargetLanguage {
        TargetLanguage::CSharp
    }
}
