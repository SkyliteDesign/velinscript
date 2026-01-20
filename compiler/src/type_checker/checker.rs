use crate::parser::ast::*;
use crate::type_checker::environment::{Environment, FunctionSignature};
use crate::type_checker::errors::{TypeError, TypeErrorKind};
use crate::stdlib::rate_limit::{is_rate_limit_decorator, parse_rate_limit_config};
use crate::compiler::language::VELISCH_LANGUAGE_NAME;

pub struct TypeChecker {
    environment: Environment,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        // Velisch Identity Check - Fingerabdruck im Type Checker
        let _velisch_check = VELISCH_LANGUAGE_NAME;
        
        let mut env = Environment::new();
        
        // Built-in types
        env.define_type("string".to_string(), Type::String);
        env.define_type("number".to_string(), Type::Number);
        env.define_type("boolean".to_string(), Type::Boolean);
        env.define_type("void".to_string(), Type::Void);
        env.define_type("null".to_string(), Type::Null);
        // Map is a generic type, so we register it as a type name
        // The actual type will be Map<K, V> which is handled by Type::Map
        env.define_type("Map".to_string(), Type::Map {
            key: Box::new(Type::String),
            value: Box::new(Type::String),
        });
        // Result is a generic type, so we register it as a type name
        // The actual type will be Result<T, E> which is handled by Type::Result
        env.define_type("Result".to_string(), Type::Result {
            ok: Box::new(Type::String),
            err: Box::new(Type::String),
        });
        
        // Built-in functions
        // generateId() -> string
        env.define_function("generateId".to_string(), FunctionSignature {
            name: "generateId".to_string(),
            params: Vec::new(),
            return_type: Some(Type::String),
        });
        
        // Standard Library: Database functions
        // db.find<T>(T, string) -> Option<T>
        env.define_function("db.find".to_string(), FunctionSignature {
            name: "db.find".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "type".to_string(),
                    param_type: Type::Named("Type".to_string()),
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "id".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        
        // db.save<T>(T) -> T
        env.define_function("db.save".to_string(), FunctionSignature {
            name: "db.save".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "entity".to_string(),
                    param_type: Type::Named("any".to_string()),
                },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        
        // db.findAll<T>(T) -> List<T>
        env.define_function("db.findAll".to_string(), FunctionSignature {
            name: "db.findAll".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "type".to_string(),
                    param_type: Type::Named("Type".to_string()),
                },
            ],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        
        // db.findMany<T>(T, Map) -> List<T>
        env.define_function("db.findMany".to_string(), FunctionSignature {
            name: "db.findMany".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "type".to_string(),
                    param_type: Type::Named("Type".to_string()),
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "query".to_string(),
                    param_type: Type::Map {
                        key: Box::new(Type::String),
                        value: Box::new(Type::Named("any".to_string())),
                    },
                },
            ],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        
        // db.delete<T>(T, string) -> boolean
        env.define_function("db.delete".to_string(), FunctionSignature {
            name: "db.delete".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "type".to_string(),
                    param_type: Type::Named("Type".to_string()),
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "id".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Boolean),
        });
        
        // Register 'db' as a special object that has methods
        // This allows db.find(), db.save(), etc. to work
        env.define_variable("db".to_string(), Type::Named("Database".to_string()));
        
        // Register 'file' as a special object for file operations
        env.define_variable("file".to_string(), Type::Named("File".to_string()));
        
        // Register 'json' as a special object for JSON operations
        env.define_variable("json".to_string(), Type::Named("Json".to_string()));
        
        // Register 'datetime' as a special object for date/time operations
        env.define_variable("datetime".to_string(), Type::Named("DateTime".to_string()));
        
        // Register 'regex' as a special object for regex operations
        env.define_variable("regex".to_string(), Type::Named("Regex".to_string()));
        
        // Register 'crypto' as a special object for crypto operations
        env.define_variable("crypto".to_string(), Type::Named("Crypto".to_string()));
        
        // Standard Library: File I/O functions
        // file.read(path: string) -> Result<string, string>
        env.define_function("file.read".to_string(), FunctionSignature {
            name: "file.read".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "path".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::String),
                err: Box::new(Type::String),
            }),
        });
        
        // file.write(path: string, content: string) -> Result<(), string>
        env.define_function("file.write".to_string(), FunctionSignature {
            name: "file.write".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "path".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "content".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::Void),
                err: Box::new(Type::String),
            }),
        });
        
        // file.exists(path: string) -> boolean
        env.define_function("file.exists".to_string(), FunctionSignature {
            name: "file.exists".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "path".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Boolean),
        });
        
        // Standard Library: JSON functions
        // json.parse(text: string) -> Result<any, string>
        env.define_function("json.parse".to_string(), FunctionSignature {
            name: "json.parse".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "text".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::Named("any".to_string())),
                err: Box::new(Type::String),
            }),
        });
        
        // json.stringify(value: any) -> string
        env.define_function("json.stringify".to_string(), FunctionSignature {
            name: "json.stringify".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "value".to_string(),
                    param_type: Type::Named("any".to_string()),
                },
            ],
            return_type: Some(Type::String),
        });
        
        // Register Standard Library types
        env.define_type("HttpClient".to_string(), Type::Named("HttpClient".to_string()));
        env.define_type("Validator".to_string(), Type::Named("Validator".to_string()));
        env.define_type("AuthService".to_string(), Type::Named("AuthService".to_string()));
        env.define_type("Logger".to_string(), Type::Named("Logger".to_string()));
        env.define_type("VelinLogger".to_string(), Type::Named("VelinLogger".to_string()));
        env.define_type("MetricsCollector".to_string(), Type::Named("MetricsCollector".to_string()));
        env.define_type("PerformanceMonitor".to_string(), Type::Named("PerformanceMonitor".to_string()));
        env.define_type("LLMClient".to_string(), Type::Named("LLMClient".to_string()));
        env.define_type("ModelLoader".to_string(), Type::Named("ModelLoader".to_string()));
        env.define_type("TrainingService".to_string(), Type::Named("TrainingService".to_string()));
        env.define_type("HttpResponse".to_string(), Type::Named("HttpResponse".to_string()));
        env.define_type("ValidationError".to_string(), Type::Named("ValidationError".to_string()));
        env.define_type("JWTToken".to_string(), Type::Named("JWTToken".to_string()));
        env.define_type("UserClaims".to_string(), Type::Named("UserClaims".to_string()));
        
        // Standard Library: HTTP Client functions
        // HttpClient.new() -> HttpClient
        env.define_function("HttpClient.new".to_string(), FunctionSignature {
            name: "HttpClient.new".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("HttpClient".to_string())),
        });
        
        // Standard Library: Validation functions
        // Validator.new() -> Validator
        env.define_function("Validator.new".to_string(), FunctionSignature {
            name: "Validator.new".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("Validator".to_string())),
        });
        
        // Standard Library: Auth functions
        // AuthService.new(secret: string) -> AuthService
        env.define_function("AuthService.new".to_string(), FunctionSignature {
            name: "AuthService.new".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "secret".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Named("AuthService".to_string())),
        });
        
        // Standard Library: Logging functions
        // Logger.new() -> Logger
        env.define_function("Logger.new".to_string(), FunctionSignature {
            name: "Logger.new".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("Logger".to_string())),
        });
        
        // VelinLogger.new() -> VelinLogger
        env.define_function("VelinLogger.new".to_string(), FunctionSignature {
            name: "VelinLogger.new".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("VelinLogger".to_string())),
        });
        
        // Standard Library: Metrics functions
        // MetricsCollector.new() -> MetricsCollector
        env.define_function("MetricsCollector.new".to_string(), FunctionSignature {
            name: "MetricsCollector.new".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("MetricsCollector".to_string())),
        });
        
        // PerformanceMonitor.new() -> PerformanceMonitor
        env.define_function("PerformanceMonitor.new".to_string(), FunctionSignature {
            name: "PerformanceMonitor.new".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("PerformanceMonitor".to_string())),
        });
        
        // Standard Library: ML/LLM functions
        // LLMClient.new(provider: string, apiKey: string) -> LLMClient
        env.define_function("LLMClient.new".to_string(), FunctionSignature {
            name: "LLMClient.new".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "provider".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "apiKey".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Named("LLMClient".to_string())),
        });
        
        // ModelLoader.new() -> ModelLoader
        env.define_function("ModelLoader.new".to_string(), FunctionSignature {
            name: "ModelLoader.new".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("ModelLoader".to_string())),
        });
        
        // TrainingService.new() -> TrainingService
        env.define_function("TrainingService.new".to_string(), FunctionSignature {
            name: "TrainingService.new".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("TrainingService".to_string())),
        });
        
        // Standard Library: DateTime functions
        // datetime.now() -> number
        env.define_function("datetime.now".to_string(), FunctionSignature {
            name: "datetime.now".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Number),
        });
        
        // datetime.nowMillis() -> number
        env.define_function("datetime.nowMillis".to_string(), FunctionSignature {
            name: "datetime.nowMillis".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Number),
        });
        
        // datetime.formatISO8601(timestamp: number) -> string
        env.define_function("datetime.formatISO8601".to_string(), FunctionSignature {
            name: "datetime.formatISO8601".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "timestamp".to_string(),
                    param_type: Type::Number,
                },
            ],
            return_type: Some(Type::String),
        });
        
        // datetime.format(timestamp: number, format: string) -> string
        env.define_function("datetime.format".to_string(), FunctionSignature {
            name: "datetime.format".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "timestamp".to_string(),
                    param_type: Type::Number,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "format".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::String),
        });
        
        // datetime.parse(isoString: string) -> Result<number, string>
        env.define_function("datetime.parse".to_string(), FunctionSignature {
            name: "datetime.parse".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "isoString".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::Number),
                err: Box::new(Type::String),
            }),
        });
        
        // Standard Library: Regex functions
        // regex.match(pattern: string, text: string) -> Result<boolean, string>
        env.define_function("regex.match".to_string(), FunctionSignature {
            name: "regex.match".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "pattern".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "text".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::Boolean),
                err: Box::new(Type::String),
            }),
        });
        
        // regex.find(pattern: string, text: string) -> Result<Option<string>, string>
        env.define_function("regex.find".to_string(), FunctionSignature {
            name: "regex.find".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "pattern".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "text".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::Optional(Box::new(Type::String))),
                err: Box::new(Type::String),
            }),
        });
        
        // regex.findAll(pattern: string, text: string) -> Result<List<string>, string>
        env.define_function("regex.findAll".to_string(), FunctionSignature {
            name: "regex.findAll".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "pattern".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "text".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::List(Box::new(Type::String))),
                err: Box::new(Type::String),
            }),
        });
        
        // regex.replace(pattern: string, text: string, replacement: string) -> Result<string, string>
        env.define_function("regex.replace".to_string(), FunctionSignature {
            name: "regex.replace".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "pattern".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "text".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "replacement".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::String),
                err: Box::new(Type::String),
            }),
        });
        
        // regex.replaceAll(pattern: string, text: string, replacement: string) -> Result<string, string>
        env.define_function("regex.replaceAll".to_string(), FunctionSignature {
            name: "regex.replaceAll".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "pattern".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "text".to_string(),
                    param_type: Type::String,
                },
                crate::type_checker::environment::ParameterInfo {
                    name: "replacement".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::String),
                err: Box::new(Type::String),
            }),
        });
        
        // Standard Library: Crypto functions
        // crypto.sha256(input: string) -> string
        env.define_function("crypto.sha256".to_string(), FunctionSignature {
            name: "crypto.sha256".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "input".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::String),
        });
        
        // crypto.uuid() -> string
        env.define_function("crypto.uuid".to_string(), FunctionSignature {
            name: "crypto.uuid".to_string(),
            params: Vec::new(),
            return_type: Some(Type::String),
        });
        
        // crypto.base64Encode(input: string) -> string
        env.define_function("crypto.base64Encode".to_string(), FunctionSignature {
            name: "crypto.base64Encode".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "input".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::String),
        });
        
        // crypto.base64Decode(input: string) -> Result<string, string>
        env.define_function("crypto.base64Decode".to_string(), FunctionSignature {
            name: "crypto.base64Decode".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo {
                    name: "input".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::Result {
                ok: Box::new(Type::String),
                err: Box::new(Type::String),
            }),
        });
        
        // --- Extended Standard Library Types ---
        env.define_type("StringStdlib".to_string(), Type::Named("StringStdlib".to_string()));
        env.define_type("MathStdlib".to_string(), Type::Named("MathStdlib".to_string()));
        env.define_type("DateStdlib".to_string(), Type::Named("DateStdlib".to_string()));
        env.define_type("FsStdlib".to_string(), Type::Named("FsStdlib".to_string()));
        env.define_type("LLMStdlib".to_string(), Type::Named("LLMStdlib".to_string()));
        env.define_type("EmbeddingStdlib".to_string(), Type::Named("EmbeddingStdlib".to_string()));
        env.define_type("AgentStdlib".to_string(), Type::Named("AgentStdlib".to_string()));
        env.define_type("ProcessStdlib".to_string(), Type::Named("ProcessStdlib".to_string()));
        env.define_type("SandboxStdlib".to_string(), Type::Named("SandboxStdlib".to_string()));
        env.define_type("WebSocketStdlib".to_string(), Type::Named("WebSocketStdlib".to_string()));
        env.define_type("UtilsStdlib".to_string(), Type::Named("UtilsStdlib".to_string()));
        env.define_type("LogStdlib".to_string(), Type::Named("LogStdlib".to_string()));
        env.define_type("PathStdlib".to_string(), Type::Named("PathStdlib".to_string()));
        env.define_type("UrlStdlib".to_string(), Type::Named("UrlStdlib".to_string()));
        env.define_type("StreamStdlib".to_string(), Type::Named("StreamStdlib".to_string()));
        env.define_type("RedisStdlib".to_string(), Type::Named("RedisStdlib".to_string()));
        env.define_type("TracingStdlib".to_string(), Type::Named("TracingStdlib".to_string()));
        
        env.define_type("Agent".to_string(), Type::Named("Agent".to_string()));
        env.define_type("WebSocket".to_string(), Type::Named("WebSocket".to_string()));
        env.define_type("SearchResult".to_string(), Type::Named("SearchResult".to_string()));
        env.define_type("RedisClient".to_string(), Type::Named("RedisClient".to_string()));
        env.define_type("Span".to_string(), Type::Named("Span".to_string()));

