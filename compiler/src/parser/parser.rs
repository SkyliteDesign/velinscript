use crate::parser::ast::*;
use crate::parser::lexer::{Lexer, Token};
use crate::error::{CompilerError, ErrorLocation};

// Legacy ParseError für Rückwärtskompatibilität
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub expected: String,
    pub found: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
    pub source_context: Option<String>,
}

impl From<ParseError> for CompilerError {
    fn from(err: ParseError) -> Self {
        CompilerError::parse_error_with_context(
            err.message,
            ErrorLocation::new(err.line, err.column),
            err.expected,
            err.found,
        )
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    source: String,
    line_starts: Vec<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: String) -> Self {
        let line_starts: Vec<usize> = source
            .char_indices()
            .enumerate()
            .filter_map(|(_i, (pos, ch))| if ch == '\n' { Some(pos) } else { None })
            .collect();
        
        Parser {
            tokens,
            current: 0,
            source,
            line_starts,
        }
    }
    
    pub fn parse(input: &str) -> Result<Program, ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().map_err(|e| ParseError {
            message: e.message,
            expected: "valid token".to_string(),
            found: "invalid token".to_string(),
            position: 0,
            line: e.line,
            column: e.column,
            source_context: None,
        })?;
        
        let mut parser = Parser::new(tokens, input.to_string());
        parser.parse_program()
    }
    
    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();
        
        while !self.is_at_end() {
            // Skip newlines and whitespace between items
            while matches!(self.peek(), Some(Token::Newline) | None) {
                if self.is_at_end() {
                    break;
                }
                self.advance();
            }
            
            if self.is_at_end() {
                break;
            }
            
            // Check if it's a let statement at top level (global variable)
            if self.check(&Token::Let) {
                items.push(self.parse_top_level_let()?);
            } else {
                items.push(self.parse_item()?);
            }
        }
        
        Ok(Program { items })
    }
    
    fn parse_top_level_let(&mut self) -> Result<Item, ParseError> {
        // Consume 'let' token
        self.advance();
        
        // Parse let statement as a global variable
        // For now, we'll treat it as a statement that needs to be wrapped
        // This is a temporary solution - in the future we might want a GlobalVariable item type
        let let_stmt = self.parse_let()?;
        // Convert LetStatement to a function that initializes the variable
        // This is a workaround until we add proper global variable support
        Ok(Item::Function(Function {
            decorators: Vec::new(),
            visibility: Visibility::Private,
            name: format!("__init_{}", let_stmt.name),
            params: Vec::new(),
            return_type: None,
            body: Block {
                statements: vec![Statement::Let(let_stmt)],
            },
            is_async: false,
            is_const: false,
        }))
    }
    
    fn parse_item(&mut self) -> Result<Item, ParseError> {
        // Skip leading newlines
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Parse decorators if present
        let decorators = self.parse_decorators()?;
        
        // Skip newlines after decorators
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Parse visibility
        let visibility = if self.check(&Token::Pub) {
            self.advance();
            Visibility::Public
        } else {
            Visibility::Private
        };
        
        // Skip newlines after visibility
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Check if async
        let is_async = if self.check(&Token::Async) {
            self.advance();
            true
        } else {
            false
        };
        
        // Skip newlines after async
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Check if const
        let is_const = if self.check(&Token::Const) {
            self.advance();
            true
        } else {
            false
        };
        
        // Skip newlines after const
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        match self.peek() {
            Some(Token::Fn) => {
                self.advance(); // consume 'fn'
                // Newlines after 'fn' are handled in parse_function
                Ok(Item::Function(self.parse_function(decorators, visibility, is_async, is_const)?))
            }
            Some(Token::Struct) => {
                self.advance(); // consume 'struct'
                Ok(Item::Struct(self.parse_struct(visibility, decorators)?))
            }
            Some(Token::Enum) => {
                self.advance(); // consume 'enum'
                Ok(Item::Enum(self.parse_enum(visibility)?))
            }
            Some(Token::Type) => {
                self.advance(); // consume 'type'
                Ok(Item::TypeAlias(self.parse_type_alias(visibility)?))
            }
            Some(Token::Use) => {
                self.advance(); // consume 'use'
                Ok(Item::Use(self.parse_use()?))
            }
            Some(Token::Mod) => {
                self.advance(); // consume 'mod'
                Ok(Item::Module(self.parse_module(visibility)?))
            }
            _ => Err(self.error("Expected function, struct, enum, type, use, or module")),
        }
    }
    
    fn parse_decorators(&mut self) -> Result<Vec<Decorator>, ParseError> {
        let mut decorators = Vec::new();
        
        // Skip leading newlines
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        while self.check(&Token::At) {
            decorators.push(self.parse_decorator()?);
            // Skip newlines after each decorator
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
        }
        
        Ok(decorators)
    }
    
    fn parse_decorator(&mut self) -> Result<Decorator, ParseError> {
        self.consume(&Token::At, "Expected '@'")?;
        
        let name = match self.peek() {
            Some(Token::Get) => {
                self.advance();
                "GET".to_string()
            }
            Some(Token::Post) => {
                self.advance();
                "POST".to_string()
            }
            Some(Token::Put) => {
                self.advance();
                "PUT".to_string()
            }
            Some(Token::Delete) => {
                self.advance();
                "DELETE".to_string()
            }
            Some(Token::Patch) => {
                self.advance();
                "PATCH".to_string()
            }
            Some(Token::Auth) => {
                self.advance();
                "Auth".to_string()
            }
            Some(Token::Role) => {
                self.advance();
                "Role".to_string()
            }
            Some(Token::Cache) => {
                self.advance();
                "Cache".to_string()
            }
            Some(Token::SEO) => {
                self.advance();
                "SEO".to_string()
            }
            Some(Token::AI) => {
                self.advance();
                "AI".to_string()
            }
            Some(Token::Identifier(name)) => {
                let name_clone = name.clone();
                self.advance();
                name_clone
            }
            _ => return Err(self.error("Expected decorator name")),
        };
        
        // Parse arguments if present (before skipping newlines)
        let args = if self.check(&Token::LParen) {
            self.parse_decorator_args()?
        } else {
            Vec::new()
        };
        
        // Skip whitespace and newlines after decorator (including args)
        while matches!(self.peek(), Some(Token::Newline) | None) {
            if self.is_at_end() {
                break;
            }
            self.advance();
        }
        
        Ok(Decorator { name, args })
    }
    
    fn parse_decorator_args(&mut self) -> Result<Vec<DecoratorArg>, ParseError> {
        self.consume(&Token::LParen, "Expected '('")?;
        let mut args = Vec::new();
        
        // Skip whitespace/newlines after opening paren
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        if !self.check(&Token::RParen) {
            loop {
                args.push(self.parse_decorator_arg()?);
                
                // Skip whitespace/newlines before comma or closing paren
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
                
                // Skip whitespace/newlines after comma
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
            }
        }
        
        // Skip whitespace/newlines before closing paren
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::RParen, "Expected ')'")?;
        Ok(args)
    }
    
    fn parse_decorator_arg(&mut self) -> Result<DecoratorArg, ParseError> {
        // Check for named argument: name: value
        if let Some(Token::Identifier(name)) = self.peek() {
            let name_clone = name.clone();
            self.advance();
            
            if self.check(&Token::Colon) {
                self.advance();
                let value = Box::new(self.parse_decorator_arg()?);
                return Ok(DecoratorArg::Named {
                    name: name_clone,
                    value,
                });
            } else {
                // It's just an identifier
                return Ok(DecoratorArg::Identifier(name_clone));
            }
        }
        
        // Parse value
        match self.peek() {
            Some(Token::String(s)) => {
                let s_clone = s.clone();
                self.advance();
                Ok(DecoratorArg::String(s_clone))
            }
            Some(Token::Number(n)) => {
                let n_clone = *n;
                self.advance();
                Ok(DecoratorArg::Number(n_clone))
            }
            Some(Token::Boolean(b)) => {
                let b_clone = *b;
                self.advance();
                Ok(DecoratorArg::Boolean(b_clone))
            }
            Some(Token::Identifier(name)) => {
                let name_clone = name.clone();
                self.advance();
                Ok(DecoratorArg::Identifier(name_clone))
            }
            _ => Err(self.error("Expected decorator argument")),
        }
    }
    
    fn parse_function(
        &mut self,
        decorators: Vec<Decorator>,
        visibility: Visibility,
        is_async: bool,
        is_const: bool,
    ) -> Result<Function, ParseError> {
        // Skip newlines after 'fn'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Skip newlines before '('
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::LParen, "Expected '('")?;
        let params = self.parse_parameters()?;
        self.consume(&Token::RParen, "Expected ')'")?;
        
        // Skip newlines before return type
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let return_type = if self.check(&Token::Colon) {
            self.advance();
            // Skip newlines after colon
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Skip newlines before block
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let body = self.parse_block()?;
        
        Ok(Function {
            decorators,
            visibility,
            name,
            params,
            return_type,
            body,
            is_async,
            is_const,
        })
    }
    
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, ParseError> {
        let mut params = Vec::new();
        
        if !self.check(&Token::RParen) {
            loop {
                let name = match self.consume_identifier()? {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                };
                
                self.consume(&Token::Colon, "Expected ':'")?;
                let param_type = self.parse_type()?;
                
                let default = if self.check(&Token::Eq) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                
                params.push(Parameter {
                    name,
                    param_type,
                    default,
                });
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }
        
        Ok(params)
    }
    
    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.consume(&Token::LBrace, "Expected '{'")?;
        let mut statements = Vec::new();
        
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            // Skip newlines before statements
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            if self.check(&Token::RBrace) {
                break;
            }
            
            statements.push(self.parse_statement()?);
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        Ok(Block { statements })
    }
    
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        // Skip leading newlines
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        match self.peek() {
            Some(Token::Let) => {
                self.advance();
                Ok(Statement::Let(self.parse_let()?))
            }
            Some(Token::Return) => {
                self.advance();
                Ok(Statement::Return(self.parse_return()?))
            }
            Some(Token::If) => {
                self.advance();
                Ok(Statement::If(self.parse_if()?))
            }
            Some(Token::For) => {
                self.advance();
                Ok(Statement::For(self.parse_for()?))
            }
            Some(Token::While) => {
                self.advance();
                Ok(Statement::While(self.parse_while()?))
            }
            Some(Token::Match) => {
                self.advance();
                Ok(Statement::Match(self.parse_match()?))
            }
            _ => {
                let expr = self.parse_expression()?;
                if self.check(&Token::Semicolon) {
                    self.advance();
                }
                Ok(Statement::Expression(ExpressionStatement { expression: expr }))
            }
        }
    }
    
    fn parse_let(&mut self) -> Result<LetStatement, ParseError> {
        // Skip newlines before 'let'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let mutable = if let Some(Token::Identifier(name)) = self.peek() {
            if name == "mut" {
                self.advance();
                true
            } else {
                false
            }
        } else {
            false
        };
        
        // Skip newlines after 'mut' if present
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Skip newlines before type or '='
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let var_type = if self.check(&Token::Colon) {
            self.advance();
            // Skip newlines after colon
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Skip newlines before '='
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::Eq, "Expected '='")?;
        
        // Skip newlines after '='
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let value = self.parse_expression()?;
        
        // Skip newlines before semicolon
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        if self.check(&Token::Semicolon) {
            self.advance();
        }
        
        Ok(LetStatement {
            name,
            var_type,
            value,
            mutable,
        })
    }
    
    fn parse_return(&mut self) -> Result<ReturnStatement, ParseError> {
        let value = if !self.check(&Token::Semicolon) && !self.check(&Token::RBrace) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.check(&Token::Semicolon) {
            self.advance();
        }
        
        Ok(ReturnStatement { value })
    }
    
    fn parse_if(&mut self) -> Result<IfStatement, ParseError> {
        self.consume(&Token::LParen, "Expected '('")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::RParen, "Expected ')'")?;
        
        let then_block = self.parse_block()?;
        
        let else_block = if self.check(&Token::Else) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Ok(IfStatement {
            condition,
            then_block,
            else_block,
        })
    }
    
    fn parse_for(&mut self) -> Result<ForStatement, ParseError> {
        self.consume(&Token::LParen, "Expected '('")?;
        let variable = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Support "in" keyword for iterators
        if !self.check(&Token::In) {
            return Err(self.error("Expected 'in'"));
        }
        self.advance();
        let iterable = self.parse_expression()?;
        self.consume(&Token::RParen, "Expected ')'")?;
        
        let body = self.parse_block()?;
        
        Ok(ForStatement {
            variable,
            iterable,
            body,
        })
    }
    
    fn parse_while(&mut self) -> Result<WhileStatement, ParseError> {
        self.consume(&Token::LParen, "Expected '('")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::RParen, "Expected ')'")?;
        
        let body = self.parse_block()?;
        
        Ok(WhileStatement { condition, body })
    }
    
    fn parse_match(&mut self) -> Result<MatchStatement, ParseError> {
        let expression = self.parse_expression()?;
        self.consume(&Token::LBrace, "Expected '{'")?;
        
        let mut arms = Vec::new();
        while !self.check(&Token::RBrace) {
            arms.push(self.parse_match_arm()?);
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        
        Ok(MatchStatement { expression, arms })
    }
    
    fn parse_match_arm(&mut self) -> Result<MatchArm, ParseError> {
        let pattern = self.parse_pattern()?;
        self.consume(&Token::FatArrow, "Expected '=>'")?;
        let body = self.parse_block()?;
        
        Ok(MatchArm { pattern, body })
    }
    
    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        // Simplified pattern parsing
        match self.peek() {
            Some(Token::String(s)) => {
                let s_clone = s.clone();
                self.advance();
                Ok(Pattern::Literal(Literal::String(s_clone)))
            }
            Some(Token::Number(n)) => {
                let n_clone = *n;
                self.advance();
                Ok(Pattern::Literal(Literal::Number(n_clone)))
            }
            Some(Token::Boolean(b)) => {
                let b_clone = *b;
                self.advance();
                Ok(Pattern::Literal(Literal::Boolean(b_clone)))
            }
            Some(Token::Identifier(name)) => {
                let name_clone = name.clone();
                self.advance();
                Ok(Pattern::Identifier(name_clone))
            }
            _ => Err(self.error("Expected pattern")),
        }
    }
    
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_and()?;
        
        while self.check(&Token::Or) {
            let op = BinaryOperator::Or;
            self.advance();
            let right = self.parse_and()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_equality()?;
        
        while self.check(&Token::And) {
            let op = BinaryOperator::And;
            self.advance();
            let right = self.parse_equality()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;
        
        while matches!(self.peek(), Some(Token::EqEq) | Some(Token::NotEq)) {
            let op = if self.check(&Token::EqEq) {
                self.advance();
                BinaryOperator::Eq
            } else {
                self.advance();
                BinaryOperator::NotEq
            };
            
            let right = self.parse_comparison()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_term()?;
        
        while matches!(
            self.peek(),
            Some(Token::Gt) | Some(Token::GtEq) | Some(Token::Lt) | Some(Token::LtEq)
        ) {
            let op = match self.peek() {
                Some(Token::Gt) => {
                    self.advance();
                    BinaryOperator::Gt
                }
                Some(Token::GtEq) => {
                    self.advance();
                    BinaryOperator::GtEq
                }
                Some(Token::Lt) => {
                    self.advance();
                    BinaryOperator::Lt
                }
                Some(Token::LtEq) => {
                    self.advance();
                    BinaryOperator::LtEq
                }
                _ => unreachable!(),
            };
            
            let right = self.parse_term()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_factor()?;
        
        while matches!(self.peek(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = if self.check(&Token::Plus) {
                self.advance();
                BinaryOperator::Add
            } else {
                self.advance();
                BinaryOperator::Subtract
            };
            
            let right = self.parse_factor()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_unary()?;
        
        while matches!(
            self.peek(),
            Some(Token::Star) | Some(Token::Slash) | Some(Token::Percent)
        ) {
            let op = match self.peek() {
                Some(Token::Star) => {
                    self.advance();
                    BinaryOperator::Multiply
                }
                Some(Token::Slash) => {
                    self.advance();
                    BinaryOperator::Divide
                }
                Some(Token::Percent) => {
                    self.advance();
                    BinaryOperator::Modulo
                }
                _ => unreachable!(),
            };
            
            let right = self.parse_unary()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        // Check for await
        if self.check(&Token::Await) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::Await {
                expr: Box::new(expr),
            });
        }
        
        if matches!(self.peek(), Some(Token::Not) | Some(Token::Minus)) {
            let op = if self.check(&Token::Not) {
                self.advance();
                UnaryOperator::Not
            } else {
                self.advance();
                UnaryOperator::Minus
            };
            
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp {
                op,
                expr: Box::new(expr),
            });
        }
        
        self.parse_call()
    }
    
    fn parse_call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.check(&Token::LParen) {
                expr = self.finish_call(expr)?;
            } else if self.check(&Token::Dot) {
                self.advance();
                let member = match self.consume_identifier()? {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                };
                expr = Expression::Member {
                    object: Box::new(expr),
                    member,
                };
            } else if self.check(&Token::LBracket) {
                self.advance();
                let index = self.parse_expression()?;
                self.consume(&Token::RBracket, "Expected ']'")?;
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParseError> {
        self.advance(); // consume '('
        let mut args = Vec::new();
        
        if !self.check(&Token::RParen) {
            loop {
                args.push(self.parse_expression()?);
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }
        
        self.consume(&Token::RParen, "Expected ')'")?;
        
        Ok(Expression::Call {
            callee: Box::new(callee),
            args,
        })
    }
    
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek() {
            Some(Token::String(s)) => {
                let s_clone = s.clone();
                self.advance();
                Ok(Expression::Literal(Literal::String(s_clone)))
            }
            Some(Token::Number(n)) => {
                let n_clone = *n;
                self.advance();
                Ok(Expression::Literal(Literal::Number(n_clone)))
            }
            Some(Token::Boolean(b)) => {
                let b_clone = *b;
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(b_clone)))
            }
            Some(Token::Identifier(name)) => {
                let name_clone = name.clone();
                self.advance();
                Ok(Expression::Identifier(name_clone))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(&Token::RParen, "Expected ')'")?;
                Ok(expr)
            }
            Some(Token::LBrace) => {
                let block = self.parse_block()?;
                Ok(Expression::Block(block))
            }
            Some(Token::If) => {
                self.advance();
                self.consume(&Token::LParen, "Expected '('")?;
                let condition = self.parse_expression()?;
                self.consume(&Token::RParen, "Expected ')'")?;
                let then_expr = self.parse_expression()?;
                self.consume(&Token::Else, "Expected 'else'")?;
                let else_expr = self.parse_expression()?;
                Ok(Expression::If {
                    condition: Box::new(condition),
                    then_expr: Box::new(then_expr),
                    else_expr: Box::new(else_expr),
                })
            }
            _ => Err(self.error("Expected expression")),
        }
    }
    
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let token = self.peek().cloned();
        match token {
            Some(Token::Identifier(name)) => {
                self.advance();
                
                // Check for generic types: List<T>, Map<K, V>
                if self.check(&Token::Lt) {
                    self.advance();
                    let mut params = Vec::new();
                    
                    loop {
                        params.push(self.parse_type()?);
                        
                        if !self.check(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    
                    self.consume(&Token::Gt, "Expected '>'")?;
                    
                    Ok(Type::Generic { name: name.clone(), params })
                } else {
                    match name.as_str() {
                        "string" => Ok(Type::String),
                        "number" => Ok(Type::Number),
                        "boolean" => Ok(Type::Boolean),
                        "void" => Ok(Type::Void),
                        "null" => Ok(Type::Null),
                        "List" => {
                            // List<T> syntax
                            if self.check(&Token::Lt) {
                                self.advance();
                                let item_type = self.parse_type()?;
                                self.consume(&Token::Gt, "Expected '>'")?;
                                Ok(Type::List(Box::new(item_type)))
                            } else {
                                Ok(Type::Named(name.clone()))
                            }
                        }
                        _ => Ok(Type::Named(name.clone())),
                    }
                }
            }
            Some(Token::LParen) => {
                // Tuple type: (T1, T2, ...)
                self.advance();
                let mut types = Vec::new();
                
                if !self.check(&Token::RParen) {
                    loop {
                        types.push(self.parse_type()?);
                        
                        if !self.check(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }
                
                self.consume(&Token::RParen, "Expected ')'")?;
                Ok(Type::Tuple(types))
            }
            _ => Err(self.error("Expected type")),
        }
    }
    
    fn parse_struct(&mut self, visibility: Visibility, decorators: Vec<Decorator>) -> Result<Struct, ParseError> {
        // Skip newlines after 'struct'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Skip newlines before '{'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::LBrace, "Expected '{'")?;
        let mut fields = Vec::new();
        
        while !self.check(&Token::RBrace) {
            // Skip newlines before field
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            if self.check(&Token::RBrace) {
                break;
            }
            
            let field_visibility = if self.check(&Token::Pub) {
                self.advance();
                Visibility::Public
            } else {
                Visibility::Private
            };
            
            // Skip newlines after visibility
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            let field_name = match self.consume_identifier()? {
                Token::Identifier(name) => name,
                _ => unreachable!(),
            };
            
            // Skip newlines before ':'
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            self.consume(&Token::Colon, "Expected ':'")?;
            
            // Skip newlines after ':'
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            let field_type = self.parse_type()?;
            
            fields.push(StructField {
                name: field_name,
                field_type,
                visibility: field_visibility,
            });
            
            // Skip newlines before comma
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            if self.check(&Token::Comma) {
                self.advance();
            }
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        
        Ok(Struct {
            name,
            fields,
            visibility,
            decorators,
        })
    }
    
    fn parse_enum(&mut self, visibility: Visibility) -> Result<Enum, ParseError> {
        // Skip newlines after 'enum'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Skip newlines before '{'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::LBrace, "Expected '{'")?;
        let mut variants = Vec::new();
        
        while !self.check(&Token::RBrace) {
            let variant_name = match self.consume_identifier()? {
                Token::Identifier(name) => name,
                _ => unreachable!(),
            };
            
            let data = if self.check(&Token::LParen) {
                self.advance();
                let mut types = Vec::new();
                
                if !self.check(&Token::RParen) {
                    loop {
                        types.push(self.parse_type()?);
                        
                        if !self.check(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }
                
                self.consume(&Token::RParen, "Expected ')'")?;
                Some(types)
            } else {
                None
            };
            
            variants.push(EnumVariant {
                name: variant_name,
                data,
            });
            
            if self.check(&Token::Comma) {
                self.advance();
            }
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        
        Ok(Enum {
            name,
            variants,
            visibility,
        })
    }
    
    fn parse_type_alias(&mut self, visibility: Visibility) -> Result<TypeAlias, ParseError> {
        // Skip newlines after 'type'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Skip newlines before '='
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::Eq, "Expected '='")?;
        let aliased_type = self.parse_type()?;
        
        Ok(TypeAlias {
            name,
            aliased_type,
            visibility,
        })
    }
    
    fn parse_module(&mut self, visibility: Visibility) -> Result<Module, ParseError> {
        // Skip newlines after 'mod'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Skip newlines before '{'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::LBrace, "Expected '{'")?;
        let mut items = Vec::new();
        
        while !self.check(&Token::RBrace) {
            items.push(self.parse_item()?);
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        
        Ok(Module {
            name,
            items,
            visibility,
        })
    }
    
    fn parse_use(&mut self) -> Result<Use, ParseError> {
        let mut path = Vec::new();
        
        loop {
            let name = match self.consume_identifier()? {
                Token::Identifier(name) => name,
                _ => unreachable!(),
            };
            path.push(name);
            
            if !self.check(&Token::Colon) || !matches!(self.peek_n(1), Some(Token::Colon)) {
                break;
            }
            self.advance(); // consume ':'
            self.advance(); // consume ':'
        }
        
        let alias = if let Some(Token::Identifier(name)) = self.peek() {
            if name == "as" {
                self.advance();
                Some(match self.consume_identifier()? {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                })
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(Use { path, alias })
    }
    
    // Helper methods
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    
    fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current + n)
    }
    
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }
    
    fn check(&self, token: &Token) -> bool {
        match (self.peek(), token) {
            (Some(t), expected) => t == expected,
            _ => false,
        }
    }
    
    fn consume(&mut self, token: &Token, message: &str) -> Result<Token, ParseError> {
        if self.check(token) {
            let token = self.peek().unwrap().clone();
            self.advance();
            Ok(token)
        } else {
            Err(self.error(message))
        }
    }
    
    fn consume_identifier(&mut self) -> Result<Token, ParseError> {
        match self.peek() {
            Some(Token::Identifier(_)) => {
                let token = self.peek().unwrap().clone();
                self.advance();
                Ok(token)
            }
            _ => Err(self.error("Expected identifier")),
        }
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Some(Token::EOF) | None)
    }
    
    fn get_line_column(&self, position: usize) -> (usize, usize) {
        // Calculate line/column from token position using line_starts
        if position >= self.source.len() {
            return (self.line_starts.len() + 1, 1);
        }
        
        // Find which line this position is on
        let mut line = 1;
        for (i, &line_start) in self.line_starts.iter().enumerate() {
            if position < line_start {
                line = i + 1;
                break;
            } else if i == self.line_starts.len() - 1 {
                line = i + 2;
            }
        }
        
        // Calculate column
        let line_start = if line > 1 {
            self.line_starts[line - 2]
        } else {
            0
        };
        let column = position - line_start + 1;
        
        (line, column)
    }
    
    fn get_source_context(&self, line: usize, column: usize) -> String {
        if line == 0 || line > self.line_starts.len() {
            return String::new();
        }
        
        let line_start = if line > 1 {
            self.line_starts[line - 2]
        } else {
            0
        };
        
        let line_end = if line < self.line_starts.len() {
            self.line_starts[line - 1]
        } else {
            self.source.len()
        };
        
        if line_start >= line_end || line_start >= self.source.len() {
            return String::new();
        }
        
        let line_content = &self.source[line_start..line_end.min(self.source.len())];
        let trimmed = line_content.trim_end();
        let indent = (column.saturating_sub(1)).min(trimmed.len());
        
        format!("{}\n{}^", trimmed, " ".repeat(indent))
    }
    
    fn error(&self, message: &str) -> ParseError {
        let found = self
            .peek()
            .map(|t| format!("{:?}", t))
            .unwrap_or_else(|| "EOF".to_string());
        
        // Calculate position from current token
        let position = self.current;
        let (line, column) = self.get_line_column(position);
        let source_context = self.get_source_context(line, column);
        
        ParseError {
            message: message.to_string(),
            expected: message.to_string(),
            found,
            position,
            line,
            column,
            source_context: Some(source_context),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_function() {
        let code = r#"
            @GET("/api/users")
            fn getUsers(): List<User> {
                return db.findAll(User);
            }
        "#;
        
        let result = Parser::parse(code);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_decorator() {
        let code = r#"
            @GET("/api/users/:id")
            fn getUser(id: string): User {
                return db.find(id);
            }
        "#;
        
        let result = Parser::parse(code);
        assert!(result.is_ok());
    }
}
