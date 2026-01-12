use crate::parser::ast::*;
use crate::type_checker::environment::Environment;
use crate::type_checker::errors::{TypeError, TypeErrorKind};

pub struct TypeChecker {
    environment: Environment,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut env = Environment::new();
        
        // Built-in types
        env.define_type("string".to_string(), Type::String);
        env.define_type("number".to_string(), Type::Number);
        env.define_type("boolean".to_string(), Type::Boolean);
        env.define_type("void".to_string(), Type::Void);
        env.define_type("null".to_string(), Type::Null);
        // Map is a generic type, so we register it as a type name
        // The actual type will be Map<K, V> which is handled by Type::Map
        env.define_type("Map".to_string(), Type::Map {
            key: Box::new(Type::String),
            value: Box::new(Type::String),
        });
        
        TypeChecker {
            environment: env,
            errors: Vec::new(),
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
        // First pass: collect all type definitions
        for item in &program.items {
            match item {
                Item::Struct(s) => {
                    // Register generic type parameters as valid types in the struct scope
                    // Type parameters are valid types within the struct definition
                    // We don't need to register them globally, but we need to track them
                    for _type_param in &s.type_params {
                        // Type parameters will be validated when used in field types
                    }
                    self.environment.define_type(
                        s.name.clone(),
                        Type::Named(s.name.clone()),
                    );
                }
                Item::Enum(e) => {
                    self.environment.define_type(
                        e.name.clone(),
                        Type::Named(e.name.clone()),
                    );
                }
                Item::TypeAlias(ta) => {
                    self.environment.define_type(
                        ta.name.clone(),
                        ta.aliased_type.clone(),
                    );
                }
                _ => {}
            }
        }
        
        // Second pass: register function signatures
        for item in &program.items {
            if let Item::Function(f) = item {
                let params: Vec<_> = f.params.iter().map(|p| {
                    crate::type_checker::environment::ParameterInfo {
                        name: p.name.clone(),
                        param_type: p.param_type.clone(),
                    }
                }).collect();
                
                let sig = crate::type_checker::environment::FunctionSignature {
                    name: f.name.clone(),
                    params,
                    return_type: f.return_type.clone(),
                };
                
                self.environment.define_function(f.name.clone(), sig);
            }
        }
        
        // Third pass: check functions and other items
        for item in &program.items {
            match item {
                Item::Function(f) => {
                    self.check_function(f)?;
                }
                Item::Struct(s) => {
                    self.check_struct(s)?;
                }
                Item::Enum(e) => {
                    self.check_enum(e)?;
                }
                Item::TypeAlias(_) => {
                    // Already handled in first pass
                }
                Item::Module(m) => {
                    // Handle modules by checking their items recursively
                    let module_env = Environment::with_parent(self.environment.clone());
                    let old_env = std::mem::replace(&mut self.environment, module_env);
                    
                    // Check all items in the module
                    for item in &m.items {
                        match item {
                            Item::Function(f) => {
                                self.check_function(f)?;
                            }
                            Item::Struct(s) => {
                                self.check_struct(s)?;
                            }
                            Item::Enum(e) => {
                                self.check_enum(e)?;
                            }
                            _ => {}
                        }
                    }
                    
                    self.environment = old_env;
                }
                Item::Use(_) => {
                    // Use statements don't need type checking
                }
            }
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    fn check_function(&mut self, function: &Function) -> Result<(), Vec<TypeError>> {
        let mut env = Environment::with_parent(self.environment.clone());
        
        // Add parameters to environment
        for param in &function.params {
            if env.has_variable(&param.name) {
                self.errors.push(TypeError::new(
                    TypeErrorKind::DuplicateDefinition(param.name.clone()),
                    format!("Duplicate parameter: {}", param.name),
                ));
            } else {
                env.define_variable(param.name.clone(), param.param_type.clone());
            }
        }
        
        // Check function body
        let old_env = std::mem::replace(&mut self.environment, env);
        let return_type = self.check_block(&function.body, function.return_type.as_ref())?;
        self.environment = old_env;
        
        // Check return type
        if let Some(expected_return) = &function.return_type {
            // Special handling: Named type from struct literal should be compatible with Generic type of same name
            let is_compatible = match (&return_type, expected_return) {
                (Type::Named(n1), Type::Generic { name: n2, .. }) if n1 == n2 => {
                    // Struct literal (Named) is compatible with generic return type (Generic)
                    // This allows returning ApiResponse { ... } where ApiResponse<void> is expected
                    true
                }
                _ => self.types_compatible(&return_type, expected_return)
            };
            
            if !is_compatible {
                self.errors.push(TypeError::type_mismatch(
                    &expected_return.to_string(),
                    &return_type.to_string(),
                ));
            }
        } else if return_type != Type::Void {
            // Function has return type but no explicit return type annotation
            // This is okay for inference
        }
        
        Ok(())
    }
    
    fn check_struct(&mut self, struct_def: &Struct) -> Result<(), Vec<TypeError>> {
        // Check that all field types are valid
        for field in &struct_def.fields {
            self.check_type(&field.field_type)?;
        }
        Ok(())
    }
    
    fn check_enum(&mut self, enum_def: &Enum) -> Result<(), Vec<TypeError>> {
        // Check that all variant types are valid
        for variant in &enum_def.variants {
            if let Some(ref types) = variant.data {
                for variant_type in types {
                    self.check_type(variant_type)?;
                }
            }
        }
        Ok(())
    }
    
    fn check_block(
        &mut self,
        block: &Block,
        expected_return: Option<&Type>,
    ) -> Result<Type, Vec<TypeError>> {
        let mut return_type = Type::Void;
        
        for statement in &block.statements {
            match statement {
                Statement::Let(let_stmt) => {
                    let value_type = self.check_expression(&let_stmt.value)?;
                    
                    if let Some(ref var_type) = let_stmt.var_type {
                        if !self.types_compatible(&value_type, var_type) {
                            self.errors.push(TypeError::type_mismatch(
                                &var_type.to_string(),
                                &value_type.to_string(),
                            ));
                        }
                        
                        if self.environment.has_variable(&let_stmt.name) {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::DuplicateDefinition(let_stmt.name.clone()),
                                format!("Variable '{}' already defined", let_stmt.name),
                            ));
                        } else {
                            self.environment.define_variable(let_stmt.name.clone(), var_type.clone());
                        }
                    } else {
                        // Type inference
                        if value_type == Type::Void {
                            self.errors.push(TypeError::cannot_infer_type());
                        } else {
                            self.environment.define_variable(let_stmt.name.clone(), value_type.clone());
                        }
                    }
                }
                Statement::Return(ret_stmt) => {
                    if let Some(ref value) = ret_stmt.value {
                        return_type = self.check_expression(value)?;
                    } else {
                        return_type = Type::Void;
                    }
                }
                Statement::Expression(expr_stmt) => {
                    self.check_expression(&expr_stmt.expression)?;
                }
                Statement::If(if_stmt) => {
                    let condition_type = self.check_expression(&if_stmt.condition)?;
                    if condition_type != Type::Boolean {
                        self.errors.push(TypeError::type_mismatch(
                            "boolean",
                            &condition_type.to_string(),
                        ));
                    }
                    
                    self.check_block(&if_stmt.then_block, expected_return)?;
                    if let Some(ref else_block) = if_stmt.else_block {
                        self.check_block(else_block, expected_return)?;
                    }
                }
                Statement::For(for_stmt) => {
                    let iterable_type = self.check_expression(&for_stmt.iterable)?;
                    
                    // Check if iterable_type is iterable (List, Array, etc.)
                    let element_type = match iterable_type {
                        Type::List(ref item_type) => item_type.as_ref().clone(),
                        Type::Generic { name, params } if name == "List" && !params.is_empty() => {
                            params[0].clone()
                        }
                        Type::String => Type::String, // String is iterable (characters)
                        _ => {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::TypeMismatch {
                                    expected: "List<T> or String".to_string(),
                                    found: iterable_type.to_string(),
                                },
                                format!("Type '{}' is not iterable", iterable_type.to_string()),
                            ));
                            Type::Void
                        }
                    };
                    
                    // Create new scope for loop variable
                    let parent_env = self.environment.clone();
                    let old_env = std::mem::replace(
                        &mut self.environment,
                        Environment::with_parent(parent_env),
                    );
                    
                    self.environment.define_variable(for_stmt.variable.clone(), element_type);
                    self.check_block(&for_stmt.body, expected_return)?;
                    self.environment = old_env;
                }
                Statement::While(while_stmt) => {
                    let condition_type = self.check_expression(&while_stmt.condition)?;
                    if condition_type != Type::Boolean {
                        self.errors.push(TypeError::type_mismatch(
                            "boolean",
                            &condition_type.to_string(),
                        ));
                    }
                    self.check_block(&while_stmt.body, expected_return)?;
                }
                Statement::Match(match_stmt) => {
                    let match_type = self.check_expression(&match_stmt.expression)?;
                    for arm in &match_stmt.arms {
                        // Check pattern matching types
                        match &arm.pattern {
                            Pattern::Literal(pat_lit) => {
                                let pat_type = self.literal_type(pat_lit);
                                if !self.types_compatible(&match_type, &pat_type) {
                                    self.errors.push(TypeError::type_mismatch(
                                        &match_type.to_string(),
                                        &pat_type.to_string(),
                                    ));
                                }
                            }
                            Pattern::Identifier(name) => {
                                // Bind pattern variable to match type
                                let parent_env = self.environment.clone();
                                let old_env = std::mem::replace(
                                    &mut self.environment,
                                    Environment::with_parent(parent_env),
                                );
                                self.environment.define_variable(name.clone(), match_type.clone());
                                self.check_block(&arm.body, expected_return)?;
                                self.environment = old_env;
                            }
                            Pattern::Tuple(_) | Pattern::Struct { .. } => {
                                // Complex patterns - for now, just check body
                                self.check_block(&arm.body, expected_return)?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(return_type)
    }
    
    fn check_expression(&mut self, expr: &Expression) -> Result<Type, Vec<TypeError>> {
        match expr {
            Expression::Literal(lit) => Ok(self.literal_type(lit)),
            Expression::Identifier(name) => {
                if let Some(var_type) = self.environment.get_variable(name) {
                    Ok(var_type)
                } else {
                    self.errors.push(TypeError::undefined_variable(name));
                    Ok(Type::Void) // Return error type
                }
            }
            Expression::BinaryOp { left, op, right } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;
                self.check_binary_operation(op, &left_type, &right_type)
            }
            Expression::UnaryOp { op, expr } => {
                let expr_type = self.check_expression(expr)?;
                self.check_unary_operation(op, &expr_type)
            }
            Expression::GenericConstructor { name, type_params, args } => {
                // Handle generic type constructors like Map<string, string>() or List<string>()
                match name.as_str() {
                    "Map" => {
                        if type_params.len() == 2 {
                            let key_type = &type_params[0];
                            let value_type = &type_params[1];
                            self.check_type(key_type)?;
                            self.check_type(value_type)?;
                            // Check constructor arguments (should be empty for Map())
                            for arg in args {
                                let _ = self.check_expression(&arg)?;
                            }
                            return Ok(Type::Map {
                                key: Box::new(key_type.clone()),
                                value: Box::new(value_type.clone()),
                            });
                        } else {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::InvalidArgumentType {
                                    position: 0,
                                    expected: "Map<K, V> requires 2 type parameters".to_string(),
                                    found: format!("{} type parameters", type_params.len()),
                                },
                                format!("Map requires 2 type parameters, found {}", type_params.len()),
                            ));
                            return Ok(Type::Void);
                        }
                    }
                    "List" => {
                        if type_params.len() == 1 {
                            let item_type = &type_params[0];
                            self.check_type(item_type)?;
                            // Check constructor arguments
                            for arg in args {
                                let _ = self.check_expression(&arg)?;
                            }
                            return Ok(Type::List(Box::new(item_type.clone())));
                        } else {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::InvalidArgumentType {
                                    position: 0,
                                    expected: "List<T> requires 1 type parameter".to_string(),
                                    found: format!("{} type parameters", type_params.len()),
                                },
                                format!("List requires 1 type parameter, found {}", type_params.len()),
                            ));
                            return Ok(Type::Void);
                        }
                    }
                    _ => {
                        // Check if it's a struct constructor with generics
                        if let Some(_struct_type) = self.environment.get_type(name) {
                            // Validate type parameters
                            for type_param in type_params {
                                self.check_type(type_param)?;
                            }
                            // Check constructor arguments
                            for arg in args {
                                let _ = self.check_expression(&arg)?;
                            }
                            return Ok(Type::Generic {
                                name: name.clone(),
                                params: type_params.clone(),
                            });
                        } else {
                            self.errors.push(TypeError::undefined_type(name));
                            return Ok(Type::Void);
                        }
                    }
                }
            }
            Expression::Call { callee, args } => {
                let _callee_type = self.check_expression(callee)?;
                
                // Look up function signature from environment
                if let Expression::Identifier(name) = callee.as_ref() {
                    if let Some(sig) = self.environment.get_function(name) {
                        // Check argument count
                        if args.len() != sig.params.len() {
                            self.errors.push(TypeError::wrong_argument_count(
                                sig.params.len(),
                                args.len(),
                            ));
                        } else {
                            // Check argument types
                            for (i, (arg, param)) in args.iter().zip(sig.params.iter()).enumerate() {
                                let arg_type = self.check_expression(arg)?;
                                if !self.types_compatible(&arg_type, &param.param_type) {
                                    self.errors.push(TypeError::new(
                                        TypeErrorKind::InvalidArgumentType {
                                            position: i,
                                            expected: param.param_type.to_string(),
                                            found: arg_type.to_string(),
                                        },
                                        format!(
                                            "Argument {}: expected {}, found {}",
                                            i + 1,
                                            param.param_type.to_string(),
                                            arg_type.to_string()
                                        ),
                                    ));
                                }
                            }
                        }
                        
                        Ok(sig.return_type.unwrap_or(Type::Void))
                    } else {
                        // Check if it's a method call (object.method())
                        if let Some(dot_pos) = name.rfind('.') {
                            let (obj_name, _method_name) = name.split_at(dot_pos);
                            let _method_name = &_method_name[1..]; // Skip the dot
                            
                            if let Some(_obj_type) = self.environment.get_variable(obj_name) {
                                // For now, assume method calls return Void
                                // In future, could check struct methods
                                Ok(Type::Void)
                            } else {
                                self.errors.push(TypeError::undefined_function(name));
                                Ok(Type::Void)
                            }
                        } else {
                            self.errors.push(TypeError::undefined_function(name));
                            Ok(Type::Void)
                        }
                    }
                } else {
                    // Handle other callable types (closures, function pointers, etc.)
                    // For now, assume they return Void
                    // In future, could infer from callable type
                    Ok(Type::Void)
                }
            }
            Expression::Await { expr } => {
                // Check that the expression is awaitable (async function call, etc.)
                let expr_type = self.check_expression(expr)?;
                // For now, return the type of the awaited expression
                // In future, could check if it's actually awaitable
                Ok(expr_type)
            }
            Expression::Member { object, member } => {
                let obj_type = self.check_expression(object)?;
                
                // Check member access
                match obj_type {
                    Type::Named(ref struct_name) => {
                        // Look up struct definition to check if member exists
                        if let Some(_struct_type) = self.environment.get_type(struct_name) {
                            // For now, return the struct type itself
                            // In future, could look up actual field type
                            Ok(Type::Void)
                        } else {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::UndefinedType(struct_name.clone()),
                                format!("Type '{}' not found", struct_name),
                            ));
                            Ok(Type::Void)
                        }
                    }
                    Type::List(_) => {
                        // List methods like .length, .push, etc.
                        match member.as_str() {
                            "length" | "size" => Ok(Type::Number),
                            "push" | "pop" | "remove" => Ok(Type::Void),
                            _ => {
                                self.errors.push(TypeError::new(
                                    TypeErrorKind::InvalidMemberAccess,
                                    format!("List has no member '{}'", member),
                                ));
                                Ok(Type::Void)
                            }
                        }
                    }
                    Type::String => {
                        // String methods
                        match member.as_str() {
                            "length" => Ok(Type::Number),
                            "toUpperCase" | "toLowerCase" | "trim" => Ok(Type::String),
                            _ => {
                                self.errors.push(TypeError::new(
                                    TypeErrorKind::InvalidMemberAccess,
                                    format!("String has no member '{}'", member),
                                ));
                                Ok(Type::Void)
                            }
                        }
                    }
                    _ => {
                        self.errors.push(TypeError::new(
                            TypeErrorKind::InvalidMemberAccess,
                            format!("Type '{}' does not support member access", obj_type.to_string()),
                        ));
                        Ok(Type::Void)
                    }
                }
            }
            Expression::Index { object, index } => {
                let obj_type = self.check_expression(object)?;
                let index_type = self.check_expression(index)?;
                
                if index_type != Type::Number {
                    self.errors.push(TypeError::type_mismatch(
                        "number",
                        &index_type.to_string(),
                    ));
                }
                
                match obj_type {
                    Type::List(ref item_type) => Ok(item_type.as_ref().clone()),
                    _ => {
                        self.errors.push(TypeError::invalid_operation("index", &obj_type.to_string()));
                        Ok(Type::Void)
                    }
                }
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                let cond_type = self.check_expression(condition)?;
                if cond_type != Type::Boolean {
                    self.errors.push(TypeError::type_mismatch(
                        "boolean",
                        &cond_type.to_string(),
                    ));
                }
                
                let then_type = self.check_expression(then_expr)?;
                let else_type = self.check_expression(else_expr)?;
                
                // Both branches should have compatible types
                if !self.types_compatible(&then_type, &else_type) {
                    self.errors.push(TypeError::type_mismatch(
                        &then_type.to_string(),
                        &else_type.to_string(),
                    ));
                }
                
                Ok(then_type)
            }
            Expression::Block(block) => {
                Ok(self.check_block(block, None)?)
            }
            Expression::StructLiteral { name, fields } => {
                // Check that the struct type exists
                // For now, return the struct type (could be generic)
                // The actual type will be determined by the return type annotation
                if let Some(_struct_def) = self.environment.get_type(name) {
                    // Validate field expressions
                    for (_field_name, field_expr) in fields {
                        let _field_type = self.check_expression(field_expr)?;
                        // Could validate field types here
                    }
                    // Return Named type - the actual generic instantiation will be checked
                    // against the expected return type
                    Ok(Type::Named(name.clone()))
                } else {
                    self.errors.push(TypeError::new(
                        TypeErrorKind::UndefinedType(name.clone()),
                        format!("Struct '{}' not found", name),
                    ));
                    Ok(Type::Void)
                }
            }
        }
    }
    
    fn check_binary_operation(
        &mut self,
        op: &BinaryOperator,
        left_type: &Type,
        right_type: &Type,
    ) -> Result<Type, Vec<TypeError>> {
        match op {
            BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply
            | BinaryOperator::Divide | BinaryOperator::Modulo => {
                if *left_type == Type::Number && *right_type == Type::Number {
                    Ok(Type::Number)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        &format!("{:?}", op),
                        &format!("{} and {}", left_type.to_string(), right_type.to_string()),
                    ));
                    Ok(Type::Void)
                }
            }
            BinaryOperator::Eq | BinaryOperator::NotEq | BinaryOperator::Lt
            | BinaryOperator::Gt | BinaryOperator::LtEq | BinaryOperator::GtEq => {
                if self.types_compatible(left_type, right_type) {
                    Ok(Type::Boolean)
                } else {
                    self.errors.push(TypeError::type_mismatch(
                        &left_type.to_string(),
                        &right_type.to_string(),
                    ));
                    Ok(Type::Boolean) // Still return boolean for comparison
                }
            }
            BinaryOperator::And | BinaryOperator::Or => {
                if *left_type == Type::Boolean && *right_type == Type::Boolean {
                    Ok(Type::Boolean)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        &format!("{:?}", op),
                        &format!("{} and {}", left_type.to_string(), right_type.to_string()),
                    ));
                    Ok(Type::Boolean)
                }
            }
        }
    }
    
    fn check_unary_operation(
        &mut self,
        op: &UnaryOperator,
        expr_type: &Type,
    ) -> Result<Type, Vec<TypeError>> {
        match op {
            UnaryOperator::Not => {
                if *expr_type == Type::Boolean {
                    Ok(Type::Boolean)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        "!",
                        &expr_type.to_string(),
                    ));
                    Ok(Type::Boolean)
                }
            }
            UnaryOperator::Minus => {
                if *expr_type == Type::Number {
                    Ok(Type::Number)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        "-",
                        &expr_type.to_string(),
                    ));
                    Ok(Type::Number)
                }
            }
        }
    }
    
    fn literal_type(&self, lit: &Literal) -> Type {
        match lit {
            Literal::String(_) => Type::String,
            Literal::Number(_) => Type::Number,
            Literal::Boolean(_) => Type::Boolean,
            Literal::Null => Type::Null,
        }
    }
    
    fn check_type(&mut self, type_def: &Type) -> Result<(), Vec<TypeError>> {
        match type_def {
            Type::String | Type::Number | Type::Boolean | Type::Void | Type::Null => Ok(()),
            Type::Named(name) => {
                // Check if it's a generic type parameter (single uppercase letter or common pattern)
                // For now, we'll be lenient and allow single-letter identifiers as type parameters
                if name.len() == 1 && name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    // This is likely a type parameter, allow it
                    Ok(())
                } else if !self.environment.has_type(name) {
                    self.errors.push(TypeError::undefined_type(name));
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Type::Generic { name, params } => {
                // Check if the generic type name is valid (e.g., List, Map, ApiResponse)
                if name == "List" || name == "Map" {
                    // Built-in generic types - OK, continue
                } else if !self.environment.has_type(name) {
                    // Check if it's a struct with generics
                    // For now, we'll check the base name
                    let base_name = name.clone();
                    if !self.environment.has_type(&base_name) {
                        self.errors.push(TypeError::undefined_type(name));
                        return Ok(()); // Return early to avoid checking params if type is invalid
                    }
                }
                for param in params {
                    self.check_type(param)?;
                }
                Ok(())
            }
            Type::Function { params, return_type } => {
                for param in params {
                    self.check_type(param)?;
                }
                self.check_type(return_type)?;
                Ok(())
            }
            Type::List(item_type) => {
                self.check_type(item_type)?;
                Ok(())
            }
            Type::Map { key, value } => {
                self.check_type(key)?;
                self.check_type(value)?;
                Ok(())
            }
            Type::Tuple(types) => {
                for t in types {
                    self.check_type(t)?;
                }
                Ok(())
            }
            Type::Optional(inner) => {
                self.check_type(inner)?;
                Ok(())
            }
        }
    }
    
    fn types_compatible(&self, t1: &Type, t2: &Type) -> bool {
        if t1 == t2 {
            return true;
        }
        
        // Handle type aliases and named types
        match (t1, t2) {
            (Type::Named(n1), Type::Named(n2)) => n1 == n2,
            (Type::Generic { name: n1, params: p1 }, Type::Generic { name: n2, params: p2 }) => {
                if n1 == n2 && p1.len() == p2.len() {
                    p1.iter().zip(p2.iter()).all(|(a, b)| self.types_compatible(a, b))
                } else {
                    false
                }
            }
            (Type::Generic { name: n1, params: p1 }, Type::Named(n2)) => {
                // ApiResponse<void> vs ApiResponse - check if base names match
                // Allow if the generic has parameters (struct literal can be instantiated)
                n1 == n2
            }
            (Type::Named(n1), Type::Generic { name: n2, params: _p2 }) => {
                // ApiResponse vs ApiResponse<void> - struct literal can match generic return type
                // This allows struct literals to be compatible with generic return types
                n1 == n2
            }
            (Type::List(l1), Type::List(l2)) => self.types_compatible(l1, l2),
            (Type::Map { key: k1, value: v1 }, Type::Map { key: k2, value: v2 }) => {
                self.types_compatible(k1, k2) && self.types_compatible(v1, v2)
            }
            (Type::Optional(o1), Type::Optional(o2)) => self.types_compatible(o1, o2),
            _ => false,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