// --- Yaml Module ---
        env.define_type("YamlStdlib".to_string(), Type::Named("YamlStdlib".to_string()));
        env.define_variable("yaml".to_string(), Type::Named("YamlStdlib".to_string()));
        env.define_function("yaml.parse".to_string(), FunctionSignature {
            name: "yaml.parse".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "yaml_string".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("yaml.parse_file".to_string(), FunctionSignature {
            name: "yaml.parse_file".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("yaml.stringify".to_string(), FunctionSignature {
            name: "yaml.stringify".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("yaml.write_file".to_string(), FunctionSignature {
            name: "yaml.write_file".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("yaml.validate".to_string(), FunctionSignature {
            name: "yaml.validate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "schema".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Boolean),
        });

        // --- Env Module ---
        env.define_type("EnvStdlib".to_string(), Type::Named("EnvStdlib".to_string()));
        env.define_variable("env".to_string(), Type::Named("EnvStdlib".to_string()));
        env.define_function("env.load".to_string(), FunctionSignature {
            name: "env.load".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Void),
        });
        env.define_function("env.get".to_string(), FunctionSignature {
            name: "env.get".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "default".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("env.get_number".to_string(), FunctionSignature {
            name: "env.get_number".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "default".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("env.get_bool".to_string(), FunctionSignature {
            name: "env.get_bool".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "default".to_string(), param_type: Type::Boolean },
            ],
            return_type: Some(Type::Boolean),
        });
        env.define_function("env.set".to_string(), FunctionSignature {
            name: "env.set".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("env.validate".to_string(), FunctionSignature {
            name: "env.validate".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "schema".to_string(), param_type: Type::String }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("env.get_secret".to_string(), FunctionSignature {
            name: "env.get_secret".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "vault".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });

        // --- Fixtures Module ---
        env.define_type("FixturesStdlib".to_string(), Type::Named("FixturesStdlib".to_string()));
        env.define_variable("fixtures".to_string(), Type::Named("FixturesStdlib".to_string()));
        env.define_function("fixtures.create".to_string(), FunctionSignature {
            name: "fixtures.create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "template".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("fixtures.create_many".to_string(), FunctionSignature {
            name: "fixtures.create_many".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "template".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "count".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("fixtures.factory".to_string(), FunctionSignature {
            name: "fixtures.factory".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "builder".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("fixtures.build".to_string(), FunctionSignature {
            name: "fixtures.build".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "factory".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "overrides".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Mocks Module ---
        env.define_type("MocksStdlib".to_string(), Type::Named("MocksStdlib".to_string()));
        env.define_variable("mocks".to_string(), Type::Named("MocksStdlib".to_string()));
        env.define_function("mocks.mock".to_string(), FunctionSignature {
            name: "mocks.mock".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "original".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "mock".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("mocks.spy".to_string(), FunctionSignature {
            name: "mocks.spy".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "target".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mocks.verify".to_string(), FunctionSignature {
            name: "mocks.verify".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "spy".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "expected_calls".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Boolean),
        });
        env.define_function("mocks.reset".to_string(), FunctionSignature {
            name: "mocks.reset".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "spy".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Void),
        });
        env.define_function("mocks.stub".to_string(), FunctionSignature {
            name: "mocks.stub".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "return_value".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Template Module ---
        env.define_type("TemplateStdlib".to_string(), Type::Named("TemplateStdlib".to_string()));
        env.define_variable("template".to_string(), Type::Named("TemplateStdlib".to_string()));
        env.define_function("template.render".to_string(), FunctionSignature {
            name: "template.render".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "template".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("template.render_file".to_string(), FunctionSignature {
            name: "template.render_file".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("template.partial".to_string(), FunctionSignature {
            name: "template.partial".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "partial_path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("template.cache".to_string(), FunctionSignature {
            name: "template.cache".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "template".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "cache_key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Void),
        });

        // --- Csv Module ---
        env.define_type("CsvStdlib".to_string(), Type::Named("CsvStdlib".to_string()));
        env.define_variable("csv".to_string(), Type::Named("CsvStdlib".to_string()));
        env.define_function("csv.read".to_string(), FunctionSignature {
            name: "csv.read".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "has_header".to_string(), param_type: Type::Boolean },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }))), err: Box::new(Type::String) }),
        });
        env.define_function("csv.write".to_string(), FunctionSignature {
            name: "csv.write".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "rows".to_string(), param_type: Type::List(Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) })) },
                crate::type_checker::environment::ParameterInfo { name: "headers".to_string(), param_type: Type::Optional(Box::new(Type::List(Box::new(Type::String)))) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("csv.parse".to_string(), FunctionSignature {
            name: "csv.parse".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "csv_string".to_string(), param_type: Type::String }],
            return_type: Some(Type::List(Box::new(Type::List(Box::new(Type::String))))),
        });
        env.define_function("csv.stringify".to_string(), FunctionSignature {
            name: "csv.stringify".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "rows".to_string(), param_type: Type::List(Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) })) },
                crate::type_checker::environment::ParameterInfo { name: "headers".to_string(), param_type: Type::List(Box::new(Type::String)) },
            ],
            return_type: Some(Type::String),
        });

        // --- Encryption Module ---
        env.define_type("EncryptionStdlib".to_string(), Type::Named("EncryptionStdlib".to_string()));
        env.define_variable("encryption".to_string(), Type::Named("EncryptionStdlib".to_string()));
        env.define_function("encryption.aes_encrypt".to_string(), FunctionSignature {
            name: "encryption.aes_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("encryption.aes_decrypt".to_string(), FunctionSignature {
            name: "encryption.aes_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("encryption.rsa_generate_keypair".to_string(), FunctionSignature {
            name: "encryption.rsa_generate_keypair".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "bits".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("encryption.rsa_encrypt".to_string(), FunctionSignature {
            name: "encryption.rsa_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "public_key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("encryption.rsa_decrypt".to_string(), FunctionSignature {
            name: "encryption.rsa_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "private_key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("encryption.fernet_generate_key".to_string(), FunctionSignature {
            name: "encryption.fernet_generate_key".to_string(),
            params: vec![],
            return_type: Some(Type::String),
        });
        env.define_function("encryption.fernet_encrypt".to_string(), FunctionSignature {
            name: "encryption.fernet_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("encryption.fernet_decrypt".to_string(), FunctionSignature {
            name: "encryption.fernet_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });

        // --- EventBus Module ---
        env.define_type("EventBusStdlib".to_string(), Type::Named("EventBusStdlib".to_string()));
        env.define_variable("event_bus".to_string(), Type::Named("EventBusStdlib".to_string()));
        env.define_function("event_bus.create".to_string(), FunctionSignature {
            name: "event_bus.create".to_string(),
            params: vec![],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("event_bus.publish".to_string(), FunctionSignature {
            name: "event_bus.publish".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "event".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.subscribe".to_string(), FunctionSignature {
            name: "event_bus.subscribe".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.unsubscribe".to_string(), FunctionSignature {
            name: "event_bus.unsubscribe".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "subscription".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.get_history".to_string(), FunctionSignature {
            name: "event_bus.get_history".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "limit".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });

        // --- MongoDB Module ---
        env.define_type("MongoDbStdlib".to_string(), Type::Named("MongoDbStdlib".to_string()));
        env.define_variable("mongodb".to_string(), Type::Named("MongoDbStdlib".to_string()));
        env.define_function("mongodb.connect".to_string(), FunctionSignature {
            name: "mongodb.connect".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.database".to_string(), FunctionSignature {
            name: "mongodb.database".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mongodb.collection".to_string(), FunctionSignature {
            name: "mongodb.collection".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "db".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mongodb.insert_one".to_string(), FunctionSignature {
            name: "mongodb.insert_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "doc".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.find".to_string(), FunctionSignature {
            name: "mongodb.find".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.find_one".to_string(), FunctionSignature {
            name: "mongodb.find_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Optional(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.update_one".to_string(), FunctionSignature {
            name: "mongodb.update_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "update".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.delete_one".to_string(), FunctionSignature {
            name: "mongodb.delete_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.aggregate".to_string(), FunctionSignature {
            name: "mongodb.aggregate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "pipeline".to_string(), param_type: Type::List(Box::new(Type::Named("any".to_string()))) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.create_index".to_string(), FunctionSignature {
            name: "mongodb.create_index".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "keys".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "unique".to_string(), param_type: Type::Boolean },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- Queue Module ---
        env.define_type("QueueStdlib".to_string(), Type::Named("QueueStdlib".to_string()));
        env.define_variable("queue".to_string(), Type::Named("QueueStdlib".to_string()));
        env.define_function("queue.create".to_string(), FunctionSignature {
            name: "queue.create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "capacity".to_string(), param_type: Type::Optional(Box::new(Type::Number)) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("queue.enqueue".to_string(), FunctionSignature {
            name: "queue.enqueue".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "item".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("queue.dequeue".to_string(), FunctionSignature {
            name: "queue.dequeue".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Optional(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("queue.peek".to_string(), FunctionSignature {
            name: "queue.peek".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Optional(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("queue.size".to_string(), FunctionSignature {
            name: "queue.size".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Number),
        });
        env.define_function("queue.is_empty".to_string(), FunctionSignature {
            name: "queue.is_empty".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("queue.is_full".to_string(), FunctionSignature {
            name: "queue.is_full".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("queue.priority_create".to_string(), FunctionSignature {
            name: "queue.priority_create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "compare".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("queue.priority_enqueue".to_string(), FunctionSignature {
            name: "queue.priority_enqueue".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "item".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "priority".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("queue.bounded_create".to_string(), FunctionSignature {
            name: "queue.bounded_create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "capacity".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Scheduler Module ---
        env.define_type("SchedulerStdlib".to_string(), Type::Named("SchedulerStdlib".to_string()));
        env.define_variable("scheduler".to_string(), Type::Named("SchedulerStdlib".to_string()));
        env.define_function("scheduler.schedule".to_string(), FunctionSignature {
            name: "scheduler.schedule".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "task".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "cron".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.schedule_interval".to_string(), FunctionSignature {
            name: "scheduler.schedule_interval".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "task".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "interval".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.cancel".to_string(), FunctionSignature {
            name: "scheduler.cancel".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "task_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.list".to_string(), FunctionSignature {
            name: "scheduler.list".to_string(),
            params: vec![],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("scheduler.get".to_string(), FunctionSignature {
            name: "scheduler.get".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "task_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.enable".to_string(), FunctionSignature {
            name: "scheduler.enable".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "task_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.disable".to_string(), FunctionSignature {
            name: "scheduler.disable".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "task_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Smtp Module ---
        env.define_type("SmtpStdlib".to_string(), Type::Named("SmtpStdlib".to_string()));
        env.define_variable("smtp".to_string(), Type::Named("SmtpStdlib".to_string()));
        env.define_function("smtp.connect".to_string(), FunctionSignature {
            name: "smtp.connect".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "config".to_string(), param_type: Type::Map { key: Box::new(Type::String), value: Box::new(Type::Named("any".to_string())) } }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("smtp.send".to_string(), FunctionSignature {
            name: "smtp.send".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "mailer".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "email".to_string(), param_type: Type::Map { key: Box::new(Type::String), value: Box::new(Type::Named("any".to_string())) } },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("smtp.template".to_string(), FunctionSignature {
            name: "smtp.template".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "template_path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Map { key: Box::new(Type::String), value: Box::new(Type::Named("any".to_string())) } },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- Workflow Module ---
        env.define_type("WorkflowStdlib".to_string(), Type::Named("WorkflowStdlib".to_string()));
        env.define_variable("workflow".to_string(), Type::Named("WorkflowStdlib".to_string()));
        env.define_function("workflow.create".to_string(), FunctionSignature {
            name: "workflow.create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "definition".to_string(), param_type: Type::Map { key: Box::new(Type::String), value: Box::new(Type::Named("any".to_string())) } }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("workflow.start".to_string(), FunctionSignature {
            name: "workflow.start".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("workflow.execute_step".to_string(), FunctionSignature {
            name: "workflow.execute_step".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "step_id".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("workflow.get_status".to_string(), FunctionSignature {
            name: "workflow.get_status".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("workflow.get_history".to_string(), FunctionSignature {
            name: "workflow.get_history".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("workflow.complete".to_string(), FunctionSignature {
            name: "workflow.complete".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("workflow.fail".to_string(), FunctionSignature {
            name: "workflow.fail".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "error".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Encoding Module ---
        env.define_type("EncodingStdlib".to_string(), Type::Named("EncodingStdlib".to_string()));
        env.define_variable("encoding".to_string(), Type::Named("EncodingStdlib".to_string()));
        env.define_function("encoding.base64_encode".to_string(), FunctionSignature {
            name: "encoding.base64_encode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("encoding.base64_decode".to_string(), FunctionSignature {
            name: "encoding.base64_decode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encoding.url_encode".to_string(), FunctionSignature {
            name: "encoding.url_encode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("encoding.url_decode".to_string(), FunctionSignature {
            name: "encoding.url_decode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("encoding.hex_encode".to_string(), FunctionSignature {
            name: "encoding.hex_encode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("encoding.hex_decode".to_string(), FunctionSignature {
            name: "encoding.hex_decode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encoding.is_valid_utf8".to_string(), FunctionSignature {
            name: "encoding.is_valid_utf8".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "bytes".to_string(), param_type: Type::List(Box::new(Type::Number)) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("encoding.fix_utf8".to_string(), FunctionSignature {
            name: "encoding.fix_utf8".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "bytes".to_string(), param_type: Type::List(Box::new(Type::Number)) }],
            return_type: Some(Type::List(Box::new(Type::Number))),
        });

        // --- Path Module ---
        env.define_type("PathStdlib".to_string(), Type::Named("PathStdlib".to_string()));
        // Note: 'path' variable is already defined in Extended Standard Library Variables
        env.define_function("path.join".to_string(), FunctionSignature {
            name: "path.join".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "parts".to_string(), param_type: Type::List(Box::new(Type::String)) }],
            return_type: Some(Type::String),
        });
        env.define_function("path.dirname".to_string(), FunctionSignature {
            name: "path.dirname".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("path.basename".to_string(), FunctionSignature {
            name: "path.basename".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("path.extname".to_string(), FunctionSignature {
            name: "path.extname".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("path.normalize".to_string(), FunctionSignature {
            name: "path.normalize".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("path.resolve".to_string(), FunctionSignature {
            name: "path.resolve".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("path.relative".to_string(), FunctionSignature {
            name: "path.relative".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "from".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "to".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("path.is_absolute".to_string(), FunctionSignature {
            name: "path.is_absolute".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("path.separator".to_string(), FunctionSignature {
            name: "path.separator".to_string(),
            params: vec![],
            return_type: Some(Type::String),
        });

        // --- Tracing Module ---
        env.define_type("TracingStdlib".to_string(), Type::Named("TracingStdlib".to_string()));
        env.define_variable("tracing".to_string(), Type::Named("TracingStdlib".to_string()));
        env.define_function("tracing.start_span".to_string(), FunctionSignature {
            name: "tracing.start_span".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("tracing.set_attribute".to_string(), FunctionSignature {
            name: "tracing.set_attribute".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "span".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("tracing.child_span".to_string(), FunctionSignature {
            name: "tracing.child_span".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "parent".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("tracing.end_span".to_string(), FunctionSignature {
            name: "tracing.end_span".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "span".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Void),
        });
        env.define_function("tracing.export".to_string(), FunctionSignature {
            name: "tracing.export".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "format".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("tracing.span_enter".to_string(), FunctionSignature {
            name: "tracing.span_enter".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "span".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Void),
        });

        // --- Redis Module ---
        env.define_type("RedisStdlib".to_string(), Type::Named("RedisStdlib".to_string()));
        env.define_variable("redis".to_string(), Type::Named("RedisStdlib".to_string()));
        env.define_function("redis.connect".to_string(), FunctionSignature {
            name: "redis.connect".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("redis.set".to_string(), FunctionSignature {
            name: "redis.set".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "ttl".to_string(), param_type: Type::Optional(Box::new(Type::Number)) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("redis.get".to_string(), FunctionSignature {
            name: "redis.get".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Optional(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("redis.delete".to_string(), FunctionSignature {
            name: "redis.delete".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("redis.hset".to_string(), FunctionSignature {
            name: "redis.hset".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "hash".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "field".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("redis.hget".to_string(), FunctionSignature {
            name: "redis.hget".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "hash".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "field".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Optional(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("redis.hgetall".to_string(), FunctionSignature {
            name: "redis.hgetall".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "hash".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }), err: Box::new(Type::String) }),
        });
        env.define_function("redis.lpush".to_string(), FunctionSignature {
            name: "redis.lpush".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("redis.rpush".to_string(), FunctionSignature {
            name: "redis.rpush".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("redis.lpop".to_string(), FunctionSignature {
            name: "redis.lpop".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Optional(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("redis.llen".to_string(), FunctionSignature {
            name: "redis.llen".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Number), err: Box::new(Type::String) }),
        });
        env.define_function("redis.sadd".to_string(), FunctionSignature {
            name: "redis.sadd".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "set".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "member".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- Stream Module ---
        env.define_type("StreamStdlib".to_string(), Type::Named("StreamStdlib".to_string()));
        env.define_variable("stream".to_string(), Type::Named("StreamStdlib".to_string()));
        env.define_function("stream.create".to_string(), FunctionSignature {
            name: "stream.create".to_string(),
            params: vec![],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("stream.map".to_string(), FunctionSignature {
            name: "stream.map".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "mapper".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("stream.filter".to_string(), FunctionSignature {
            name: "stream.filter".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "predicate".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("stream.reduce".to_string(), FunctionSignature {
            name: "stream.reduce".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "reducer".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "initial".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("stream.batch".to_string(), FunctionSignature {
            name: "stream.batch".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "size".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("stream.buffer".to_string(), FunctionSignature {
            name: "stream.buffer".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "size".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("stream.merge".to_string(), FunctionSignature {
            name: "stream.merge".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream1".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "stream2".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("stream.zip".to_string(), FunctionSignature {
            name: "stream.zip".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream1".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "stream2".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Url Module ---
        env.define_type("UrlStdlib".to_string(), Type::Named("UrlStdlib".to_string()));
        env.define_variable("url".to_string(), Type::Named("UrlStdlib".to_string()));
        env.define_function("url.parse".to_string(), FunctionSignature {
            name: "url.parse".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url_str".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("url.protocol".to_string(), FunctionSignature {
            name: "url.protocol".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.hostname".to_string(), FunctionSignature {
            name: "url.hostname".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.port".to_string(), FunctionSignature {
            name: "url.port".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Number),
        });
        env.define_function("url.pathname".to_string(), FunctionSignature {
            name: "url.pathname".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.search".to_string(), FunctionSignature {
            name: "url.search".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.hash".to_string(), FunctionSignature {
            name: "url.hash".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.format".to_string(), FunctionSignature {
            name: "url.format".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "components".to_string(), param_type: Type::Map { key: Box::new(Type::String), value: Box::new(Type::Named("any".to_string())) } }],
            return_type: Some(Type::String),
        });
        env.define_function("url.parse_query".to_string(), FunctionSignature {
            name: "url.parse_query".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "query_str".to_string(), param_type: Type::String }],
            return_type: Some(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }),
        });

        // --- Audit Module ---
        env.define_type("AuditStdlib".to_string(), Type::Named("AuditStdlib".to_string()));
        env.define_variable("audit".to_string(), Type::Named("AuditStdlib".to_string()));
        env.define_function("audit.log".to_string(), FunctionSignature {
            name: "audit.log".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Map { key: Box::new(Type::String), value: Box::new(Type::Named("any".to_string())) } }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("audit.query".to_string(), FunctionSignature {
            name: "audit.query".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "filters".to_string(), param_type: Type::Map { key: Box::new(Type::String), value: Box::new(Type::Named("any".to_string())) } }],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("audit.export".to_string(), FunctionSignature {
            name: "audit.export".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "format".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "filters".to_string(), param_type: Type::Map { key: Box::new(Type::String), value: Box::new(Type::Named("any".to_string())) } },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- TestModule Standard Library ---
        env.define_type("TestModuleStdlib".to_string(), Type::Named("TestModuleStdlib".to_string()));
        env.define_variable("test_module".to_string(), Type::Named("TestModuleStdlib".to_string()));
        env.define_function("test_module.process_data".to_string(), FunctionSignature {
            name: "test_module.process_data".to_string(),
            params: vec![
                ParameterInfo {
                    name: "input".to_string(),
                    param_type: Type::String,
                },
                ParameterInfo {
                    name: "options".to_string(),
                    param_type: Type::Named("object".to_string()),
                },
            ],
            return_type: Some(Type::Named("object".to_string())),
        });
        env.define_function("test_module.validate_input".to_string(), FunctionSignature {
            name: "test_module.validate_input".to_string(),
            params: vec![
                ParameterInfo {
                    name: "data".to_string(),
                    param_type: Type::Any,
                },
            ],
            return_type: Some(Type::Boolean),
        });
        env.define_function("test_module.transform_format".to_string(), FunctionSignature {
            name: "test_module.transform_format".to_string(),
            params: vec![
                ParameterInfo {
                    name: "data".to_string(),
                    param_type: Type::Named("object".to_string()),
                },
                ParameterInfo {
                    name: "target_format".to_string(),
                    param_type: Type::String,
                },
            ],
            return_type: Some(Type::String),
        });

// --- Extended Standard Library Variables ---
        env.define_variable("string".to_string(), Type::Named("StringStdlib".to_string()));
        env.define_variable("math".to_string(), Type::Named("MathStdlib".to_string()));
        env.define_variable("date".to_string(), Type::Named("DateStdlib".to_string()));
        env.define_variable("fs".to_string(), Type::Named("FsStdlib".to_string()));
        env.define_variable("llm".to_string(), Type::Named("LLMStdlib".to_string()));
        env.define_variable("embedding".to_string(), Type::Named("EmbeddingStdlib".to_string()));
        env.define_variable("agent".to_string(), Type::Named("AgentStdlib".to_string()));
        env.define_variable("process".to_string(), Type::Named("ProcessStdlib".to_string()));
        env.define_variable("sandbox".to_string(), Type::Named("SandboxStdlib".to_string()));
        env.define_variable("websocket".to_string(), Type::Named("WebSocketStdlib".to_string()));
        env.define_variable("utils".to_string(), Type::Named("UtilsStdlib".to_string()));
        env.define_variable("log".to_string(), Type::Named("LogStdlib".to_string()));
        env.define_variable("path".to_string(), Type::Named("PathStdlib".to_string()));
        env.define_variable("url".to_string(), Type::Named("UrlStdlib".to_string()));
        env.define_variable("stream".to_string(), Type::Named("StreamStdlib".to_string()));
        env.define_variable("redis".to_string(), Type::Named("RedisStdlib".to_string()));
        env.define_variable("tracing".to_string(), Type::Named("TracingStdlib".to_string()));

        // Register new functions (batch 1: String)
        env.define_function("string.split".to_string(), FunctionSignature {
            name: "string.split".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "delimiter".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::List(Box::new(Type::String))),
        });
        env.define_function("string.join".to_string(), FunctionSignature {
            name: "string.join".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::List(Box::new(Type::String)) },
                crate::type_checker::environment::ParameterInfo { name: "delimiter".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        
        // --- String Module Extensions ---
        env.define_function("string.replace".to_string(), FunctionSignature {
            name: "string.replace".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "old".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "new".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("string.trim".to_string(), FunctionSignature {
            name: "string.trim".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("string.slugify".to_string(), FunctionSignature {
            name: "string.slugify".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("string.to_int".to_string(), FunctionSignature {
            name: "string.to_int".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Number), err: Box::new(Type::String) }),
        });
        env.define_function("string.to_float".to_string(), FunctionSignature {
            name: "string.to_float".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Number), err: Box::new(Type::String) }),
        });
        env.define_function("string.capitalize".to_string(), FunctionSignature {
            name: "string.capitalize".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("string.lowercase".to_string(), FunctionSignature {
            name: "string.lowercase".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("string.uppercase".to_string(), FunctionSignature {
            name: "string.uppercase".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("string.starts_with".to_string(), FunctionSignature {
            name: "string.starts_with".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "prefix".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Boolean),
        });
        env.define_function("string.ends_with".to_string(), FunctionSignature {
            name: "string.ends_with".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "suffix".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Boolean),
        });

        // --- Math Module ---
        env.define_function("math.clamp".to_string(), FunctionSignature {
            name: "math.clamp".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "min".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "max".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("math.lerp".to_string(), FunctionSignature {
            name: "math.lerp".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "a".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "b".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "t".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("math.round_to".to_string(), FunctionSignature {
            name: "math.round_to".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "decimals".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("math.random_range".to_string(), FunctionSignature {
            name: "math.random_range".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "min".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "max".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("math.min".to_string(), FunctionSignature {
            name: "math.min".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "a".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "b".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("math.max".to_string(), FunctionSignature {
            name: "math.max".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "a".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "b".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("math.abs".to_string(), FunctionSignature {
            name: "math.abs".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Number),
        });
        env.define_function("math.floor".to_string(), FunctionSignature {
            name: "math.floor".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Number),
        });
        env.define_function("math.ceil".to_string(), FunctionSignature {
            name: "math.ceil".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Number),
        });

        // --- Date Module ---
        env.define_function("date.add_days".to_string(), FunctionSignature {
            name: "date.add_days".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "timestamp".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "days".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("date.add_hours".to_string(), FunctionSignature {
            name: "date.add_hours".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "timestamp".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "hours".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("date.add_minutes".to_string(), FunctionSignature {
            name: "date.add_minutes".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "timestamp".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "minutes".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("date.format_relative".to_string(), FunctionSignature {
            name: "date.format_relative".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "timestamp".to_string(), param_type: Type::Number }],
            return_type: Some(Type::String),
        });
        env.define_function("date.is_weekend".to_string(), FunctionSignature {
            name: "date.is_weekend".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "timestamp".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("date.is_weekday".to_string(), FunctionSignature {
            name: "date.is_weekday".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "timestamp".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Boolean),
        });

        // --- FS Module ---
        env.define_function("fs.read_json".to_string(), FunctionSignature {
            name: "fs.read_json".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("fs.write_json".to_string(), FunctionSignature {
            name: "fs.write_json".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("fs.copy".to_string(), FunctionSignature {
            name: "fs.copy".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "source".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "dest".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("fs.move_file".to_string(), FunctionSignature {
            name: "fs.move_file".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "source".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "dest".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("fs.get_size".to_string(), FunctionSignature {
            name: "fs.get_size".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Number), err: Box::new(Type::String) }),
        });
        env.define_function("fs.is_empty".to_string(), FunctionSignature {
            name: "fs.is_empty".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Boolean),
        });

        // --- Config Module ---
        env.define_variable("config".to_string(), Type::Void); // config module namespace
        
        env.define_function("config.get_env".to_string(), FunctionSignature {
            name: "config.get_env".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        
        env.define_function("config.get_or_default".to_string(), FunctionSignature {
            name: "config.get_or_default".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "default".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });

        env.define_function("config.load_dotenv".to_string(), FunctionSignature {
            name: "config.load_dotenv".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- Flow Runtime ---
        env.define_variable("flow".to_string(), Type::Void);
        
        env.define_function("flow.snapshot_input".to_string(), FunctionSignature {
            name: "flow.snapshot_input".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Void),
        });

        // --- LLM Module ---
        env.define_function("llm.summarize".to_string(), FunctionSignature {
            name: "llm.summarize".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("llm.classify".to_string(), FunctionSignature {
            name: "llm.classify".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "categories".to_string(), param_type: Type::List(Box::new(Type::String)) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("llm.extract_entities".to_string(), FunctionSignature {
            name: "llm.extract_entities".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }))), err: Box::new(Type::String) }),
        });
        env.define_function("llm.generate".to_string(), FunctionSignature {
            name: "llm.generate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "title".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "style".to_string(), param_type: Type::String }, // Optional param logic needed?
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("llm.translate".to_string(), FunctionSignature {
            name: "llm.translate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "target_lang".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("llm.sentiment".to_string(), FunctionSignature {
            name: "llm.sentiment".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("llm.complete".to_string(), FunctionSignature {
            name: "llm.complete".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "prompt".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "max_tokens".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("llm.embed".to_string(), FunctionSignature {
            name: "llm.embed".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Number))), err: Box::new(Type::String) }),
        });
        env.define_function("llm.chat".to_string(), FunctionSignature {
            name: "llm.chat".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "messages".to_string(), param_type: Type::List(Box::new(Type::Named("any".to_string()))) }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- Embedding Module ---
        env.define_function("embedding.compare".to_string(), FunctionSignature {
            name: "embedding.compare".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "a".to_string(), param_type: Type::List(Box::new(Type::Number)) },
                crate::type_checker::environment::ParameterInfo { name: "b".to_string(), param_type: Type::List(Box::new(Type::Number)) },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("embedding.similarity".to_string(), FunctionSignature {
            name: "embedding.similarity".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "a".to_string(), param_type: Type::List(Box::new(Type::Number)) },
                crate::type_checker::environment::ParameterInfo { name: "b".to_string(), param_type: Type::List(Box::new(Type::Number)) },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("embedding.cluster".to_string(), FunctionSignature {
            name: "embedding.cluster".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::List(Box::new(Type::List(Box::new(Type::Number)))) },
                crate::type_checker::environment::ParameterInfo { name: "k".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::List(Box::new(Type::List(Box::new(Type::Number))))))), err: Box::new(Type::String) }),
        });
        env.define_function("embedding.normalize".to_string(), FunctionSignature {
            name: "embedding.normalize".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "embedding".to_string(), param_type: Type::List(Box::new(Type::Number)) }],
            return_type: Some(Type::List(Box::new(Type::Number))),
        });
        env.define_function("embedding.distance".to_string(), FunctionSignature {
            name: "embedding.distance".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "a".to_string(), param_type: Type::List(Box::new(Type::Number)) },
                crate::type_checker::environment::ParameterInfo { name: "b".to_string(), param_type: Type::List(Box::new(Type::Number)) },
            ],
            return_type: Some(Type::Number),
        });

        // --- Csv Module ---
        env.define_type("CsvStdlib".to_string(), Type::Named("CsvStdlib".to_string()));
        env.define_variable("csv".to_string(), Type::Named("CsvStdlib".to_string()));
        env.define_function("csv.read".to_string(), FunctionSignature {
            name: "csv.read".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "has_header".to_string(), param_type: Type::Boolean },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }))), err: Box::new(Type::String) }),
        });
        env.define_function("csv.write".to_string(), FunctionSignature {
            name: "csv.write".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "rows".to_string(), param_type: Type::List(Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) })) },
                crate::type_checker::environment::ParameterInfo { name: "headers".to_string(), param_type: Type::Optional(Box::new(Type::List(Box::new(Type::String)))) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("csv.parse".to_string(), FunctionSignature {
            name: "csv.parse".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "csv_string".to_string(), param_type: Type::String }],
            return_type: Some(Type::List(Box::new(Type::List(Box::new(Type::String))))),
        });
        env.define_function("csv.stringify".to_string(), FunctionSignature {
            name: "csv.stringify".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "rows".to_string(), param_type: Type::List(Box::new(Type::List(Box::new(Type::String)))) },
                crate::type_checker::environment::ParameterInfo { name: "headers".to_string(), param_type: Type::List(Box::new(Type::String)) },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("csv.validate".to_string(), FunctionSignature {
            name: "csv.validate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "schema".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });

        // --- Encryption Module ---
        env.define_type("EncryptionStdlib".to_string(), Type::Named("EncryptionStdlib".to_string()));
        env.define_variable("encryption".to_string(), Type::Named("EncryptionStdlib".to_string()));
        env.define_function("encryption.aes_encrypt".to_string(), FunctionSignature {
            name: "encryption.aes_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.aes_decrypt".to_string(), FunctionSignature {
            name: "encryption.aes_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.rsa_generate_keypair".to_string(), FunctionSignature {
            name: "encryption.rsa_generate_keypair".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "bits".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.rsa_encrypt".to_string(), FunctionSignature {
            name: "encryption.rsa_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "public_key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.rsa_decrypt".to_string(), FunctionSignature {
            name: "encryption.rsa_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "private_key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.fernet_generate_key".to_string(), FunctionSignature {
            name: "encryption.fernet_generate_key".to_string(),
            params: Vec::new(),
            return_type: Some(Type::String),
        });
        env.define_function("encryption.fernet_encrypt".to_string(), FunctionSignature {
            name: "encryption.fernet_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.fernet_decrypt".to_string(), FunctionSignature {
            name: "encryption.fernet_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.generate_key".to_string(), FunctionSignature {
            name: "encryption.generate_key".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "algorithm".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.store_key".to_string(), FunctionSignature {
            name: "encryption.store_key".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key_id".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "vault".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.retrieve_key".to_string(), FunctionSignature {
            name: "encryption.retrieve_key".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "key_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- Event Bus Module ---
        env.define_type("EventBusStdlib".to_string(), Type::Named("EventBusStdlib".to_string()));
        env.define_variable("event_bus".to_string(), Type::Named("EventBusStdlib".to_string()));
        env.define_function("event_bus.create".to_string(), FunctionSignature {
            name: "event_bus.create".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("event_bus.publish".to_string(), FunctionSignature {
            name: "event_bus.publish".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "event".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.subscribe".to_string(), FunctionSignature {
            name: "event_bus.subscribe".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.unsubscribe".to_string(), FunctionSignature {
            name: "event_bus.unsubscribe".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "subscription".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.get_history".to_string(), FunctionSignature {
            name: "event_bus.get_history".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "limit".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });

        // --- MongoDB Module ---
        env.define_type("MongoDbStdlib".to_string(), Type::Named("MongoDbStdlib".to_string()));
        env.define_variable("mongodb".to_string(), Type::Named("MongoDbStdlib".to_string()));
        env.define_function("mongodb.connect".to_string(), FunctionSignature {
            name: "mongodb.connect".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.database".to_string(), FunctionSignature {
            name: "mongodb.database".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mongodb.collection".to_string(), FunctionSignature {
            name: "mongodb.collection".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "db".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mongodb.insert_one".to_string(), FunctionSignature {
            name: "mongodb.insert_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "doc".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.find".to_string(), FunctionSignature {
            name: "mongodb.find".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.find_one".to_string(), FunctionSignature {
            name: "mongodb.find_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.update_one".to_string(), FunctionSignature {
            name: "mongodb.update_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "update".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.delete_one".to_string(), FunctionSignature {
            name: "mongodb.delete_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.aggregate".to_string(), FunctionSignature {
            name: "mongodb.aggregate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "pipeline".to_string(), param_type: Type::List(Box::new(Type::Named("any".to_string()))) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.create_index".to_string(), FunctionSignature {
            name: "mongodb.create_index".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "keys".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "unique".to_string(), param_type: Type::Boolean },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- Queue Module ---
        env.define_type("QueueStdlib".to_string(), Type::Named("QueueStdlib".to_string()));
        env.define_variable("queue".to_string(), Type::Named("QueueStdlib".to_string()));
        env.define_function("queue.create".to_string(), FunctionSignature {
            name: "queue.create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "capacity".to_string(), param_type: Type::Optional(Box::new(Type::Number)) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("queue.enqueue".to_string(), FunctionSignature {
            name: "queue.enqueue".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "item".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("queue.dequeue".to_string(), FunctionSignature {
            name: "queue.dequeue".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Optional(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("queue.peek".to_string(), FunctionSignature {
            name: "queue.peek".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Optional(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("queue.size".to_string(), FunctionSignature {
            name: "queue.size".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Number),
        });
        env.define_function("queue.is_empty".to_string(), FunctionSignature {
            name: "queue.is_empty".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("queue.is_full".to_string(), FunctionSignature {
            name: "queue.is_full".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("queue.priority_create".to_string(), FunctionSignature {
            name: "queue.priority_create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "compare".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("queue.priority_enqueue".to_string(), FunctionSignature {
            name: "queue.priority_enqueue".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "item".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "priority".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("queue.bounded_create".to_string(), FunctionSignature {
            name: "queue.bounded_create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "capacity".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("embedding.find_nearest".to_string(), FunctionSignature {
            name: "embedding.find_nearest".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "query".to_string(), param_type: Type::List(Box::new(Type::Number)) },
                crate::type_checker::environment::ParameterInfo { name: "candidates".to_string(), param_type: Type::List(Box::new(Type::List(Box::new(Type::Number)))) },
                crate::type_checker::environment::ParameterInfo { name: "k".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::List(Box::new(Type::List(Box::new(Type::Number))))),
        });
        env.define_function("embedding.average".to_string(), FunctionSignature {
            name: "embedding.average".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "embeddings".to_string(), param_type: Type::List(Box::new(Type::List(Box::new(Type::Number)))) }],
            return_type: Some(Type::List(Box::new(Type::Number))),
        });
        env.define_function("embedding.dimension".to_string(), FunctionSignature {
            name: "embedding.dimension".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "embedding".to_string(), param_type: Type::List(Box::new(Type::Number)) }],
            return_type: Some(Type::Number),
        });

        // --- Agent Module ---
        env.define_function("agent.memory.store".to_string(), FunctionSignature {
            name: "agent.memory.store".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("agent.memory.search".to_string(), FunctionSignature {
            name: "agent.memory.search".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "query".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("agent.task.run".to_string(), FunctionSignature {
            name: "agent.task.run".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "description".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("agent.task.plan".to_string(), FunctionSignature {
            name: "agent.task.plan".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "goal".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("agent.memory.get".to_string(), FunctionSignature {
            name: "agent.memory.get".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("agent.memory.delete".to_string(), FunctionSignature {
            name: "agent.memory.delete".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("agent.task.execute".to_string(), FunctionSignature {
            name: "agent.task.execute".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "plan".to_string(), param_type: Type::List(Box::new(Type::String)) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("agent.create".to_string(), FunctionSignature {
            name: "agent.create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("Agent".to_string())),
        });
        env.define_function("agent.think".to_string(), FunctionSignature {
            name: "agent.think".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "context".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- Process Module ---
        env.define_function("process.spawn".to_string(), FunctionSignature {
            name: "process.spawn".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "command".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "args".to_string(), param_type: Type::List(Box::new(Type::String)) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Number), err: Box::new(Type::String) }),
        });
        env.define_function("process.kill".to_string(), FunctionSignature {
            name: "process.kill".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "pid".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("process.restart".to_string(), FunctionSignature {
            name: "process.restart".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "pid".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("process.status".to_string(), FunctionSignature {
            name: "process.status".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "pid".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("process.list".to_string(), FunctionSignature {
            name: "process.list".to_string(),
            params: Vec::new(),
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("process.wait".to_string(), FunctionSignature {
            name: "process.wait".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "pid".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::Number), err: Box::new(Type::String) }),
        });
        env.define_function("process.get_output".to_string(), FunctionSignature {
            name: "process.get_output".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "pid".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("process.is_running".to_string(), FunctionSignature {
            name: "process.is_running".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "pid".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("process.get_memory".to_string(), FunctionSignature {
            name: "process.get_memory".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "pid".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::Number), err: Box::new(Type::String) }),
        });

        // --- Sandbox Module ---
        env.define_function("sandbox.build".to_string(), FunctionSignature {
            name: "sandbox.build".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "project_path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("sandbox.test".to_string(), FunctionSignature {
            name: "sandbox.test".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "project_path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("sandbox.validate".to_string(), FunctionSignature {
            name: "sandbox.validate".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "code".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("sandbox.run".to_string(), FunctionSignature {
            name: "sandbox.run".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "code".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("sandbox.lint".to_string(), FunctionSignature {
            name: "sandbox.lint".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "code".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("sandbox.format".to_string(), FunctionSignature {
            name: "sandbox.format".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "code".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("sandbox.check_types".to_string(), FunctionSignature {
            name: "sandbox.check_types".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "code".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("sandbox.optimize".to_string(), FunctionSignature {
            name: "sandbox.optimize".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "code".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- Rollback Module Extensions ---
        env.define_function("rollback.list_snapshots".to_string(), FunctionSignature {
            name: "rollback.list_snapshots".to_string(),
            params: Vec::new(),
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("rollback.delete_snapshot".to_string(), FunctionSignature {
            name: "rollback.delete_snapshot".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "snapshot_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("rollback.compare".to_string(), FunctionSignature {
            name: "rollback.compare".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "snapshot1".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "snapshot2".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("rollback.get_info".to_string(), FunctionSignature {
            name: "rollback.get_info".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "snapshot_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("rollback.auto_snapshot".to_string(), FunctionSignature {
            name: "rollback.auto_snapshot".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "interval_seconds".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- HTTP Module Extensions ---
        env.define_function("http.patch".to_string(), FunctionSignature {
            name: "http.patch".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "body".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("HttpResponse".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("http.head".to_string(), FunctionSignature {
            name: "http.head".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("HttpResponse".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("http.options".to_string(), FunctionSignature {
            name: "http.options".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("HttpResponse".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("http.set_timeout".to_string(), FunctionSignature {
            name: "http.set_timeout".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("HttpClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "ms".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Named("HttpClient".to_string())),
        });
        env.define_function("http.set_headers".to_string(), FunctionSignature {
            name: "http.set_headers".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("HttpClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "headers".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("HttpClient".to_string())),
        });

        // --- WebSocket Module ---
        env.define_function("websocket.connect".to_string(), FunctionSignature {
            name: "websocket.connect".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("WebSocket".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("websocket.send".to_string(), FunctionSignature {
            name: "websocket.send".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "ws".to_string(), param_type: Type::Named("WebSocket".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "message".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("websocket.receive".to_string(), FunctionSignature {
            name: "websocket.receive".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "ws".to_string(), param_type: Type::Named("WebSocket".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("websocket.close".to_string(), FunctionSignature {
            name: "websocket.close".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "ws".to_string(), param_type: Type::Named("WebSocket".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("websocket.is_connected".to_string(), FunctionSignature {
            name: "websocket.is_connected".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "ws".to_string(), param_type: Type::Named("WebSocket".to_string()) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("websocket.ping".to_string(), FunctionSignature {
            name: "websocket.ping".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "ws".to_string(), param_type: Type::Named("WebSocket".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("websocket.subscribe".to_string(), FunctionSignature {
            name: "websocket.subscribe".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "ws".to_string(), param_type: Type::Named("WebSocket".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("websocket.on_message".to_string(), FunctionSignature {
            name: "websocket.on_message".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "ws".to_string(), param_type: Type::Named("WebSocket".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "callback".to_string(), param_type: Type::Named("any".to_string()) }, // Function type TODO
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- Utils Module ---
        env.define_function("utils.uuid".to_string(), FunctionSignature {
            name: "utils.uuid".to_string(),
            params: Vec::new(),
            return_type: Some(Type::String),
        });
        env.define_function("utils.sleep".to_string(), FunctionSignature {
            name: "utils.sleep".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "ms".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Void),
        });
        env.define_function("utils.retry".to_string(), FunctionSignature {
            name: "utils.retry".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "fn".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "times".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("utils.debounce".to_string(), FunctionSignature {
            name: "utils.debounce".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "fn".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "ms".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("utils.throttle".to_string(), FunctionSignature {
            name: "utils.throttle".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "fn".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "ms".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("utils.memoize".to_string(), FunctionSignature {
            name: "utils.memoize".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "fn".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("utils.timeout".to_string(), FunctionSignature {
            name: "utils.timeout".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "fn".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "ms".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("utils.parallel".to_string(), FunctionSignature {
            name: "utils.parallel".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "tasks".to_string(), param_type: Type::List(Box::new(Type::Named("any".to_string()))) }],
            return_type: Some(Type::List(Box::new(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }))),
        });
        env.define_function("utils.cache".to_string(), FunctionSignature {
            name: "utils.cache".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "fn".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Log Module ---
        env.define_function("log.info".to_string(), FunctionSignature {
            name: "log.info".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "message".to_string(), param_type: Type::String }],
            return_type: Some(Type::Void),
        });
        env.define_function("log.warn".to_string(), FunctionSignature {
            name: "log.warn".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "message".to_string(), param_type: Type::String }],
            return_type: Some(Type::Void),
        });
        env.define_function("log.error".to_string(), FunctionSignature {
            name: "log.error".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "message".to_string(), param_type: Type::String }],
            return_type: Some(Type::Void),
        });
        env.define_function("log.debug".to_string(), FunctionSignature {
            name: "log.debug".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "message".to_string(), param_type: Type::String }],
            return_type: Some(Type::Void),
        });
        env.define_function("log.trace".to_string(), FunctionSignature {
            name: "log.trace".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "message".to_string(), param_type: Type::String }],
            return_type: Some(Type::Void),
        });
        env.define_function("log.set_level".to_string(), FunctionSignature {
            name: "log.set_level".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "level".to_string(), param_type: Type::String }],
            return_type: Some(Type::Void),
        });
        env.define_function("log.with_context".to_string(), FunctionSignature {
            name: "log.with_context".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("Logger".to_string())),
        });
        env.define_function("log.to_file".to_string(), FunctionSignature {
            name: "log.to_file".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("log.json".to_string(), FunctionSignature {
            name: "log.json".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "message".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Void),
        });

        // --- Path Library Functions ---
        env.define_function("path.join".to_string(), FunctionSignature {
            name: "path.join".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "parts".to_string(), param_type: Type::List(Box::new(Type::String)) },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("path.dirname".to_string(), FunctionSignature {
            name: "path.dirname".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("path.basename".to_string(), FunctionSignature {
            name: "path.basename".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("path.extname".to_string(), FunctionSignature {
            name: "path.extname".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("path.normalize".to_string(), FunctionSignature {
            name: "path.normalize".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("path.resolve".to_string(), FunctionSignature {
            name: "path.resolve".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("path.relative".to_string(), FunctionSignature {
            name: "path.relative".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "from".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "to".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("path.is_absolute".to_string(), FunctionSignature {
            name: "path.is_absolute".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("path.separator".to_string(), FunctionSignature {
            name: "path.separator".to_string(),
            params: vec![],
            return_type: Some(Type::String),
        });

        // --- URL Library Functions ---
        env.define_function("url.parse".to_string(), FunctionSignature {
            name: "url.parse".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("Url".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("url.protocol".to_string(), FunctionSignature {
            name: "url.protocol".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("Url".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.hostname".to_string(), FunctionSignature {
            name: "url.hostname".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("Url".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.port".to_string(), FunctionSignature {
            name: "url.port".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("Url".to_string()) }],
            return_type: Some(Type::Number),
        });
        env.define_function("url.pathname".to_string(), FunctionSignature {
            name: "url.pathname".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("Url".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.search".to_string(), FunctionSignature {
            name: "url.search".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("Url".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.hash".to_string(), FunctionSignature {
            name: "url.hash".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::Named("Url".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.format".to_string(), FunctionSignature {
            name: "url.format".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "components".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("url.parse_query".to_string(), FunctionSignature {
            name: "url.parse_query".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "query".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("url.stringify_query".to_string(), FunctionSignature {
            name: "url.stringify_query".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "params".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });

        // --- Stream Library Functions ---
        env.define_function("stream.create".to_string(), FunctionSignature {
            name: "stream.create".to_string(),
            params: vec![],
            return_type: Some(Type::Named("Stream".to_string())),
        });
        env.define_function("stream.map".to_string(), FunctionSignature {
            name: "stream.map".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("Stream".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "mapper".to_string(), param_type: Type::Named("fn".to_string()) },
            ],
            return_type: Some(Type::Named("Stream".to_string())),
        });
        env.define_function("stream.filter".to_string(), FunctionSignature {
            name: "stream.filter".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("Stream".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "predicate".to_string(), param_type: Type::Named("fn".to_string()) },
            ],
            return_type: Some(Type::Named("Stream".to_string())),
        });
        env.define_function("stream.reduce".to_string(), FunctionSignature {
            name: "stream.reduce".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("Stream".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "reducer".to_string(), param_type: Type::Named("fn".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "initial".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("stream.batch".to_string(), FunctionSignature {
            name: "stream.batch".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("Stream".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "size".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Named("Stream".to_string())),
        });
        env.define_function("stream.buffer".to_string(), FunctionSignature {
            name: "stream.buffer".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream".to_string(), param_type: Type::Named("Stream".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "size".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Named("Stream".to_string())),
        });
        env.define_function("stream.merge".to_string(), FunctionSignature {
            name: "stream.merge".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream1".to_string(), param_type: Type::Named("Stream".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "stream2".to_string(), param_type: Type::Named("Stream".to_string()) },
            ],
            return_type: Some(Type::Named("Stream".to_string())),
        });
        env.define_function("stream.zip".to_string(), FunctionSignature {
            name: "stream.zip".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "stream1".to_string(), param_type: Type::Named("Stream".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "stream2".to_string(), param_type: Type::Named("Stream".to_string()) },
            ],
            return_type: Some(Type::Named("Stream".to_string())),
        });

        // --- Redis Library Functions ---
        env.define_function("redis.connect".to_string(), FunctionSignature {
            name: "redis.connect".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("RedisClient".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("redis.set".to_string(), FunctionSignature {
            name: "redis.set".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("redis.get".to_string(), FunctionSignature {
            name: "redis.get".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Optional(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("redis.delete".to_string(), FunctionSignature {
            name: "redis.delete".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("redis.hset".to_string(), FunctionSignature {
            name: "redis.hset".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "hash".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "field".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("redis.hget".to_string(), FunctionSignature {
            name: "redis.hget".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "hash".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "field".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Optional(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("redis.hgetall".to_string(), FunctionSignature {
            name: "redis.hgetall".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "hash".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }), err: Box::new(Type::String) }),
        });
        env.define_function("redis.lpush".to_string(), FunctionSignature {
            name: "redis.lpush".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("redis.rpush".to_string(), FunctionSignature {
            name: "redis.rpush".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("redis.lpop".to_string(), FunctionSignature {
            name: "redis.lpop".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Optional(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("redis.llen".to_string(), FunctionSignature {
            name: "redis.llen".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "list".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Number), err: Box::new(Type::String) }),
        });
        env.define_function("redis.sadd".to_string(), FunctionSignature {
            name: "redis.sadd".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "set".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "member".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("redis.sismember".to_string(), FunctionSignature {
            name: "redis.sismember".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "set".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "member".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("redis.smembers".to_string(), FunctionSignature {
            name: "redis.smembers".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "set".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::String))), err: Box::new(Type::String) }),
        });
        env.define_function("redis.publish".to_string(), FunctionSignature {
            name: "redis.publish".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("RedisClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "channel".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "message".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- Tracing Library Functions ---
        env.define_function("tracing.start_span".to_string(), FunctionSignature {
            name: "tracing.start_span".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("Span".to_string())),
        });
        env.define_function("tracing.set_attribute".to_string(), FunctionSignature {
            name: "tracing.set_attribute".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "span".to_string(), param_type: Type::Named("Span".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("tracing.child_span".to_string(), FunctionSignature {
            name: "tracing.child_span".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "parent".to_string(), param_type: Type::Named("Span".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("Span".to_string())),
        });
        env.define_function("tracing.end_span".to_string(), FunctionSignature {
            name: "tracing.end_span".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "span".to_string(), param_type: Type::Named("Span".to_string()) }],
            return_type: Some(Type::Void),
        });
        env.define_function("tracing.export".to_string(), FunctionSignature {
            name: "tracing.export".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "format".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- Encoding Module ---
        env.define_function("encoding.base64_encode".to_string(), FunctionSignature {
            name: "encoding.base64_encode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("encoding.base64_decode".to_string(), FunctionSignature {
            name: "encoding.base64_decode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encoding.url_encode".to_string(), FunctionSignature {
            name: "encoding.url_encode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("encoding.url_decode".to_string(), FunctionSignature {
            name: "encoding.url_decode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("encoding.hex_encode".to_string(), FunctionSignature {
            name: "encoding.hex_encode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("encoding.hex_decode".to_string(), FunctionSignature {
            name: "encoding.hex_decode".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "input".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encoding.is_valid_utf8".to_string(), FunctionSignature {
            name: "encoding.is_valid_utf8".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "bytes".to_string(), param_type: Type::List(Box::new(Type::Number)) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("encoding.fix_utf8".to_string(), FunctionSignature {
            name: "encoding.fix_utf8".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "bytes".to_string(), param_type: Type::List(Box::new(Type::Number)) }],
            return_type: Some(Type::List(Box::new(Type::Number))),
        });

        // --- Queue Module ---
        env.define_function("queue.create".to_string(), FunctionSignature {
            name: "queue.create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "capacity".to_string(), param_type: Type::Optional(Box::new(Type::Number)) }],
            return_type: Some(Type::Named("Queue".to_string())),
        });
        env.define_function("queue.enqueue".to_string(), FunctionSignature {
            name: "queue.enqueue".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("Queue".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "item".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("queue.dequeue".to_string(), FunctionSignature {
            name: "queue.dequeue".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("Queue".to_string()) }],
            return_type: Some(Type::Optional(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("queue.peek".to_string(), FunctionSignature {
            name: "queue.peek".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("Queue".to_string()) }],
            return_type: Some(Type::Optional(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("queue.size".to_string(), FunctionSignature {
            name: "queue.size".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("Queue".to_string()) }],
            return_type: Some(Type::Number),
        });
        env.define_function("queue.is_empty".to_string(), FunctionSignature {
            name: "queue.is_empty".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("Queue".to_string()) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("queue.is_full".to_string(), FunctionSignature {
            name: "queue.is_full".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "queue".to_string(), param_type: Type::Named("Queue".to_string()) }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("queue.priority".to_string(), FunctionSignature {
            name: "queue.priority".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "compare".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("PriorityQueue".to_string())),
        });
        env.define_function("queue.bounded".to_string(), FunctionSignature {
            name: "queue.bounded".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "capacity".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Named("Queue".to_string())),
        });

        // --- MongoDB Module ---
        env.define_function("mongodb.connect".to_string(), FunctionSignature {
            name: "mongodb.connect".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "url".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("MongoClient".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.database".to_string(), FunctionSignature {
            name: "mongodb.database".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "client".to_string(), param_type: Type::Named("MongoClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("Database".to_string())),
        });
        env.define_function("mongodb.collection".to_string(), FunctionSignature {
            name: "mongodb.collection".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "db".to_string(), param_type: Type::Named("Database".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("Collection".to_string())),
        });
        env.define_function("mongodb.insert_one".to_string(), FunctionSignature {
            name: "mongodb.insert_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("Collection".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "doc".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.find".to_string(), FunctionSignature {
            name: "mongodb.find".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("Collection".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.find_one".to_string(), FunctionSignature {
            name: "mongodb.find_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("Collection".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Optional(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.update_one".to_string(), FunctionSignature {
            name: "mongodb.update_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("Collection".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "update".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.delete_one".to_string(), FunctionSignature {
            name: "mongodb.delete_one".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("Collection".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "filter".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.aggregate".to_string(), FunctionSignature {
            name: "mongodb.aggregate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("Collection".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "pipeline".to_string(), param_type: Type::List(Box::new(Type::Named("any".to_string()))) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("mongodb.create_index".to_string(), FunctionSignature {
            name: "mongodb.create_index".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "collection".to_string(), param_type: Type::Named("Collection".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "keys".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "unique".to_string(), param_type: Type::Boolean },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });

        // --- SMTP Module ---
        env.define_function("smtp.connect".to_string(), FunctionSignature {
            name: "smtp.connect".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "config".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("SmtpClient".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("smtp.send".to_string(), FunctionSignature {
            name: "smtp.send".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "mailer".to_string(), param_type: Type::Named("SmtpClient".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "email".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("smtp.template".to_string(), FunctionSignature {
            name: "smtp.template".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "template_path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- CSV Module ---
        env.define_function("csv.read".to_string(), FunctionSignature {
            name: "csv.read".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "has_header".to_string(), param_type: Type::Boolean },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }))), err: Box::new(Type::String) }),
        });
        env.define_function("csv.write".to_string(), FunctionSignature {
            name: "csv.write".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "rows".to_string(), param_type: Type::List(Box::new(Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) })) },
                crate::type_checker::environment::ParameterInfo { name: "headers".to_string(), param_type: Type::Optional(Box::new(Type::List(Box::new(Type::String)))) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("csv.parse".to_string(), FunctionSignature {
            name: "csv.parse".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "csv_string".to_string(), param_type: Type::String }],
            return_type: Some(Type::List(Box::new(Type::List(Box::new(Type::String))))),
        });
        env.define_function("csv.stringify".to_string(), FunctionSignature {
            name: "csv.stringify".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "rows".to_string(), param_type: Type::List(Box::new(Type::List(Box::new(Type::String)))) },
                crate::type_checker::environment::ParameterInfo { name: "headers".to_string(), param_type: Type::List(Box::new(Type::String)) },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("csv.validate".to_string(), FunctionSignature {
            name: "csv.validate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "schema".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });

        // --- YAML Module ---
        env.define_function("yaml.parse".to_string(), FunctionSignature {
            name: "yaml.parse".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "yaml_string".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("yaml.parse_file".to_string(), FunctionSignature {
            name: "yaml.parse_file".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("yaml.stringify".to_string(), FunctionSignature {
            name: "yaml.stringify".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("yaml.write_file".to_string(), FunctionSignature {
            name: "yaml.write_file".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("yaml.validate".to_string(), FunctionSignature {
            name: "yaml.validate".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "schema".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Boolean), err: Box::new(Type::String) }),
        });

        // --- Audit Module ---
        env.define_function("audit.log".to_string(), FunctionSignature {
            name: "audit.log".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "log_data".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("audit.query".to_string(), FunctionSignature {
            name: "audit.query".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "filters".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });
        env.define_function("audit.export".to_string(), FunctionSignature {
            name: "audit.export".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "format".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "filters".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- Encryption Module ---
        env.define_function("encryption.aes_encrypt".to_string(), FunctionSignature {
            name: "encryption.aes_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.aes_decrypt".to_string(), FunctionSignature {
            name: "encryption.aes_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.rsa_generate_keypair".to_string(), FunctionSignature {
            name: "encryption.rsa_generate_keypair".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "bits".to_string(), param_type: Type::Number }],
            return_type: Some(Type::Result { ok: Box::new(Type::Named("any".to_string())), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.rsa_encrypt".to_string(), FunctionSignature {
            name: "encryption.rsa_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "public_key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.rsa_decrypt".to_string(), FunctionSignature {
            name: "encryption.rsa_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "private_key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.fernet_generate_key".to_string(), FunctionSignature {
            name: "encryption.fernet_generate_key".to_string(),
            params: Vec::new(),
            return_type: Some(Type::String),
        });
        env.define_function("encryption.fernet_encrypt".to_string(), FunctionSignature {
            name: "encryption.fernet_encrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.fernet_decrypt".to_string(), FunctionSignature {
            name: "encryption.fernet_decrypt".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "encrypted".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.generate_key".to_string(), FunctionSignature {
            name: "encryption.generate_key".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "algorithm".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.store_key".to_string(), FunctionSignature {
            name: "encryption.store_key".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key_id".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "vault".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("encryption.retrieve_key".to_string(), FunctionSignature {
            name: "encryption.retrieve_key".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "key_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- Alerting Module ---
        env.define_function("alerting.create_rule".to_string(), FunctionSignature {
            name: "alerting.create_rule".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "rule".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("alerting.check".to_string(), FunctionSignature {
            name: "alerting.check".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "metric".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "rules".to_string(), param_type: Type::List(Box::new(Type::Named("any".to_string()))) },
            ],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("alerting.trigger".to_string(), FunctionSignature {
            name: "alerting.trigger".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "alert".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("alerting.history".to_string(), FunctionSignature {
            name: "alerting.history".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "filters".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::List(Box::new(Type::Named("any".to_string())))), err: Box::new(Type::String) }),
        });

        // --- NLP Module ---
        env.define_function("nlp.tokenize".to_string(), FunctionSignature {
            name: "nlp.tokenize".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::List(Box::new(Type::String))),
        });
        env.define_function("nlp.sentiment".to_string(), FunctionSignature {
            name: "nlp.sentiment".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::String),
        });
        env.define_function("nlp.ner".to_string(), FunctionSignature {
            name: "nlp.ner".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String }],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("nlp.keywords".to_string(), FunctionSignature {
            name: "nlp.keywords".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "count".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::List(Box::new(Type::String))),
        });
        env.define_function("nlp.similarity".to_string(), FunctionSignature {
            name: "nlp.similarity".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text1".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "text2".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("nlp.summarize".to_string(), FunctionSignature {
            name: "nlp.summarize".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "text".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "sentences".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::String),
        });

        // --- Workflow Module ---
        env.define_function("workflow.create".to_string(), FunctionSignature {
            name: "workflow.create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "definition".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("workflow.start".to_string(), FunctionSignature {
            name: "workflow.start".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("workflow.execute_step".to_string(), FunctionSignature {
            name: "workflow.execute_step".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "step_id".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("workflow.get_status".to_string(), FunctionSignature {
            name: "workflow.get_status".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::String),
        });
        env.define_function("workflow.get_history".to_string(), FunctionSignature {
            name: "workflow.get_history".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("workflow.complete".to_string(), FunctionSignature {
            name: "workflow.complete".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("workflow.fail".to_string(), FunctionSignature {
            name: "workflow.fail".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "workflow".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "error".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Scheduler Module ---
        env.define_function("scheduler.schedule".to_string(), FunctionSignature {
            name: "scheduler.schedule".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "task".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "cron".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.schedule_interval".to_string(), FunctionSignature {
            name: "scheduler.schedule_interval".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "task".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "interval".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.cancel".to_string(), FunctionSignature {
            name: "scheduler.cancel".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "task_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.list".to_string(), FunctionSignature {
            name: "scheduler.list".to_string(),
            params: Vec::new(),
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("scheduler.get".to_string(), FunctionSignature {
            name: "scheduler.get".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "task_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.enable".to_string(), FunctionSignature {
            name: "scheduler.enable".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "task_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("scheduler.disable".to_string(), FunctionSignature {
            name: "scheduler.disable".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "task_id".to_string(), param_type: Type::String }],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Event Bus Module ---
        env.define_function("event_bus.create".to_string(), FunctionSignature {
            name: "event_bus.create".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Named("EventBus".to_string())),
        });
        env.define_function("event_bus.publish".to_string(), FunctionSignature {
            name: "event_bus.publish".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("EventBus".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "event".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.subscribe".to_string(), FunctionSignature {
            name: "event_bus.subscribe".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("EventBus".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.unsubscribe".to_string(), FunctionSignature {
            name: "event_bus.unsubscribe".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "subscription".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("event_bus.get_history".to_string(), FunctionSignature {
            name: "event_bus.get_history".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "bus".to_string(), param_type: Type::Named("EventBus".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "topic".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "limit".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });

        // --- Fixtures Module ---
        env.define_function("fixtures.create".to_string(), FunctionSignature {
            name: "fixtures.create".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "template".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("fixtures.create_many".to_string(), FunctionSignature {
            name: "fixtures.create_many".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "template".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "count".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::List(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("fixtures.factory".to_string(), FunctionSignature {
            name: "fixtures.factory".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "builder".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("fixtures.build".to_string(), FunctionSignature {
            name: "fixtures.build".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "factory".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "overrides".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Mocks Module ---
        env.define_function("mocks.mock".to_string(), FunctionSignature {
            name: "mocks.mock".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "original".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "mock".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mocks.spy".to_string(), FunctionSignature {
            name: "mocks.spy".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "target".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mocks.verify".to_string(), FunctionSignature {
            name: "mocks.verify".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "spy".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "expected_calls".to_string(), param_type: Type::List(Box::new(Type::Named("any".to_string()))) },
            ],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mocks.reset".to_string(), FunctionSignature {
            name: "mocks.reset".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "spy".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });
        env.define_function("mocks.stub".to_string(), FunctionSignature {
            name: "mocks.stub".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "return_value".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Named("any".to_string())),
        });

        // --- Template Module ---
        env.define_function("template.render".to_string(), FunctionSignature {
            name: "template.render".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "template".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("template.render_file".to_string(), FunctionSignature {
            name: "template.render_file".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("template.partial".to_string(), FunctionSignature {
            name: "template.partial".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "partial_path".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "data".to_string(), param_type: Type::Named("any".to_string()) },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });
        env.define_function("template.cache".to_string(), FunctionSignature {
            name: "template.cache".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "template".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "cache_key".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::String),
        });

        // --- Env Module ---
        env.define_function("env.load".to_string(), FunctionSignature {
            name: "env.load".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "path".to_string(), param_type: Type::String }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("env.get".to_string(), FunctionSignature {
            name: "env.get".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "default".to_string(), param_type: Type::Optional(Box::new(Type::String)) },
            ],
            return_type: Some(Type::String),
        });
        env.define_function("env.get_number".to_string(), FunctionSignature {
            name: "env.get_number".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "default".to_string(), param_type: Type::Number },
            ],
            return_type: Some(Type::Number),
        });
        env.define_function("env.get_bool".to_string(), FunctionSignature {
            name: "env.get_bool".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "default".to_string(), param_type: Type::Boolean },
            ],
            return_type: Some(Type::Boolean),
        });
        env.define_function("env.set".to_string(), FunctionSignature {
            name: "env.set".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("env.validate".to_string(), FunctionSignature {
            name: "env.validate".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "schema".to_string(), param_type: Type::Named("any".to_string()) }],
            return_type: Some(Type::Result { ok: Box::new(Type::Void), err: Box::new(Type::String) }),
        });
        env.define_function("env.get_secret".to_string(), FunctionSignature {
            name: "env.get_secret".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "vault".to_string(), param_type: Type::String },
            ],
            return_type: Some(Type::Result { ok: Box::new(Type::String), err: Box::new(Type::String) }),
        });

        // --- Metrics Module ---
        env.define_function("metrics.increment".to_string(), FunctionSignature {
            name: "metrics.increment".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "labels".to_string(), param_type: Type::Optional(Box::new(Type::String)) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("metrics.gauge".to_string(), FunctionSignature {
            name: "metrics.gauge".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "labels".to_string(), param_type: Type::Optional(Box::new(Type::String)) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("metrics.histogram".to_string(), FunctionSignature {
            name: "metrics.histogram".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "name".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Number },
                crate::type_checker::environment::ParameterInfo { name: "labels".to_string(), param_type: Type::Optional(Box::new(Type::String)) },
            ],
            return_type: Some(Type::Void),
        });

        // --- Cache Module ---
        env.define_function("cache.set".to_string(), FunctionSignature {
            name: "cache.set".to_string(),
            params: vec![
                crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String },
                crate::type_checker::environment::ParameterInfo { name: "value".to_string(), param_type: Type::Named("any".to_string()) },
                crate::type_checker::environment::ParameterInfo { name: "ttl".to_string(), param_type: Type::Optional(Box::new(Type::Number)) },
            ],
            return_type: Some(Type::Void),
        });
        env.define_function("cache.get".to_string(), FunctionSignature {
            name: "cache.get".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String }],
            return_type: Some(Type::Optional(Box::new(Type::Named("any".to_string())))),
        });
        env.define_function("cache.remove".to_string(), FunctionSignature {
            name: "cache.remove".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("cache.clear".to_string(), FunctionSignature {
            name: "cache.clear".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Void),
        });
        env.define_function("cache.exists".to_string(), FunctionSignature {
            name: "cache.exists".to_string(),
            params: vec![crate::type_checker::environment::ParameterInfo { name: "key".to_string(), param_type: Type::String }],
            return_type: Some(Type::Boolean),
        });
        env.define_function("cache.size".to_string(), FunctionSignature {
            name: "cache.size".to_string(),
            params: Vec::new(),
            return_type: Some(Type::Number),
        });

        // Register module variables
        env.define_variable("metrics".to_string(), Type::Named("MetricsStdlib".to_string()));
        env.define_variable("cache".to_string(), Type::Named("CacheStdlib".to_string()));
        env.define_variable("encoding".to_string(), Type::Named("EncodingStdlib".to_string()));
        env.define_variable("queue".to_string(), Type::Named("QueueStdlib".to_string()));
        env.define_variable("mongodb".to_string(), Type::Named("MongoDbStdlib".to_string()));
        env.define_variable("smtp".to_string(), Type::Named("SmtpStdlib".to_string()));
        env.define_variable("csv".to_string(), Type::Named("CsvStdlib".to_string()));
        env.define_variable("yaml".to_string(), Type::Named("YamlStdlib".to_string()));
        env.define_variable("audit".to_string(), Type::Named("AuditStdlib".to_string()));
        env.define_variable("encryption".to_string(), Type::Named("EncryptionStdlib".to_string()));
        env.define_variable("alerting".to_string(), Type::Named("AlertingStdlib".to_string()));
        env.define_variable("nlp".to_string(), Type::Named("NlpStdlib".to_string()));
        env.define_variable("workflow".to_string(), Type::Named("WorkflowStdlib".to_string()));
        env.define_variable("scheduler".to_string(), Type::Named("SchedulerStdlib".to_string()));
        env.define_variable("event_bus".to_string(), Type::Named("EventBusStdlib".to_string()));
        env.define_variable("fixtures".to_string(), Type::Named("FixturesStdlib".to_string()));
        env.define_variable("mocks".to_string(), Type::Named("MocksStdlib".to_string()));
        env.define_variable("template".to_string(), Type::Named("TemplateStdlib".to_string()));
        env.define_variable("env".to_string(), Type::Named("EnvStdlib".to_string()));

    
        TypeChecker {
            environment: env,
            errors: Vec::new(),
        }
    }

    fn flatten_member_access(&self, expr: &Expression) -> Option<String> {
        match expr {
            Expression::Identifier(name) => Some(name.clone()),
            Expression::Member { object, member } => {
                let obj_name = self.flatten_member_access(object)?;
                Some(format!("{}.{}", obj_name, member))
            }
            _ => None,
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
        // First pass: collect all type definitions
        for item in &program.items {
            match item {
                Item::Struct(s) => {
                    // Register generic type parameters as valid types in the struct scope
                    // Type parameters are valid types within the struct definition
                    // We don't need to register them globally, but we need to track them
                    for _type_param in &s.type_params {
                        // Type parameters will be validated when used in field types
                    }
                    self.environment.define_type(
                        s.name.clone(),
                        Type::Named(s.name.clone()),
                    );
                    // Store struct definition for field access
                    self.environment.define_struct(s.name.clone(), s.clone());
                }
                Item::Enum(e) => {
                    self.environment.define_type(
                        e.name.clone(),
                        Type::Named(e.name.clone()),
                    );
                    // Store enum definition for variant access
                    self.environment.define_enum(e.name.clone(), e.clone());
                }
                Item::TypeAlias(ta) => {
                    self.environment.define_type(
                        ta.name.clone(),
                        ta.aliased_type.clone(),
                    );
                }
                Item::Trait(t) => {
                    // Register trait as a type
                    self.environment.define_type(
                        t.name.clone(),
                        Type::Named(t.name.clone()),
                    );
                }
                _ => {}
            }
        }
        
        // Second pass: register function signatures
        for item in &program.items {
            if let Item::Function(f) = item {
                let params: Vec<_> = f.params.iter().map(|p| {
                    crate::type_checker::environment::ParameterInfo {
                        name: p.name.clone(),
                        param_type: p.param_type.clone(),
                    }
                }).collect();
                
                let sig = crate::type_checker::environment::FunctionSignature {
                    name: f.name.clone(),
                    params,
                    return_type: f.return_type.clone(),
                };
                
                self.environment.define_function(f.name.clone(), sig);
            }
        }
        
        // Debug: print registered functions
        // println!("Registered functions: {:?}", self.environment.get_all_function_names());
        
        // Pass 2.5: Register modules and their contents (recursive Pass 1 & 2)
        // This ensures modules are available before they are used in other modules
        for item in &program.items {
            if let Item::Module(m) = item {
                 let mut module_env = Environment::with_parent(self.environment.clone());
                 
                 // Register types in module
                 for item in &m.items {
                     match item {
                         Item::Struct(s) => {
                             module_env.define_type(s.name.clone(), Type::Named(s.name.clone()));
                             module_env.define_struct(s.name.clone(), s.clone());
                         }
                         Item::Enum(e) => {
                             module_env.define_type(e.name.clone(), Type::Named(e.name.clone()));
                             module_env.define_enum(e.name.clone(), e.clone());
                         }
                          Item::Trait(t) => {
                             module_env.define_type(t.name.clone(), Type::Named(t.name.clone()));
                         }
                         Item::TypeAlias(ta) => {
                             module_env.define_type(ta.name.clone(), ta.aliased_type.clone());
                         }
                         _ => {}
                     }
                 }
                 
                 // Register functions in module
                 for item in &m.items {
                     if let Item::Function(f) = item {
                          let params: Vec<_> = f.params.iter().map(|p| {
                             crate::type_checker::environment::ParameterInfo {
                                 name: p.name.clone(),
                                 param_type: p.param_type.clone(),
                             }
                         }).collect();
                         
                         let sig = crate::type_checker::environment::FunctionSignature {
                              name: f.name.clone(),
                              params,
                              return_type: f.return_type.clone(),
                          };
                          module_env.define_function(f.name.clone(), sig);
                     }
                 }

                 // Register module in parent environment
                 self.environment.define_module(m.name.clone(), module_env);
            }
        }

        // Third pass: check functions and other items
        for item in &program.items {
            match item {
                Item::Function(f) => {
                    self.check_function(f)?;
                }
                Item::Struct(s) => {
                    self.check_struct(s)?;
                }
                Item::Enum(e) => {
                    self.check_enum(e)?;
                }
                Item::TypeAlias(_) => {
                    // Already handled in first pass
                }
                Item::Trait(t) => {
                    self.check_trait(t)?;
                }
                Item::Impl(i) => {
                    self.check_impl(i)?;
                }
                Item::Module(m) => {
                    // Handle modules by checking their items recursively
                    // Use the environment created in Pass 2.5
                    let mut module_env = self.environment.get_module(&m.name).unwrap(); // Should exist
                    
                    // Update parent to current environment (containing all registered modules)
                    module_env.set_parent(self.environment.clone());
                    
                    let old_env = std::mem::replace(&mut self.environment, module_env);
                    
                    // Check all items in the module
                    for item in &m.items {
                        match item {
                            Item::Function(f) => {
                                self.check_function(f)?;
                            }
                            Item::Struct(s) => {
                                self.check_struct(s)?;
                            }
                            Item::Enum(e) => {
                                self.check_enum(e)?;
                            }
                             Item::Trait(t) => {
                                self.check_trait(t)?;
                            }
                            Item::Impl(i) => {
                                self.check_impl(i)?;
                            }
                            Item::Use(u) => {
                                 // Import types/functions from module into current environment (module scope)
                                 let module_name = u.path.join(".");
                                 if let Some(module_env) = self.environment.get_module(&module_name) {
                                     for (name, type_def) in &module_env.types { self.environment.define_type(name.clone(), type_def.clone()); }
                                     for (name, struct_def) in &module_env.structs { self.environment.define_struct(name.clone(), struct_def.clone()); }
                                     for (name, enum_def) in &module_env.enums { self.environment.define_enum(name.clone(), enum_def.clone()); }
                                     for (name, func_sig) in &module_env.functions { self.environment.define_function(name.clone(), func_sig.clone()); }
                                 }
                            }
                            // Recursive modules not fully supported here without refactoring
                            _ => {}
                        }
                    }
                    
                    self.environment = old_env;
                }
                Item::Use(u) => {
                     // Import types/functions from module into current environment
                     let module_name = u.path.join(".");
                     if let Some(module_env) = self.environment.get_module(&module_name) {
                         // Import types
                         for (name, type_def) in &module_env.types {
                             self.environment.define_type(name.clone(), type_def.clone());
                         }
                         // Import structs
                         for (name, struct_def) in &module_env.structs {
                             self.environment.define_struct(name.clone(), struct_def.clone());
                         }
                         // Import enums
                         for (name, enum_def) in &module_env.enums {
                             self.environment.define_enum(name.clone(), enum_def.clone());
                         }
                         // Import functions
                         for (name, func_sig) in &module_env.functions {
                             self.environment.define_function(name.clone(), func_sig.clone());
                         }
                     }
                }
            }
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    fn check_function(&mut self, function: &Function) -> Result<(), Vec<TypeError>> {
        // Validate decorators
        for decorator in &function.decorators {
            if is_rate_limit_decorator(&decorator.name) {
                self.validate_rate_limit_decorator(decorator)?;
            }
        }
        
        let mut env = Environment::with_parent(self.environment.clone());
        
        // Add parameters to environment
        for param in &function.params {
            if env.has_variable(&param.name) {
                self.errors.push(TypeError::new(
                    TypeErrorKind::DuplicateDefinition(param.name.clone()),
                    format!("Duplicate parameter: {}", param.name),
                ));
            } else {
                env.define_variable(param.name.clone(), param.param_type.clone());
            }
        }
        
        // Check function body
        let old_env = std::mem::replace(&mut self.environment, env);
        let return_type = self.check_block(&function.body, function.return_type.as_ref())?;
        self.environment = old_env;
        
        // Check return type
        if let Some(expected_return) = &function.return_type {
            // Special handling: Named type from struct literal should be compatible with Generic type of same name
            let is_compatible = match (&return_type, expected_return) {
                (Type::Named(n1), Type::Generic { name: n2, .. }) if n1 == n2 => {
                    // Struct literal (Named) is compatible with generic return type (Generic)
                    // This allows returning ApiResponse { ... } where ApiResponse<void> is expected
                    true
                }
                _ => self.types_compatible(&return_type, expected_return)
            };
            
            if !is_compatible {
                self.errors.push(TypeError::type_mismatch(
                    &expected_return.to_string(),
                    &return_type.to_string(),
                ));
            }
        } else if return_type != Type::Void {
            // Function has return type but no explicit return type annotation
            // This is okay for inference
        }
        
        Ok(())
    }
    
    /// Validiert @RateLimit Decorator Argumente
    fn validate_rate_limit_decorator(&mut self, decorator: &Decorator) -> Result<(), Vec<TypeError>> {
        // Prüfe ob Konfiguration geparst werden kann
        if parse_rate_limit_config(&decorator.args).is_none() {
            self.errors.push(TypeError::invalid_operation(
                "@RateLimit",
                "Invalid decorator arguments",
            ));
            return Ok(());
        }
        
        // Validiere einzelne Argumente
        for arg in &decorator.args {
            match arg {
                DecoratorArg::Named { name, value } => {
                    match name.as_str() {
                        "requests" => {
                            if !matches!(value.as_ref(), DecoratorArg::Number(_)) {
                                self.errors.push(TypeError::type_mismatch(
                                    "number",
                                    match value.as_ref() {
                                        DecoratorArg::String(_) => "string",
                                        DecoratorArg::Boolean(_) => "boolean",
                                        DecoratorArg::Identifier(_) => "identifier",
                                        _ => "unknown",
                                    },
                                ));
                            }
                        }
                        "window" => {
                            if !matches!(value.as_ref(), DecoratorArg::String(_)) {
                                self.errors.push(TypeError::type_mismatch(
                                    "string",
                                    match value.as_ref() {
                                        DecoratorArg::Number(_) => "number",
                                        DecoratorArg::Boolean(_) => "boolean",
                                        DecoratorArg::Identifier(_) => "identifier",
                                        _ => "unknown",
                                    },
                                ));
                            }
                        }
                        "strategy" => {
                            if !matches!(value.as_ref(), DecoratorArg::String(_)) {
                                self.errors.push(TypeError::type_mismatch(
                                    "string",
                                    match value.as_ref() {
                                        DecoratorArg::Number(_) => "number",
                                        DecoratorArg::Boolean(_) => "boolean",
                                        DecoratorArg::Identifier(_) => "identifier",
                                        _ => "unknown",
                                    },
                                ));
                            } else if let DecoratorArg::String(s) = value.as_ref() {
                                let valid_strategies = ["fixed-window", "fixedWindow", "sliding-window", "slidingWindow", "token-bucket", "tokenBucket"];
                                if !valid_strategies.contains(&s.as_str()) {
                                    self.errors.push(TypeError::invalid_operation(
                                        "@RateLimit",
                                        &format!("Invalid strategy '{}'. Must be one of: fixed-window, sliding-window, token-bucket", s),
                                    ));
                                }
                            }
                        }
                        "key" => {
                            if !matches!(value.as_ref(), DecoratorArg::String(_)) {
                                self.errors.push(TypeError::type_mismatch(
                                    "string",
                                    match value.as_ref() {
                                        DecoratorArg::Number(_) => "number",
                                        DecoratorArg::Boolean(_) => "boolean",
                                        DecoratorArg::Identifier(_) => "identifier",
                                        _ => "unknown",
                                    },
                                ));
                            }
                        }
                        _ => {
                            self.errors.push(TypeError::invalid_operation(
                                "@RateLimit",
                                &format!("Unknown argument '{}'. Valid arguments are: requests, window, strategy, key", name),
                            ));
                        }
                    }
                }
                _ => {
                    // Positionale Argumente werden nicht unterstützt
                    self.errors.push(TypeError::invalid_operation(
                        "@RateLimit",
                        "Only named arguments are supported (e.g., requests: 100, window: \"1m\")",
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    fn check_struct(&mut self, struct_def: &Struct) -> Result<(), Vec<TypeError>> {
        // Check that all field types are valid
        for field in &struct_def.fields {
            self.check_type(&field.field_type)?;
        }
        Ok(())
    }
    
    fn check_enum(&mut self, enum_def: &Enum) -> Result<(), Vec<TypeError>> {
        // Check that all variant types are valid
        for variant in &enum_def.variants {
            if let Some(ref types) = variant.data {
                for variant_type in types {
                    self.check_type(variant_type)?;
                }
            }
        }
        Ok(())
    }
    
    fn check_trait(&mut self, trait_def: &Trait) -> Result<(), Vec<TypeError>> {
        // Check trait methods
        for method in &trait_def.methods {
            // Check parameter types
            for param in &method.params {
                self.check_type(&param.param_type)?;
            }
            // Check return type
            if let Some(ref return_type) = method.return_type {
                self.check_type(return_type)?;
            }
        }
        Ok(())
    }
    
    fn check_impl(&mut self, impl_def: &Impl) -> Result<(), Vec<TypeError>> {
        // Check that the trait exists (if not blank impl)
        if !impl_def.trait_name.is_empty() {
            if !self.environment.has_type(&impl_def.trait_name) {
                self.errors.push(TypeError::undefined_type(&impl_def.trait_name));
            }
        }
        
        // Check the type being implemented
        self.check_type(&impl_def.for_type)?;
        
        // Check impl methods
        for method in &impl_def.methods {
            self.check_function(method)?;
        }
        
        Ok(())
    }
    
    fn check_block(
        &mut self,
        block: &Block,
        expected_return: Option<&Type>,
    ) -> Result<Type, Vec<TypeError>> {
        let mut return_type = Type::Void;
        
        for statement in &block.statements {
            match statement {
                Statement::Let(let_stmt) => {
                    let value_type = self.check_expression(&let_stmt.value)?;
                    
                    if let Some(ref var_type) = let_stmt.var_type {
                        if !self.types_compatible(&value_type, var_type) {
                            self.errors.push(TypeError::type_mismatch(
                                &var_type.to_string(),
                                &value_type.to_string(),
                            ));
                        }
                        
                        if self.environment.has_variable(&let_stmt.name) {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::DuplicateDefinition(let_stmt.name.clone()),
                                format!("Variable '{}' already defined", let_stmt.name),
                            ));
                        } else {
                            self.environment.define_variable(let_stmt.name.clone(), var_type.clone());
                        }
                    } else {
                        // Type inference
                        if value_type == Type::Void {
                            // Try to infer from constructor calls (e.g., HttpClient.new())
                            let mut inferred = false;
                            if let Expression::Call { callee, args: _ } = &let_stmt.value {
                                if let Expression::Member { object, member } = callee.as_ref() {
                                    if member == "new" {
                                        if let Expression::Identifier(class_name) = object.as_ref() {
                                            // Check if it's a registered Standard Library class constructor
                                            let constructor_name = format!("{}.new", class_name);
                                            if let Some(sig) = self.environment.get_function(&constructor_name) {
                                                // Use the constructor's return type
                                                let inferred_type = sig.return_type.unwrap_or(Type::Void);
                                                if inferred_type != Type::Void {
                                                    self.environment.define_variable(let_stmt.name.clone(), inferred_type);
                                                    inferred = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if !inferred {
                                self.errors.push(TypeError::cannot_infer_type());
                            }
                        } else {
                            self.environment.define_variable(let_stmt.name.clone(), value_type.clone());
                        }
                    }
                }
                Statement::Return(ret_stmt) => {
                    if let Some(ref value) = ret_stmt.value {
                        return_type = self.check_expression(value)?;
                    } else {
                        return_type = Type::Void;
                    }
                }
                Statement::Expression(expr_stmt) => {
                    self.check_expression(&expr_stmt.expression)?;
                }
                Statement::If(if_stmt) => {
                    let condition_type = self.check_expression(&if_stmt.condition)?;
                    if condition_type != Type::Boolean {
                        self.errors.push(TypeError::type_mismatch(
                            "boolean",
                            &condition_type.to_string(),
                        ));
                    }
                    
                    self.check_block(&if_stmt.then_block, expected_return)?;
                    if let Some(ref else_block) = if_stmt.else_block {
                        self.check_block(else_block, expected_return)?;
                    }
                }
                Statement::For(for_stmt) => {
                    let iterable_type = self.check_expression(&for_stmt.iterable)?;
                    
                    // Check if iterable_type is iterable (List, Array, etc.)
                    let element_type = match iterable_type {
                        Type::List(ref item_type) => item_type.as_ref().clone(),
                        Type::Generic { name, params } if name == "List" && !params.is_empty() => {
                            params[0].clone()
                        }
                        Type::String => Type::String, // String is iterable (characters)
                        _ => {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::TypeMismatch {
                                    expected: "List<T> or String".to_string(),
                                    found: iterable_type.to_string(),
                                },
                                format!("Type '{}' is not iterable", iterable_type.to_string()),
                            ));
                            Type::Void
                        }
                    };
                    
                    // Create new scope for loop variable
                    let parent_env = self.environment.clone();
                    let old_env = std::mem::replace(
                        &mut self.environment,
                        Environment::with_parent(parent_env),
                    );
                    
                    self.environment.define_variable(for_stmt.variable.clone(), element_type);
                    self.check_block(&for_stmt.body, expected_return)?;
                    self.environment = old_env;
                }
                Statement::While(while_stmt) => {
                    let condition_type = self.check_expression(&while_stmt.condition)?;
                    if condition_type != Type::Boolean {
                        self.errors.push(TypeError::type_mismatch(
                            "boolean",
                            &condition_type.to_string(),
                        ));
                    }
                    self.check_block(&while_stmt.body, expected_return)?;
                }
                Statement::Match(match_stmt) => {
                    let match_type = self.check_expression(&match_stmt.expression)?;
                    for arm in &match_stmt.arms {
                        // Check pattern matching types
                        let pattern_env = self.check_pattern(&arm.pattern, &match_type)?;
                        
                        // Check guard if present
                        if let Some(ref guard) = arm.guard {
                            let guard_type = self.check_expression(guard)?;
                            if guard_type != Type::Boolean {
                                self.errors.push(TypeError::type_mismatch(
                                    "boolean",
                                    &guard_type.to_string(),
                                ));
                            }
                        }
                        
                        // Check body with pattern bindings in scope
                        let _parent_env = self.environment.clone();
                        let old_env = std::mem::replace(
                            &mut self.environment,
                            pattern_env,
                        );
                        self.check_block(&arm.body, expected_return)?;
                        self.environment = old_env;
                    }
                }
                Statement::Throw(throw_stmt) => {
                    let _ = self.check_expression(&throw_stmt.expression)?;
                    // Treat throw as returning the expected type (since it never returns)
                    if let Some(expected) = expected_return {
                        return_type = expected.clone();
                    }
                }
                Statement::Break(_) => {
                    // Valid in loops
                }
            }
        }
        
        Ok(return_type)
    }
    
    fn check_struct_literal(&mut self, name: &str, fields: &[(String, Expression)]) -> Result<Type, Vec<TypeError>> {
        if self.environment.get_type(name).is_some() {
             for (_, expr) in fields {
                 let _ = self.check_expression(expr)?;
             }
             Ok(Type::Named(name.to_string()))
        } else {
             self.errors.push(TypeError::undefined_type(name));
             Ok(Type::Void)
        }
    }

    fn check_list_literal(&mut self, elements: &[Expression]) -> Result<Type, Vec<TypeError>> {
        if elements.is_empty() {
            return Ok(Type::List(Box::new(Type::Void)));
        }
        let first_type = self.check_expression(&elements[0])?;
        for expr in &elements[1..] {
            let _ = self.check_expression(expr)?;
        }
        Ok(Type::List(Box::new(first_type)))
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<Type, Vec<TypeError>> {
        match expr {
            Expression::Literal(lit) => Ok(self.literal_type(lit)),
            Expression::Identifier(name) => {
                if let Some(var_type) = self.environment.get_variable(name) {
                    Ok(var_type)
                } else if let Some(func_sig) = self.environment.get_function(name) {
                    // Identifier is a function name - return its return type or Function type
                    // This allows functions to be referenced (though they should usually be called)
                    Ok(func_sig.return_type.unwrap_or(Type::Void))
                } else if self.environment.get_type(name).is_some() {
                    // Support using Type names as values (e.g. for reflection/DB calls)
                    // We treat them as a special "Type" type or String for now
                    Ok(Type::Named("Type".to_string()))
                } else if self.environment.get_module(name).is_some() {
                    Ok(Type::Named(format!("Module:{}", name)))
                } else {
                    self.errors.push(TypeError::undefined_variable(name));
                    Ok(Type::Void) // Return error type
                }
            }
            Expression::BinaryOp { left, op, right } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;
                self.check_binary_operation(op, &left_type, &right_type)
            }
            Expression::UnaryOp { op, expr } => {
                let expr_type = self.check_expression(expr)?;
                self.check_unary_operation(op, &expr_type)
            }
            Expression::Assignment { target, value } => {
                let _target_type = self.check_expression(target)?;
                let value_type = self.check_expression(value)?;
                Ok(value_type)
            }
            Expression::StructLiteral { name, fields } => {
                self.check_struct_literal(name, fields)
            }
            Expression::MapLiteral(fields) => {
                for (_, expr) in fields {
                    let _ = self.check_expression(expr)?;
                }
                Ok(Type::Map {
                    key: Box::new(Type::String),
                    value: Box::new(Type::Any)
                })
            }
            Expression::ListLiteral(elements) => {
                self.check_list_literal(elements)
            }
            Expression::GenericConstructor { name, type_params, args } => {
                // Handle generic type constructors like Map<string, string>() or List<string>()
                match name.as_str() {
                    "Map" => {
                        if type_params.len() == 2 {
                            let key_type = &type_params[0];
                            let value_type = &type_params[1];
                            self.check_type(key_type)?;
                            self.check_type(value_type)?;
                            // Check constructor arguments (should be empty for Map())
                            for arg in args {
                                let _ = self.check_expression(&arg)?;
                            }
                            return Ok(Type::Map {
                                key: Box::new(key_type.clone()),
                                value: Box::new(value_type.clone()),
                            });
                        } else {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::InvalidArgumentType {
                                    position: 0,
                                    expected: "Map<K, V> requires 2 type parameters".to_string(),
                                    found: format!("{} type parameters", type_params.len()),
                                },
                                format!("Map requires 2 type parameters, found {}", type_params.len()),
                            ));
                            return Ok(Type::Void);
                        }
                    }
                    "List" => {
                        if type_params.len() == 1 {
                            let item_type = &type_params[0];
                            self.check_type(item_type)?;
                            // Check constructor arguments
                            for arg in args {
                                let _ = self.check_expression(&arg)?;
                            }
                            return Ok(Type::List(Box::new(item_type.clone())));
                        } else {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::InvalidArgumentType {
                                    position: 0,
                                    expected: "List<T> requires 1 type parameter".to_string(),
                                    found: format!("{} type parameters", type_params.len()),
                                },
                                format!("List requires 1 type parameter, found {}", type_params.len()),
                            ));
                            return Ok(Type::Void);
                        }
                    }
                    _ => {
                        // Check if it's a struct constructor with generics
                        if let Some(_struct_type) = self.environment.get_type(name) {
                            // Validate type parameters
                            for type_param in type_params {
                                self.check_type(type_param)?;
                            }
                            // Check constructor arguments
                            for arg in args {
                                let _ = self.check_expression(&arg)?;
                            }
                            return Ok(Type::Generic {
                                name: name.clone(),
                                params: type_params.clone(),
                            });
                        } else {
                            self.errors.push(TypeError::undefined_type(name));
                            return Ok(Type::Void);
                        }
                    }
                }
            }
            Expression::Call { callee, args } => {
                // Handle constructor calls and module calls
                // Try to flatten member access (e.g. agent.memory.store -> "agent.memory.store")
                if let Some(full_name) = self.flatten_member_access(callee.as_ref()) {
                    if let Some(sig) = self.environment.get_function(&full_name) {
                        // Check argument count
                        if args.len() != sig.params.len() {
                            self.errors.push(TypeError::wrong_argument_count(
                                sig.params.len(),
                                args.len(),
                            ));
                        } else {
                            // Check argument types
                            for (i, (arg, param)) in args.iter().zip(sig.params.iter()).enumerate() {
                                let arg_type = self.check_expression(arg)?;
                                if !self.types_compatible(&arg_type, &param.param_type) {
                                    self.errors.push(TypeError::new(
                                        TypeErrorKind::InvalidArgumentType {
                                            position: i,
                                            expected: param.param_type.to_string(),
                                            found: arg_type.to_string(),
                                        },
                                        format!(
                                            "Argument {}: expected {}, found {}",
                                            i + 1,
                                            param.param_type.to_string(),
                                            arg_type.to_string()
                                        ),
                                    ));
                                }
                            }
                        }
                        
                        return Ok(sig.return_type.unwrap_or(Type::Void));
                    }
                }

                // Check for method calls on types (Map, List, etc.)
                if let Expression::Member { object, member } = callee.as_ref() {
                    let object_type = self.check_expression(object)?;
                    
                    match &object_type {
                        Type::Map { key, value } => {
                            if member == "insert" {
                                if args.len() != 2 {
                                    self.errors.push(TypeError::wrong_argument_count(2, args.len()));
                                } else {
                                    let key_arg_type = self.check_expression(&args[0])?;
                                    let value_arg_type = self.check_expression(&args[1])?;
                                    
                                    if !self.types_compatible(&key_arg_type, key) {
                                        self.errors.push(TypeError::type_mismatch(&key.to_string(), &key_arg_type.to_string()));
                                    }
                                    if !self.types_compatible(&value_arg_type, value) {
                                        self.errors.push(TypeError::type_mismatch(&value.to_string(), &value_arg_type.to_string()));
                                    }
                                }
                                return Ok(Type::Void);
                            } else if member == "get" {
                                if args.len() != 1 {
                                    self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                                } else {
                                    let key_arg_type = self.check_expression(&args[0])?;
                                    if !self.types_compatible(&key_arg_type, key) {
                                        self.errors.push(TypeError::type_mismatch(&key.to_string(), &key_arg_type.to_string()));
                                    }
                                }
                                return Ok(Type::Optional(value.clone()));
                            } else if member == "contains" {
                                if args.len() != 1 {
                                    self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                                } else {
                                    let key_arg_type = self.check_expression(&args[0])?;
                                    if !self.types_compatible(&key_arg_type, key) {
                                        self.errors.push(TypeError::type_mismatch(&key.to_string(), &key_arg_type.to_string()));
                                    }
                                }
                                return Ok(Type::Boolean);
                            }
                        }
                        Type::List(item_type) => {
                            if member == "push" {
                                if args.len() != 1 {
                                    self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                                } else {
                                    let arg_type = self.check_expression(&args[0])?;
                                    if !self.types_compatible(&arg_type, item_type) {
                                        self.errors.push(TypeError::type_mismatch(&item_type.to_string(), &arg_type.to_string()));
                                    }
                                }
                                return Ok(Type::Void);
                            } else if member == "len" { // len() method
                                return Ok(Type::Number);
                            }
                        }
                        Type::String => {
                             if member == "startsWith" || member == "endsWith" || member == "contains" {
                                 if args.len() != 1 {
                                     self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                                 } else {
                                     let arg_type = self.check_expression(&args[0])?;
                                     if arg_type != Type::String {
                                         self.errors.push(TypeError::type_mismatch("string", &arg_type.to_string()));
                                     }
                                 }
                                 return Ok(Type::Boolean);
                             } else if member == "length" { // length property access handled in Member? No, it might be a method in some contexts or property
                                 // If it's a property, it should be handled in Expression::Member, not Call. 
                                 // But if the user calls it like string.length(), it's here.
                                 return Ok(Type::Number);
                             }
                        }
                        _ => {}
                    }
                }

                if let Expression::Member { object, member } = callee.as_ref() {
                    if member == "new" {
                        if let Expression::Identifier(class_name) = object.as_ref() {
                            // Check if it's a registered Standard Library class constructor
                            let constructor_name = format!("{}.new", class_name);
                            if let Some(sig) = self.environment.get_function(&constructor_name) {
                                // Check argument count
                                if args.len() != sig.params.len() {
                                    self.errors.push(TypeError::wrong_argument_count(
                                        sig.params.len(),
                                        args.len(),
                                    ));
                                } else {
                                    // Check argument types
                                    for (i, (arg, param)) in args.iter().zip(sig.params.iter()).enumerate() {
                                        let arg_type = self.check_expression(arg)?;
                                        if !self.types_compatible(&arg_type, &param.param_type) {
                                            self.errors.push(TypeError::new(
                                                TypeErrorKind::InvalidArgumentType {
                                                    position: i,
                                                    expected: param.param_type.to_string(),
                                                    found: arg_type.to_string(),
                                                },
                                                format!(
                                                    "Argument {}: expected {}, found {}",
                                                    i + 1,
                                                    param.param_type.to_string(),
                                                    arg_type.to_string()
                                                ),
                                            ));
                                        }
                                    }
                                }
                                return Ok(sig.return_type.unwrap_or(Type::Void));
                            }
                        }
                    }
                    
                    let obj_type = self.check_expression(object)?;
                    
                    // Handle Standard Library class method calls in Call expression
                    if let Type::Named(ref class_name) = obj_type {
                        match class_name.as_str() {
                            "HttpClient" => {
                                match member.as_str() {
                                    "get" | "post" | "put" | "delete" | "patch" => {
                                        // Check arguments
                                        if args.len() >= 1 {
                                            let url_type = self.check_expression(&args[0])?;
                                            if url_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &url_type.to_string()));
                                            }
                                        }
                                        return Ok(Type::Named("HttpResponse".to_string()));
                                    }
                                    _ => {}
                                }
                            }
                            "Validator" => {
                                match member.as_str() {
                                    "required" | "minLength" | "maxLength" | "email" | "pattern" | "min" | "max" | "range" | "custom" => {
                                        // Fluent interface - returns self
                                        return Ok(Type::Named("Validator".to_string()));
                                    }
                                    "isValid" => {
                                        if !args.is_empty() {
                                            self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                        }
                                        return Ok(Type::Boolean);
                                    }
                                    "errors" => {
                                        if !args.is_empty() {
                                            self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                        }
                                        return Ok(Type::List(Box::new(Type::Named("ValidationError".to_string()))));
                                    }
                                    _ => {}
                                }
                            }
                            "AuthService" => {
                                match member.as_str() {
                                    "generateToken" => {
                                        // Check argument is UserClaims
                                        if args.len() == 1 {
                                            let _claims_type = self.check_expression(&args[0])?;
                                        } else {
                                            self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                                        }
                                        return Ok(Type::Named("JWTToken".to_string()));
                                    }
                                    "verifyToken" => {
                                        if args.len() == 1 {
                                            let token_type = self.check_expression(&args[0])?;
                                            if token_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &token_type.to_string()));
                                            }
                                        } else {
                                            self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                                        }
                                        return Ok(Type::Optional(Box::new(Type::Named("UserClaims".to_string()))));
                                    }
                                    "extractUserId" => {
                                        if args.len() == 1 {
                                            let token_type = self.check_expression(&args[0])?;
                                            if token_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &token_type.to_string()));
                                            }
                                        } else {
                                            self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                                        }
                                        return Ok(Type::Optional(Box::new(Type::String)));
                                    }
                                    "hasRole" => {
                                        if args.len() == 2 {
                                            let _token_type = self.check_expression(&args[0])?;
                                            let role_type = self.check_expression(&args[1])?;
                                            if role_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &role_type.to_string()));
                                            }
                                        } else {
                                            self.errors.push(TypeError::wrong_argument_count(2, args.len()));
                                        }
                                        return Ok(Type::Boolean);
                                    }
                                    _ => {}
                                }
                            }
                            "Logger" | "VelinLogger" => {
                                match member.as_str() {
                                    "info" | "debug" | "warn" | "error" | "trace" | "log" => {
                                        // Logging methods take a string message
                                        if args.len() == 1 {
                                            let msg_type = self.check_expression(&args[0])?;
                                            if msg_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &msg_type.to_string()));
                                            }
                                        }
                                        return Ok(Type::Void);
                                    }
                                    _ => {}
                                }
                            }
                            "MetricsCollector" => {
                                match member.as_str() {
                                    "incrementCounter" | "setGauge" | "observeHistogram" => {
                                        // These methods take name and optional labels
                                        if args.len() >= 1 {
                                            let name_type = self.check_expression(&args[0])?;
                                            if name_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &name_type.to_string()));
                                            }
                                        }
                                        return Ok(Type::Void);
                                    }
                                    "getMetrics" => {
                                        if !args.is_empty() {
                                            self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                        }
                                        return Ok(Type::List(Box::new(Type::Named("Metric".to_string()))));
                                    }
                                    "getMetric" => {
                                        if args.len() >= 1 {
                                            let _name_type = self.check_expression(&args[0])?;
                                        }
                                        return Ok(Type::Optional(Box::new(Type::Named("Metric".to_string()))));
                                    }
                                    "exportPrometheus" => {
                                        if !args.is_empty() {
                                            self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                        }
                                        return Ok(Type::String);
                                    }
                                    _ => {}
                                }
                            }
                            "PerformanceMonitor" => {
                                match member.as_str() {
                                    "startOperation" | "endOperation" => {
                                        if args.len() == 1 {
                                            let name_type = self.check_expression(&args[0])?;
                                            if name_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &name_type.to_string()));
                                            }
                                        }
                                        return Ok(Type::Void);
                                    }
                                    "collector" => {
                                        if !args.is_empty() {
                                            self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                        }
                                        return Ok(Type::Named("MetricsCollector".to_string()));
                                    }
                                    _ => {}
                                }
                            }
                            "LLMClient" => {
                                match member.as_str() {
                                    "generate" | "complete" => {
                                        if args.len() >= 1 {
                                            let prompt_type = self.check_expression(&args[0])?;
                                            if prompt_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &prompt_type.to_string()));
                                            }
                                        }
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::String),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "embed" => {
                                        if args.len() >= 1 {
                                            let text_type = self.check_expression(&args[0])?;
                                            if text_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &text_type.to_string()));
                                            }
                                        }
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::List(Box::new(Type::Number))),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            "ModelLoader" => {
                                match member.as_str() {
                                    "loadModel" => {
                                        // loadModel(name, type, path)
                                        if args.len() >= 3 {
                                            let _name_type = self.check_expression(&args[0])?;
                                            let _type_type = self.check_expression(&args[1])?;
                                            let path_type = self.check_expression(&args[2])?;
                                            if path_type != Type::String {
                                                self.errors.push(TypeError::type_mismatch("string", &path_type.to_string()));
                                            }
                                        }
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Void),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "predict" => {
                                        if args.len() >= 2 {
                                            let _name_type = self.check_expression(&args[0])?;
                                            let _input_type = self.check_expression(&args[1])?;
                                        }
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::String),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            "TrainingService" => {
                                match member.as_str() {
                                    "addExample" => {
                                        if args.len() == 2 {
                                            let _input_type = self.check_expression(&args[0])?;
                                            let _output_type = self.check_expression(&args[1])?;
                                        }
                                        return Ok(Type::Void);
                                    }
                                    "train" => {
                                        if args.len() >= 1 {
                                            let _model_name_type = self.check_expression(&args[0])?;
                                        }
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Void),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "trainWithOnnx" | "trainWithTensorflow" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Named("ModelTrainingResult".to_string())),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "evaluateModel" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Named("ModelEvaluationResult".to_string())),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            "HttpResponse" => {
                                match member.as_str() {
                                    "json" => {
                                        if !args.is_empty() {
                                            self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                        }
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Named("any".to_string())),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "text" => {
                                        if !args.is_empty() {
                                            self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                        }
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::String),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "status" => {
                                        if !args.is_empty() {
                                            self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                        }
                                        return Ok(Type::Number);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    // Handle List method calls
                    if let Type::List(ref item_type) = obj_type {
                        match member.as_str() {
                            "length" | "size" | "len" => {
                                if !args.is_empty() {
                                    self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                }
                                return Ok(Type::Number);
                            }
                            "join" => {
                                if args.len() != 1 {
                                    self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                                } else {
                                    let arg_type = self.check_expression(&args[0])?;
                                    if arg_type != Type::String {
                                        self.errors.push(TypeError::type_mismatch("string", &arg_type.to_string()));
                                    }
                                }
                                return Ok(Type::String);
                            }
                            "push" | "pop" | "remove" | "clear" => {
                                // These methods modify the list
                                return Ok(Type::Void);
                            }
                            "map" | "filter" => {
                                // These take a closure and return a new List
                                return Ok(Type::List(item_type.clone()));
                            }
                            "find" | "contains" => {
                                // These take a closure and return Optional<item_type>
                                return Ok(Type::Optional(Box::new(*item_type.clone())));
                            }
                            "reduce" => {
                                // reduce takes a closure and initial value, returns item_type
                                return Ok(*item_type.clone());
                            }
                            _ => {
                                // Unknown method - don't error, might be handled elsewhere
                                return Ok(Type::Void);
                            }
                        }
                    }
                    
                    // Handle Standard Library class method calls
                    if let Type::Named(ref class_name) = obj_type {
                        match class_name.as_str() {
                            "HttpClient" => {
                                match member.as_str() {
                                    "get" | "post" | "put" | "delete" | "patch" => {
                                        // HTTP methods return HttpResponse
                                        return Ok(Type::Named("HttpResponse".to_string()));
                                    }
                                    _ => {}
                                }
                            }
                            "Validator" => {
                                match member.as_str() {
                                    "required" | "minLength" | "maxLength" | "email" | "pattern" | "min" | "max" | "range" | "custom" => {
                                        // Fluent interface - returns self
                                        return Ok(Type::Named("Validator".to_string()));
                                    }
                                    "isValid" => {
                                        return Ok(Type::Boolean);
                                    }
                                    "errors" => {
                                        return Ok(Type::List(Box::new(Type::Named("ValidationError".to_string()))));
                                    }
                                    _ => {}
                                }
                            }
                            "AuthService" => {
                                match member.as_str() {
                                    "generateToken" => {
                                        return Ok(Type::Named("JWTToken".to_string()));
                                    }
                                    "verifyToken" => {
                                        return Ok(Type::Optional(Box::new(Type::Named("UserClaims".to_string()))));
                                    }
                                    "extractUserId" => {
                                        return Ok(Type::Optional(Box::new(Type::String)));
                                    }
                                    "hasRole" => {
                                        return Ok(Type::Boolean);
                                    }
                                    _ => {}
                                }
                            }
                            "Logger" | "VelinLogger" => {
                                match member.as_str() {
                                    "info" | "debug" | "warn" | "error" | "trace" | "log" => {
                                        return Ok(Type::Void);
                                    }
                                    _ => {}
                                }
                            }
                            "MetricsCollector" => {
                                match member.as_str() {
                                    "incrementCounter" | "setGauge" | "observeHistogram" => {
                                        return Ok(Type::Void);
                                    }
                                    "getMetrics" => {
                                        return Ok(Type::List(Box::new(Type::Named("Metric".to_string()))));
                                    }
                                    "getMetric" => {
                                        return Ok(Type::Optional(Box::new(Type::Named("Metric".to_string()))));
                                    }
                                    "exportPrometheus" => {
                                        return Ok(Type::String);
                                    }
                                    _ => {}
                                }
                            }
                            "PerformanceMonitor" => {
                                match member.as_str() {
                                    "startOperation" | "endOperation" => {
                                        return Ok(Type::Void);
                                    }
                                    "collector" => {
                                        return Ok(Type::Named("MetricsCollector".to_string()));
                                    }
                                    _ => {}
                                }
                            }
                            "LLMClient" => {
                                match member.as_str() {
                                    "generate" | "complete" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::String),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "embed" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::List(Box::new(Type::Number))),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            "ModelLoader" => {
                                match member.as_str() {
                                    "loadModel" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Void),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "predict" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::String),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            "TrainingService" => {
                                match member.as_str() {
                                    "addExample" => {
                                        return Ok(Type::Void);
                                    }
                                    "train" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Void),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "trainWithOnnx" | "trainWithTensorflow" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Named("ModelTrainingResult".to_string())),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "evaluateModel" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Named("ModelEvaluationResult".to_string())),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            "HttpResponse" => {
                                match member.as_str() {
                                    "json" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::Named("any".to_string())),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "text" => {
                                        return Ok(Type::Result {
                                            ok: Box::new(Type::String),
                                            err: Box::new(Type::String),
                                        });
                                    }
                                    "status" => {
                                        return Ok(Type::Number);
                                    }
                                    _ => {
                                        return Ok(Type::Void);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    // Handle String method calls
                    if obj_type == Type::String {
                        match member.as_str() {
                            "length" | "size" | "len" => {
                                if !args.is_empty() {
                                    self.errors.push(TypeError::wrong_argument_count(0, args.len()));
                                }
                                return Ok(Type::Number);
                            }
                            _ => {
                                return Ok(Type::Void);
                            }
                        }
                    }
                    
                    // Handle Standard Library method calls (db, file, json, datetime, regex, crypto)
                    if let Type::Named(ref name) = obj_type {
                        let method_name = match name.as_str() {
                            "Database" => format!("db.{}", member),
                            "File" => format!("file.{}", member),
                            "Json" => format!("json.{}", member),
                            "DateTime" => format!("datetime.{}", member),
                            "Regex" => format!("regex.{}", member),
                            "Crypto" => format!("crypto.{}", member),
                            _ => return Ok(Type::Void),
                        };
                        
                        if let Some(sig) = self.environment.get_function(&method_name) {
                            // Check arguments
                            if sig.params.len() != args.len() {
                                self.errors.push(TypeError::wrong_argument_count(sig.params.len(), args.len()));
                            } else {
                                for (_i, (param, arg)) in sig.params.iter().zip(args.iter()).enumerate() {
                                    let arg_type = self.check_expression(arg)?;
                                    if !self.types_compatible(&arg_type, &param.param_type) {
                                        self.errors.push(TypeError::type_mismatch(
                                            &param.param_type.to_string(),
                                            &arg_type.to_string(),
                                        ));
                                    }
                                }
                            }
                            return Ok(sig.return_type.unwrap_or(Type::Void));
                        }
                        
                        // Special handling for db.find() and db.findAll() when function not found via normal lookup
                        if name == "Database" && member == "find" {
                            // db.find(User, id) - first arg is type, second is id
                            if args.len() == 2 {
                                // Check if first argument is a type identifier
                                if let Expression::Identifier(type_name) = &args[0] {
                                    if self.environment.has_type(type_name) {
                                        // Check second argument (id) is a string
                                        let id_type = self.check_expression(&args[1])?;
                                        if id_type != Type::String {
                                            self.errors.push(TypeError::type_mismatch("string", &id_type.to_string()));
                                        }
                                        // Return Optional<T> where T is the type passed
                                        return Ok(Type::Optional(Box::new(Type::Named(type_name.clone()))));
                                    } else {
                                        self.errors.push(TypeError::undefined_type(type_name));
                                    }
                                }
                            } else {
                                self.errors.push(TypeError::wrong_argument_count(2, args.len()));
                            }
                            return Ok(Type::Optional(Box::new(Type::Named("User".to_string()))));
                        } else if name == "Database" && member == "findAll" {
                            // db.findAll(User) - first arg is type
                            if args.len() == 1 {
                                if let Expression::Identifier(type_name) = &args[0] {
                                    if self.environment.has_type(type_name) {
                                        // Return List<T> where T is the type passed
                                        return Ok(Type::List(Box::new(Type::Named(type_name.clone()))));
                                    } else {
                                        self.errors.push(TypeError::undefined_type(type_name));
                                    }
                                }
                            } else {
                                self.errors.push(TypeError::wrong_argument_count(1, args.len()));
                            }
                            return Ok(Type::List(Box::new(Type::Named("User".to_string()))));
                        }
                    }
                }
                
                let _callee_type = self.check_expression(callee)?;
                
                // Look up function signature from environment
                if let Expression::Identifier(name) = callee.as_ref() {
                    if let Some(sig) = self.environment.get_function(name) {
                        // Check argument count
                        if args.len() != sig.params.len() {
                            self.errors.push(TypeError::wrong_argument_count(
                                sig.params.len(),
                                args.len(),
                            ));
                        } else {
                            // Check argument types
                            for (i, (arg, param)) in args.iter().zip(sig.params.iter()).enumerate() {
                                let arg_type = self.check_expression(arg)?;
                                if !self.types_compatible(&arg_type, &param.param_type) {
                                    self.errors.push(TypeError::new(
                                        TypeErrorKind::InvalidArgumentType {
                                            position: i,
                                            expected: param.param_type.to_string(),
                                            found: arg_type.to_string(),
                                        },
                                        format!(
                                            "Argument {}: expected {}, found {}",
                                            i + 1,
                                            param.param_type.to_string(),
                                            arg_type.to_string()
                                        ),
                                    ));
                                }
                            }
                        }
                        
                        Ok(sig.return_type.unwrap_or(Type::Void))
                    } else {
                        // Check if it's a method call (object.method())
                        if let Some(dot_pos) = name.rfind('.') {
                            let (obj_name, _method_name) = name.split_at(dot_pos);
                            let _method_name = &_method_name[1..]; // Skip the dot
                            
                            if let Some(_obj_type) = self.environment.get_variable(obj_name) {
                                // For now, assume method calls return Void
                                // In future, could check struct methods
                                Ok(Type::Void)
                            } else {
                                // Don't error for unknown functions - might be runtime functions
                                Ok(Type::Void)
                            }
                        } else {
                            // Don't error for unknown functions - might be runtime functions
                            Ok(Type::Void)
                        }
                    }
                } else {
                    // Handle other callable types (closures, function pointers, etc.)
                    // For now, assume they return Void
                    // In future, could infer from callable type
                    Ok(Type::Void)
                }
            }
            Expression::Await { expr } => {
                // Check that the expression is awaitable (async function call, etc.)
                let expr_type = self.check_expression(expr)?;
                // For now, return the type of the awaited expression
                // In future, could check if it's actually awaitable
                Ok(expr_type)
            }
            Expression::Member { object, member } => {
                let obj_type = self.check_expression(object)?;
                
                // Check member access
                match obj_type {
                    Type::Generic { name, params } if name == "List" && params.len() == 1 => {
                        // Handle List<T> as generic type
                        let item_type = &params[0];
                        match member.as_str() {
                            "length" | "size" | "len" => Ok(Type::Number),
                            "push" | "pop" | "remove" | "clear" => Ok(Type::Void),
                            "join" => Ok(Type::String),
                            "map" | "filter" => Ok(Type::List(Box::new(item_type.clone()))),
                            "find" | "contains" => Ok(Type::Optional(Box::new(item_type.clone()))),
                            "reduce" => Ok(item_type.clone()),
                            "sort" | "reverse" => Ok(Type::Void),
                            "chunk" | "slice" => Ok(Type::List(Box::new(item_type.clone()))),
                            _ => Ok(Type::Void),
                        }
                    }
                    Type::String => {
                        // String methods
                        match member.as_str() {
                            "length" => Ok(Type::Number),
                            "toUpperCase" | "toLowerCase" | "trim" => Ok(Type::String),
                            _ => {
                                self.errors.push(TypeError::new(
                                    TypeErrorKind::InvalidMemberAccess,
                                    format!("String has no member '{}'", member),
                                ));
                                Ok(Type::Void)
                            }
                        }
                    }
                    Type::Named(ref class_name) => {
                        if class_name == "any" { return Ok(Type::Named("any".to_string())); }
                        // Check for Module type
                        if class_name.starts_with("Module:") {
                            let module_name = &class_name[7..];
                            if let Some(module_env) = self.environment.get_module(module_name) {
                                if let Some(func) = module_env.get_function(member) {
                                    return Ok(func.return_type.unwrap_or(Type::Void));
                                } else if let Some(var) = module_env.get_variable(member) {
                                    return Ok(var);
                                } else {
                                    self.errors.push(TypeError::undefined_variable(member));
                                    return Ok(Type::Void);
                                }
                            }
                        }

                        // First check if it's a struct with fields
                        if let Some(struct_def) = self.environment.get_struct(class_name) {
                            // Find the field and return its type
                            if let Some(field) = struct_def.fields.iter().find(|f| f.name == *member) {
                                return Ok(field.field_type.clone());
                            }
                            // If not a field, might be a method - continue to check Standard Library classes
                        }
                        
                        // Handle Standard Library class methods
                        match class_name.as_str() {
                            "HttpClient" => {
                                match member.as_str() {
                                    "get" | "post" | "put" | "delete" | "patch" => {
                                        // HTTP methods return HttpResponse (async, but we return the type)
                                        Ok(Type::Named("HttpResponse".to_string()))
                                    }
                                    _ => Ok(Type::Void) // Unknown method, don't error
                                }
                            }
                            "Validator" => {
                                match member.as_str() {
                                    "required" | "minLength" | "maxLength" | "email" | "pattern" | "min" | "max" | "range" | "custom" => {
                                        // Fluent interface - returns self
                                        Ok(Type::Named("Validator".to_string()))
                                    }
                                    "isValid" => Ok(Type::Boolean),
                                    "errors" => Ok(Type::List(Box::new(Type::Named("ValidationError".to_string())))),
                                    _ => Ok(Type::Void)
                                }
                            }
                            "AuthService" => {
                                match member.as_str() {
                                    "generateToken" => Ok(Type::Named("JWTToken".to_string())),
                                    "verifyToken" => Ok(Type::Optional(Box::new(Type::Named("UserClaims".to_string())))),
                                    "extractUserId" => Ok(Type::Optional(Box::new(Type::String))),
                                    "hasRole" => Ok(Type::Boolean),
                                    _ => Ok(Type::Void)
                                }
                            }
                            "Logger" | "VelinLogger" => {
                                match member.as_str() {
                                    "info" | "debug" | "warn" | "error" | "trace" | "log" => {
                                        Ok(Type::Void) // Logging methods return void
                                    }
                                    "setLevel" | "addContext" | "removeContext" | "enableJsonFormat" | "enableRotation" => {
                                        Ok(Type::Void)
                                    }
                                    _ => Ok(Type::Void)
                                }
                            }
                            "DateTime" => {
                                // Handle datetime function calls via function lookup
                                let func_name = format!("datetime.{}", member);
                                if let Some(sig) = self.environment.get_function(&func_name) {
                                    Ok(sig.return_type.unwrap_or(Type::Void))
                                } else {
                                    Ok(Type::Void)
                                }
                            }
                            "Regex" => {
                                // Handle regex function calls via function lookup
                                let func_name = format!("regex.{}", member);
                                if let Some(sig) = self.environment.get_function(&func_name) {
                                    Ok(sig.return_type.unwrap_or(Type::Void))
                                } else {
                                    Ok(Type::Void)
                                }
                            }
                            "Crypto" => {
                                // Handle crypto function calls via function lookup
                                let func_name = format!("crypto.{}", member);
                                if let Some(sig) = self.environment.get_function(&func_name) {
                                    Ok(sig.return_type.unwrap_or(Type::Void))
                                } else {
                                    Ok(Type::Void)
                                }
                            }
                            "File" => {
                                // Handle file function calls via function lookup
                                let func_name = format!("file.{}", member);
                                if let Some(sig) = self.environment.get_function(&func_name) {
                                    Ok(sig.return_type.unwrap_or(Type::Void))
                                } else {
                                    Ok(Type::Void)
                                }
                            }
                            "Json" => {
                                // Handle json function calls via function lookup
                                let func_name = format!("json.{}", member);
                                if let Some(sig) = self.environment.get_function(&func_name) {
                                    Ok(sig.return_type.unwrap_or(Type::Void))
                                } else {
                                    Ok(Type::Void)
                                }
                            }
                            "MetricsCollector" => {
                                match member.as_str() {
                                    "incrementCounter" | "setGauge" | "observeHistogram" => {
                                        Ok(Type::Void)
                                    }
                                    "getMetrics" => Ok(Type::List(Box::new(Type::Named("Metric".to_string())))),
                                    "getMetric" => Ok(Type::Optional(Box::new(Type::Named("Metric".to_string())))),
                                    "exportPrometheus" => Ok(Type::String),
                                    _ => Ok(Type::Void)
                                }
                            }
                            "PerformanceMonitor" => {
                                match member.as_str() {
                                    "startOperation" | "endOperation" => Ok(Type::Void),
                                    "collector" => Ok(Type::Named("MetricsCollector".to_string())),
                                    _ => Ok(Type::Void)
                                }
                            }
                            "LLMClient" => {
                                match member.as_str() {
                                    "generate" | "complete" => Ok(Type::Result {
                                        ok: Box::new(Type::String),
                                        err: Box::new(Type::String),
                                    }),
                                    "embed" => Ok(Type::Result {
                                        ok: Box::new(Type::List(Box::new(Type::Number))),
                                        err: Box::new(Type::String),
                                    }),
                                    _ => Ok(Type::Void)
                                }
                            }
                            "ModelLoader" => {
                                match member.as_str() {
                                    "loadModel" => Ok(Type::Result {
                                        ok: Box::new(Type::Void),
                                        err: Box::new(Type::String),
                                    }),
                                    "predict" => Ok(Type::Result {
                                        ok: Box::new(Type::String),
                                        err: Box::new(Type::String),
                                    }),
                                    _ => Ok(Type::Void)
                                }
                            }
                            "TrainingService" => {
                                match member.as_str() {
                                    "addExample" => Ok(Type::Void),
                                    "train" => Ok(Type::Result {
                                        ok: Box::new(Type::Void),
                                        err: Box::new(Type::String),
                                    }),
                                    "trainWithOnnx" | "trainWithTensorflow" => Ok(Type::Result {
                                        ok: Box::new(Type::Named("ModelTrainingResult".to_string())),
                                        err: Box::new(Type::String),
                                    }),
                                    "evaluateModel" => Ok(Type::Result {
                                        ok: Box::new(Type::Named("ModelEvaluationResult".to_string())),
                                        err: Box::new(Type::String),
                                    }),
                                    _ => Ok(Type::Void)
                                }
                            }
                            "HttpResponse" => {
                                match member.as_str() {
                                    "json" => Ok(Type::Result {
                                        ok: Box::new(Type::Named("any".to_string())),
                                        err: Box::new(Type::String),
                                    }),
                                    "text" => Ok(Type::Result {
                                        ok: Box::new(Type::String),
                                        err: Box::new(Type::String),
                                    }),
                                    "status" => Ok(Type::Number),
                                    _ => Ok(Type::Void)
                                }
                            }
                            _ => {
                                // Check if it's a struct with fields
                                if let Some(struct_def) = self.environment.get_struct(class_name) {
                                    if let Some(field) = struct_def.fields.iter().find(|f| f.name == *member) {
                                        Ok(field.field_type.clone())
                                    } else {
                                        // Don't error - might be a method
                                        Ok(Type::Void)
                                    }
                                } else {
                                    // Don't error - might be a runtime type
                                    Ok(Type::Void)
                                }
                            }
                        }
                    }
                    Type::Optional(ref inner) => {
                        if let Type::Named(ref name) = **inner {
                             if name == "any" {
                                 return Ok(Type::Named("any".to_string()));
                             }
                        }
                        self.errors.push(TypeError::new(
                            TypeErrorKind::InvalidMemberAccess,
                            format!("Type '{}' does not support member access", obj_type.to_string()),
                        ));
                        Ok(Type::Void)
                    }
                    _ => {
                        self.errors.push(TypeError::new(
                            TypeErrorKind::InvalidMemberAccess,
                            format!("Type '{}' does not support member access", obj_type.to_string()),
                        ));
                        Ok(Type::Void)
                    }
                }
            }
            Expression::Index { object, index } => {
                let obj_type = self.check_expression(object)?;
                let index_type = self.check_expression(index)?;
                
                if index_type != Type::Number {
                    self.errors.push(TypeError::type_mismatch(
                        "number",
                        &index_type.to_string(),
                    ));
                }
                
                match obj_type {
                    Type::List(ref item_type) => Ok(item_type.as_ref().clone()),
                    _ => {
                        self.errors.push(TypeError::invalid_operation("index", &obj_type.to_string()));
                        Ok(Type::Void)
                    }
                }
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                let cond_type = self.check_expression(condition)?;
                if cond_type != Type::Boolean {
                    self.errors.push(TypeError::type_mismatch(
                        "boolean",
                        &cond_type.to_string(),
                    ));
                }
                
                let then_type = self.check_expression(then_expr)?;
                let else_type = self.check_expression(else_expr)?;
                
                // Both branches should have compatible types
                if !self.types_compatible(&then_type, &else_type) {
                    self.errors.push(TypeError::type_mismatch(
                        &then_type.to_string(),
                        &else_type.to_string(),
                    ));
                }
                
                Ok(then_type)
            }
            Expression::Lambda { params, return_type, body } => {
                // Create new environment for lambda parameters
                let parent_env = self.environment.clone();
                let mut lambda_env = Environment::with_parent(parent_env);
                
                // Add parameters to lambda environment
                for param in params {
                    lambda_env.define_variable(param.name.clone(), param.param_type.clone());
                }
                
                // Check lambda body
                let old_env = std::mem::replace(&mut self.environment, lambda_env);
                let body_type = match body.as_ref() {
                    Expression::Block(block) => {
                        self.check_block(block, return_type.as_ref())?
                    }
                    _ => {
                        self.check_expression(body)?
                    }
                };
                self.environment = old_env;
                
                // Determine return type
                let lambda_return_type = if let Some(ref ret_type) = return_type {
                    if !self.types_compatible(&body_type, ret_type) {
                        self.errors.push(TypeError::type_mismatch(
                            &ret_type.to_string(),
                            &body_type.to_string(),
                        ));
                    }
                    ret_type.clone()
                } else {
                    // Type inference
                    body_type
                };
                
                // Create function type for lambda
                let param_types: Vec<Type> = params.iter().map(|p| p.param_type.clone()).collect();
                Ok(Type::Function {
                    params: param_types,
                    return_type: Box::new(lambda_return_type),
                })
            }
            Expression::Block(block) => {
                Ok(self.check_block(block, None)?)
            }
            Expression::FormatString { parts } => {
                // Type-check all expressions in the format string
                for part in parts {
                    if let FormatStringPart::Expression(expr) = part {
                        // Check that the expression is valid (but don't enforce a specific type)
                        // Format strings can contain any Display-able type
                        let _expr_type = self.check_expression(expr)?;
                    }
                }
                // Format strings always return String type
                Ok(Type::String)
            }
        }
    }
    
    fn check_binary_operation(
        &mut self,
        op: &BinaryOperator,
        left_type: &Type,
        right_type: &Type,
    ) -> Result<Type, Vec<TypeError>> {
        // Support 'any' type for dynamic operations
        if let Type::Named(name) = left_type { if name == "any" { return Ok(Type::Named("any".to_string())); } }
        if let Type::Named(name) = right_type { if name == "any" { return Ok(Type::Named("any".to_string())); } }

        match op {
            BinaryOperator::Add => {
                // String concatenation
                if *left_type == Type::String && *right_type == Type::String {
                    Ok(Type::String)
                }
                // Number addition
                else if *left_type == Type::Number && *right_type == Type::Number {
                    Ok(Type::Number)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        "+",
                        &format!("{} and {}", left_type.to_string(), right_type.to_string()),
                    ));
                    Ok(Type::Void)
                }
            }
            BinaryOperator::Subtract | BinaryOperator::Multiply
            | BinaryOperator::Divide | BinaryOperator::Modulo => {
                if *left_type == Type::Number && *right_type == Type::Number {
                    Ok(Type::Number)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        &format!("{:?}", op),
                        &format!("{} and {}", left_type.to_string(), right_type.to_string()),
                    ));
                    Ok(Type::Void)
                }
            }
            BinaryOperator::Eq | BinaryOperator::NotEq | BinaryOperator::Lt
            | BinaryOperator::Gt | BinaryOperator::LtEq | BinaryOperator::GtEq => {
                if self.types_compatible(left_type, right_type) {
                    Ok(Type::Boolean)
                } else {
                    self.errors.push(TypeError::type_mismatch(
                        &left_type.to_string(),
                        &right_type.to_string(),
                    ));
                    Ok(Type::Boolean) // Still return boolean for comparison
                }
            }
            BinaryOperator::And | BinaryOperator::Or => {
                if *left_type == Type::Boolean && *right_type == Type::Boolean {
                    Ok(Type::Boolean)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        &format!("{:?}", op),
                        &format!("{} and {}", left_type.to_string(), right_type.to_string()),
                    ));
                    Ok(Type::Boolean)
                }
            }
        }
    }
    
    fn check_unary_operation(
        &mut self,
        op: &UnaryOperator,
        expr_type: &Type,
    ) -> Result<Type, Vec<TypeError>> {
        match op {
            UnaryOperator::Not => {
                if *expr_type == Type::Boolean {
                    Ok(Type::Boolean)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        "!",
                        &expr_type.to_string(),
                    ));
                    Ok(Type::Boolean)
                }
            }
            UnaryOperator::Minus => {
                if *expr_type == Type::Number {
                    Ok(Type::Number)
                } else {
                    self.errors.push(TypeError::invalid_operation(
                        "-",
                        &expr_type.to_string(),
                    ));
                    Ok(Type::Number)
                }
            }
        }
    }
    
    fn literal_type(&self, lit: &Literal) -> Type {
        match lit {
            Literal::String(_) => Type::String,
            Literal::Number(_) => Type::Number,
            Literal::Boolean(_) => Type::Boolean,
            Literal::Null => Type::Null,
        }
    }
    
    fn check_type(&mut self, type_def: &Type) -> Result<(), Vec<TypeError>> {
        match type_def {
            Type::String | Type::Number | Type::Boolean | Type::Void | Type::Null | Type::Any => Ok(()),
            Type::Named(name) => {
                // Check if it's a generic type parameter (single uppercase letter or common pattern)
                // For now, we'll be lenient and allow single-letter identifiers as type parameters
                if name.len() == 1 && name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    // This is likely a type parameter, allow it
                    Ok(())
                } else if !self.environment.has_type(name) {
                    self.errors.push(TypeError::undefined_type(name));
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Type::Generic { name, params } => {
                // Check if the generic type name is valid (e.g., List, Map, Result, ApiResponse)
                if name == "List" || name == "Map" || name == "Result" {
                    // Built-in generic types - OK, continue
                    // For Result, we need exactly 2 type parameters
                    if name == "Result" && params.len() != 2 {
                        self.errors.push(TypeError::new(
                            crate::type_checker::errors::TypeErrorKind::WrongArgumentCount {
                                expected: 2,
                                found: params.len(),
                            },
                            format!("Result requires exactly 2 type parameters, found {}", params.len()),
                        ));
                        return Ok(());
                    }
                } else if !self.environment.has_type(name) {
                    // Check if it's a struct with generics
                    // For now, we'll check the base name
                    let base_name = name.clone();
                    if !self.environment.has_type(&base_name) {
                        self.errors.push(TypeError::undefined_type(name));
                        return Ok(()); // Return early to avoid checking params if type is invalid
                    }
                }
                for param in params {
                    self.check_type(param)?;
                }
                Ok(())
            }
            Type::Function { params, return_type } => {
                for param in params {
                    self.check_type(param)?;
                }
                self.check_type(return_type)?;
                Ok(())
            }
            Type::List(item_type) => {
                self.check_type(item_type)?;
                Ok(())
            }
            Type::Map { key, value } => {
                self.check_type(key)?;
                self.check_type(value)?;
                Ok(())
            }
            Type::Tuple(types) => {
                for t in types {
                    self.check_type(t)?;
                }
                Ok(())
            }
            Type::Optional(inner) => {
                self.check_type(inner)?;
                Ok(())
            }
            Type::Result { ok, err } => {
                self.check_type(ok)?;
                self.check_type(err)?;
                Ok(())
            }
        }
    }
    
    fn types_compatible(&self, t1: &Type, t2: &Type) -> bool {
        // "any" type is compatible with everything
        if let Type::Named(n) = t1 {
            if n == "any" { return true; }
        }
        if let Type::Named(n) = t2 {
            if n == "any" { return true; }
        }

        if t1 == t2 {
            return true;
        }
        
        // Handle type aliases and named types
        match (t1, t2) {
            (Type::Named(n1), Type::Named(n2)) => n1 == n2,
            (Type::Generic { name: n1, params: p1 }, Type::Generic { name: n2, params: p2 }) => {
                if n1 == n2 && p1.len() == p2.len() {
                    p1.iter().zip(p2.iter()).all(|(a, b)| self.types_compatible(a, b))
                } else {
                    false
                }
            }
            (Type::Generic { name: n1, params: _p1 }, Type::Named(n2)) => {
                // ApiResponse<void> vs ApiResponse - check if base names match
                // Allow if the generic has parameters (struct literal can be instantiated)
                n1 == n2
            }
            (Type::Named(n1), Type::Generic { name: n2, params: _p2 }) => {
                // ApiResponse vs ApiResponse<void> - struct literal can match generic return type
                // This allows struct literals to be compatible with generic return types
                n1 == n2
            }
            (Type::List(l1), Type::List(l2)) => self.types_compatible(l1, l2),
            (Type::List(l1), Type::Generic { name: n2, params: p2 }) if n2 == "List" && p2.len() == 1 => {
                // List<T> (concrete) vs List<T> (generic type annotation)
                self.types_compatible(l1, &p2[0])
            }
            (Type::Generic { name: n1, params: p1 }, Type::List(l2)) if n1 == "List" && p1.len() == 1 => {
                // List<T> (generic type annotation) vs List<T> (concrete)
                self.types_compatible(&p1[0], l2)
            }
            (Type::Map { key: k1, value: v1 }, Type::Map { key: k2, value: v2 }) => {
                self.types_compatible(k1, k2) && self.types_compatible(v1, v2)
            }
            (Type::Map { key: k1, value: v1 }, Type::Generic { name: n2, params: p2 }) if n2 == "Map" && p2.len() == 2 => {
                // Map<K, V> (concrete) vs Map<K, V> (generic type annotation)
                self.types_compatible(k1, &p2[0]) && self.types_compatible(v1, &p2[1])
            }
            (Type::Generic { name: n1, params: p1 }, Type::Map { key: k2, value: v2 }) if n1 == "Map" && p1.len() == 2 => {
                // Map<K, V> (generic type annotation) vs Map<K, V> (concrete)
                self.types_compatible(&p1[0], k2) && self.types_compatible(&p1[1], v2)
            }
            (Type::Optional(o1), Type::Optional(o2)) => self.types_compatible(o1, o2),
            (Type::Optional(o1), t2) => {
                // Optional<T> is compatible with T (can be unwrapped)
                self.types_compatible(o1, t2)
            },
            (t1, Type::Optional(o2)) => {
                // T is compatible with Optional<T> (can be wrapped)
                self.types_compatible(t1, o2)
            },
            (Type::Result { ok: ok1, err: err1 }, Type::Result { ok: ok2, err: err2 }) => {
                self.types_compatible(ok1, ok2) && self.types_compatible(err1, err2)
            }
            _ => false,
        }
    }
    
    /// Checks a pattern and returns an environment with pattern bindings
    fn check_pattern(&mut self, pattern: &Pattern, match_type: &Type) -> Result<Environment, Vec<TypeError>> {
        let parent_env = self.environment.clone();
        let mut pattern_env = Environment::with_parent(parent_env);
        
        match pattern {
            Pattern::Literal(pat_lit) => {
                let pat_type = self.literal_type(pat_lit);
                if !self.types_compatible(match_type, &pat_type) {
                    self.errors.push(TypeError::type_mismatch(
                        &match_type.to_string(),
                        &pat_type.to_string(),
                    ));
                }
            }
            Pattern::Identifier(name) => {
                // Bind pattern variable to match type
                pattern_env.define_variable(name.clone(), match_type.clone());
            }
            Pattern::Wildcard => {
                // Wildcard matches anything, no bindings
            }
            Pattern::Range { start, end, .. } => {
                // Check that start and end are numbers
                let start_type = self.check_expression(start)?;
                let end_type = self.check_expression(end)?;
                if start_type != Type::Number || end_type != Type::Number {
                    self.errors.push(TypeError::type_mismatch(
                        "number",
                        &format!("{:?}..{:?}", start_type, end_type),
                    ));
                }
                // Range pattern matches number type
                if !self.types_compatible(match_type, &Type::Number) {
                    self.errors.push(TypeError::type_mismatch(
                        "number",
                        &match_type.to_string(),
                    ));
                }
            }
            Pattern::Tuple(patterns) => {
                // Check if match_type is a tuple
                if let Type::Tuple(types) = match_type {
                    if patterns.len() != types.len() {
                    self.errors.push(TypeError::new(
                        TypeErrorKind::TypeMismatch {
                            expected: format!("tuple with {} elements", types.len()),
                            found: format!("tuple pattern with {} elements", patterns.len()),
                        },
                        format!("Tuple pattern length {} doesn't match tuple type length {}", patterns.len(), types.len()),
                    ));
                    } else {
                        for (pat, ty) in patterns.iter().zip(types.iter()) {
                            let _ = self.check_pattern(pat, ty)?;
                        }
                    }
                } else {
                    self.errors.push(TypeError::type_mismatch(
                        "tuple",
                        &match_type.to_string(),
                    ));
                }
            }
            Pattern::Struct { name, fields } => {
                // Check if match_type matches struct name
                if let Type::Named(type_name) = match_type {
                    if name != type_name {
                        self.errors.push(TypeError::type_mismatch(
                            type_name,
                            name,
                        ));
                    }
                } else if let Type::Generic { name: gen_name, .. } = match_type {
                    if name != gen_name {
                        self.errors.push(TypeError::type_mismatch(
                            gen_name,
                            name,
                        ));
                    }
                }
                
                // Check field types and bind field patterns
                if let Some(struct_def) = self.environment.get_struct(&name) {
                    for (field_name, field_pattern) in fields {
                        // Find the field in struct definition
                        if let Some(field) = struct_def.fields.iter().find(|f| f.name == *field_name) {
                            // Check that the pattern matches the field type
                            let _field_env = self.check_pattern(field_pattern, &field.field_type)?;
                            // Pattern bindings are already handled in check_pattern
                        } else {
                            self.errors.push(TypeError::new(
                                TypeErrorKind::InvalidMemberAccess,
                                format!("Struct '{}' has no field '{}'", name, field_name),
                            ));
                        }
                    }
                }
            }
            Pattern::EnumVariant { name, data } => {
                // Check if match_type is the enum type
                if let Type::Named(type_name) = match_type {
                    // Extract enum name from variant name (e.g., "Result::Ok" -> "Result")
                    if let Some(enum_name) = name.split("::").next() {
                        if enum_name != type_name {
                            self.errors.push(TypeError::type_mismatch(
                                type_name,
                                enum_name,
                            ));
                        } else {
                            // Check variant exists in enum
                            if let Some(enum_def) = self.environment.get_enum(enum_name) {
                                let variant_name = name.split("::").nth(1).unwrap_or(&name);
                                if !enum_def.variants.iter().any(|v| v.name == variant_name) {
                                    self.errors.push(TypeError::new(
                                        TypeErrorKind::UndefinedType(variant_name.to_string()),
                                        format!("Enum '{}' has no variant '{}'", enum_name, variant_name),
                                    ));
                                } else {
                                    // Check variant data patterns if present
                                    if let Some(data_patterns) = data {
                                        if let Some(variant) = enum_def.variants.iter().find(|v| v.name == variant_name) {
                                            if let Some(variant_data_types) = &variant.data {
                                                if data_patterns.len() != variant_data_types.len() {
                                                    self.errors.push(TypeError::wrong_argument_count(
                                                        variant_data_types.len(),
                                                        data_patterns.len(),
                                                    ));
                                                } else {
                                                    for (pattern, data_type) in data_patterns.iter().zip(variant_data_types.iter()) {
                                                        let _pattern_env = self.check_pattern(pattern, data_type)?;
                                                        // Pattern bindings are already handled in check_pattern
                                                    }
                                                }
                                            } else if !data_patterns.is_empty() {
                                                self.errors.push(TypeError::new(
                                                    TypeErrorKind::WrongArgumentCount {
                                                        expected: 0,
                                                        found: data_patterns.len(),
                                                    },
                                                    format!("Variant '{}' has no data fields", variant_name),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Pattern::Or(patterns) => {
                // Check that at least one pattern matches
                // For now, check all patterns
                for pat in patterns {
                    let _ = self.check_pattern(pat, match_type)?;
                }
            }
        }
        
        Ok(pattern_env)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
