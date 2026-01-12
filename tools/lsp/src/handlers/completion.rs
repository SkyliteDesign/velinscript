// Completion Handler

use tower_lsp::lsp_types::*;
use velin_compiler::parser::ast::*;

pub fn get_completions(program: &Program, position: Position) -> Vec<CompletionItem> {
    let mut completions = Vec::new();
    
    // Keywords
    let keywords = vec![
        "fn", "let", "return", "if", "else", "for", "while", "match",
        "struct", "enum", "type", "pub", "use", "mod", "async", "await",
    ];
    
    for keyword in keywords {
        completions.push(CompletionItem {
            label: keyword.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(format!("Keyword: {}", keyword)),
            ..Default::default()
        });
    }
    
    // Decorators
    let decorators = vec![
        ("GET", "HTTP GET decorator"),
        ("POST", "HTTP POST decorator"),
        ("PUT", "HTTP PUT decorator"),
        ("DELETE", "HTTP DELETE decorator"),
        ("Auth", "Authentication decorator"),
        ("Role", "Role-based access decorator"),
        ("test", "Test decorator"),
        ("Cache", "Cache decorator"),
        ("Secure", "Security middleware decorator"),
    ];
    
    for (name, detail) in decorators {
        completions.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(detail.to_string()),
            insert_text: Some(format!("@{}", name)),
            ..Default::default()
        });
    }
    
    // Functions aus dem Program
    for item in &program.items {
        if let Item::Function(f) = item {
            let params_str = f.params
                .iter()
                .map(|p| format!("{}: {}", p.name, p.param_type.to_string()))
                .collect::<Vec<_>>()
                .join(", ");
            
            let return_type_str = f.return_type
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "void".to_string());
            
            completions.push(CompletionItem {
                label: f.name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(format!("fn {}({}) -> {}", f.name, params_str, return_type_str)),
                documentation: Some(Documentation::String(format!(
                    "Function: {}\nParameters: {}\nReturns: {}",
                    f.name, params_str, return_type_str
                ))),
                ..Default::default()
            });
        }
    }
    
    // Types (Structs, Enums)
    for item in &program.items {
        match item {
            Item::Struct(s) => {
                completions.push(CompletionItem {
                    label: s.name.clone(),
                    kind: Some(CompletionItemKind::STRUCT),
                    detail: Some(format!("Struct: {}", s.name)),
                    ..Default::default()
                });
            }
            Item::Enum(e) => {
                completions.push(CompletionItem {
                    label: e.name.clone(),
                    kind: Some(CompletionItemKind::ENUM),
                    detail: Some(format!("Enum: {}", e.name)),
                    ..Default::default()
                });
            }
            _ => {}
        }
    }
    
    // Standard Library Functions
    let stdlib_functions = vec![
        ("db.find", "Find entity by ID"),
        ("db.findAll", "Find all entities"),
        ("db.save", "Save entity"),
        ("db.delete", "Delete entity"),
        ("assert", "Assert condition"),
        ("generateId", "Generate unique ID"),
        ("currentUser", "Get current user"),
        // Response Functions
        ("successResponse", "Create successful API response"),
        ("successResponseWithCache", "Create successful API response with cache info"),
        ("errorResponse", "Create error API response"),
        // Error Functions
        ("createError", "Create application error"),
        ("createValidationError", "Create validation error"),
        ("createNotFoundError", "Create not found error"),
        ("createUnauthorizedError", "Create unauthorized error"),
        ("errorCodeToString", "Convert error code to string"),
        // Logging Functions
        ("logRequest", "Log HTTP request"),
        ("logResponse", "Log HTTP response"),
        ("logError", "Log error"),
        ("logPerformance", "Log performance metrics"),
        ("logInfo", "Log info message"),
        ("logWarning", "Log warning message"),
        ("logDebug", "Log debug message"),
        // Cache Functions
        ("cacheGet", "Get value from cache"),
        ("cacheSet", "Set value in cache"),
        ("cacheInvalidate", "Invalidate cache entry"),
        ("cacheClear", "Clear cache"),
        ("generateRequestId", "Generate unique request ID"),
        // Security Functions
        ("applySecurityMiddleware", "Apply security middleware"),
        ("validateApiKey", "Validate API key"),
        ("checkRateLimit", "Check rate limit"),
        ("sanitizeInput", "Sanitize user input"),
        ("sanitizeRequest", "Sanitize HTTP request"),
        ("applySecurityHeaders", "Apply security headers"),
    ];
    
    for (name, detail) in stdlib_functions {
        completions.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(detail.to_string()),
            ..Default::default()
        });
    }
    
    completions
}
