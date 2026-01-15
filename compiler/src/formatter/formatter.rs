use crate::parser::ast::*;
use crate::formatter::config::FormatConfig;
use std::fmt::Write;

pub struct Formatter {
    config: FormatConfig,
    output: String,
    indent_level: usize,
}

impl Formatter {
    pub fn new(config: FormatConfig) -> Self {
        Formatter {
            config,
            output: String::new(),
            indent_level: 0,
        }
    }
    
    pub fn format(&mut self, program: &Program) -> String {
        self.output.clear();
        self.indent_level = 0;
        
        for (i, item) in program.items.iter().enumerate() {
            if i > 0 {
                self.writeln("");
            }
            self.format_item(item);
        }
        
        self.output.clone()
    }
    
    fn format_item(&mut self, item: &Item) {
        match item {
            Item::Function(f) => self.format_function(f),
            Item::Struct(s) => self.format_struct(s),
            Item::Enum(e) => self.format_enum(e),
            Item::TypeAlias(ta) => self.format_type_alias(ta),
            Item::Module(m) => self.format_module(m),
            Item::Use(u) => self.format_use(u),
            Item::Trait(_t) => {
                // Traits are formatted as-is for now
            }
            Item::Impl(_i) => {
                // Impls are formatted as-is for now
            }
        }
    }
    
    fn format_function(&mut self, function: &Function) {
        // Format decorators
        for decorator in &function.decorators {
            self.format_decorator(decorator);
        }
        
        // Format function signature
        if function.visibility == Visibility::Public {
            self.write("pub ");
        }
        
        if function.is_async {
            self.write("async ");
        }
        
        self.write("fn ");
        self.write(&function.name);
        self.write("(");
        
        // Format parameters
        for (i, param) in function.params.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.format_parameter(param);
        }
        
        self.write(")");
        
        // Format return type
        if let Some(ref return_type) = function.return_type {
            self.write(": ");
            self.format_type(return_type);
        }
        
