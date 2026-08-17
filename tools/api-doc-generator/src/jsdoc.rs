// JSDoc Parser für VelinScript
// Parst JSDoc-ähnliche Kommentare aus VelinScript Code

use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields will be used for future OpenAPI integration
pub struct JSDocComment {
    pub description: String, // Used for future OpenAPI integration
    pub params: HashMap<String, String>, // Used for future OpenAPI integration
    pub returns: Option<String>, // Used for future OpenAPI integration
    pub throws: Option<String>, // Used for future OpenAPI integration
    pub example: Option<String>, // Used for future OpenAPI integration
    pub tags: HashMap<String, Vec<String>>, // Used for future OpenAPI integration
}

pub struct JSDocParser;

impl JSDocParser {
    /// Parst JSDoc-Kommentare aus Code
    pub fn parse(code: &str) -> HashMap<String, JSDocComment> {
        let mut docs = HashMap::new();
        let lines: Vec<&str> = code.lines().collect();
        
        let mut i = 0;
        while i < lines.len() {
            if lines[i].trim().starts_with("///") {
                if let Some((name, doc)) = Self::parse_comment_block(&lines, &mut i) {
                    docs.insert(name, doc);
                }
            }
            i += 1;
        }
        
        docs
    }
    
    fn parse_comment_block(lines: &[&str], start: &mut usize) -> Option<(String, JSDocComment)> {
        let mut comment_lines = Vec::new();
        let mut i = *start;
        
        // Sammle alle aufeinanderfolgenden /// Zeilen
        while i < lines.len() && lines[i].trim().starts_with("///") {
            let line = lines[i].trim_start_matches("///").trim();
            comment_lines.push(line);
            i += 1;
        }
        
        *start = i - 1;
        
        if comment_lines.is_empty() {
            return None;
        }
        
        // Finde die nächste Funktion/Struct nach dem Kommentar
        let mut function_name = None;
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("fn ") {
                // Extrahiere Funktionsname
                if let Some(name_start) = line.find("fn ") {
                    let rest = &line[name_start + 3..];
                    if let Some(name_end) = rest.find(|c: char| c == '(' || c == ' ') {
                        function_name = Some(rest[..name_end].trim().to_string());
                        break;
                    }
                }
            } else if line.starts_with("struct ") {
                if let Some(name_start) = line.find("struct ") {
                    let rest = &line[name_start + 7..];
                    if let Some(name_end) = rest.find(|c: char| c == '{' || c == ' ') {
                        function_name = Some(rest[..name_end].trim().to_string());
                        break;
                    }
                }
            }
            i += 1;
        }
        
        let name = function_name?;
        let doc = Self::parse_jsdoc_content(&comment_lines);
        
        Some((name, doc))
    }
    
    fn parse_jsdoc_content(lines: &[&str]) -> JSDocComment {
        let mut description = String::new();
        let mut params = HashMap::new();
        let mut returns = None;
        let mut throws = None;
        let mut example = None;
        let mut tags = HashMap::new();
        
        let mut in_description = true;
        
        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.starts_with("@param") {
                in_description = false;
                if let Some(param_info) = Self::parse_param(trimmed) {
                    params.insert(param_info.0, param_info.1);
                }
            } else if trimmed.starts_with("@returns") || trimmed.starts_with("@return") {
                in_description = false;
                returns = Some(trimmed.trim_start_matches("@returns").trim_start_matches("@return").trim().to_string());
            } else if trimmed.starts_with("@throws") {
                in_description = false;
                throws = Some(trimmed.trim_start_matches("@throws").trim().to_string());
            } else if trimmed.starts_with("@example") {
                in_description = false;
                example = Some(trimmed.trim_start_matches("@example").trim().to_string());
            } else if trimmed.starts_with("@") {
                in_description = false;
                // Andere Tags
                if let Some((tag_name, tag_value)) = Self::parse_tag(trimmed) {
                    tags.entry(tag_name).or_insert_with(Vec::new).push(tag_value);
                }
            } else if in_description && !trimmed.is_empty() {
                if !description.is_empty() {
                    description.push('\n');
                }
                description.push_str(trimmed);
            }
        }
        
        JSDocComment {
            description: description.trim().to_string(),
            params,
            returns,
            throws,
            example,
            tags,
        }
    }
    
    fn parse_param(line: &str) -> Option<(String, String)> {
        // @param name - description
        let rest = line.trim_start_matches("@param").trim();
        if let Some(dash_pos) = rest.find('-') {
            let name = rest[..dash_pos].trim().to_string();
            let desc = rest[dash_pos + 1..].trim().to_string();
            Some((name, desc))
        } else {
            None
        }
    }
    
    fn parse_tag(line: &str) -> Option<(String, String)> {
        // @tag value
        if let Some(space_pos) = line.find(' ') {
            let tag_name = line[1..space_pos].to_string();
            let tag_value = line[space_pos + 1..].trim().to_string();
            Some((tag_name, tag_value))
        } else {
            None
        }
    }
}
