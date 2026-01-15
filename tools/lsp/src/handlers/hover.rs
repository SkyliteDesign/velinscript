// Hover Handler

use tower_lsp::lsp_types::*;
use velin_compiler::parser::ast::*;

pub fn get_hover(program: &Program, _position: Position, word: &str) -> Option<Hover> {
    // Suche nach Funktionen
    for item in &program.items {
        if let Item::Function(f) = item {
            if f.name == word {
                let params_str = f.params
                    .iter()
                    .map(|p| format!("{}: {}", p.name, p.param_type.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                let return_type_str = f.return_type
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "void".to_string());
                
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "```velin\nfn {}({}) -> {}\n```\n\nFunction: {}",
                            f.name, params_str, return_type_str, f.name
                        ),
                    }),
                    range: None,
                });
            }
        }
    }
    
    // Suche nach Structs
    for item in &program.items {
        if let Item::Struct(s) = item {
            if s.name == word {
                let fields_str = s.fields
                    .iter()
                    .map(|f| format!("{}: {}", f.name, f.field_type.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "```velin\nstruct {} {{\n  {}\n}}\n```\n\nStruct: {}",
                            s.name,
                            fields_str.replace(", ", ",\n  "),
                            s.name
                        ),
                    }),
                    range: None,
                });
            }
        }
    }
    
    // Suche nach Enums
    for item in &program.items {
        if let Item::Enum(e) = item {
            if e.name == word {
                let variants_str = e.variants
                    .iter()
                    .map(|v| v.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "```velin\nenum {} {{\n  {}\n}}\n```\n\nEnum: {}",
                            e.name,
                            variants_str.replace(", ", ",\n  "),
                            e.name
                        ),
                    }),
                    range: None,
                });
            }
        }
    }
    
    // Suche nach Traits
    for item in &program.items {
        if let Item::Trait(t) = item {
            if t.name == word {
                let methods_str = t.methods
                    .iter()
                    .map(|m| {
                        let params_str = m.params
                            .iter()
                            .map(|p| format!("{}: {}", p.name, p.param_type.to_string()))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let return_str = m.return_type
                            .as_ref()
                            .map(|t| format!(" -> {}", t.to_string()))
                            .unwrap_or_else(|| "".to_string());
                        format!("  {}({}){}", m.name, params_str, return_str)
                    })
                    .collect::<Vec<_>>()
                    .join(";\n");
                
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "```velin\ntrait {} {{\n{}\n}}\n```\n\nTrait: {}",
                            t.name,
                            methods_str,
                            t.name
                        ),
                    }),
                    range: None,
                });
            }
        }
    }
    
    // Standard Library Info
    let stdlib_info = match word {
        "db.find" => Some("Find entity by ID\n\n```velin\ndb.find<T>(id: string) -> Optional<T>\n```"),
        "db.findAll" => Some("Find all entities\n\n```velin\ndb.findAll<T>() -> List<T>\n```"),
        "db.save" => Some("Save entity\n\n```velin\ndb.save<T>(entity: T) -> T\n```"),
        "db.delete" => Some("Delete entity\n\n```velin\ndb.delete<T>(id: string) -> boolean\n```"),
        "assert" => Some("Assert condition\n\n```velin\nassert(condition: boolean) -> void\n```"),
        // Response Functions
        "successResponse" => Some("Creates a successful ApiResponse<T>\n\n```velin\nsuccessResponse<T>(data: T, requestId: string, startTime: number) -> ApiResponse<T>\n```"),
        "errorResponse" => Some("Creates an error ApiResponse<T>\n\n```velin\nerrorResponse<T>(errorCode: string, message: string, requestId: string, details: Map<string, string>) -> ApiResponse<T>\n```"),
        // Error Functions
        "createError" => Some("Creates an AppError\n\n```velin\ncreateError(code: ApiErrorCode, message: string, details: Map<string, string>, cause: string) -> AppError\n```"),
        "createValidationError" => Some("Creates a validation error\n\n```velin\ncreateValidationError(field: string, message: string) -> AppError\n```"),
        // Logging Functions
        "logRequest" => Some("Logs HTTP request\n\n```velin\nlogRequest(request: HttpRequest, endpoint: string, requestId: string) -> void\n```"),
        "logResponse" => Some("Logs HTTP response\n\n```velin\nlogResponse(response: HttpResponse, duration: number, requestId: string) -> void\n```"),
        "logError" => Some("Logs error with AppError\n\n```velin\nlogError(message: string, requestId: string, error: AppError, metadata: Map<string, string>) -> void\n```"),
        // Cache Functions
        "cacheGet" => Some("Gets value from cache\n\n```velin\ncacheGet<T>(key: string) -> T\n```"),
        "cacheSet" => Some("Sets value in cache\n\n```velin\ncacheSet<T>(key: string, value: T, ttl: number) -> void\n```"),
        // Security Functions
        "applySecurityMiddleware" => Some("Applies security middleware\n\n```velin\napplySecurityMiddleware(request: HttpRequest, endpoint: string) -> ApiResponse<void>\n```"),
        "sanitizeInput" => Some("Sanitizes user input\n\n```velin\nsanitizeInput(input: string) -> string\n```"),
        _ => None,
    };
    
    if let Some(info) = stdlib_info {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: info.to_string(),
            }),
            range: None,
        });
    }
    
    None
}
