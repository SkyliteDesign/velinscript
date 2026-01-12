use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    Return,
    If,
    Else,
    For,
    While,
    Match,
    Type,
    Struct,
    Enum,
    Impl,
    Trait,
    Pub,
    Use,
    Mod,
    Const,
    Static,
    Async,
    Await,
    In,
    
    // Decorators
    At, // @
    
    // HTTP Methods (als Decorators)
    Get,
    Post,
    Put,
    Delete,
    Patch,
    
    // Security Decorators
    Auth,
    Role,
    Cache,
    SEO,
    AI,
    
    // Literals
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    
    // Operators
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Eq,          // =
    EqEq,        // ==
    NotEq,       // !=
    Lt,          // <
    Gt,          // >
    LtEq,        // <=
    GtEq,        // >=
    And,         // &&
    Or,          // ||
    Not,         // !
    
    // Punctuation
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    Dot,         // .
    Arrow,       // ->
    FatArrow,    // =>
    
    // Special
    Newline,
    EOF,
    Unknown(char),
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: Chars<'a>,
    current: Option<char>,
    position: usize,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current: None,
            position: 0,
            line: 1,
            column: 0,
        };
        lexer.advance();
        // Skip BOM if present at the start
        if let Some('\u{feff}') = lexer.current {
            lexer.advance();
        }
        lexer
    }
    
    fn advance(&mut self) {
        self.current = self.input.next();
        if let Some(ch) = self.current {
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else if ch == '\r' {
                // Handle CRLF: skip \r, next char should be \n
                if let Some('\n') = self.input.clone().next() {
                    // Don't increment position yet, \n will be handled next
                    self.column = 0;
                } else {
                    self.column += 1;
                }
            } else {
                self.column += 1;
            }
        }
        self.position += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current {
            // Skip BOM (Byte Order Mark) if present
            if ch == '\u{feff}' {
                self.advance();
                continue;
            }
            
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        // We already consumed the first '/' in next_token()
        // Now check for second '/' or '*'
        if let Some('/') = self.current {
            // Single-line comment: //
            self.advance(); // consume second '/'
            // Skip until newline (but don't consume the newline)
            while let Some(ch) = self.current {
                if ch == '\n' || ch == '\r' {
                    // Don't consume newline, let it be tokenized separately
                    break;
                }
                self.advance();
            }
        } else if let Some('*') = self.current {
            // Multi-line comment: /*
            self.advance(); // consume '*'
            while let Some(ch) = self.current {
                if ch == '*' {
                    self.advance();
                    if let Some('/') = self.current {
                        self.advance(); // consume closing '/'
                        break;
                    }
                } else {
                    self.advance();
                }
            }
        }
    }
    
    fn read_string(&mut self) -> Result<String, LexerError> {
        let mut string = String::new();
        let quote = self.current.unwrap();
        self.advance();
        
        while let Some(ch) = self.current {
            match ch {
                '"' | '\'' if ch == quote => {
                    self.advance();
                    return Ok(string);
                }
                '\\' => {
                    self.advance();
                    if let Some(escaped) = self.current {
                        match escaped {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            '\'' => string.push('\''),
                            _ => string.push(escaped),
                        }
                        self.advance();
                    }
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }
        
        Err(LexerError {
            message: "Unterminated string".to_string(),
            line: self.line,
            column: self.column,
        })
    }
    
    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        
        while let Some(ch) = self.current {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        num_str.parse().unwrap_or(0.0)
    }
    
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        
        while let Some(ch) = self.current {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        ident
    }
    
    fn read_keyword_or_identifier(&mut self) -> Token {
        let ident = self.read_identifier();
        
        match ident.as_str() {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "match" => Token::Match,
            "type" => Token::Type,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "impl" => Token::Impl,
            "trait" => Token::Trait,
            "pub" => Token::Pub,
            "use" => Token::Use,
            "mod" => Token::Mod,
            "const" => Token::Const,
            "static" => Token::Static,
            "async" => Token::Async,
            "await" => Token::Await,
            "in" => Token::In,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "GET" => Token::Get,
            "POST" => Token::Post,
            "PUT" => Token::Put,
            "DELETE" => Token::Delete,
            "PATCH" => Token::Patch,
            "Auth" => Token::Auth,
            "Role" => Token::Role,
            "Cache" => Token::Cache,
            "SEO" => Token::SEO,
            "AI" => Token::AI,
            _ => Token::Identifier(ident),
        }
    }
    
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        
        if let Some(ch) = self.current {
            let token = match ch {
                '@' => {
                    self.advance();
                    Token::At
                }
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    if let Some('>') = self.current {
                        self.advance();
                        Token::Arrow
                    } else {
                        Token::Minus
                    }
                }
                '*' => {
                    self.advance();
                    Token::Star
                }
                '/' => {
                    self.advance();
                    if let Some('/') | Some('*') = self.current {
                        self.skip_comment();
                        return self.next_token();
                    }
                    Token::Slash
                }
                '%' => {
                    self.advance();
                    Token::Percent
                }
                '=' => {
                    self.advance();
                    if let Some('=') = self.current {
                        self.advance();
                        Token::EqEq
                    } else if let Some('>') = self.current {
                        self.advance();
                        Token::FatArrow
                    } else {
                        Token::Eq
                    }
                }
                '!' => {
                    self.advance();
                    if let Some('=') = self.current {
                        self.advance();
                        Token::NotEq
                    } else {
                        Token::Not
                    }
                }
                '<' => {
                    self.advance();
                    if let Some('=') = self.current {
                        self.advance();
                        Token::LtEq
                    } else {
                        Token::Lt
                    }
                }
                '>' => {
                    self.advance();
                    if let Some('=') = self.current {
                        self.advance();
                        Token::GtEq
                    } else {
                        Token::Gt
                    }
                }
                '&' => {
                    self.advance();
                    if let Some('&') = self.current {
                        self.advance();
                        Token::And
                    } else {
                        Token::Unknown('&')
                    }
                }
                '|' => {
                    self.advance();
                    if let Some('|') = self.current {
                        self.advance();
                        Token::Or
                    } else {
                        Token::Unknown('|')
                    }
                }
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                '{' => {
                    self.advance();
                    Token::LBrace
                }
                '}' => {
                    self.advance();
                    Token::RBrace
                }
                '[' => {
                    self.advance();
                    Token::LBracket
                }
                ']' => {
                    self.advance();
                    Token::RBracket
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                ';' => {
                    self.advance();
                    Token::Semicolon
                }
                ':' => {
                    self.advance();
                    Token::Colon
                }
                '.' => {
                    self.advance();
                    Token::Dot
                }
                '\r' => {
                    self.advance();
                    // Check if next is \n (CRLF)
                    if let Some('\n') = self.current {
                        self.advance();
                    }
                    Token::Newline
                }
                '\n' => {
                    self.advance();
                    Token::Newline
                }
                '"' | '\'' => {
                    let string = self.read_string()?;
                    Token::String(string)
                }
                '0'..='9' => {
                    let num = self.read_number();
                    Token::Number(num)
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    return Ok(self.read_keyword_or_identifier());
                }
                _ => {
                    let unknown = ch;
                    self.advance();
                    Token::Unknown(unknown)
                }
            };
            
            Ok(token)
        } else {
            Ok(Token::EOF)
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            if token == Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("fn hello()");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0], Token::Fn);
        assert_eq!(tokens[1], Token::Identifier("hello".to_string()));
        assert_eq!(tokens[2], Token::LParen);
        assert_eq!(tokens[3], Token::RParen);
    }
    
    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("\"hello world\"");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0], Token::String("hello world".to_string()));
    }
    
    #[test]
    fn test_decorator() {
        let mut lexer = Lexer::new("@GET(\"/api/users\")");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0], Token::At);
        assert_eq!(tokens[1], Token::Get);
        assert_eq!(tokens[2], Token::LParen);
        assert_eq!(tokens[3], Token::String("/api/users".to_string()));
        assert_eq!(tokens[4], Token::RParen);
    }
    
    #[test]
    fn test_comment_at_start() {
        let mut lexer = Lexer::new("// Comment\nfn test() {}");
        let tokens = lexer.tokenize().unwrap();
        
        // After comment and newline, should get Fn token
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Fn);
    }
}