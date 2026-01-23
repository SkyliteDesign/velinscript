use super::{CodeGenerator, CodegenConfig, TargetLanguage};
use crate::parser::ast::*;
use crate::codegen::framework::{Framework, FrameworkSelector};
use anyhow::Result;

pub struct JavaCodeGenerator {
    buffer: String,
    indent_level: usize,
    framework: Option<Framework>,
    package_name: String,
}

impl JavaCodeGenerator {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            indent_level: 0,
            framework: None,
            package_name: "com.example.app".to_string(),
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
            Type::String => "String".to_string(),
            Type::Number => "Double".to_string(), // Or int/double based on inference, using Double for safety
            Type::Boolean => "Boolean".to_string(),
            Type::Void => "void".to_string(),
            Type::Any => "Object".to_string(),
            Type::List(inner) => format!("List<{}>", self.map_type(inner)),
            Type::Optional(inner) => self.map_type(inner), // Java refs are nullable
            Type::Named(name) => name.clone(),
            Type::Generic { name, params } => {
                let params_str = params
                    .iter()
                    .map(|p| self.map_type(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}<{}>", name, params_str)
            }
            Type::Map { key, value } => format!("Map<{}, {}>", self.map_type(key), self.map_type(value)),
            _ => "Object".to_string(),
        }
    }

    fn generate_struct(&mut self, s: &Struct) {
        self.writeln(&format!("public class {} {{", s.name));
        self.indent();
        
        // Fields
        for field in &s.fields {
            self.writeln(&format!("public {} {};", self.map_type(&field.field_type), field.name));
        }
        
        // Constructor
        self.writeln("");
        self.writeln(&format!("public {}() {{}}", s.name));
        
        self.dedent();
        self.writeln("}");
        self.writeln("");
    }

    fn generate_function(&mut self, f: &Function) {
        // Annotations
        if self.framework == Some(Framework::Spring) {
            for decorator in &f.decorators {
                let name = decorator.name.trim_start_matches('@');
                match name {
                    "Get" | "Post" | "Put" | "Delete" => {
                        let method = match name {
                            "Get" => "GetMapping",
                            "Post" => "PostMapping",
                            "Put" => "PutMapping",
                            "Delete" => "DeleteMapping",
                            _ => "",
                        };
                        let path = if let Some(arg) = decorator.args.first() {
                            match arg {
                                DecoratorArg::String(s) => s.clone(),
                                _ => "/".to_string(),
                            }
                        } else {
                            "/".to_string()
                        };
                        self.writeln(&format!("@{}(\"{}\")", method, path));
                    }
                    _ => {}
                }
            }
        }

        // Signature
        let params: Vec<String> = f.params.iter()
            .map(|p| {
                let mut prefix = String::new();
                if self.framework == Some(Framework::Spring) {
                    // Check if path variable or request body
                    // Simplified heuristic: simple types = PathVariable/RequestParam, complex = RequestBody
                     let is_primitive = matches!(p.param_type, Type::String | Type::Number | Type::Boolean);
                     if !is_primitive {
                         prefix = "@RequestBody ".to_string();
                     } else {
                         // Assume RequestParam by default, could be PathVariable if in route
                         prefix = "@RequestParam ".to_string(); 
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

        // Spring Controller methods usually return ResponseEntity or the object
        self.writeln(&format!("public {} {}({}) {{", ret_type, f.name, params.join(", ")));
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
                    self.buffer.push_str("return ");
                    self.generate_expression(val);
                    self.buffer.push_str(";\n");
                } else {
                    self.writeln("return;");
                }
            }
            Statement::Expression(expr) => {
                self.buffer.push_str(&"    ".repeat(self.indent_level));
                self.generate_expression(&expr.expression);
                self.buffer.push_str(";\n");
            }
            Statement::Let(decl) => {
                 let kw = if decl.mutable { "" } else { "final " };
                 self.buffer.push_str(&"    ".repeat(self.indent_level));
                 let type_ann = if let Some(t) = &decl.var_type {
                     self.map_type(t)
                 } else {
                     "var".to_string()
                 };
                 self.buffer.push_str(&format!("{}{} {} = ", kw, type_ann, decl.name));
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
            _ => {} // Implement others
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
                    BinaryOperator::Eq => "==", // Java objects might need .equals, but primitives use ==. For now assume == is ok or primitives.
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
                self.buffer.push_str("List.of(");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.generate_expression(item);
                }
                self.buffer.push(')');
            }
            Expression::MapLiteral(entries) => {
                self.buffer.push_str("Map.of(");
                for (i, (key, value)) in entries.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.buffer.push_str(&format!("\"{}\", ", key));
                    self.generate_expression(value);
                }
                self.buffer.push(')');
            }
            Expression::StructLiteral { name, fields } => {
                self.buffer.push_str(&format!("new {}() {{", name));
                if !fields.is_empty() {
                    self.buffer.push_str("{ ");
                    for (field_name, value) in fields {
                        self.buffer.push_str(&format!("this.{} = ", field_name));
                        self.generate_expression(value);
                        self.buffer.push_str("; ");
                    }
                    self.buffer.push_str("}");
                }
                self.buffer.push_str("}");
            }
            _ => self.buffer.push_str("/* expr */"),
        }
    }
}

impl CodeGenerator for JavaCodeGenerator {
    fn generate(&mut self, program: &Program, config: &CodegenConfig) -> Result<String> {
        self.framework = Some(FrameworkSelector::detect_framework(program, config.framework.as_deref()));
        
        self.writeln(&format!("package {};", self.package_name));
        self.writeln("");
        self.writeln("import java.util.*;");
        if let Some(fw) = self.framework {
            self.buffer.push_str(&FrameworkSelector::generate_imports(fw));
        }
        self.writeln("");

        if self.framework == Some(Framework::Spring) {
            self.writeln("@SpringBootApplication");
            self.writeln("@RestController");
            self.writeln("public class Application {");
            self.indent();
            
            // Main method
            self.writeln("public static void main(String[] args) {");
            self.indent();
            self.writeln("SpringApplication.run(Application.class, args);");
            self.dedent();
            self.writeln("}");
            self.writeln("");
        } else {
             self.writeln("public class Main {");
             self.indent();
        }

        // Structs need to be static nested classes if inside Main/App, or separate files.
        // For simplicity, static nested.
        for item in &program.items {
            if let Item::Struct(s) = item {
                // Hack: generate struct as static nested
                self.writeln(&format!("public static class {} {{", s.name));
                self.indent();
                for field in &s.fields {
                    self.writeln(&format!("public {} {};", self.map_type(&field.field_type), field.name));
                }
                self.dedent();
                self.writeln("}");
                self.writeln("");
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
        TargetLanguage::Java
    }
}
