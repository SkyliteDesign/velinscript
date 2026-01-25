use crate::autofix::report::AutoFixReport;
use crate::parser::lexer::{Lexer, Token};

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

        // 4. Common Parse Errors (NEU)
        if let Some((fixed, report)) = self.fix_common_parse_errors(code, &token_locations) {
            return (fixed, vec![report]);
        }

        (code.to_string(), vec![])
    }

    fn fix_unbalanced_braces(
        &self,
        code: &str,
        tokens: &[Token],
    ) -> Option<(String, AutoFixReport)> {
        let mut stack = Vec::new();

        for token in tokens {
            match token {
                Token::LBrace => stack.push("{"),
                Token::LBracket => stack.push("["),
                Token::LParen => stack.push("("),
                Token::RBrace => {
                    if let Some(last) = stack.last() {
                        if *last == "{" {
                            stack.pop();
                        }
                    }
                }
                Token::RBracket => {
                    if let Some(last) = stack.last() {
                        if *last == "[" {
                            stack.pop();
                        }
                    }
                }
                Token::RParen => {
                    if let Some(last) = stack.last() {
                        if *last == "(" {
                            stack.pop();
                        }
                    }
                }
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
                    }
                    "[" => {
                        closing_str.push_str("]");
                        original_missing.push_str("[");
                    }
                    "(" => {
                        closing_str.push_str(")");
                        original_missing.push_str("(");
                    }
                    _ => {}
                }
            }

            if !closing_str.is_empty() {
                let mut fixed_code = code.to_string();
                fixed_code.push_str(&closing_str);

                return Some((
                    fixed_code,
                    AutoFixReport::new(
                        &self.filename,
                        code.lines().count(),
                        0,
                        "UnbalancedBraces",
                        format!("Missing closing tokens for {}", original_missing),
                        format!("Appended {}", closing_str.replace("\n", "\\n")),
                    ),
                ));
            }
        }

        None
    }

    fn fix_function_signatures(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            if let Token::Fn = token {
                if i + 2 < locations.len() {
                    let is_func_start = matches!(locations[i + 1].2, Token::Identifier(_))
                        && matches!(locations[i + 2].2, Token::LParen);

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

                                        return Some((
                                            fixed_code,
                                            AutoFixReport::new(
                                                &self.filename,
                                                0,
                                                0,
                                                "FixFunctionSignature",
                                                "Missing ')' in function signature",
                                                "Inserted ')'",
                                            ),
                                        ));
                                    }
                                    return None;
                                }
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

    fn fix_generics(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            if let Token::Identifier(name) = token {
                if name == "List" || name == "Option" || name == "Result" {
                    if i + 1 < locations.len() && locations[i + 1].2 == Token::Lt {
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
                                        return Some((
                                            fixed_code,
                                            AutoFixReport::new(
                                                &self.filename,
                                                0,
                                                0,
                                                "FixGenericType",
                                                format!("Unclosed generic {}", name),
                                                "Inserted '>'",
                                            ),
                                        ));
                                    }
                                    return None;
                                }
                                _ => j += 1,
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Erkennt und behebt häufige Parse-Fehler auf Token-Ebene
    fn fix_common_parse_errors(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        // 1. Expected '=' (found: Semicolon) - in Struct-Definitionen
        if let Some(result) = self.fix_expected_equals_in_struct_definition(code, locations) {
            return Some(result);
        }

        // 1b. Expected '=' (found: Semicolon) - in let/const Statements
        if let Some(result) = self.fix_expected_equals_found_semicolon(code, locations) {
            return Some(result);
        }

        // 2. Expected ';' (found: ...)
        if let Some(result) = self.fix_expected_semicolon(code, locations) {
            return Some(result);
        }

        // 3. Expected ':' (found: ...)
        if let Some(result) = self.fix_expected_colon(code, locations) {
            return Some(result);
        }

        // 4. Expected ')' (found: ...)
        if let Some(result) = self.fix_expected_paren(code, locations) {
            return Some(result);
        }

        // 5. Expected '}' (found: ...)
        if let Some(result) = self.fix_expected_brace(code, locations) {
            return Some(result);
        }

        // 6. Expected '=>' (found: ...)
        if let Some(result) = self.fix_expected_fat_arrow(code, locations) {
            return Some(result);
        }

        // 7. Expected type (found: Number/String/Boolean) - in Struct-Literalen
        if let Some(result) = self.fix_expected_type_in_struct_literal(code, locations) {
            return Some(result);
        }

        // 7b. Expected type (found: Number/String/Boolean) - allgemein
        if let Some(result) = self.fix_expected_type(code, locations) {
            return Some(result);
        }

        // 8. Expected identifier (found: ...) - einfache Fälle zuerst
        if let Some(result) = self.fix_expected_identifier(code, locations) {
            return Some(result);
        }

        // 8b. Expected identifier (found: ...) - mit Levenshtein-Distance
        if let Some(result) = self.fix_expected_identifier_with_levenshtein(code, locations) {
            return Some(result);
        }

        // 9. Expected expression (found: ...)
        if let Some(result) = self.fix_expected_expression(code, locations) {
            return Some(result);
        }

        // 10. Keyword-Tippfehler
        if let Some(result) = self.fix_keyword_typos(code, locations) {
            return Some(result);
        }

        // 11. Fehlende Kommas in Struct-Literalen
        if let Some(result) = self.fix_missing_comma_in_struct(code, locations) {
            return Some(result);
        }

        // 12. Fehlende Doppelpunkte in Struct-Literalen
        if let Some(result) = self.fix_missing_colon_in_struct(code, locations) {
            return Some(result);
        }

        // 13. Operator-Verwechslungen (= vs ==)
        if let Some(result) = self.fix_operator_confusion(code, locations) {
            return Some(result);
        }

        // 14. Fehlende Parameter-Typen
        if let Some(result) = self.fix_missing_parameter_types(code, locations) {
            return Some(result);
        }

        // 15. Unbalancierte Strings/Kommentare
        if let Some(result) = self.fix_unbalanced_strings_comments(code, locations) {
            return Some(result);
        }

        None
    }

    /// Behebt "Expected '=' (found: Semicolon)" Fehler in Struct-Definitionen
    /// Kontext: Nach Struct-Feld (Identifier : Type), vor Semicolon
    /// Problem: Parser denkt, es ist ein `let` Statement, aber es ist ein Struct-Feld
    fn fix_expected_equals_in_struct_definition(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // Suche nach: Identifier + Colon + Type + Semicolon
            // Der Parser erwartet hier ein '=', aber es sollte ein Semicolon sein (oder Komma)
            if matches!(token, Token::Colon) {
                // Prüfe, ob wir in einer Struct-Definition sind
                let mut in_struct_definition = false;
                let mut j = i;

                // Suche nach 'struct' Keyword
                while j > 0 {
                    j -= 1;
                    if matches!(locations[j].2, Token::Struct) {
                        // Prüfe, ob nach 'struct' ein Identifier kommt
                        if j + 1 < locations.len() {
                            if let Token::Identifier(_) = locations[j + 1].2 {
                                // Prüfe, ob nach Identifier ein LBrace kommt
                                let mut k = j + 2;
                                while k < locations.len() && k < i {
                                    if matches!(locations[k].2, Token::LBrace) {
                                        in_struct_definition = true;
                                        break;
                                    }
                                    if matches!(
                                        locations[k].2,
                                        Token::Semicolon | Token::RBrace | Token::Fn | Token::Let
                                    ) {
                                        break;
                                    }
                                    k += 1;
                                }
                            }
                        }
                        break;
                    }
                    if matches!(
                        locations[j].2,
                        Token::Semicolon | Token::RBrace | Token::Fn | Token::Let | Token::Return
                    ) {
                        break;
                    }
                }

                // Wenn wir in einer Struct-Definition sind
                if in_struct_definition {
                    // Prüfe, ob nach Colon ein Type kommt (Identifier oder generischer Typ)
                    if i + 1 < locations.len() {
                        let mut k = i + 1;
                        let mut found_type = false;

                        // Skip Newlines
                        while k < locations.len() && matches!(locations[k].2, Token::Newline) {
                            k += 1;
                        }

                        // Prüfe auf Type (Identifier oder List<...>)
                        if k < locations.len() {
                            if let Token::Identifier(_) = locations[k].2 {
                                found_type = true;
                                // Prüfe auf generischen Typ (List<...>)
                                if k + 1 < locations.len()
                                    && matches!(locations[k + 1].2, Token::Lt)
                                {
                                    // Skip generische Parameter
                                    let mut l = k + 2;
                                    while l < locations.len() {
                                        if matches!(locations[l].2, Token::Gt) {
                                            k = l;
                                            break;
                                        }
                                        l += 1;
                                    }
                                }
                            }
                        }

                        // Prüfe, ob nach Type ein Semicolon kommt (ohne Komma dazwischen)
                        if found_type {
                            k += 1;
                            while k < locations.len() && matches!(locations[k].2, Token::Newline) {
                                k += 1;
                            }

                            if k < locations.len() {
                                if matches!(locations[k].2, Token::Semicolon) {
                                    // Das ist korrekt für ein Struct-Feld!
                                    // Der Parser-Bug: Er erwartet ein '=', aber es sollte ein Semicolon sein
                                    // Lösung: Der Code ist korrekt, aber der Parser muss gefixt werden
                                    // Für AutoFix: Entferne das Semicolon und füge ein Komma hinzu (falls es das letzte Feld ist)
                                    // Oder: Behalte das Semicolon, da es optional ist
                                    // Da der Code korrekt ist, machen wir nichts - der Parser muss gefixt werden
                                } else if matches!(locations[k].2, Token::Comma) {
                                    // Komma vorhanden - das ist korrekt
                                } else if matches!(locations[k].2, Token::RBrace) {
                                    // RBrace direkt nach Type - das ist auch korrekt (letztes Feld ohne Komma)
                                    // Der Parser-Bug: Er erwartet ein '=', aber es sollte ein RBrace sein
                                    // Workaround: Füge ein Komma vor RBrace hinzu, um den Parser zu beruhigen
                                    let rbrace_start = locations[k].0;
                                    if self.is_safe_to_fix(code, rbrace_start) {
                                        let mut fixed_code = code.to_string();
                                        fixed_code.insert(rbrace_start, ',');

                                        let line = code[..rbrace_start].lines().count();
                                        let column = rbrace_start.saturating_sub(
                                            code[..rbrace_start].rfind('\n').unwrap_or(0),
                                        );

                                        return Some((fixed_code, AutoFixReport::new(
                                            &self.filename,
                                            line,
                                            column,
                                            "FixExpectedEqualsInStruct",
                                            "Expected '=' (found: RBrace) in struct definition",
                                            "Added comma before closing brace to fix parser issue"
                                        )));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Behebt "Expected '=' (found: Semicolon)" Fehler
    /// Kontext: Nach `let`/`const` und Identifier, vor Semicolon
    fn fix_expected_equals_found_semicolon(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // Suche nach: let/const + Identifier + Semicolon (ohne = dazwischen)
            if matches!(token, Token::Let | Token::Const) {
                // Prüfe, ob nach let/const ein Identifier kommt
                if i + 1 < locations.len() {
                    if let Token::Identifier(_) = locations[i + 1].2 {
                        // Prüfe, ob nach Identifier direkt ein Semicolon kommt (ohne =)
                        let mut j = i + 2;
                        let mut found_equals = false;

                        // Skip optional type annotation (Identifier : Type)
                        while j < locations.len() {
                            match &locations[j].2 {
                                Token::Colon => {
                                    // Type annotation vorhanden, skip type
                                    j += 1;
                                    // Skip type tokens
                                    while j < locations.len() {
                                        match &locations[j].2 {
                                            Token::Identifier(_)
                                            | Token::Number(_)
                                            | Token::String(_)
                                            | Token::Lt
                                            | Token::Gt
                                            | Token::Comma
                                            | Token::Dot => {
                                                j += 1;
                                            }
                                            _ => break,
                                        }
                                    }
                                }
                                Token::Eq => {
                                    found_equals = true;
                                    break;
                                }
                                Token::Semicolon => {
                                    // Wir haben let/const + Identifier + Semicolon ohne =
                                    if !found_equals {
                                        let semicolon_start = locations[j].0;

                                        // Kontext-Validierung: Prüfe, ob wir in einem String oder Kommentar sind
                                        if self.is_safe_to_fix(code, semicolon_start) {
                                            let mut fixed_code = code.to_string();
                                            // Ersetze ';' durch ' = null;'
                                            fixed_code.replace_range(
                                                semicolon_start..semicolon_start + 1,
                                                " = null;",
                                            );

                                            let line = code[..semicolon_start].lines().count();
                                            let column = semicolon_start.saturating_sub(
                                                code[..semicolon_start].rfind('\n').unwrap_or(0),
                                            );

                                            return Some((
                                                fixed_code,
                                                AutoFixReport::new(
                                                    &self.filename,
                                                    line,
                                                    column,
                                                    "FixExpectedEquals",
                                                    "Expected '=' (found: Semicolon)",
                                                    "Replaced ';' with ' = null;'",
                                                ),
                                            ));
                                        }
                                    }
                                    break;
                                }
                                Token::Newline => {
                                    j += 1;
                                }
                                _ => {
                                    // Anderes Token gefunden, kein Match
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Behebt fehlende Semikolons nach Expression-Statements
    fn fix_expected_semicolon(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        // Fokussiere auf einfache Fälle: Identifier/Number/String + Newline/EOF ohne Semicolon
        for (i, (_, end, token)) in locations.iter().enumerate() {
            // Suche nach: Identifier/Number/String + Newline/EOF (ohne Semicolon dazwischen)
            if matches!(
                token,
                Token::Identifier(_)
                    | Token::Number(_)
                    | Token::String(_)
                    | Token::Boolean(_)
                    | Token::Return
            ) {
                let mut j = i + 1;
                let mut found_semicolon = false;
                let mut found_operator = false;

                // Skip whitespace/newlines und prüfe auf Operatoren
                while j < locations.len() {
                    match &locations[j].2 {
                        Token::Semicolon => {
                            found_semicolon = true;
                            break;
                        }
                        Token::Plus
                        | Token::Minus
                        | Token::Star
                        | Token::Slash
                        | Token::Percent
                        | Token::EqEq
                        | Token::NotEq
                        | Token::Lt
                        | Token::Gt
                        | Token::LtEq
                        | Token::GtEq
                        | Token::And
                        | Token::Or
                        | Token::Dot
                        | Token::LParen
                        | Token::LBracket => {
                            found_operator = true;
                            break;
                        }
                        Token::Newline | Token::EOF => {
                            if !found_semicolon && !found_operator {
                                // Möglicherweise fehlendes Semicolon
                                // Prüfe Kontext: Wenn nach Newline ein Keyword kommt, ist es wahrscheinlich ein Statement-Ende
                                if j + 1 < locations.len() {
                                    if matches!(
                                        &locations[j + 1].2,
                                        Token::Let
                                            | Token::Return
                                            | Token::If
                                            | Token::For
                                            | Token::While
                                            | Token::Fn
                                            | Token::Struct
                                            | Token::Enum
                                            | Token::Const
                                    ) {
                                        let insert_pos = *end;
                                        if self.is_safe_to_fix(code, insert_pos) {
                                            let mut fixed_code = code.to_string();
                                            fixed_code.insert(insert_pos, ';');

                                            let line = code[..insert_pos].lines().count();
                                            let column = insert_pos.saturating_sub(
                                                code[..insert_pos].rfind('\n').unwrap_or(0),
                                            );

                                            return Some((
                                                fixed_code,
                                                AutoFixReport::new(
                                                    &self.filename,
                                                    line,
                                                    column,
                                                    "FixExpectedSemicolon",
                                                    "Expected ';' after expression",
                                                    "Inserted ';'",
                                                ),
                                            ));
                                        }
                                    }
                                }
                                break;
                            }
                            break;
                        }
                        Token::LBrace => {
                            // Teil eines größeren Ausdrucks, kein Statement-Ende
                            break;
                        }
                        _ => {
                            j += 1;
                        }
                    }
                }
            }
        }
        None
    }

    /// Behebt fehlende Doppelpunkte in Funktions-Parametern
    /// Kontext: Nach Parameter-Name in Funktion, vor Typ
    fn fix_expected_colon(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // Suche nach: fn + Identifier + ( + Identifier + Token (ohne :)
            if matches!(token, Token::Fn) {
                if i + 3 < locations.len() {
                    // fn + name + (
                    if matches!(locations[i + 1].2, Token::Identifier(_))
                        && matches!(locations[i + 2].2, Token::LParen)
                    {
                        let mut j = i + 3;
                        while j < locations.len() {
                            match &locations[j].2 {
                                Token::RParen => break,
                                Token::Identifier(_) => {
                                    // Parameter-Name gefunden
                                    if j + 1 < locations.len() {
                                        match &locations[j + 1].2 {
                                            Token::Colon => {
                                                // Colon vorhanden, skip type und weiter
                                                j += 2;
                                                // Skip type tokens
                                                while j < locations.len() {
                                                    match &locations[j].2 {
                                                        Token::Comma
                                                        | Token::RParen
                                                        | Token::Eq => break,
                                                        _ => j += 1,
                                                    }
                                                }
                                            }
                                            Token::Identifier(_) => {
                                                // Fehlender Colon! Füge ihn ein
                                                let param_end = locations[j].1;
                                                let insert_pos = param_end;
                                                if self.is_safe_to_fix(code, insert_pos) {
                                                    let mut fixed_code = code.to_string();
                                                    fixed_code.insert(insert_pos, ':');

                                                    let line = code[..insert_pos].lines().count();
                                                    let column = insert_pos.saturating_sub(
                                                        code[..insert_pos].rfind('\n').unwrap_or(0),
                                                    );

                                                    return Some((
                                                        fixed_code,
                                                        AutoFixReport::new(
                                                            &self.filename,
                                                            line,
                                                            column,
                                                            "FixExpectedColon",
                                                            "Expected ':' after parameter name",
                                                            "Inserted ':'",
                                                        ),
                                                    ));
                                                }
                                                break;
                                            }
                                            Token::Comma | Token::RParen => {
                                                // Parameter ohne Typ - könnte auch fehlender Colon sein
                                                // Aber wir sind vorsichtig und fixen nur wenn eindeutig
                                                j += 1;
                                            }
                                            _ => {
                                                j += 1;
                                            }
                                        }
                                    } else {
                                        j += 1;
                                    }
                                }
                                Token::Comma => j += 1,
                                _ => j += 1,
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Behebt fehlende schließende Klammern
    fn fix_expected_paren(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        let mut paren_stack = Vec::new();

        for (i, (start, end, token)) in locations.iter().enumerate() {
            match token {
                Token::LParen => {
                    paren_stack.push((i, *start, *end));
                }
                Token::RParen => {
                    if paren_stack.pop().is_some() {
                        // Balanciert
                    } else {
                        // Zu viele schließende Klammern - entferne diese
                        if self.is_safe_to_fix(code, *start) {
                            let mut fixed_code = code.to_string();
                            fixed_code.replace_range(*start..*end, "");

                            let line = code[..*start].lines().count();
                            let column =
                                start.saturating_sub(code[..*start].rfind('\n').unwrap_or(0));

                            return Some((
                                fixed_code,
                                AutoFixReport::new(
                                    &self.filename,
                                    line,
                                    column,
                                    "FixExpectedParen",
                                    "Unexpected ')'",
                                    "Removed ')'",
                                ),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        // Prüfe auf fehlende schließende Klammern
        if let Some((_, open_start, open_end)) = paren_stack.last() {
            // Finde die Position nach der öffnenden Klammer
            let mut insert_pos = *open_end;

            // Suche nach dem nächsten passenden Token
            if let Some((next_start, _, next_token)) = locations.iter().find(|(_, _, t)| {
                matches!(
                    t,
                    Token::LBrace | Token::Semicolon | Token::Newline | Token::EOF | Token::Comma
                )
            }) {
                insert_pos = *next_start;
            } else {
                // Am Ende der Datei
                insert_pos = code.len();
            }

            if self.is_safe_to_fix(code, insert_pos) {
                let mut fixed_code = code.to_string();
                fixed_code.insert(insert_pos, ')');

                let line = code[..insert_pos].lines().count();
                let column = insert_pos.saturating_sub(code[..insert_pos].rfind('\n').unwrap_or(0));

                return Some((
                    fixed_code,
                    AutoFixReport::new(
                        &self.filename,
                        line,
                        column,
                        "FixExpectedParen",
                        "Expected ')'",
                        "Inserted ')'",
                    ),
                ));
            }
        }

        None
    }

    /// Behebt fehlende schließende geschweifte Klammern
    fn fix_expected_brace(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        let mut brace_stack = Vec::new();

        for (i, (start, end, token)) in locations.iter().enumerate() {
            match token {
                Token::LBrace => {
                    brace_stack.push((i, *start, *end));
                }
                Token::RBrace => {
                    if brace_stack.pop().is_some() {
                        // Balanciert
                    } else {
                        // Zu viele schließende Klammern - entferne diese
                        if self.is_safe_to_fix(code, *start) {
                            let mut fixed_code = code.to_string();
                            fixed_code.replace_range(*start..*end, "");

                            let line = code[..*start].lines().count();
                            let column =
                                start.saturating_sub(code[..*start].rfind('\n').unwrap_or(0));

                            return Some((
                                fixed_code,
                                AutoFixReport::new(
                                    &self.filename,
                                    line,
                                    column,
                                    "FixExpectedBrace",
                                    "Unexpected '}'",
                                    "Removed '}'",
                                ),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        // Prüfe auf fehlende schließende Klammern
        if let Some((_, _open_start, _)) = brace_stack.last() {
            // Füge schließende Klammer am Ende der Datei ein
            let insert_pos = code.len();

            if self.is_safe_to_fix(code, insert_pos) {
                let mut fixed_code = code.to_string();
                // Füge Newline hinzu, wenn nicht bereits vorhanden
                if !fixed_code.ends_with('\n') {
                    fixed_code.push('\n');
                }
                fixed_code.push('}');

                let line = code.lines().count() + 1;

                return Some((
                    fixed_code,
                    AutoFixReport::new(
                        &self.filename,
                        line,
                        0,
                        "FixExpectedBrace",
                        "Expected '}'",
                        "Inserted '}' at end of file",
                    ),
                ));
            }
        }

        None
    }

    /// Behebt fehlende '=>' in Match-Patterns
    fn fix_expected_fat_arrow(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // Suche nach: match + Expression + { + Pattern + Token (ohne =>)
            if matches!(token, Token::Match) {
                if i + 2 < locations.len() && matches!(locations[i + 2].2, Token::LBrace) {
                    // In einem Match-Block
                    let mut j = i + 3;
                    while j < locations.len() {
                        match &locations[j].2 {
                            Token::RBrace => break,
                            Token::FatArrow => {
                                // => vorhanden, skip expression nach =>
                                j += 1;
                                // Skip bis zum nächsten Pattern oder RBrace
                                while j < locations.len() {
                                    match &locations[j].2 {
                                        Token::Comma | Token::RBrace => break,
                                        _ => j += 1,
                                    }
                                }
                            }
                            Token::Comma => {
                                // Pattern-Ende ohne =>
                                if j > 0 {
                                    let prev_token = &locations[j - 1].2;
                                    if matches!(
                                        prev_token,
                                        Token::Identifier(_)
                                            | Token::Number(_)
                                            | Token::String(_)
                                            | Token::Boolean(_)
                                    ) {
                                        // Pattern gefunden, aber kein =>
                                        let insert_pos = locations[j].0;
                                        if self.is_safe_to_fix(code, insert_pos) {
                                            let mut fixed_code = code.to_string();
                                            fixed_code.insert_str(insert_pos, " => ");

                                            let line = code[..insert_pos].lines().count();
                                            let column = insert_pos.saturating_sub(
                                                code[..insert_pos].rfind('\n').unwrap_or(0),
                                            );

                                            return Some((
                                                fixed_code,
                                                AutoFixReport::new(
                                                    &self.filename,
                                                    line,
                                                    column,
                                                    "FixExpectedFatArrow",
                                                    "Expected '=>' in match pattern",
                                                    "Inserted '=>'",
                                                ),
                                            ));
                                        }
                                    }
                                }
                                j += 1;
                            }
                            _ => j += 1,
                        }
                    }
                }
            }
        }
        None
    }

    /// Prüft, ob eine Position sicher zum Fixen ist (nicht in String oder Kommentar)
    fn is_safe_to_fix(&self, code: &str, position: usize) -> bool {
        if position >= code.len() {
            return false;
        }

        // Einfache Prüfung: Stelle sicher, dass wir nicht in einem String-Literal sind
        // (Komplexere Prüfung würde einen vollständigen Parser erfordern)
        let before = &code[..position];
        let mut in_string = false;
        let mut escape_next = false;
        let mut quote_char = None;

        for ch in before.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                continue;
            }

            if ch == '"' || ch == '\'' {
                if quote_char.is_none() {
                    in_string = true;
                    quote_char = Some(ch);
                } else if quote_char == Some(ch) {
                    in_string = false;
                    quote_char = None;
                }
            }
        }

        !in_string
    }

    /// Behebt "Expected type (found: Number/String/Boolean)" Fehler in Struct-Literalen
    /// Kontext: In Struct-Literalen nach ':' wird ein Expression erwartet, nicht ein Typ
    /// Problem: Parser denkt, es ist eine Struct-Definition statt Struct-Literal
    /// Lösung: Prüfe Kontext - wenn nach Colon ein Expression kommt (nicht Typ), dann ist es ein Struct-Literal
    fn fix_expected_type_in_struct_literal(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        // Suche nach: Identifier + Colon + Expression in einem Struct-Kontext
        // Wenn der Parser "Expected type" sagt, aber nach Colon ein Expression kommt,
        // dann ist das ein Struct-Literal und der Parser-Bug muss umgangen werden
        for (i, (_, _, token)) in locations.iter().enumerate() {
            if matches!(token, Token::Colon) {
                // Prüfe, ob wir in einem Struct-Literal sind (nach LBrace)
                let mut in_struct_literal = false;
                let mut struct_start = 0;
                let mut j = i;

                // Suche nach LBrace
                while j > 0 {
                    j -= 1;
                    if matches!(locations[j].2, Token::LBrace) {
                        struct_start = j;
                        // Prüfe, ob vor LBrace ein Identifier kommt (Struct-Name)
                        if j > 0 {
                            if let Token::Identifier(_) = locations[j - 1].2 {
                                // Prüfe Kontext: Ist es nach return/let/assignment?
                                let mut k = j - 1;
                                while k > 0 {
                                    k -= 1;
                                    if matches!(
                                        locations[k].2,
                                        Token::Return
                                            | Token::Let
                                            | Token::Const
                                            | Token::Eq
                                            | Token::Colon
                                    ) {
                                        in_struct_literal = true;
                                        break;
                                    }
                                    if matches!(
                                        locations[k].2,
                                        Token::Semicolon
                                            | Token::LBrace
                                            | Token::LParen
                                            | Token::Fn
                                            | Token::Struct
                                    ) {
                                        break;
                                    }
                                }
                            }
                        }
                        break;
                    }
                    if matches!(locations[j].2, Token::RBrace | Token::Semicolon) {
                        break;
                    }
                }

                // Wenn wir in einem Struct-Literal sind und nach Colon ein Expression kommt
                if in_struct_literal && i + 1 < locations.len() {
                    match &locations[i + 1].2 {
                        Token::Number(n) => {
                            // Das ist korrekt für ein Struct-Literal!
                            // Der Parser-Bug: Er erwartet einen Typ, aber es ist ein Expression
                            // Workaround: Füge Klammern um den Wert hinzu, um den Parser zu beruhigen
                            let number_start = locations[i + 1].0;
                            let number_end = locations[i + 1].1;

                            if self.is_safe_to_fix(code, number_start) {
                                let mut fixed_code = code.to_string();
                                // Füge Klammern hinzu: (number) statt number
                                fixed_code.insert(number_start, '(');
                                fixed_code.insert(number_end + 1, ')');

                                let line = code[..number_start].lines().count();
                                let column = number_start
                                    .saturating_sub(code[..number_start].rfind('\n').unwrap_or(0));

                                return Some((
                                    fixed_code,
                                    AutoFixReport::new(
                                        &self.filename,
                                        line,
                                        column,
                                        "FixExpectedTypeInStructLiteral",
                                        "Expected type (found: Number) in struct literal",
                                        "Added parentheses around number to fix parser issue",
                                    ),
                                ));
                            }
                        }
                        Token::String(s) => {
                            // Das ist korrekt für ein Struct-Literal!
                            // Der Parser-Bug: Er erwartet einen Typ, aber es ist ein Expression
                            // Workaround: Füge Klammern um den String hinzu
                            let string_start = locations[i + 1].0;
                            let string_end = locations[i + 1].1;

                            if self.is_safe_to_fix(code, string_start) {
                                let mut fixed_code = code.to_string();
                                fixed_code.insert(string_start, '(');
                                fixed_code.insert(string_end + 1, ')');

                                let line = code[..string_start].lines().count();
                                let column = string_start
                                    .saturating_sub(code[..string_start].rfind('\n').unwrap_or(0));

                                return Some((
                                    fixed_code,
                                    AutoFixReport::new(
                                        &self.filename,
                                        line,
                                        column,
                                        "FixExpectedTypeInStructLiteral",
                                        "Expected type (found: String) in struct literal",
                                        "Added parentheses around string to fix parser issue",
                                    ),
                                ));
                            }
                        }
                        Token::Boolean(_) | Token::Identifier(_) => {
                            // Das ist korrekt für ein Struct-Literal!
                            // Der Parser-Bug: Er erwartet einen Typ, aber es ist ein Expression
                            // Workaround: Füge Klammern um den Wert hinzu
                            let value_start = locations[i + 1].0;
                            let value_end = locations[i + 1].1;

                            if self.is_safe_to_fix(code, value_start) {
                                let mut fixed_code = code.to_string();
                                fixed_code.insert(value_start, '(');
                                fixed_code.insert(value_end + 1, ')');

                                let line = code[..value_start].lines().count();
                                let column = value_start
                                    .saturating_sub(code[..value_start].rfind('\n').unwrap_or(0));

                                return Some((
                                    fixed_code,
                                    AutoFixReport::new(
                                        &self.filename,
                                        line,
                                        column,
                                        "FixExpectedTypeInStructLiteral",
                                        "Expected type (found: Expression) in struct literal",
                                        "Added parentheses around expression to fix parser issue",
                                    ),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Behebt "Expected type (found: Number/String/Boolean)" Fehler
    /// Kontext: Nach ':' in Typ-Annotationen, aber statt Typ wurde ein Literal gefunden
    fn fix_expected_type(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // Suche nach: Identifier + Colon + Number/String/Boolean (statt Typ)
            if matches!(token, Token::Colon) {
                // Prüfe, ob wir in einem Struct-Literal sind (dann ist das korrekt und kein Fix nötig)
                let mut in_struct_literal = false;
                let mut j = i;
                while j > 0 {
                    j -= 1;
                    if matches!(locations[j].2, Token::LBrace) {
                        // Prüfe, ob vor LBrace ein Identifier kommt (Struct-Name)
                        if j > 0 {
                            if let Token::Identifier(_) = locations[j - 1].2 {
                                // Prüfe, ob wir in einem return/let/assignment Kontext sind (Struct-Literal)
                                let mut k = j - 1;
                                while k > 0 {
                                    k -= 1;
                                    if matches!(
                                        locations[k].2,
                                        Token::Return
                                            | Token::Let
                                            | Token::Const
                                            | Token::Eq
                                            | Token::Colon
                                    ) {
                                        in_struct_literal = true;
                                        break;
                                    }
                                    if matches!(
                                        locations[k].2,
                                        Token::Semicolon | Token::LBrace | Token::LParen
                                    ) {
                                        break;
                                    }
                                }
                            }
                        }
                        break;
                    }
                    if matches!(locations[j].2, Token::RBrace | Token::Semicolon) {
                        break;
                    }
                }

                // Wenn wir in einem Struct-Literal sind, ist das kein Fehler - kein Fix nötig
                if in_struct_literal {
                    continue;
                }

                if i > 0 && i + 1 < locations.len() {
                    // Prüfe, ob nach Colon ein Literal kommt (statt Typ)
                    match &locations[i + 1].2 {
                        Token::Number(_) => {
                            // Zahl statt Typ - ersetze durch 'number'
                            let number_start = locations[i + 1].0;
                            let number_end = locations[i + 1].1;

                            if self.is_safe_to_fix(code, number_start) {
                                let mut fixed_code = code.to_string();
                                fixed_code.replace_range(number_start..number_end, "number");

                                let line = code[..number_start].lines().count();
                                let column = number_start
                                    .saturating_sub(code[..number_start].rfind('\n').unwrap_or(0));

                                return Some((
                                    fixed_code,
                                    AutoFixReport::new(
                                        &self.filename,
                                        line,
                                        column,
                                        "FixExpectedType",
                                        "Expected type (found: Number)",
                                        "Replaced number literal with 'number' type",
                                    ),
                                ));
                            }
                        }
                        Token::String(_) => {
                            // String statt Typ - ersetze durch 'string'
                            let string_start = locations[i + 1].0;
                            let string_end = locations[i + 1].1;

                            if self.is_safe_to_fix(code, string_start) {
                                let mut fixed_code = code.to_string();
                                fixed_code.replace_range(string_start..string_end, "string");

                                let line = code[..string_start].lines().count();
                                let column = string_start
                                    .saturating_sub(code[..string_start].rfind('\n').unwrap_or(0));

                                return Some((
                                    fixed_code,
                                    AutoFixReport::new(
                                        &self.filename,
                                        line,
                                        column,
                                        "FixExpectedType",
                                        "Expected type (found: String)",
                                        "Replaced string literal with 'string' type",
                                    ),
                                ));
                            }
                        }
                        Token::Boolean(_) => {
                            // Boolean statt Typ - ersetze durch 'boolean'
                            let bool_start = locations[i + 1].0;
                            let bool_end = locations[i + 1].1;

                            if self.is_safe_to_fix(code, bool_start) {
                                let mut fixed_code = code.to_string();
                                fixed_code.replace_range(bool_start..bool_end, "boolean");

                                let line = code[..bool_start].lines().count();
                                let column = bool_start
                                    .saturating_sub(code[..bool_start].rfind('\n').unwrap_or(0));

                                return Some((
                                    fixed_code,
                                    AutoFixReport::new(
                                        &self.filename,
                                        line,
                                        column,
                                        "FixExpectedType",
                                        "Expected type (found: Boolean)",
                                        "Replaced boolean literal with 'boolean' type",
                                    ),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Behebt "Expected identifier" Fehler
    /// Kontext: Nach Keywords oder Operatoren, wo ein Identifier erwartet wird
    fn fix_expected_identifier(
        &self,
        _code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        // Suche nach häufigen Mustern: Keyword + falsches Token
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // let/const + falsches Token
            if matches!(token, Token::Let | Token::Const) {
                if i + 1 < locations.len() {
                    match &locations[i + 1].2 {
                        Token::Number(_) | Token::String(_) | Token::Boolean(_) => {
                            // Fehlender Identifier - könnte ein Tippfehler sein
                            // Aber wir können nicht sicher sein, was gemeint war
                            // Daher lassen wir diesen Fall erstmal aus
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Behebt "Expected expression" Fehler
    /// Kontext: Nach Operatoren oder Keywords, wo ein Expression erwartet wird
    fn fix_expected_expression(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        // Suche nach häufigen Mustern: Operator + falsches Token
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // return + falsches Token
            if matches!(token, Token::Return) {
                if i + 1 < locations.len() {
                    match &locations[i + 1].2 {
                        Token::Semicolon | Token::Newline | Token::EOF => {
                            // return ohne Expression - füge null ein
                            let insert_pos = locations[i + 1].0;
                            if self.is_safe_to_fix(code, insert_pos) {
                                let mut fixed_code = code.to_string();
                                fixed_code.insert_str(insert_pos, " null");

                                let line = code[..insert_pos].lines().count();
                                let column = insert_pos
                                    .saturating_sub(code[..insert_pos].rfind('\n').unwrap_or(0));

                                return Some((
                                    fixed_code,
                                    AutoFixReport::new(
                                        &self.filename,
                                        line,
                                        column,
                                        "FixExpectedExpression",
                                        "Expected expression after 'return'",
                                        "Inserted 'null'",
                                    ),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Behebt Keyword-Tippfehler (funtion → fn, retrun → return, etc.)
    fn fix_keyword_typos(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        let keyword_corrections: &[(&str, &str)] = &[
            ("funtion", "fn"),
            ("function", "fn"),
            ("retrun", "return"),
            ("retun", "return"),
            ("iff", "if"),
            ("els", "else"),
            ("elsif", "else if"),
            ("whil", "while"),
            ("whlie", "while"),
            ("forr", "for"),
            ("lett", "let"),
            ("constt", "const"),
            ("tru", "true"),
            ("fals", "false"),
            ("nul", "null"),
            ("nulll", "null"),
        ];

        for (_i, (start, end, token)) in locations.iter().enumerate() {
            if let Token::Identifier(name) = token {
                // Prüfe auf Keyword-Tippfehler
                for (typo, correct) in keyword_corrections {
                    if name.as_str() == *typo {
                        if self.is_safe_to_fix(code, *start) {
                            let mut fixed_code = code.to_string();
                            fixed_code.replace_range(*start..*end, correct);

                            let line = code[..*start].lines().count();
                            let column =
                                start.saturating_sub(code[..*start].rfind('\n').unwrap_or(0));

                            return Some((
                                fixed_code,
                                AutoFixReport::new(
                                    &self.filename,
                                    line,
                                    column,
                                    "FixKeywordTypo",
                                    format!("Typo in keyword: '{}'", name),
                                    format!("Corrected to '{}'", correct),
                                ),
                            ));
                        }
                    }
                }
            }
        }
        None
    }

    /// Behebt fehlende Kommas in Struct-Literalen
    /// Beispiel: Struct { a: 1 b: 2 } → Struct { a: 1, b: 2 }
    fn fix_missing_comma_in_struct(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // Suche nach: LBrace + ... + Identifier + Colon + Expression + Identifier (ohne Comma)
            if matches!(token, Token::LBrace) {
                let mut j = i + 1;
                while j < locations.len() {
                    match &locations[j].2 {
                        Token::RBrace => break,
                        Token::Identifier(_) => {
                            // Möglicherweise ein Feld-Name
                            if j + 2 < locations.len() {
                                // Identifier + Colon + Expression
                                if matches!(locations[j + 1].2, Token::Colon) {
                                    // Suche nach dem Ende des Expressions
                                    let mut expr_end = j + 2;
                                    while expr_end < locations.len() {
                                        match &locations[expr_end].2 {
                                            Token::Comma | Token::RBrace => break,
                                            Token::Identifier(_) => {
                                                // Nächstes Feld ohne Comma!
                                                if locations[expr_end].2 != Token::RBrace {
                                                    let insert_pos = locations[expr_end].0;
                                                    if self.is_safe_to_fix(code, insert_pos) {
                                                        let mut fixed_code = code.to_string();
                                                        fixed_code.insert(insert_pos, ',');

                                                        let line =
                                                            code[..insert_pos].lines().count();
                                                        let column = insert_pos.saturating_sub(
                                                            code[..insert_pos]
                                                                .rfind('\n')
                                                                .unwrap_or(0),
                                                        );

                                                        return Some((
                                                            fixed_code,
                                                            AutoFixReport::new(
                                                                &self.filename,
                                                                line,
                                                                column,
                                                                "FixMissingCommaInStruct",
                                                                "Missing comma in struct literal",
                                                                "Inserted ','",
                                                            ),
                                                        ));
                                                    }
                                                }
                                                break;
                                            }
                                            _ => expr_end += 1,
                                        }
                                    }
                                }
                            }
                            j += 1;
                        }
                        _ => j += 1,
                    }
                }
            }
        }
        None
    }

    /// Behebt fehlende Doppelpunkte in Struct-Literalen
    /// Beispiel: Struct { a 1, b 2 } → Struct { a: 1, b: 2 }
    fn fix_missing_colon_in_struct(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            if matches!(token, Token::LBrace) {
                let mut j = i + 1;
                while j < locations.len() {
                    match &locations[j].2 {
                        Token::RBrace => break,
                        Token::Identifier(_) => {
                            // Feld-Name gefunden
                            if j + 1 < locations.len() {
                                match &locations[j + 1].2 {
                                    Token::Colon => {
                                        // Colon vorhanden, skip value und weiter
                                        j += 2;
                                        // Skip value tokens
                                        while j < locations.len() {
                                            match &locations[j].2 {
                                                Token::Comma | Token::RBrace => break,
                                                _ => j += 1,
                                            }
                                        }
                                    }
                                    Token::Number(_)
                                    | Token::String(_)
                                    | Token::Boolean(_)
                                    | Token::Identifier(_) => {
                                        // Fehlender Colon! Füge ihn ein
                                        let field_end = locations[j].1;
                                        let insert_pos = field_end;
                                        if self.is_safe_to_fix(code, insert_pos) {
                                            let mut fixed_code = code.to_string();
                                            fixed_code.insert(insert_pos, ':');

                                            let line = code[..insert_pos].lines().count();
                                            let column = insert_pos.saturating_sub(
                                                code[..insert_pos].rfind('\n').unwrap_or(0),
                                            );

                                            return Some((
                                                fixed_code,
                                                AutoFixReport::new(
                                                    &self.filename,
                                                    line,
                                                    column,
                                                    "FixMissingColonInStruct",
                                                    "Missing ':' in struct literal",
                                                    "Inserted ':'",
                                                ),
                                            ));
                                        }
                                        j += 1;
                                    }
                                    _ => j += 1,
                                }
                            } else {
                                j += 1;
                            }
                        }
                        _ => j += 1,
                    }
                }
            }
        }
        None
    }

    /// Behebt Operator-Verwechslungen (= vs ==)
    /// Beispiel: if (x = y) → if (x == y)
    fn fix_operator_confusion(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            // Suche nach: if/while + LParen + ... + Eq (statt EqEq)
            if matches!(token, Token::If | Token::While) {
                if i + 1 < locations.len() && matches!(locations[i + 1].2, Token::LParen) {
                    // In einer Bedingung
                    let mut j = i + 2;
                    while j < locations.len() {
                        match &locations[j].2 {
                            Token::RParen => break,
                            Token::Eq => {
                                // = in Bedingung - sollte wahrscheinlich == sein
                                let eq_start = locations[j].0;
                                let eq_end = locations[j].1;
                                if self.is_safe_to_fix(code, eq_start) {
                                    let mut fixed_code = code.to_string();
                                    // Ersetze = durch ==
                                    fixed_code.replace_range(eq_start..eq_end, "==");

                                    let line = code[..eq_start].lines().count();
                                    let column = eq_start
                                        .saturating_sub(code[..eq_start].rfind('\n').unwrap_or(0));

                                    return Some((
                                        fixed_code,
                                        AutoFixReport::new(
                                            &self.filename,
                                            line,
                                            column,
                                            "FixOperatorConfusion",
                                            "Assignment '=' in condition (should be '==')",
                                            "Replaced '=' with '=='",
                                        ),
                                    ));
                                }
                                break;
                            }
                            _ => j += 1,
                        }
                    }
                }
            }
        }
        None
    }

    /// Behebt fehlende Parameter-Typen
    /// Beispiel: fn test(x, y) → fn test(x: any, y: any)
    fn fix_missing_parameter_types(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        for (i, (_, _, token)) in locations.iter().enumerate() {
            if matches!(token, Token::Fn) {
                if i + 2 < locations.len() && matches!(locations[i + 2].2, Token::LParen) {
                    let mut j = i + 3;
                    while j < locations.len() {
                        match &locations[j].2 {
                            Token::RParen => break,
                            Token::Identifier(_) => {
                                // Parameter-Name gefunden
                                if j + 1 < locations.len() {
                                    match &locations[j + 1].2 {
                                        Token::Colon => {
                                            // Typ vorhanden, skip
                                            j += 2;
                                            // Skip type tokens
                                            while j < locations.len() {
                                                match &locations[j].2 {
                                                    Token::Comma | Token::RParen => break,
                                                    _ => j += 1,
                                                }
                                            }
                                        }
                                        Token::Comma | Token::RParen => {
                                            // Fehlender Typ! Füge ': any' ein
                                            let param_end = locations[j].1;
                                            let insert_pos = param_end;
                                            if self.is_safe_to_fix(code, insert_pos) {
                                                let mut fixed_code = code.to_string();
                                                fixed_code.insert_str(insert_pos, ": any");

                                                let line = code[..insert_pos].lines().count();
                                                let column = insert_pos.saturating_sub(
                                                    code[..insert_pos].rfind('\n').unwrap_or(0),
                                                );

                                                return Some((
                                                    fixed_code,
                                                    AutoFixReport::new(
                                                        &self.filename,
                                                        line,
                                                        column,
                                                        "FixMissingParameterType",
                                                        "Missing parameter type",
                                                        "Inserted ': any'",
                                                    ),
                                                ));
                                            }
                                            j += 1;
                                        }
                                        _ => j += 1,
                                    }
                                } else {
                                    j += 1;
                                }
                            }
                            _ => j += 1,
                        }
                    }
                }
            }
        }
        None
    }

    /// Behebt unbalancierte Strings und Kommentare
    fn fix_unbalanced_strings_comments(
        &self,
        code: &str,
        _locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        let mut in_string = false;
        let mut quote_char = None;
        let mut escape_next = false;

        for (i, ch) in code.char_indices() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                continue;
            }

            if ch == '"' || ch == '\'' {
                if !in_string {
                    in_string = true;
                    quote_char = Some(ch);
                } else if quote_char == Some(ch) {
                    in_string = false;
                    quote_char = None;
                }
            }
        }

        // Prüfe auf unbalancierte Strings
        if in_string {
            // Füge schließendes Anführungszeichen am Ende der Datei ein
            let insert_pos = code.len();
            if self.is_safe_to_fix(code, insert_pos) {
                let mut fixed_code = code.to_string();
                if let Some(quote) = quote_char {
                    fixed_code.push(quote);
                } else {
                    fixed_code.push('"');
                }

                let line = code.lines().count();

                return Some((
                    fixed_code,
                    AutoFixReport::new(
                        &self.filename,
                        line,
                        0,
                        "FixUnbalancedString",
                        "Unbalanced string literal",
                        "Inserted closing quote",
                    ),
                ));
            }
        }

        // Prüfe auf unbalancierte Block-Kommentare
        let mut comment_depth = 0;
        let mut in_line_comment = false;

        for (i, ch) in code.char_indices() {
            if i + 1 < code.len() {
                let next_ch = code.chars().nth(i + 1).unwrap_or('\0');

                if !in_line_comment && ch == '/' && next_ch == '*' {
                    comment_depth += 1;
                } else if comment_depth > 0 && ch == '*' && next_ch == '/' {
                    comment_depth -= 1;
                } else if ch == '/' && next_ch == '/' {
                    in_line_comment = true;
                } else if ch == '\n' {
                    in_line_comment = false;
                }
            }
        }

        if comment_depth > 0 {
            // Füge schließenden Kommentar ein
            let insert_pos = code.len();
            if self.is_safe_to_fix(code, insert_pos) {
                let mut fixed_code = code.to_string();
                fixed_code.push_str(" */");

                let line = code.lines().count();

                return Some((
                    fixed_code,
                    AutoFixReport::new(
                        &self.filename,
                        line,
                        0,
                        "FixUnbalancedComment",
                        "Unbalanced block comment",
                        "Inserted '*/'",
                    ),
                ));
            }
        }

        None
    }

    /// Erweiterte fix_expected_identifier mit Levenshtein-Distance
    fn fix_expected_identifier_with_levenshtein(
        &self,
        code: &str,
        locations: &[(usize, usize, Token)],
    ) -> Option<(String, AutoFixReport)> {
        // Liste bekannter Keywords und Typen
        let known_identifiers = &[
            "fn", "let", "const", "return", "if", "else", "while", "for", "match", "string",
            "number", "boolean", "any", "void", "null", "List", "Map", "Option", "Result",
        ];

        for (i, (start, end, token)) in locations.iter().enumerate() {
            if let Token::Identifier(name) = token {
                // Prüfe, ob der Identifier einem bekannten ähnlich ist
                for known in known_identifiers {
                    let distance = self.levenshtein_distance(name.as_str(), known);
                    // Wenn die Distanz klein ist (max 2 Zeichen Unterschied) und der Identifier nicht zu kurz
                    if distance <= 2 && name.len() >= 2 && distance < name.len() / 2 {
                        // Prüfe Kontext: Ist es ein Keyword-Kontext?
                        let is_keyword_context = i > 0
                            && matches!(
                                locations[i - 1].2,
                                Token::Let
                                    | Token::Const
                                    | Token::Fn
                                    | Token::Return
                                    | Token::If
                                    | Token::While
                                    | Token::For
                            );

                        if is_keyword_context
                            || *known == "fn"
                            || *known == "return"
                            || *known == "if"
                        {
                            if self.is_safe_to_fix(code, *start) {
                                let mut fixed_code = code.to_string();
                                fixed_code.replace_range(*start..*end, known);

                                let line = code[..*start].lines().count();
                                let column =
                                    start.saturating_sub(code[..*start].rfind('\n').unwrap_or(0));

                                return Some((
                                    fixed_code,
                                    AutoFixReport::new(
                                        &self.filename,
                                        line,
                                        column,
                                        "FixIdentifierTypo",
                                        format!("Typo in identifier: '{}'", name),
                                        format!("Corrected to '{}'", known),
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Berechnet Levenshtein-Distance zwischen zwei Strings
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s1_len = s1_chars.len();
        let s2_len = s2_chars.len();

        if s1_len == 0 {
            return s2_len;
        }
        if s2_len == 0 {
            return s1_len;
        }

        let mut matrix = vec![vec![0; s2_len + 1]; s1_len + 1];

        for i in 0..=s1_len {
            matrix[i][0] = i;
        }
        for j in 0..=s2_len {
            matrix[0][j] = j;
        }

        for i in 1..=s1_len {
            for j in 1..=s2_len {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                    0
                } else {
                    1
                };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[s1_len][s2_len]
    }
}
