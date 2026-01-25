use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::parser::ast::*;
use anyhow::Result;

pub struct DesugaringPass;

impl DesugaringPass {
    pub fn new() -> Self {
        Self
    }

    fn desugar_program(&self, program: &mut Program) {
        for item in &mut program.items {
            self.desugar_item(item);
        }
    }

    fn desugar_item(&self, item: &mut Item) {
        match item {
            Item::Function(func) => {
                self.desugar_block(&mut func.body);
            }
            Item::Module(module) => {
                for sub_item in &mut module.items {
                    self.desugar_item(sub_item);
                }
            }
            Item::Impl(impl_block) => {
                for method in &mut impl_block.methods {
                    self.desugar_block(&mut method.body);
                }
            }
            Item::TopLevelCode(expr_stmt) => {
                // Desugar expressions in top-level code
                // Expressions can contain blocks (e.g., lambda expressions)
                self.desugar_expression(&mut expr_stmt.expression);
            }
            _ => {}
        }
    }

    fn desugar_block(&self, block: &mut Block) {
        let mut new_statements = Vec::new();

        for statement in block.statements.drain(..) {
            match statement {
                Statement::Try(try_stmt) => {
                    let desugared = self.desugar_try_statement(try_stmt);
                    new_statements.extend(desugared);
                }
                Statement::If(mut if_stmt) => {
                    self.desugar_block(&mut if_stmt.then_block);
                    if let Some(ref mut else_block) = if_stmt.else_block {
                        self.desugar_block(else_block);
                    }
                    new_statements.push(Statement::If(if_stmt));
                }
                Statement::For(mut for_stmt) => {
                    self.desugar_block(&mut for_stmt.body);
                    new_statements.push(Statement::For(for_stmt));
                }
                Statement::While(mut while_stmt) => {
                    self.desugar_block(&mut while_stmt.body);
                    new_statements.push(Statement::While(while_stmt));
                }
                Statement::Match(mut match_stmt) => {
                    for arm in &mut match_stmt.arms {
                        self.desugar_block(&mut arm.body);
                    }
                    new_statements.push(Statement::Match(match_stmt));
                }
                _ => {
                    new_statements.push(statement);
                }
            }
        }

        block.statements = new_statements;
    }

    fn desugar_try_statement(&self, try_stmt: TryStatement) -> Vec<Statement> {
        let mut result = Vec::new();

        // 1. Transform try-block: First desugar nested try-catch, then wrap returns in Result.ok()
        let mut try_block = try_stmt.try_block;
        // Desugar any nested try-catch blocks first
        self.desugar_block(&mut try_block);
        // Then wrap returns in Result.ok()
        self.wrap_returns_in_try_block(&mut try_block);

        // 2. Create lambda expression for try-block
        let try_lambda = Expression::Lambda {
            params: Vec::new(),
            return_type: None, // Will be inferred by type checker
            body: Box::new(Expression::Block(try_block)),
        };

        // 3. Call lambda and store result
        let try_result_var = "__try_result".to_string();
        let try_result_expr = Expression::Call {
            callee: Box::new(try_lambda),
            args: Vec::new(),
        };

        result.push(Statement::Let(LetStatement {
            name: try_result_var.clone(),
            var_type: None, // Will be inferred
            value: try_result_expr,
            mutable: false,
        }));

        // 4. Handle catch blocks
        if !try_stmt.catch_blocks.is_empty() {
            let catch_statements =
                self.create_catch_dispatch(&try_result_var, &try_stmt.catch_blocks);
            result.extend(catch_statements);
        }

        // 5. Handle finally block
        if let Some(finally_block) = try_stmt.finally_block {
            let finally_statements = self.create_finally_block(finally_block);
            result.extend(finally_statements);
        }

        result
    }

    fn wrap_returns_in_try_block(&self, block: &mut Block) {
        // Wrap returns in Result.ok()
        // Note: Nested try-catch blocks are already desugared by desugar_block
        // before wrap_returns_in_try_block is called
        for statement in &mut block.statements {
            match statement {
                Statement::Return(return_stmt) => {
                    if let Some(value) = return_stmt.value.take() {
                        // Wrap in Result.ok()
                        let wrapped = Expression::Call {
                            callee: Box::new(Expression::Member {
                                object: Box::new(Expression::Identifier("Result".to_string())),
                                member: "ok".to_string(),
                            }),
                            args: vec![value],
                        };
                        return_stmt.value = Some(wrapped);
                    } else {
                        // return; -> return Result.ok(());
                        return_stmt.value = Some(Expression::Call {
                            callee: Box::new(Expression::Member {
                                object: Box::new(Expression::Identifier("Result".to_string())),
                                member: "ok".to_string(),
                            }),
                            args: vec![Expression::Literal(Literal::Null)],
                        });
                    }
                }
                Statement::If(if_stmt) => {
                    self.wrap_returns_in_try_block(&mut if_stmt.then_block);
                    if let Some(ref mut else_block) = if_stmt.else_block {
                        self.wrap_returns_in_try_block(else_block);
                    }
                }
                Statement::For(for_stmt) => {
                    self.wrap_returns_in_try_block(&mut for_stmt.body);
                }
                Statement::While(while_stmt) => {
                    self.wrap_returns_in_try_block(&mut while_stmt.body);
                }
                Statement::Match(match_stmt) => {
                    for arm in &mut match_stmt.arms {
                        self.wrap_returns_in_try_block(&mut arm.body);
                    }
                }
                _ => {}
            }
        }
    }

