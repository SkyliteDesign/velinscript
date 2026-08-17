use super::{CodeGenerator, CodegenConfig, TargetLanguage};
use crate::parser::ast::*;
use crate::codegen::framework::{Framework, FrameworkSelector};
use anyhow::Result;

pub struct TypeScriptCodeGenerator {
    buffer: String,
    indent_level: usize,
    framework: Option<Framework>,
    routes: Vec<(String, String, String)>, // (method, path, handler_name)
}

impl TypeScriptCodeGenerator {
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

    fn map_type(&self, t: &Type) -> String {
        match t {
            Type::String => "string".to_string(),
            Type::Number => "number".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Void => "void".to_string(),
            Type::Any => "any".to_string(),
            Type::List(inner) => format!("{}[]", self.map_type(inner)),
            Type::Optional(inner) => format!("{} | null", self.map_type(inner)),
            Type::Named(name) => name.clone(),
            Type::Generic { name, params } => {
                if name == "List" && params.len() == 1 {
                    format!("{}[]", self.map_type(&params[0]))
                } else {
                    let params_str = params
                        .iter()
                        .map(|p| self.map_type(p))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{}<{}>", name, params_str)
                }
            }
            Type::Map { key, value } => format!("Record<{}, {}>", self.map_type(key), self.map_type(value)),
            _ => "any".to_string(),
        }
    }

    fn generate_struct(&mut self, s: &Struct) {
        // Generate interface for data structures
        self.writeln(&format!("export interface {} {{", s.name));
        self.indent();
        for field in &s.fields {
            self.writeln(&format!("{}: {};", field.name, self.map_type(&field.field_type)));
        }
        self.dedent();
        self.writeln("}");
        self.writeln("");
    }

