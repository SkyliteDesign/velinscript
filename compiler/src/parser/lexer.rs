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
    Throw,
    Break,
    Try,
    Catch,
    Finally,
    Type,
    Struct,
    Enum,
    Impl,
    Trait,
    Interface,
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
    FormatString(Vec<FormatStringPart>),
    Number(f64),
    Boolean(bool),
    Null,
    Identifier(String),

    // Operators
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Eq,      // =
    EqEq,    // ==
    NotEq,   // !=
    Lt,      // <
    Gt,      // >
    LtEq,    // <=
    GtEq,    // >=
    And,     // &&
    Or,      // ||
    Not,     // !

    // Punctuation
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    Comma,     // ,
    Semicolon, // ;
    Colon,     // :
    Dot,       // .
    Arrow,     // ->
    FatArrow,  // =>
    DotDot,    // ..
    DotDotEq,  // ..=

    // Special
    Newline,
    EOF,
    Unknown(char),
    DocComment(String),
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub input: Chars<'a>,
    pub current: Option<char>,
    pub position: usize,
    pub byte_position: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatStringPart {
    Text(String),
    Expression(String), // The expression inside {}
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
            byte_position: 0,
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
        if let Some(ch) = self.current {
            self.byte_position += ch.len_utf8();
        }
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

    pub fn skip_whitespace(&mut self) {
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

    fn skip_comment(&mut self) -> Option<Token> {
        // We already consumed the first '/' in next_token()
        // Now check for second '/' or '*'
        if let Some('/') = self.current {
            // Single-line comment: //
            self.advance(); // consume second '/'

            // Check for third '/' (Doc Comment)
            let is_doc_comment = if let Some('/') = self.current {
                self.advance();
                true
            } else {
                false
            };

            let mut comment_content = String::new();

            // Skip until newline (but don't consume the newline)
            while let Some(ch) = self.current {
                if ch == '\n' || ch == '\r' {
                    // Don't consume newline, let it be tokenized separately
                    break;
                }
                if is_doc_comment {
                    comment_content.push(ch);
                }
                self.advance();
            }

            if is_doc_comment {
                return Some(Token::DocComment(comment_content.trim().to_string()));
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
        None
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

    fn read_format_string(&mut self) -> Result<Vec<FormatStringPart>, LexerError> {
        let quote = self.current.unwrap();
        self.advance();
        let mut parts = Vec::new();
        let mut current_text = String::new();

        while let Some(ch) = self.current {
            match ch {
                '"' | '\'' if ch == quote => {
                    self.advance();
                    if !current_text.is_empty() {
                        parts.push(FormatStringPart::Text(current_text));
                    }
                    return Ok(parts);
                }
                '{' => {
                    // Start of expression
                    if !current_text.is_empty() {
                        parts.push(FormatStringPart::Text(current_text));
                        current_text = String::new();
                    }
                    self.advance(); // consume '{'

                    // Read expression until '}'
                    let mut expr = String::new();
                    let mut brace_count = 1;

                    while let Some(ch) = self.current {
                        match ch {
                            '{' => {
                                brace_count += 1;
                                expr.push(ch);
                                self.advance();
                            }
                            '}' => {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    self.advance(); // consume '}'
                                    parts.push(FormatStringPart::Expression(
                                        expr.trim().to_string(),
                                    ));
                                    break;
                                } else {
                                    expr.push(ch);
                                    self.advance();
                                }
                            }
                            '\\' => {
                                // Escape sequence
                                self.advance();
                                if let Some(escaped) = self.current {
                                    match escaped {
                                        'n' => expr.push('\n'),
                                        't' => expr.push('\t'),
                                        'r' => expr.push('\r'),
                                        '\\' => expr.push('\\'),
                                        '{' => expr.push('{'),
                                        '}' => expr.push('}'),
                                        '"' => expr.push('"'),
                                        '\'' => expr.push('\''),
                                        _ => {
                                            expr.push('\\');
                                            expr.push(escaped);
                                        }
                                    }
                                    self.advance();
                                }
                            }
                            _ => {
                                expr.push(ch);
                                self.advance();
                            }
                        }
                    }

                    if brace_count > 0 {
                        return Err(LexerError {
                            message: "Unterminated format string expression".to_string(),
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
                '\\' => {
                    self.advance();
                    if let Some(escaped) = self.current {
                        match escaped {
                            'n' => current_text.push('\n'),
                            't' => current_text.push('\t'),
                            'r' => current_text.push('\r'),
                            '\\' => current_text.push('\\'),
                            '"' => current_text.push('"'),
                            '\'' => current_text.push('\''),
                            '{' => current_text.push('{'),
                            '}' => current_text.push('}'),
                            _ => current_text.push(escaped),
                        }
                        self.advance();
                    }
                }
                _ => {
                    current_text.push(ch);
                    self.advance();
                }
            }
        }

        Err(LexerError {
            message: "Unterminated format string".to_string(),
            line: self.line,
            column: self.column,
        })
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        let mut has_dot = false;

        while let Some(ch) = self.current {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' {
                // Prüfe, ob das nächste Zeichen auch ein '.' ist (Range-Operator)
                let next_ch = self.input.clone().next();
                if next_ch == Some('.') {
                    // Zwei aufeinanderfolgende Punkte = Range-Operator, nicht Teil der Zahl
                    break;
                }
                // Ein einzelner Punkt ist Teil der Dezimalzahl
                if has_dot {
                    // Bereits ein Punkt vorhanden, nicht noch einen akzeptieren
                    break;
                }
                has_dot = true;
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
            "throw" => Token::Throw,
            "break" => Token::Break,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "finally" => Token::Finally,
            "type" => Token::Type,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "impl" => Token::Impl,
            "trait" => Token::Trait,
            "interface" => Token::Interface,
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
            "null" => Token::Null,
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

        #[cfg(debug_assertions)]
        {
            if let Some(ch) = self.current {
                if ch == '}' || ch == ';' {
                    eprintln!("🔍 LEXER next_token() - Aktuelles Zeichen: '{:?}' (0x{:02x}), Line: {}, Column: {}", 
                             ch, ch as u32, self.line, self.column);
                }
            }
        }

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
                        if let Some(token) = self.skip_comment() {
                            return Ok(token);
                        }
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
                    #[cfg(debug_assertions)]
                    {
                        eprintln!(
                            "🔍 LEXER: RBrace tokenisiert - Line: {}, Column: {}, Position: {}",
                            self.line, self.column, self.position
                        );
                    }
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
                    if let Some('.') = self.current {
                        self.advance();
                        if let Some('=') = self.current {
                            self.advance();
                            Token::DotDotEq
                        } else {
                            Token::DotDot
                        }
                    } else {
                        Token::Dot
                    }
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
                    // Check if this is a format string by peeking ahead
                    let quote = ch;
                    let mut peek_iter = self.input.clone();
                    let mut is_format_string = false;

                    // Skip the quote
                    peek_iter.next();

                    // Look for '{' before the closing quote
                    while let Some(peek_ch) = peek_iter.next() {
                        if peek_ch == quote {
                            break;
                        }
                        if peek_ch == '{' {
                            is_format_string = true;
                            break;
                        }
                        if peek_ch == '\\' {
                            // Skip escaped character
                            peek_iter.next();
                            continue;
                        }
                    }

                    if is_format_string {
                        let parts = self.read_format_string()?;
                        Token::FormatString(parts)
                    } else {
                        let string = self.read_string()?;
                        Token::String(string)
                    }
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