    fn create_catch_dispatch(
        &self,
        try_result_var: &str,
        catch_blocks: &[CatchBlock],
    ) -> Vec<Statement> {
        let mut result = Vec::new();

        // Check if result is error
        let is_err_check = Expression::Call {
            callee: Box::new(Expression::Member {
                object: Box::new(Expression::Identifier(try_result_var.to_string())),
                member: "isErr".to_string(),
            }),
            args: Vec::new(),
        };

        // Get error value
        let error_var = "__error".to_string();
        let get_error = Expression::Call {
            callee: Box::new(Expression::Member {
                object: Box::new(Expression::Identifier(try_result_var.to_string())),
                member: "err".to_string(),
            }),
            args: Vec::new(),
        };
        let unwrap_error = Expression::Call {
            callee: Box::new(Expression::Member {
                object: Box::new(get_error),
                member: "unwrap".to_string(),
            }),
            args: Vec::new(),
        };

        // If we have multiple catch blocks with types, use match
        let has_typed_catches = catch_blocks.iter().any(|cb| cb.error_type.is_some());

        if has_typed_catches && catch_blocks.len() > 1 {
            // Multiple typed catch blocks: use match
            let mut match_arms = Vec::new();

            for catch_block in catch_blocks {
                // Desugar nested try-catch in catch blocks
                let mut catch_body = catch_block.body.clone();
                self.desugar_block(&mut catch_body);

                if let Some(ref error_type) = catch_block.error_type {
                    // Typed catch: match on error type
                    let pattern = Pattern::EnumVariant {
                        name: match error_type {
                            Type::Named(name) => name.clone(),
                            _ => "Error".to_string(), // Fallback
                        },
                        data: catch_block.error_var.as_ref().map(|_| {
                            vec![Pattern::Identifier(
                                catch_block.error_var.clone().unwrap_or("err".to_string()),
                            )]
                        }),
                    };

                    // If error_var is specified, bind it
                    if let Some(ref var_name) = catch_block.error_var {
                        catch_body.statements.insert(
                            0,
                            Statement::Let(LetStatement {
                                name: var_name.clone(),
                                var_type: None,
                                value: Expression::Identifier(error_var.clone()),
                                mutable: false,
                            }),
                        );
                    }

                    match_arms.push(MatchArm {
                        pattern,
                        guard: None,
                        body: catch_body,
                    });
                } else {
                    // Generic catch: wildcard pattern
                    if let Some(ref var_name) = catch_block.error_var {
                        catch_body.statements.insert(
                            0,
                            Statement::Let(LetStatement {
                                name: var_name.clone(),
                                var_type: None,
                                value: Expression::Identifier(error_var.clone()),
                                mutable: false,
                            }),
                        );
                    }

                    match_arms.push(MatchArm {
                        pattern: Pattern::Wildcard,
                        guard: None,
                        body: catch_body,
                    });
                }
            }

            // Create if statement with match
            let match_stmt = Statement::Match(MatchStatement {
                expression: Expression::Identifier(error_var.clone()),
                arms: match_arms,
            });

            let catch_block = Block {
                statements: vec![
                    Statement::Let(LetStatement {
                        name: error_var,
                        var_type: None,
                        value: unwrap_error,
                        mutable: false,
                    }),
                    match_stmt,
                ],
            };

            result.push(Statement::If(IfStatement {
                condition: is_err_check,
                then_block: catch_block,
                else_block: None,
            }));
        } else {
            // Single catch block or no typed catches: simple if
            let catch_block = &catch_blocks[0];
            let mut catch_body = catch_block.body.clone();

            // Desugar nested try-catch in catch block
            self.desugar_block(&mut catch_body);

            // Bind error variable if specified
            if let Some(ref var_name) = catch_block.error_var {
                catch_body.statements.insert(
                    0,
                    Statement::Let(LetStatement {
                        name: var_name.clone(),
                        var_type: None,
                        value: unwrap_error,
                        mutable: false,
                    }),
                );
            }

            result.push(Statement::If(IfStatement {
                condition: is_err_check,
                then_block: catch_body,
                else_block: None,
            }));
        }

        result
    }

    fn create_finally_block(&self, finally_block: Block) -> Vec<Statement> {
        // Desugar nested try-catch in finally block
        let mut finally_block = finally_block;
        self.desugar_block(&mut finally_block);

        // Create lambda for finally block
        let finally_lambda = Expression::Lambda {
            params: Vec::new(),
            return_type: None,
            body: Box::new(Expression::Block(finally_block)),
        };

        // Call lambda
        let finally_call = Expression::Call {
            callee: Box::new(finally_lambda),
            args: Vec::new(),
        };

        // Execute finally
        vec![Statement::Expression(ExpressionStatement {
            expression: finally_call,
        })]
    }