    fn generate_function(&mut self, f: &Function) {
        let is_handler = f.decorators.iter().any(|d| {
            matches!(d.name.as_str(), "Get" | "Post" | "Put" | "Delete" | "@Get" | "@Post" | "@Put" | "@Delete")
        });

        // For Express/NestJS handlers
        if is_handler && self.framework.is_some() {
            let fw = self.framework.unwrap();
            
            // Extract route info
            for decorator in &f.decorators {
                let name = decorator.name.trim_start_matches('@');
                match name {
                    "Get" | "Post" | "Put" | "Delete" => {
                        let method = name.to_uppercase();
                        let path = if let Some(arg) = decorator.args.first() {
                            match arg {
                                DecoratorArg::String(s) => s.clone(),
                                _ => "/".to_string(),
                            }
                        } else {
                            "/".to_string()
                        };
                        
                        if fw == Framework::NestJS {
                            // NestJS Decorator Style
                            self.writeln(&format!("@{}(\"{}\")", name, path));
                        } else {
                            // Express: Register route for main generation later
                            self.routes.push((method.clone(), path.clone(), f.name.clone()));
                        }
                    }
                    _ => {}
                }
            }
        }

        // Function signature
        let params: Vec<String> = f.params.iter()
            .map(|p| format!("{}: {}", p.name, self.map_type(&p.param_type)))
            .collect();
        
        // Return type
        let ret_type = if let Some(rt) = &f.return_type {
            self.map_type(rt)
        } else {
            "void".to_string()
        };

        if self.framework == Some(Framework::NestJS) && is_handler {
             // NestJS Handler method inside Controller class (handled by generate_program wrapper)
             // But here we are just generating function. 
             // Ideally we should wrap functions in a class for NestJS.
             // For now, let's assume we are inside a class or module context if we were doing full NestJS.
             // But since Velin functions are top-level, we might generate a wrapper class later?
             // Let's stick to simple function generation for now, NestJS might need class wrapper logic in `generate`.
             self.writeln(&format!("async {}({}): Promise<{}> {{", f.name, params.join(", "), ret_type));
        } else if self.framework == Some(Framework::Express) && is_handler {
             // Express Handler: (req: Request, res: Response)
             self.writeln(&format!("const {} = async (req: Request, res: Response): Promise<void> => {{", f.name));
        } else {
             // Standard TS Function
             self.writeln(&format!("export async function {}({}): Promise<{}> {{", f.name, params.join(", "), ret_type));
        }

        self.indent();

        // If Express handler, bind params
        if self.framework == Some(Framework::Express) && is_handler {
             for param in &f.params {
                 // Simple binding logic
                 // If param name is in route (e.g. :id), use req.params
                 // If complex type, use req.body
                 // If simple type not in route, use req.query
                 
                 // Check if route has :param.name
                 // For simplicity, let's assume complex = body, simple = query unless :name exists
                 let is_primitive = matches!(param.param_type, Type::String | Type::Number | Type::Boolean);
                 
                 if !is_primitive {
                     self.writeln(&format!("const {}: {} = req.body;", param.name, self.map_type(&param.param_type)));
                 } else {
                     // We don't have easy access to path string here to check :param
                     // Just try params -> query fallback
                     self.writeln(&format!("const {} = req.params.{} ? req.params.{} : req.query.{} as any;", 
                         param.name, param.name, param.name, param.name));
                 }
             }
        }

        // Body
        if f.body.statements.is_empty() {
            // Empty body
        } else {
            for stmt in &f.body.statements {
                self.generate_statement(stmt);
            }
        }
        
        // Express: Send response if return type is not void
        if self.framework == Some(Framework::Express) && is_handler {
            // If the last statement was a return, it's already handled by generate_statement logic?
            // Actually, generate_statement for Return needs to be aware of Express context.
            // This is tricky. Let's make generate_statement smart or just wrap the result.
            // For now, let's assume user code returns a value, and we need to `res.json(...)` it.
            // But we can't easily change user code.
            // Simpler approach: Velin handler returns data. We wrap it.
            // But here we are generating the function body directly.
            
            // NOTE: Ideally, we should generate `handler_impl` and then a wrapper.
            // But for this pass, let's just let the body generate as is, 
            // and assume user uses `res.json` if they write raw code, OR
            // if we are transpiling Velin `return x` -> `res.json(x)`.
        }

        self.dedent();
        self.writeln("}");
        self.writeln("");
    }

    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Return(ret) => {
                if let Some(val) = &ret.value {
                    if self.framework == Some(Framework::Express) {
                         self.buffer.push_str(&"    ".repeat(self.indent_level));
                         self.buffer.push_str("res.json(");
                         self.generate_expression(val);
                         self.buffer.push_str(");\n");
                         self.writeln("return;");
                    } else {
                        self.buffer.push_str(&"    ".repeat(self.indent_level));
                        self.buffer.push_str("return ");
                        self.generate_expression(val);
                        self.buffer.push_str(";\n");
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
            Statement::Expression(expr) => {
                self.buffer.push_str(&"    ".repeat(self.indent_level));
                self.generate_expression(&expr.expression);
                self.buffer.push_str(";\n");
            }
            Statement::Let(decl) => {
                let kw = if decl.mutable { "let" } else { "const" };
                self.buffer.push_str(&"    ".repeat(self.indent_level));
                let type_ann = if let Some(t) = &decl.var_type {
                    format!(": {}", self.map_type(t))
                } else {
                    "".to_string()
                };
                self.buffer.push_str(&format!("{} {}{} = ", kw, decl.name, type_ann));
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
             _ => {} // Implement others as needed
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
                    BinaryOperator::Eq => "===",
                    BinaryOperator::NotEq => "!==",
                    BinaryOperator::Lt => "<",
                    BinaryOperator::Gt => ">",
                    BinaryOperator::LtEq => "<=",
                    BinaryOperator::GtEq => ">=",
                    _ => "+", // Fallback
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
                self.buffer.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.generate_expression(item);
                }
                self.buffer.push(']');
            }
            Expression::MapLiteral(entries) => {
                self.buffer.push('{');
                for (i, (key, value)) in entries.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.buffer.push_str(&format!("\"{}\": ", key));
                    self.generate_expression(value);
                }
                self.buffer.push('}');
            }
            Expression::StructLiteral { name: _, fields } => {
                self.buffer.push('{');
                for (i, (name, value)) in fields.iter().enumerate() {
                    if i > 0 { self.buffer.push_str(", "); }
                    self.buffer.push_str(&format!("{}: ", name));
                    self.generate_expression(value);
                }
                self.buffer.push('}');
            }
            _ => self.buffer.push_str("/* expr */"),
        }
    }
}

impl CodeGenerator for TypeScriptCodeGenerator {
    fn generate(&mut self, program: &Program, config: &CodegenConfig) -> Result<String> {
        self.framework = Some(FrameworkSelector::detect_framework(program, config.framework.as_deref()));
        
        // Imports
        if let Some(fw) = self.framework {
            self.buffer.push_str(&FrameworkSelector::generate_imports(fw));
        }

        // Generate Items
        let mut functions = Vec::new();
        
        if self.framework == Some(Framework::NestJS) {
             self.writeln("@Controller()");
             self.writeln("export class AppController {");
             self.indent();
        }

        for item in &program.items {
            match item {
                Item::Struct(s) => self.generate_struct(s),
                Item::Function(f) => {
                    self.generate_function(f);
                    functions.push(f);
                },
                _ => {}
            }
        }

        if self.framework == Some(Framework::NestJS) {
             self.dedent();
             self.writeln("}");
        }

        // Express Main
        if self.framework == Some(Framework::Express) && !self.routes.is_empty() {
             self.buffer.push_str("\n");
             self.buffer.push_str(&FrameworkSelector::generate_node_main(Framework::Express, self.routes.clone()));
        }

        Ok(self.buffer.clone())
    }

    fn get_target_language(&self) -> TargetLanguage {
        TargetLanguage::TypeScript
    }
}