        self.write(" ");
        self.format_block(&function.body);
    }
    
    fn format_decorator(&mut self, decorator: &Decorator) {
        self.write("@");
        self.write(&decorator.name);
        
        if !decorator.args.is_empty() {
            self.write("(");
            for (i, arg) in decorator.args.iter().enumerate() {
                if i > 0 {
                    self.write(", ");
                }
                self.format_decorator_arg(arg);
            }
            self.write(")");
        }
        self.writeln("");
    }
    
    fn format_decorator_arg(&mut self, arg: &DecoratorArg) {
        match arg {
            DecoratorArg::String(s) => {
                self.write("\"");
                self.write(s);
                self.write("\"");
            }
            DecoratorArg::Number(n) => {
                self.write(&n.to_string());
            }
            DecoratorArg::Boolean(b) => {
                self.write(if *b { "true" } else { "false" });
            }
            DecoratorArg::Identifier(id) => {
                self.write(id);
            }
            DecoratorArg::Named { name, value } => {
                self.write(name);
                self.write(": ");
                self.format_decorator_arg(value);
            }
        }
    }
    
    fn format_parameter(&mut self, param: &Parameter) {
        self.write(&param.name);
        self.write(": ");
        self.format_type(&param.param_type);
        
        if let Some(ref default) = param.default {
            self.write(" = ");
            self.format_expression(default);
        }
    }
    
    fn format_struct(&mut self, struct_def: &Struct) {
        if struct_def.visibility == Visibility::Public {
            self.write("pub ");
        }
        
        self.write("struct ");
        self.write(&struct_def.name);
        self.write(" {");
        self.writeln("");
        
        self.indent_level += 1;
        for (i, field) in struct_def.fields.iter().enumerate() {
            if i > 0 {
                self.writeln("");
            }
            self.indent();
            self.format_struct_field(field);
        }
        self.indent_level -= 1;
        
        self.writeln("");
        self.indent();
        self.write("}");
    }
    
    fn format_struct_field(&mut self, field: &StructField) {
        if field.visibility == Visibility::Public {
            self.write("pub ");
        }
        
        self.write(&field.name);
        self.write(": ");
        self.format_type(&field.field_type);
        self.write(",");
    }
    
    fn format_enum(&mut self, enum_def: &Enum) {
        if enum_def.visibility == Visibility::Public {
            self.write("pub ");
        }
        
        self.write("enum ");
        self.write(&enum_def.name);
        self.write(" {");
        self.writeln("");
        
        self.indent_level += 1;
        for (i, variant) in enum_def.variants.iter().enumerate() {
            if i > 0 {
                self.writeln("");
            }
            self.indent();
            self.format_enum_variant(variant);
        }
        self.indent_level -= 1;
        
        self.writeln("");
        self.indent();
        self.write("}");
    }
    
    fn format_enum_variant(&mut self, variant: &EnumVariant) {
        self.write(&variant.name);
        
        if let Some(ref data) = variant.data {
            if !data.is_empty() {
                self.write("(");
                for (i, typ) in data.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_type(typ);
                }
                self.write(")");
            }
        }
        
        self.write(",");
    }
    
    fn format_type_alias(&mut self, type_alias: &TypeAlias) {
        if type_alias.visibility == Visibility::Public {
            self.write("pub ");
        }
        
        self.write("type ");
        self.write(&type_alias.name);
        self.write(" = ");
        self.format_type(&type_alias.aliased_type);
        self.write(";");
    }
    
    fn format_module(&mut self, module: &Module) {
        if module.visibility == Visibility::Public {
            self.write("pub ");
        }
        
        self.write("mod ");
        self.write(&module.name);
        self.write(" {");
        self.writeln("");
        
        self.indent_level += 1;
        for item in &module.items {
            self.format_item(item);
            self.writeln("");
        }
        self.indent_level -= 1;
        
        self.indent();
        self.write("}");
    }
    
    fn format_use(&mut self, use_stmt: &Use) {
        self.write("use ");
        self.write(&use_stmt.path.join("::"));
        
        if let Some(ref alias) = use_stmt.alias {
            self.write(" as ");
            self.write(alias);
        }
        
        self.write(";");
    }
    
    fn format_block(&mut self, block: &Block) {
        self.write("{");
        self.writeln("");
        
        self.indent_level += 1;
        for statement in &block.statements {
            self.indent();
            self.format_statement(statement);
            self.writeln("");
        }
        self.indent_level -= 1;
        
        self.indent();
        self.write("}");
    }
    
    fn format_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_stmt) => self.format_let_statement(let_stmt),
            Statement::Return(return_stmt) => self.format_return_statement(return_stmt),
            Statement::Expression(expr_stmt) => {
                self.format_expression(&expr_stmt.expression);
                self.write(";");
            }
            Statement::If(if_stmt) => self.format_if_statement(if_stmt),
            Statement::For(for_stmt) => self.format_for_statement(for_stmt),
            Statement::While(while_stmt) => self.format_while_statement(while_stmt),
            Statement::Match(match_stmt) => self.format_match_statement(match_stmt),
        }
    }
    
    fn format_let_statement(&mut self, let_stmt: &LetStatement) {
        if let_stmt.mutable {
            self.write("let mut ");
        } else {
            self.write("let ");
        }
        
        self.write(&let_stmt.name);
        
        if let Some(ref var_type) = let_stmt.var_type {
            self.write(": ");
            self.format_type(var_type);
        }
        
        self.write(" = ");
        self.format_expression(&let_stmt.value);
        self.write(";");
    }
    
    fn format_return_statement(&mut self, return_stmt: &ReturnStatement) {
        self.write("return");
        
        if let Some(ref value) = return_stmt.value {
            self.write(" ");
            self.format_expression(value);
        }
        
        self.write(";");
    }
    
    fn format_if_statement(&mut self, if_stmt: &IfStatement) {
        self.write("if ");
        self.format_expression(&if_stmt.condition);
        self.write(" ");
        self.format_block(&if_stmt.then_block);
        
        if let Some(ref else_block) = if_stmt.else_block {
            self.write(" else ");
            self.format_block(else_block);
        }
    }
    
    fn format_for_statement(&mut self, for_stmt: &ForStatement) {
        self.write("for ");
        self.write(&for_stmt.variable);
        self.write(" in ");
        self.format_expression(&for_stmt.iterable);
        self.write(" ");
        self.format_block(&for_stmt.body);
    }
    
    fn format_while_statement(&mut self, while_stmt: &WhileStatement) {
        self.write("while ");
        self.format_expression(&while_stmt.condition);
        self.write(" ");
        self.format_block(&while_stmt.body);
    }
    
    fn format_match_statement(&mut self, match_stmt: &MatchStatement) {
        self.write("match ");
        self.format_expression(&match_stmt.expression);
        self.write(" {");
        self.writeln("");
        
        self.indent_level += 1;
        for (i, arm) in match_stmt.arms.iter().enumerate() {
            if i > 0 {
                self.writeln("");
            }
            self.indent();
            self.format_pattern(&arm.pattern);
            
            // Format guard if present
            if let Some(ref guard) = arm.guard {
                self.write(" if ");
                self.format_expression(guard);
            }
            
            self.write(" => ");
            self.format_block(&arm.body);
            if i < match_stmt.arms.len() - 1 {
                self.write(",");
            }
        }
        self.indent_level -= 1;
        
        self.writeln("");
        self.indent();
        self.write("}");
    }
    
    fn format_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Literal(lit) => self.format_literal(lit),
            Pattern::Identifier(id) => self.write(id),
            Pattern::Tuple(patterns) => {
                self.write("(");
                for (i, p) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_pattern(p);
                }
                self.write(")");
            }
            Pattern::Wildcard => self.write("_"),
            Pattern::Range { start, end, inclusive } => {
                self.format_expression(start);
                if *inclusive {
                    self.write("..=");
                } else {
                    self.write("..");
                }
                self.format_expression(end);
            }
            Pattern::EnumVariant { name, data } => {
                self.write(name);
                if let Some(ref patterns) = data {
                    self.write("(");
                    for (i, p) in patterns.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.format_pattern(p);
                    }
                    self.write(")");
                }
            }
            Pattern::Or(patterns) => {
                for (i, p) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.write(" | ");
                    }
                    self.format_pattern(p);
                }
            }
            Pattern::Struct { name, fields } => {
                self.write(name);
                self.write(" { ");
                for (i, (field_name, field_pattern)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write(field_name);
                    self.write(": ");
                    self.format_pattern(field_pattern);
                }
                self.write(" }");
            }
        }
    }
    
    fn format_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(lit) => self.format_literal(lit),
            Expression::Identifier(id) => self.write(id),
            Expression::BinaryOp { left, op, right } => {
                self.format_expression(left);
                self.write(" ");
                self.format_binary_operator(op);
                self.write(" ");
                self.format_expression(right);
            }
            Expression::UnaryOp { op, expr } => {
                self.format_unary_operator(op);
                self.format_expression(expr);
            }
            Expression::Call { callee, args } => {
                self.format_expression(callee);
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_expression(arg);
                }
                self.write(")");
            }
            Expression::Member { object, member } => {
                self.format_expression(object);
                self.write(".");
                self.write(member);
            }
            Expression::Index { object, index } => {
                self.format_expression(object);
                self.write("[");
                self.format_expression(index);
                self.write("]");
            }
            Expression::If { condition, then_expr, else_expr } => {
                self.write("if ");
                self.format_expression(condition);
                self.write(" ");
                self.format_expression(then_expr);
                self.write(" else ");
                self.format_expression(else_expr);
            }
            Expression::Block(block) => self.format_block(block),
            Expression::Await { expr } => {
                self.write("await ");
                self.format_expression(expr);
            }
            Expression::StructLiteral { name, fields } => {
                self.write(name);
                self.write(" {");
                self.writeln("");
                self.indent_level += 1;
                
                for (i, (field_name, field_expr)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.writeln(",");
                    }
                    self.indent();
                    self.write(field_name);
                    self.write(": ");
                    self.format_expression(field_expr);
                }
                
                self.writeln("");
                self.indent_level -= 1;
                self.indent();
                self.write("}");
            }
            Expression::Lambda { params, return_type, body } => {
                self.write("(");
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_parameter(param);
                }
                self.write(")");
                
                if let Some(ref ret_type) = return_type {
                    self.write(": ");
                    self.format_type(ret_type);
                }
                
                self.write(" => ");
                
                match body.as_ref() {
                    Expression::Block(_) => {
                        self.format_expression(body);
                    }
                    _ => {
                        self.format_expression(body);
                    }
                }
            }
            Expression::GenericConstructor { name, type_params, args } => {
                self.write(name);
                if !type_params.is_empty() {
                    self.write("<");
                    for (i, tp) in type_params.iter().enumerate() {
                        if i > 0 {
                            self.write(", ");
                        }
                        self.format_type(tp);
                    }
                    self.write(">");
                }
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.format_expression(arg);
                }
                self.write(")");
            }
            Expression::FormatString { parts } => {
                self.write("\"");
                for part in parts {
                    match part {
                        FormatStringPart::Text(text) => {
                            // Escape quotes and backslashes
                            let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
                            self.write(&escaped);
                        }
                        FormatStringPart::Expression(expr) => {
                            self.write("{");
                            self.format_expression(expr);
                            self.write("}");
                        }
                    }
                }
                self.write("\"");
            }
        }
    }
    
    fn format_literal(&mut self, lit: &Literal) {
        match lit {
            Literal::String(s) => {
                self.write("\"");
                self.write(s);
                self.write("\"");
            }
            Literal::Number(n) => {
                self.write(&n.to_string());
            }
            Literal::Boolean(b) => {
                self.write(if *b { "true" } else { "false" });
            }
            Literal::Null => {
                self.write("null");
            }
        }
    }
    
    fn format_type(&mut self, typ: &Type) {
        self.write(&typ.to_string());
    }
    
    fn format_binary_operator(&mut self, op: &BinaryOperator) {
        let op_str = match op {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Eq => "==",
            BinaryOperator::NotEq => "!=",
            BinaryOperator::Lt => "<",
            BinaryOperator::Gt => ">",
            BinaryOperator::LtEq => "<=",
            BinaryOperator::GtEq => ">=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
        };
        self.write(op_str);
    }
    
    fn format_unary_operator(&mut self, op: &UnaryOperator) {
        let op_str = match op {
            UnaryOperator::Not => "!",
            UnaryOperator::Minus => "-",
        };
        self.write(op_str);
    }
    
    // Helper methods
    fn write(&mut self, s: &str) {
        let _ = write!(self.output, "{}", s);
    }
    
    fn writeln(&mut self, s: &str) {
        let _ = writeln!(self.output, "{}", s);
    }
    
    fn indent(&mut self) {
        let indent = self.config.indent_string();
        for _ in 0..self.indent_level {
            self.write(&indent);
        }
    }
}