    fn desugar_expression(&self, expr: &mut Expression) {
        match expr {
            Expression::Block(block) => {
                self.desugar_block(block);
            }
            Expression::Lambda { body, .. } => {
                if let Expression::Block(block) = body.as_mut() {
                    self.desugar_block(block);
                } else {
                    // Single expression body - desugar it
                    self.desugar_expression(body);
                }
            }
            Expression::Call { args, .. } => {
                for arg in args {
                    self.desugar_expression(arg);
                }
            }
            Expression::Member { object, .. } => {
                self.desugar_expression(object);
            }
            Expression::Index { object, index, .. } => {
                self.desugar_expression(object);
                self.desugar_expression(index);
            }
            Expression::BinaryOp { left, right, .. } => {
                self.desugar_expression(left);
                self.desugar_expression(right);
            }
            Expression::UnaryOp { expr, .. } => {
                self.desugar_expression(expr);
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                self.desugar_expression(condition);
                self.desugar_expression(then_expr);
                self.desugar_expression(else_expr);
            }
            Expression::Assignment { target, value } => {
                self.desugar_expression(target);
                self.desugar_expression(value);
            }
            Expression::FormatString { parts } => {
                for part in parts {
                    if let FormatStringPart::Expression(expr) = part {
                        self.desugar_expression(expr);
                    }
                }
            }
            Expression::LLMCall { args, .. } => {
                for arg in args {
                    self.desugar_expression(arg);
                }
            }
            Expression::ListLiteral(elements) => {
                for element in elements {
                    self.desugar_expression(element);
                }
            }
            Expression::MapLiteral(entries) => {
                for (_key, value) in entries {
                    self.desugar_expression(value);
                }
            }
            Expression::StructLiteral { fields, .. } => {
                for (_name, value) in fields {
                    self.desugar_expression(value);
                }
            }
            Expression::GenericConstructor { args, .. } => {
                for arg in args {
                    self.desugar_expression(arg);
                }
            }
            _ => {} // Literals, Identifiers, etc. don't need desugaring
        }
    }
}

impl Pass for DesugaringPass {
    fn name(&self) -> &str {
        "Desugaring"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if let Some(ref mut program) = context.program {
            self.desugar_program(program);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::context::CompilationContext;
    use crate::parser::parser::Parser;

    #[test]
    fn test_desugar_simple_try_catch() {
        let source = r#"
        fn test() {
            try {
                return someFunction();
            } catch (err) {
                handleError(err);
            }
        }
        "#;

        let program = Parser::parse(source).unwrap();
        let mut context = CompilationContext::new("test.velin".to_string(), source.to_string());
        context.program = Some(program);

        let pass = DesugaringPass::new();
        pass.run(&mut context).unwrap();

        // Verify that Try statements are desugared
        if let Some(ref program) = context.program {
            for item in &program.items {
                if let crate::parser::ast::Item::Function(func) = item {
                    for stmt in &func.body.statements {
                        // After desugaring, there should be no Try statements
                        assert!(!matches!(stmt, crate::parser::ast::Statement::Try(_)));
                    }
                }
            }
        }
    }

    #[test]
    fn test_desugar_multiple_catch() {
        let source = r#"
        fn test() {
            try {
                return processData();
            } catch (err: ValidationError) {
                handleValidationError(err);
            } catch (err: NetworkError) {
                handleNetworkError(err);
            } catch (err) {
                handleGenericError(err);
            }
        }
        "#;

        let program = Parser::parse(source).unwrap();
        let mut context = CompilationContext::new("test.velin".to_string(), source.to_string());
        context.program = Some(program);

        let pass = DesugaringPass::new();
        pass.run(&mut context).unwrap();

        // Verify desugaring
        if let Some(ref program) = context.program {
            for item in &program.items {
                if let crate::parser::ast::Item::Function(func) = item {
                    for stmt in &func.body.statements {
                        assert!(!matches!(stmt, crate::parser::ast::Statement::Try(_)));
                    }
                }
            }
        }
    }

    #[test]
    fn test_desugar_with_finally() {
        let source = r#"
        fn test() {
            try {
                return openFile();
            } catch (err) {
                log.error(err);
            } finally {
                closeResources();
            }
        }
        "#;

        let program = Parser::parse(source).unwrap();
        let mut context = CompilationContext::new("test.velin".to_string(), source.to_string());
        context.program = Some(program);

        let pass = DesugaringPass::new();
        pass.run(&mut context).unwrap();

        // Verify desugaring
        if let Some(ref program) = context.program {
            for item in &program.items {
                if let crate::parser::ast::Item::Function(func) = item {
                    for stmt in &func.body.statements {
                        assert!(!matches!(stmt, crate::parser::ast::Statement::Try(_)));
                    }
                }
            }
        }
    }
}
