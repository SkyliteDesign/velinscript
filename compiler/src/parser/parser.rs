use crate::parser::ast::*;
use crate::parser::lexer::{Lexer, Token, FormatStringPart as LexerFormatStringPart};
use crate::error::{CompilerError, ErrorLocation};
use crate::compiler::language::VELISCH_LANGUAGE_NAME;

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
        // Velisch Identity Check - Fingerabdruck im Parser
        let _velisch_check = VELISCH_LANGUAGE_NAME;
        
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
        
        // Skip leading newlines (comments are already skipped by lexer)
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
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
        
        // Skip newlines after 'let'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Check for 'mut' keyword
        let mutable = if let Some(Token::Identifier(name)) = self.peek() {
            if name == "mut" {
                self.advance();
                // Skip newlines after 'mut'
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                true
            } else {
                false
            }
        } else {
            false
        };
        
        // Get variable name
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Skip newlines before type or '='
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Parse type annotation if present
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
        
        let let_stmt = LetStatement {
            name,
            var_type,
            value,
            mutable,
        };
        
        // Convert LetStatement to a function that initializes the variable
        // This is a workaround until we add proper global variable support
        Ok(Item::Function(Function {
            decorators: Vec::new(),
            visibility: Visibility::Private,
            name: format!("__init_{}", let_stmt.name),
            type_params: Vec::new(),
            params: Vec::new(),
            return_type: None,
            body: Block {
                statements: vec![Statement::Let(let_stmt)],
            },
            is_async: false,
            is_const: false,
            documentation: None,
        }))
    }
    
    fn parse_item(&mut self) -> Result<Item, ParseError> {
        // Skip leading newlines
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }

        // Parse doc comments
        let mut documentation = None;
        // Accumulate multiple doc comments
        let mut doc_string = String::new();
        while let Some(Token::DocComment(doc)) = self.peek() {
            if !doc_string.is_empty() {
                doc_string.push('\n');
            }
            doc_string.push_str(doc);
            self.advance();
            // Skip newlines between doc comments
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
        }
        if !doc_string.is_empty() {
            documentation = Some(doc_string);
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
                let func = self.parse_function(decorators, visibility, is_async, is_const, documentation)?;
                // For top-level functions, type_params are already parsed in parse_function
                Ok(Item::Function(func))
            }
            Some(Token::Struct) => {
                self.advance(); // consume 'struct'
                Ok(Item::Struct(self.parse_struct(visibility, decorators, documentation)?))
            }
            Some(Token::Enum) => {
                self.advance(); // consume 'enum'
                Ok(Item::Enum(self.parse_enum(visibility, documentation)?))
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
                Ok(Item::Module(self.parse_module(visibility, documentation)?))
            }
            Some(Token::Trait) | Some(Token::Interface) => {
                self.advance(); // consume 'trait' or 'interface'
                Ok(Item::Trait(self.parse_trait(visibility)?))
            }
            Some(Token::Impl) => {
                self.advance(); // consume 'impl'
                Ok(Item::Impl(self.parse_impl()?))
            }
            _ => Err(self.error("Expected function, struct, enum, type, use, module, trait, interface, or impl")),
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
        documentation: Option<String>,
    ) -> Result<Function, ParseError> {
        // Skip newlines after 'fn'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Parse generic type parameters with constraints if present
        let mut type_params = Vec::new();
        if self.check(&Token::Lt) {
            self.advance(); // consume '<'
            
            // Skip newlines after '<'
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            loop {
                let param_name = match self.consume_identifier()? {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                };
                
                // Skip newlines after param name
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                // Parse constraints if present (T: Trait1 & Trait2)
                let mut constraints = Vec::new();
                if self.check(&Token::Colon) {
                    self.advance(); // consume ':'
                    
                    // Skip newlines after ':'
                    while matches!(self.peek(), Some(Token::Newline)) {
                        self.advance();
                    }
                    
                    // Parse constraint list (Trait1 & Trait2)
                    let mut trait_names = Vec::new();
                    loop {
                        let trait_name = match self.consume_identifier()? {
                            Token::Identifier(name) => name,
                            _ => unreachable!(),
                        };
                        trait_names.push(trait_name);
                        
                        // Skip newlines after trait name
                        while matches!(self.peek(), Some(Token::Newline)) {
                            self.advance();
                        }
                        
                        // Check for '&' (multiple constraints)
                        if self.check(&Token::And) {
                            self.advance(); // consume '&'
                            
                            // Skip newlines after '&'
                            while matches!(self.peek(), Some(Token::Newline)) {
                                self.advance();
                            }
                        } else {
                            break;
                        }
                    }
                    
                    if trait_names.len() == 1 {
                        constraints.push(GenericConstraint::Trait(trait_names[0].clone()));
                    } else {
                        constraints.push(GenericConstraint::Multiple(trait_names));
                    }
                }
                
                type_params.push(GenericParam {
                    name: param_name,
                    constraints,
                });
                
                // Skip newlines before comma or closing '>'
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
                
                // Skip newlines after comma
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
            }
            
            // Skip newlines before closing '>'
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            self.consume(&Token::Gt, "Expected '>'")?;
        }
        
        // Skip newlines before '('
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::LParen, "Expected '('")?;
        let params = self.parse_parameters()?;
        
        // Skip newlines before closing paren
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
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
            type_params,
            params,
            return_type,
            body,
            is_async,
            is_const,
            documentation,
        })
    }
    
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, ParseError> {
        let mut params = Vec::new();
        
        // Skip newlines before parameters
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        if !self.check(&Token::RParen) {
            loop {
                // Skip newlines before parameter name
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                let name = match self.consume_identifier()? {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                };
                
                // Skip newlines before colon
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                self.consume(&Token::Colon, "Expected ':'")?;
                
                // Skip newlines after colon
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                let param_type = self.parse_type()?;
                
                // Skip newlines after type
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                let default = if self.check(&Token::Eq) {
                    self.advance();
                    // Skip newlines after equals
                    while matches!(self.peek(), Some(Token::Newline)) {
                        self.advance();
                    }
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                
                params.push(Parameter {
                    name,
                    param_type,
                    default,
                });
                
                // Skip newlines before comma or closing paren
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
                
                // Skip newlines after comma
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
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
            Some(Token::Throw) => {
                self.advance();
                Ok(Statement::Throw(self.parse_throw()?))
            }
            Some(Token::Break) => {
                self.advance();
                if self.check(&Token::Semicolon) {
                    self.advance();
                }
                Ok(Statement::Break(BreakStatement))
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
    
    fn parse_throw(&mut self) -> Result<ThrowStatement, ParseError> {
        let expression = self.parse_expression()?;
        if self.check(&Token::Semicolon) {
            self.advance();
        }
        Ok(ThrowStatement { expression })
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
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            // Skip newlines before match arm
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            if self.check(&Token::RBrace) {
                break;
            }
            
            arms.push(self.parse_match_arm()?);
            
            // Skip optional comma after arm
            if self.check(&Token::Comma) {
                self.advance();
            }
            
            // Skip newlines after comma or arm
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        
        Ok(MatchStatement { expression, arms })
    }
    
    fn parse_match_arm(&mut self) -> Result<MatchArm, ParseError> {
        // Skip leading whitespace/newlines
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let pattern = self.parse_pattern()?;
        
        // Parse optional guard: `if condition` (with or without parentheses)
        let guard = if self.check(&Token::If) {
            self.advance();
            
            // Skip whitespace
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            // Check if there's a parenthesis (optional)
            let condition = if self.check(&Token::LParen) {
                self.advance();
                let cond = self.parse_expression()?;
                self.consume(&Token::RParen, "Expected ')' after guard condition")?;
                cond
            } else {
                // No parentheses - parse expression directly
                self.parse_expression()?
            };
            
            Some(condition)
        } else {
            None
        };
        
        self.consume(&Token::FatArrow, "Expected '=>'")?;
        
        // Skip whitespace before body
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Body can be a block or a single expression
        let body = if self.check(&Token::LBrace) {
            self.parse_block()?
        } else {
            // Single expression body - wrap in block
            let expr = self.parse_expression()?;
            Block {
                statements: vec![Statement::Expression(ExpressionStatement { expression: expr })],
            }
        };
        
        Ok(MatchArm { pattern, guard, body })
    }
    
    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        // Parse pattern with support for Or patterns (pattern1 | pattern2)
        let mut patterns = vec![self.parse_pattern_atom()?];
        
        // Accept both Token::Or (||) and Token::Unknown('|') for pattern matching
        while self.check(&Token::Or) || matches!(self.peek(), Some(Token::Unknown('|'))) {
            if matches!(self.peek(), Some(Token::Unknown('|'))) {
                self.advance(); // consume Token::Unknown('|')
            } else if self.check(&Token::Or) {
                self.advance(); // consume Token::Or
            }
            patterns.push(self.parse_pattern_atom()?);
        }
        
        if patterns.len() == 1 {
            Ok(patterns.into_iter().next().unwrap())
        } else {
            Ok(Pattern::Or(patterns))
        }
    }
    
    fn parse_pattern_atom(&mut self) -> Result<Pattern, ParseError> {
        match self.peek() {
            Some(Token::String(s)) => {
                let s_clone = s.clone();
                self.advance();
                Ok(Pattern::Literal(Literal::String(s_clone)))
            }
            Some(Token::Number(n)) => {
                let n_clone = *n;
                self.advance();
                
                // Check for range pattern: number..number or number..=number
                // The lexer tokenizes ..= as a single Token::DotDotEq
                // But we also handle the case where it might be tokenized as DotDot + Eq
                if self.check(&Token::DotDotEq) {
                    // Single token case: ..=
                    self.advance(); // consume DotDotEq
                    let end = if let Some(Token::Number(end_num)) = self.peek() {
                        let num = *end_num;
                        self.advance();
                        Expression::Literal(Literal::Number(num))
                    } else {
                        self.parse_expression()?
                    };
                    return Ok(Pattern::Range {
                        start: Box::new(Expression::Literal(Literal::Number(n_clone))),
                        end: Box::new(end),
                        inclusive: true,
                    });
                } else if self.check(&Token::DotDot) {
                    // Two token case: DotDot followed by Eq (for ..=) or just DotDot (for ..)
                    self.advance(); // consume DotDot
                    let inclusive = if self.check(&Token::Eq) {
                        self.advance(); // consume Eq
                        true
                    } else {
                        false
                    };
                    // Parse the end expression (should be a number for range patterns)
                    let end = if let Some(Token::Number(end_num)) = self.peek() {
                        let num = *end_num;
                        self.advance();
                        Expression::Literal(Literal::Number(num))
                    } else {
                        self.parse_expression()?
                    };
                    return Ok(Pattern::Range {
                        start: Box::new(Expression::Literal(Literal::Number(n_clone))),
                        end: Box::new(end),
                        inclusive,
                    });
                }
                
                Ok(Pattern::Literal(Literal::Number(n_clone)))
            }
            Some(Token::Boolean(b)) => {
                let b_clone = *b;
                self.advance();
                Ok(Pattern::Literal(Literal::Boolean(b_clone)))
            }
            Some(Token::Identifier(name)) => {
                // Check for wildcard pattern: _
                if name == "_" {
                    self.advance();
                    return Ok(Pattern::Wildcard);
                }
                
                let name_clone = name.clone();
                self.advance();
                
                // Check for enum variant pattern: EnumName::Variant or EnumName::Variant(data)
                if self.check(&Token::Colon) {
                    self.advance();
                    if self.check(&Token::Colon) {
                        self.advance();
                        // Enum variant
                        if let Some(Token::Identifier(variant_name)) = self.peek() {
                            let variant = variant_name.clone();
                            self.advance();
                            
                            // Check for variant data: Variant(data1, data2)
                            if self.check(&Token::LParen) {
                                self.advance();
                                let mut data = Vec::new();
                                if !self.check(&Token::RParen) {
                                    loop {
                                        data.push(self.parse_pattern()?);
                                        if self.check(&Token::RParen) {
                                            break;
                                        }
                                        self.consume(&Token::Comma, "Expected ',' or ')'")?;
                                    }
                                }
                                self.consume(&Token::RParen, "Expected ')'")?;
                                return Ok(Pattern::EnumVariant {
                                    name: format!("{}::{}", name_clone, variant),
                                    data: Some(data),
                                });
                            }
                            
                            return Ok(Pattern::EnumVariant {
                                name: format!("{}::{}", name_clone, variant),
                                data: None,
                            });
                        }
                    }
                    // Not an enum variant, treat as identifier with type annotation
                    // This is for patterns like: Error(err: DatabaseError)
                    if let Some(Token::Identifier(type_name)) = self.peek() {
                        let _type_name_clone = type_name.clone();
                        self.advance();
                        // This is a binding with type annotation: identifier: Type
                        // For now, we'll treat it as an identifier pattern
                        // The type annotation will be handled in type checking
                        return Ok(Pattern::Identifier(name_clone));
                    }
                }
                
                // Check for struct pattern: StructName { field1: pattern1, field2: pattern2 }
                if self.check(&Token::LBrace) {
                    self.advance();
                    let mut fields = Vec::new();
                    if !self.check(&Token::RBrace) {
                        loop {
                            let field_name = if let Some(Token::Identifier(name)) = self.peek() {
                                let name_clone = name.clone();
                                self.advance();
                                name_clone
                            } else {
                                return Err(self.error("Expected field name"));
                            };
                            
                            // Optional pattern binding: field: pattern or field: "literal"
                            // OR field: Type (type annotation, skip it)
                            let field_pattern = if self.check(&Token::Colon) {
                                self.advance();
                                // Check if it's a type annotation (field: Type) or a pattern (field: pattern)
                                // Type annotations in patterns are identifiers, patterns can be literals or other patterns
                                match self.peek() {
                                    Some(Token::String(s)) => {
                                        // Literal pattern: name: "admin"
                                        let s_clone = s.clone();
                                        self.advance();
                                        Pattern::Literal(Literal::String(s_clone))
                                    }
                                    Some(Token::Number(n)) => {
                                        // Literal pattern: age: 18
                                        let n_clone = *n;
                                        self.advance();
                                        Pattern::Literal(Literal::Number(n_clone))
                                    }
                                    Some(Token::Identifier(_)) => {
                                        // Could be a type annotation or a pattern identifier
                                        // In patterns, we treat it as a pattern identifier
                                        self.parse_pattern()?
                                    }
                                    _ => self.parse_pattern()?
                                }
                            } else {
                                Pattern::Identifier(field_name.clone())
                            };
                            
                            fields.push((field_name, field_pattern));
                            
                            // Skip newlines before comma or closing brace
                            while matches!(self.peek(), Some(Token::Newline)) {
                                self.advance();
                            }
                            
                            if self.check(&Token::RBrace) {
                                break;
                            }
                            if self.check(&Token::Comma) {
                                self.advance();
                            } else {
                                return Err(self.error("Expected ',' or '}'"));
                            }
                        }
                    }
                    self.consume(&Token::RBrace, "Expected '}'")?;
                    return Ok(Pattern::Struct {
                        name: name_clone,
                        fields,
                    });
                }
                
                // Check for tuple pattern: (pattern1, pattern2, ...)
                if self.check(&Token::LParen) {
                    self.advance();
                    let mut patterns = Vec::new();
                    if !self.check(&Token::RParen) {
                        loop {
                            patterns.push(self.parse_pattern()?);
                            if self.check(&Token::RParen) {
                                break;
                            }
                            self.consume(&Token::Comma, "Expected ',' or ')'")?;
                        }
                    }
                    self.consume(&Token::RParen, "Expected ')'")?;
                    return Ok(Pattern::Tuple(patterns));
                }
                
                Ok(Pattern::Identifier(name_clone))
            }
            Some(Token::DotDot) | Some(Token::DotDotEq) => {
                // Range pattern starting with .. (e.g., ..10)
                let inclusive = self.check(&Token::DotDotEq);
                self.advance();
                let end = self.parse_expression()?;
                Ok(Pattern::Range {
                    start: Box::new(Expression::Literal(Literal::Number(0.0))), // Default start
                    end: Box::new(end),
                    inclusive,
                })
            }
            _ => Err(self.error("Expected pattern"))
        }
    }
    
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_or()?;
        
        if self.check(&Token::Eq) {
            self.advance();
            let value = self.parse_assignment()?; // Right-associative
            
            // Check if left side is a valid assignment target
            match expr {
                Expression::Identifier(_) | Expression::Member { .. } | Expression::Index { .. } => {
                    Ok(Expression::Assignment {
                        target: Box::new(expr),
                        value: Box::new(value),
                    })
                }
                _ => Err(self.error("Invalid assignment target")),
            }
        } else {
            Ok(expr)
        }
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
                // Skip newlines before argument
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                args.push(self.parse_expression()?);

                // Skip newlines after argument
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }

                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
                
                // Skip newlines after comma
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
            }
        }

        // Skip newlines before closing paren
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }

        self.consume(&Token::RParen, "Expected ')'")?;

        Ok(Expression::Call {
            callee: Box::new(callee),
            args,
        })
    }
    
    fn is_struct_literal_start(&self) -> bool {
        if !self.check(&Token::LBrace) {
            return false;
        }
        
        let mut offset = 1;
        // Skip newlines after LBrace
        while let Some(Token::Newline) = self.peek_n(offset) {
            offset += 1;
        }
        
        match self.peek_n(offset) {
            Some(Token::RBrace) => true,
            Some(Token::Identifier(_)) => {
                matches!(self.peek_n(offset + 1), Some(Token::Colon))
            }
            _ => false,
        }
    }

    fn parse_struct_literal_fields(&mut self) -> Result<Vec<(String, Expression)>, ParseError> {
        self.consume(&Token::LBrace, "Expected '{'")?;
        let mut fields = Vec::new();
        
        while !self.check(&Token::RBrace) {
             // Skip newlines
             while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
             if self.check(&Token::RBrace) { break; }

             let name = match self.consume_identifier()? {
                 Token::Identifier(n) => n,
                 _ => unreachable!(),
             };
             
             self.consume(&Token::Colon, "Expected ':'")?;
             let value = self.parse_expression()?;
             fields.push((name, value));
             
             // Skip newlines
             while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }

             if self.check(&Token::Comma) {
                 self.advance();
             }
             
             // Skip newlines after comma
             while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
        }
        self.consume(&Token::RBrace, "Expected '}'")?;
        Ok(fields)
    }

    fn is_map_literal_start(&self) -> bool {
        if !self.check(&Token::LBrace) {
            return false;
        }
        
        let mut offset = 1;
        // Skip newlines after LBrace
        while let Some(Token::Newline) = self.peek_n(offset) {
            offset += 1;
        }
        
        // Check for { "string": ...
        if let Some(Token::String(_)) = self.peek_n(offset) {
            // Check if followed by colon
             let mut next_offset = offset + 1;
             while let Some(Token::Newline) = self.peek_n(next_offset) {
                next_offset += 1;
            }
            if let Some(Token::Colon) = self.peek_n(next_offset) {
                return true;
            }
        }
        
        // Check for { identifier: ...
        if let Some(Token::Identifier(_)) = self.peek_n(offset) {
             let mut next_offset = offset + 1;
             while let Some(Token::Newline) = self.peek_n(next_offset) {
                next_offset += 1;
            }
            if let Some(Token::Colon) = self.peek_n(next_offset) {
                return true;
            }
        }
        
        false
    }

    fn parse_map_literal(&mut self) -> Result<Vec<(String, Expression)>, ParseError> {
        self.consume(&Token::LBrace, "Expected '{'")?;
        let mut entries = Vec::new();
        
        while !self.check(&Token::RBrace) {
             // Skip newlines
             while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
             if self.check(&Token::RBrace) { break; }

             let key = if let Some(Token::String(s)) = self.peek() {
                 let s = s.clone();
                 self.advance();
                 s
             } else if let Some(Token::Identifier(s)) = self.peek() {
                 let s = s.clone();
                 self.advance();
                 s
             } else {
                 return Err(self.error("Expected string key or identifier in map literal"));
             };
             
             // Skip newlines
             while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
             
             self.consume(&Token::Colon, "Expected ':'")?;
             
             // Skip newlines
             while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
             
             let value = self.parse_expression()?;
             entries.push((key, value));
             
             // Skip newlines
             while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }

             if self.check(&Token::Comma) {
                 self.advance();
             }
             
             // Skip newlines after comma
             while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
        }
        self.consume(&Token::RBrace, "Expected '}'")?;
        Ok(entries)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek() {
            Some(Token::FormatString(parts)) => {
                let parts_clone = parts.clone();
                self.advance();
                
                // Convert lexer FormatStringParts to AST FormatStringParts
                let mut ast_parts = Vec::new();
                for part in parts_clone {
                    match part {
                        LexerFormatStringPart::Text(text) => {
                            ast_parts.push(FormatStringPart::Text(text));
                        }
                        LexerFormatStringPart::Expression(expr_str) => {
                            // Parse the expression string
                            let mut expr_parser = Parser::new(
                                Lexer::new(&expr_str).tokenize()
                                    .map_err(|e| ParseError {
                                        message: format!("Failed to tokenize format string expression: {}", e.message),
                                        expected: "valid expression".to_string(),
                                        found: expr_str.clone(),
                                        position: 0,
                                        line: e.line,
                                        column: e.column,
                                        source_context: None,
                                    })?,
                                expr_str.clone(),
                            );
                            let expr = expr_parser.parse_expression()?;
                            ast_parts.push(FormatStringPart::Expression(Box::new(expr)));
                        }
                    }
                }
                Ok(Expression::FormatString { parts: ast_parts })
            }
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
            Some(Token::Null) => {
                self.advance();
                Ok(Expression::Literal(Literal::Null))
            }
            Some(Token::Identifier(name)) => {
                let name_clone = name.clone();
                self.advance();
                
                // Check for generic type constructor: List<string>()
                if self.check(&Token::Lt) {
                    // Parse generic type parameters
                    self.advance(); // consume '<'
                    let mut type_params = Vec::new();
                    
                    loop {
                        type_params.push(self.parse_type()?);
                        
                        // Skip newlines before comma or closing '>'
                        while matches!(self.peek(), Some(Token::Newline)) {
                            self.advance();
                        }
                        
                        if !self.check(&Token::Comma) {
                            break;
                        }
                        self.advance();
                        
                        // Skip newlines after comma
                        while matches!(self.peek(), Some(Token::Newline)) {
                            self.advance();
                        }
                    }
                    
                    // Skip newlines before closing '>'
                    while matches!(self.peek(), Some(Token::Newline)) {
                        self.advance();
                    }
                    
                    self.consume(&Token::Gt, "Expected '>'")?;
                    
                    // Skip newlines after '>'
                    while matches!(self.peek(), Some(Token::Newline)) {
                        self.advance();
                    }
                    
                    // Check if followed by function call: List<string>()
                    if self.check(&Token::LParen) {
                        // Parse function call arguments
                        self.advance(); // consume '('
                        let mut args = Vec::new();
                        
                        // Skip newlines after opening paren
                        while matches!(self.peek(), Some(Token::Newline)) {
                            self.advance();
                        }
                        
                        if !self.check(&Token::RParen) {
                            loop {
                                args.push(self.parse_expression()?);
                                
                                // Skip newlines before comma or closing paren
                                while matches!(self.peek(), Some(Token::Newline)) {
                                    self.advance();
                                }
                                
                                if !self.check(&Token::Comma) {
                                    break;
                                }
                                self.advance();
                                
                                // Skip newlines after comma
                                while matches!(self.peek(), Some(Token::Newline)) {
                                    self.advance();
                                }
                            }
                        }
                        
                        // Skip newlines before closing paren
                        while matches!(self.peek(), Some(Token::Newline)) {
                            self.advance();
                        }
                        
                        self.consume(&Token::RParen, "Expected ')'")?;
                        
                        // Create a generic type constructor call
                        Ok(Expression::GenericConstructor {
                            name: name_clone,
                            type_params,
                            args,
                        })
                    } else {
                        // Just a generic type reference, not a constructor call
                        // This is a type expression, which we'll handle differently
                        // For now, return as identifier (will be handled by type checker)
                        Ok(Expression::Identifier(name_clone))
                    }
                } else {
                    // Check if followed by struct literal: StructName { ... }
                    if self.check(&Token::LBrace) && self.is_struct_literal_start() {
                        let fields = self.parse_struct_literal_fields()?;
                        Ok(Expression::StructLiteral {
                            name: name_clone,
                            fields,
                        })
                    } else {
                        Ok(Expression::Identifier(name_clone))
                    }
                }
            }
            Some(Token::LParen) => {
                self.advance();
                
                // Check if this is a lambda: (params) => expression
                // Simple heuristic: if we see identifier followed by colon, it's likely a lambda parameter
                let is_lambda = if let Some(Token::Identifier(_)) = self.peek() {
                    // Save position
                    let saved_pos = self.current;
                    
                    // Try to parse a parameter
                    let mut looks_like_lambda = false;
                    if let Ok(Token::Identifier(_)) = self.consume_identifier() {
                        if self.check(&Token::Colon) {
                            looks_like_lambda = true;
                        }
                    }
                    
                    // Restore position
                    self.current = saved_pos;
                    looks_like_lambda
                } else {
                    false
                };
                
                if is_lambda {
                    // Parse lambda parameters
                    let mut params = Vec::new();
                    
                    if !self.check(&Token::RParen) {
                        loop {
                            let param_name = match self.consume_identifier()? {
                                Token::Identifier(name) => name,
                                _ => return Err(self.error("Expected parameter name")),
                            };
                            
                            self.consume(&Token::Colon, "Expected ':' after parameter name")?;
                            let param_type = self.parse_type()?;
                            
                            params.push(Parameter {
                                name: param_name,
                                param_type,
                                default: None,
                            });
                            
                            if self.check(&Token::RParen) {
                                break;
                            }
                            self.consume(&Token::Comma, "Expected ',' or ')'")?;
                        }
                    }
                    
                    self.consume(&Token::RParen, "Expected ')'")?;
                    self.consume(&Token::FatArrow, "Expected '=>'")?;
                    
                    // Parse lambda body (can be expression or block)
                    let body = if self.check(&Token::LBrace) {
                        // Block body
                        let block = self.parse_block()?;
                        Expression::Block(block)
                    } else {
                        // Single expression body
                        self.parse_expression()?
                    };
                    
                    Ok(Expression::Lambda {
                        params,
                        return_type: None, // Type inference
                        body: Box::new(body),
                    })
                } else {
                    // Normal parenthesized expression
                    let expr = self.parse_expression()?;
                    self.consume(&Token::RParen, "Expected ')'")?;
                    Ok(expr)
                }
            }
            Some(Token::LBrace) => {
                if self.is_map_literal_start() {
                    let entries = self.parse_map_literal()?;
                    Ok(Expression::MapLiteral(entries))
                } else {
                    let block = self.parse_block()?;
                    Ok(Expression::Block(block))
                }
            }
            Some(Token::LBracket) => {
                self.advance(); // consume '['
                let mut elements = Vec::new();
                
                // Skip newlines
                while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
                
                if !self.check(&Token::RBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        
                        // Skip newlines
                        while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
                        
                        if !self.check(&Token::Comma) {
                            break;
                        }
                        self.advance();
                        
                        // Skip newlines
                        while matches!(self.peek(), Some(Token::Newline)) { self.advance(); }
                    }
                }
                
                self.consume(&Token::RBracket, "Expected ']'")?;
                Ok(Expression::ListLiteral(elements))
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
                    
                    // Skip newlines after '<'
                    while matches!(self.peek(), Some(Token::Newline)) {
                        self.advance();
                    }
                    
                    loop {
                        params.push(self.parse_type()?);
                        
                        // Skip newlines before comma or closing '>'
                        while matches!(self.peek(), Some(Token::Newline)) {
                            self.advance();
                        }
                        
                        if !self.check(&Token::Comma) {
                            break;
                        }
                        self.advance();
                        
                        // Skip newlines after comma
                        while matches!(self.peek(), Some(Token::Newline)) {
                            self.advance();
                        }
                    }
                    
                    // Skip newlines before closing '>'
                    while matches!(self.peek(), Some(Token::Newline)) {
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
                        "Result" => {
                            // Result<T, E> syntax
                            if self.check(&Token::Lt) {
                                self.advance();
                                let ok_type = self.parse_type()?;
                                
                                // Skip newlines before comma
                                while matches!(self.peek(), Some(Token::Newline)) {
                                    self.advance();
                                }
                                
                                self.consume(&Token::Comma, "Expected ',' in Result<T, E>")?;
                                
                                // Skip newlines after comma
                                while matches!(self.peek(), Some(Token::Newline)) {
                                    self.advance();
                                }
                                
                                let err_type = self.parse_type()?;
                                
                                // Skip newlines before closing '>'
                                while matches!(self.peek(), Some(Token::Newline)) {
                                    self.advance();
                                }
                                
                                self.consume(&Token::Gt, "Expected '>'")?;
                                Ok(Type::Result {
                                    ok: Box::new(ok_type),
                                    err: Box::new(err_type),
                                })
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
    
    fn parse_struct(&mut self, visibility: Visibility, decorators: Vec<Decorator>, documentation: Option<String>) -> Result<Struct, ParseError> {
        // Skip newlines after 'struct'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Parse generic type parameters if present
        let mut type_params = Vec::new();
        if self.check(&Token::Lt) {
            self.advance(); // consume '<'
            
            // Skip newlines after '<'
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            loop {
                let param_name = match self.consume_identifier()? {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                };
                type_params.push(param_name);
                
                // Skip newlines before comma or closing '>'
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
                
                // Skip newlines after comma
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
            }
            
            // Skip newlines before closing '>'
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            self.consume(&Token::Gt, "Expected '>'")?;
            
            // Skip newlines after '>'
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
        }
        
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
            
            let field_decorators = self.parse_decorators()?;
            
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
                decorators: field_decorators,
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
            type_params,
            fields,
            visibility,
            decorators,
            documentation,
        })
    }
    
    fn parse_enum(&mut self, visibility: Visibility, documentation: Option<String>) -> Result<Enum, ParseError> {
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
        
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            // Skip newlines before variant
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            if self.check(&Token::RBrace) {
                break;
            }
            
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
            documentation,
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
    
    fn parse_module(&mut self, visibility: Visibility, _documentation: Option<String>) -> Result<Module, ParseError> {
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
            // Skip newlines inside module body
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            if self.check(&Token::RBrace) {
                break;
            }
            
            items.push(self.parse_item()?);
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        
        Ok(Module {
            name,
            items,
            visibility,
            documentation: None,
        })
    }
    
    fn parse_trait(&mut self, visibility: Visibility) -> Result<Trait, ParseError> {
        // Skip newlines after 'trait' or 'interface'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        let name = match self.consume_identifier()? {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        
        // Parse generic type parameters if present
        let mut type_params = Vec::new();
        if self.check(&Token::Lt) {
            self.advance();
            loop {
                let param_name = match self.consume_identifier()? {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                };
                type_params.push(param_name);
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.consume(&Token::Gt, "Expected '>'")?;
        }
        
        // Skip newlines before '{'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::LBrace, "Expected '{'")?;
        let mut methods = Vec::new();
        
        // Skip newlines after '{'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        while !self.check(&Token::RBrace) {
            // Parse trait method
            let method_name = match self.consume_identifier()? {
                Token::Identifier(name) => name,
                _ => unreachable!(),
            };
            
            // Skip newlines after method name
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            self.consume(&Token::LParen, "Expected '('")?;
            let params = self.parse_parameters()?;
            self.consume(&Token::RParen, "Expected ')'")?;
            
            // Skip newlines after params
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            let return_type = if self.check(&Token::Colon) {
                self.advance();
                // Skip newlines after ':'
                while matches!(self.peek(), Some(Token::Newline)) {
                    self.advance();
                }
                Some(self.parse_type()?)
            } else {
                None
            };
            
            // Skip newlines after return type
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            // Trait methods end with semicolon (no body)
            self.consume(&Token::Semicolon, "Expected ';' after trait method")?;
            
            methods.push(TraitMethod {
                name: method_name,
                params,
                return_type,
            });
            
            // Skip newlines after semicolon
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        
        Ok(Trait {
            name,
            type_params,
            methods,
            visibility,
        })
    }
    
    fn parse_impl(&mut self) -> Result<Impl, ParseError> {
        // Skip newlines after 'impl'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Parse trait name (optional - can be blank impl)
        let trait_name = if matches!(self.peek(), Some(Token::Identifier(_))) {
            let name = match self.consume_identifier()? {
                Token::Identifier(name) => name,
                _ => unreachable!(),
            };
            
            // Skip newlines after trait name
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
            
            // Check for 'for'
            if matches!(self.peek(), Some(Token::Identifier(_))) {
                let for_keyword = match self.peek() {
                    Some(Token::Identifier(s)) if s == "for" => {
                        self.advance();
                        true
                    }
                    _ => false,
                };
                
                if !for_keyword {
                    // This is a blank impl, trait_name is actually the type name
                    return Err(self.error("Expected 'for' after trait name in impl"));
                }
            } else {
                return Err(self.error("Expected 'for' after trait name in impl"));
            }
            
            name
        } else {
            // Blank impl - no trait name
            String::new()
        };
        
        // Skip newlines after 'for'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        // Parse type that implements the trait
        let for_type = self.parse_type()?;
        
        // Parse generic type parameters if present
        let mut type_params = Vec::new();
        if self.check(&Token::Lt) {
            self.advance();
            loop {
                let param_name = match self.consume_identifier()? {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                };
                type_params.push(param_name);
                
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.consume(&Token::Gt, "Expected '>'")?;
        }
        
        // Skip newlines before '{'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        self.consume(&Token::LBrace, "Expected '{'")?;
        let mut methods = Vec::new();
        
        // Skip newlines after '{'
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }
        
        while !self.check(&Token::RBrace) {
            // Parse impl method (same as function)
            let decorators = self.parse_decorators()?;
            let visibility = if self.check(&Token::Pub) {
                self.advance();
                Visibility::Public
            } else {
                Visibility::Private
            };
            
            let is_async = if self.check(&Token::Async) {
                self.advance();
                true
            } else {
                false
            };
            
            let is_const = if self.check(&Token::Const) {
                self.advance();
                true
            } else {
                false
            };
            
            if !self.check(&Token::Fn) {
                return Err(self.error("Expected 'fn' in impl block"));
            }
            
            self.advance(); // consume 'fn'
            methods.push(self.parse_function(decorators, visibility, is_async, is_const, None)?);
            
            // Skip newlines after method
            while matches!(self.peek(), Some(Token::Newline)) {
                self.advance();
            }
        }
        
        self.consume(&Token::RBrace, "Expected '}'")?;
        
        Ok(Impl {
            trait_name,
            for_type,
            type_params,
            methods,
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
        
        // Calculate position from current token index
        // Since we don't have token positions, we approximate by using a fraction of source length
        // This is a workaround - ideally tokens should store their source positions
        let position = if self.current < self.tokens.len() {
            // Approximate: assume tokens are evenly distributed
            (self.current * self.source.len()) / self.tokens.len().max(1)
        } else {
            self.source.len()
        };
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
