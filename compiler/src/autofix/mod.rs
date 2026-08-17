use crate::parser::lexer::{Lexer, Token};
use crate::autofix::report::AutoFixReport;

pub mod report;

pub struct AutoFixer {
    filename: String,
}

pub struct AutoFixResult {
    pub code: String,
    pub reports: Vec<AutoFixReport>,
    pub fixed: bool,
}

impl AutoFixer {
    pub fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
        }
    }

    pub fn fix(&self, code: &str) -> AutoFixResult {
        let mut current_code = code.to_string();
        let mut all_reports = Vec::new();
        let mut iteration = 0;
        let max_iterations = 5;

        loop {
            iteration += 1;
            if iteration > max_iterations {
                break;
            }

            let (new_code, reports) = self.run_pass(&current_code);
            
            if reports.is_empty() {
                break;
            }

            current_code = new_code;
            all_reports.extend(reports);
        }

        let fixed = !all_reports.is_empty();

        AutoFixResult {
            code: current_code,
            reports: all_reports,
            fixed,
        }
    }

    fn run_pass(&self, code: &str) -> (String, Vec<AutoFixReport>) {
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        let mut token_locations = Vec::new();

        loop {
            lexer.skip_whitespace();
            let start = lexer.byte_position;
            
            match lexer.next_token() {
                Ok(token) => {
                    if token == Token::EOF {
                        break;
                    }
                    let end = lexer.byte_position;
                    tokens.push(token.clone());
                    token_locations.push((start, end, token));
                }
                Err(_) => break,
            }
        }

        // 1. Unbalanced Braces (Append at EOF)
        if let Some((fixed, report)) = self.fix_unbalanced_braces(code, &tokens) {
            return (fixed, vec![report]);
        }
        
        // 2. Function Signatures (Missing Paren)
        if let Some((fixed, report)) = self.fix_function_signatures(code, &token_locations) {
            return (fixed, vec![report]);
        }
        
        // 3. List Generic (List<T -> List<T>)
        if let Some((fixed, report)) = self.fix_generics(code, &token_locations) {
            return (fixed, vec![report]);
        }

        (code.to_string(), vec![])
    }

    fn fix_unbalanced_braces(&self, code: &str, tokens: &[Token]) -> Option<(String, AutoFixReport)> {
        let mut stack = Vec::new();
        
        for token in tokens {
            match token {
                Token::LBrace => stack.push("{"),
                Token::LBracket => stack.push("["),
                Token::LParen => stack.push("("),
                Token::RBrace => {
                    if let Some(last) = stack.last() {
                        if *last == "{" { stack.pop(); }
                    }
                },
                Token::RBracket => {
                    if let Some(last) = stack.last() {
                        if *last == "[" { stack.pop(); }
                    }
                },
                Token::RParen => {
                    if let Some(last) = stack.last() {
                        if *last == "(" { stack.pop(); }
                    }
                },
                _ => {}
            }
        }

        if !stack.is_empty() {
            let mut closing_str = String::new();
            let mut original_missing = String::new();
            
            while let Some(open) = stack.pop() {
                match open {
                    "{" => {
                        closing_str.push_str("\n}");
                        original_missing.push_str("{");
                    },
                    "[" => {
                         closing_str.push_str("]");
                         original_missing.push_str("[");
                    },
                    "(" => {
                        closing_str.push_str(")");
                        original_missing.push_str("(");
                    },
                    _ => {}
                }
            }

            if !closing_str.is_empty() {
                let mut fixed_code = code.to_string();
                fixed_code.push_str(&closing_str);
                
                return Some((fixed_code, AutoFixReport::new(
                    &self.filename,
                    code.lines().count(),
                    0,
                    "UnbalancedBraces",
                    format!("Missing closing tokens for {}", original_missing),
                    format!("Appended {}", closing_str.replace("\n", "\\n")),
                )));
            }
        }
        
        None
    }

    fn fix_function_signatures(&self, code: &str, locations: &[(usize, usize, Token)]) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            if let Token::Fn = token {
                if i + 2 < locations.len() {
                    let is_func_start = matches!(locations[i+1].2, Token::Identifier(_)) && 
                                      matches!(locations[i+2].2, Token::LParen);
                    
                    if is_func_start {
                        let mut j = i + 3;
                        
                        while j < locations.len() {
                            let (start, _, tok) = &locations[j];
                            match tok {
                                Token::RParen => break,
                                Token::LBrace | Token::Arrow => {
                                    let offset = *start;
                                    if offset <= code.len() {
                                        let mut fixed_code = code.to_string();
                                        fixed_code.insert(offset, ')'); 
                                        fixed_code.insert(offset + 1, ' ');
                                        
                                        return Some((fixed_code, AutoFixReport::new(
                                            &self.filename,
                                            0, 
                                            0,
                                            "FixFunctionSignature",
                                            "Missing ')' in function signature",
                                            "Inserted ')'"
                                        )));
                                    }
                                    return None;
                                },
                                Token::Fn => break,
                                _ => j += 1,
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn fix_generics(&self, code: &str, locations: &[(usize, usize, Token)]) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            if let Token::Identifier(name) = token {
                if name == "List" || name == "Option" || name == "Result" {
                     if i + 1 < locations.len() && locations[i+1].2 == Token::Lt {
                         let mut j = i + 2;
                         while j < locations.len() {
                             let (start, _, tok) = &locations[j];
                             match tok {
                                 Token::Gt => break,
                                 Token::Semicolon | Token::Eq | Token::LBrace | Token::RParen => {
                                     let offset = *start;
                                     if offset <= code.len() {
                                         let mut fixed_code = code.to_string();
                                         fixed_code.insert(offset, '>');
                                         return Some((fixed_code, AutoFixReport::new(
                                             &self.filename,
                                             0,
                                             0,
                                             "FixGenericType",
                                             format!("Unclosed generic {}", name),
                                             "Inserted '>'"
                                         )));
                                     }
                                     return None;
                                 },
                                 _ => j += 1,
                             }
                         }
                     }
                }
            }
        }
        None
    }
}
