use crate::parser::ast::*;

pub struct OpenAPIGenerator {
    #[allow(dead_code)]
    output: String,
}

#[derive(Debug, Clone)]
pub struct OpenAPISpec {
    pub openapi: String,
    pub info: OpenAPIInfo,
    pub paths: std::collections::HashMap<String, PathItem>,
    pub components: Option<Components>,
}

#[derive(Debug, Clone)]
pub struct Components {
    pub schemas: std::collections::HashMap<String, Schema>,
    pub security_schemes: std::collections::HashMap<String, SecurityScheme>,
}

#[derive(Debug, Clone)]
pub struct OpenAPIInfo {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PathItem {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub delete: Option<Operation>,
    pub patch: Option<Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: String,
    pub parameters: Vec<Parameter>,
    pub request_body: Option<RequestBody>,
    pub responses: std::collections::HashMap<String, Response>,
    pub security: Vec<SecurityRequirement>,
}

#[derive(Debug, Clone)]
pub struct RequestBody {
    pub content: std::collections::HashMap<String, MediaType>,
}

#[derive(Debug, Clone)]
pub struct MediaType {
    pub schema: Schema,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub description: String,
    pub content: Option<std::collections::HashMap<String, MediaType>>,
}

#[derive(Debug, Clone)]
pub struct SecurityRequirement {
    pub name: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SecurityScheme {
    pub scheme_type: String,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
    pub name: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub schema_type: String,
    pub format: Option<String>,
    pub items: Option<Box<Schema>>,
    pub properties: Option<std::collections::HashMap<String, Schema>>,
    pub ref_path: Option<String>,
}

impl OpenAPIGenerator {
    pub fn new() -> Self {
        OpenAPIGenerator {
            output: String::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        let mut spec = OpenAPISpec {
            openapi: "3.0.0".to_string(),
            info: OpenAPIInfo {
                title: "VelinScript API".to_string(),
                version: "1.0.0".to_string(),
                description: Some("API generated from VelinScript".to_string()),
            },
            paths: std::collections::HashMap::new(),
            components: Some(Components {
                schemas: std::collections::HashMap::new(),
                security_schemes: std::collections::HashMap::new(),
            }),
        };

        // Extract structs for components/schemas
        for item in &program.items {
            if let Item::Struct(s) = item {
                self.extract_struct_schema(&mut spec, s);
            }
        }

        // Extract security schemes
        self.extract_security_schemes(&mut spec, program);

        // Extract API endpoints from functions
        for item in &program.items {
            if let Item::Function(f) = item {
                self.extract_endpoint(&mut spec, f);
            }
        }

        // Generate JSON
        self.generate_json(&spec)
    }
    
    fn extract_struct_schema(&self, spec: &mut OpenAPISpec, struct_def: &Struct) {
        if let Some(ref mut components) = spec.components {
            let mut properties = std::collections::HashMap::new();
            
            for field in &struct_def.fields {
                let field_schema = self.type_to_schema(&field.field_type);
                properties.insert(field.name.clone(), field_schema);
            }
            
            let schema = Schema {
                schema_type: "object".to_string(),
                format: None,
                items: None,
                properties: Some(properties),
                ref_path: None,
            };
            
            components.schemas.insert(struct_def.name.clone(), schema);
        }
    }
    
    fn extract_security_schemes(&self, spec: &mut OpenAPISpec, program: &Program) {
        let mut has_auth = false;
        let mut has_oauth2 = false;
        
        for item in &program.items {
            if let Item::Function(f) = item {
                for decorator in &f.decorators {
                    match decorator.name.as_str() {
                        "Auth" => has_auth = true,
                        "OAuth2" | "OIDC" => has_oauth2 = true,
                        _ => {}
                    }
                }
            }
        }
        
        if let Some(ref mut components) = spec.components {
            if has_auth {
                components.security_schemes.insert(
                    "bearerAuth".to_string(),
                    SecurityScheme {
                        scheme_type: "http".to_string(),
                        scheme: Some("bearer".to_string()),
                        bearer_format: Some("JWT".to_string()),
                        name: None,
                        location: None,
                    },
                );
            }
            
            if has_oauth2 {
                components.security_schemes.insert(
                    "oauth2".to_string(),
                    SecurityScheme {
                        scheme_type: "oauth2".to_string(),
                        scheme: None,
                        bearer_format: None,
                        name: None,
                        location: None,
                    },
                );
            }
        }
    }

    fn extract_endpoint(&self, spec: &mut OpenAPISpec, function: &Function) {
        let mut path: Option<String> = None;
        let mut method: Option<String> = None;
        let mut has_auth = false;
        let mut roles: Vec<String> = Vec::new();
        let mut summary: Option<String> = None;
        let mut description: Option<String> = None;
        let mut tags: Vec<String> = Vec::new();

        // Extract decorators
        for decorator in &function.decorators {
            match decorator.name.as_str() {
                "GET" | "POST" | "PUT" | "DELETE" | "PATCH" => {
                    method = Some(decorator.name.to_lowercase());
                    if let Some(DecoratorArg::String(p)) = decorator.args.first() {
                        path = Some(p.clone());
                    }
                }
                "Auth" => {
                    has_auth = true;
                }
                "Role" => {
                    if let Some(DecoratorArg::String(role)) = decorator.args.first() {
                        roles.push(role.clone());
                    }
                }
                "Summary" | "@Summary" => {
                    if let Some(DecoratorArg::String(s)) = decorator.args.first() {
                        summary = Some(s.clone());
                    }
                }
                "Description" | "@Description" => {
                    if let Some(DecoratorArg::String(d)) = decorator.args.first() {
                        description = Some(d.clone());
                    }
                }
                "Tag" | "@Tag" => {
                    if let Some(DecoratorArg::String(tag)) = decorator.args.first() {
                        tags.push(tag.clone());
                    }
                }
                _ => {}
            }
        }

        if let (Some(path_str), Some(method_str)) = (path, method) {
            let path_item = spec.paths.entry(path_str.clone()).or_insert_with(|| PathItem {
                get: None,
                post: None,
                put: None,
                delete: None,
                patch: None,
            });

            let operation = Operation {
                summary: summary.or(Some(function.name.clone())),
                description,
                operation_id: self.to_snake_case(&function.name),
                parameters: self.extract_parameters(function),
                request_body: self.extract_request_body(function, &method_str),
                responses: self.extract_responses(function),
                security: if has_auth {
                    vec![SecurityRequirement {
                        name: "bearerAuth".to_string(),
                        scopes: roles,
                    }]
                } else {
                    vec![]
                },
            };

            match method_str.as_str() {
                "get" => path_item.get = Some(operation),
                "post" => path_item.post = Some(operation),
                "put" => path_item.put = Some(operation),
                "delete" => path_item.delete = Some(operation),
                "patch" => path_item.patch = Some(operation),
                _ => {}
            }
        }
    }

    fn extract_parameters(&self, function: &Function) -> Vec<Parameter> {
        let mut params = Vec::new();
        
        // Extract path from decorators to identify path parameters
        let mut path_str: Option<String> = None;
        for decorator in &function.decorators {
            if matches!(decorator.name.as_str(), "GET" | "POST" | "PUT" | "DELETE" | "PATCH") {
                if let Some(DecoratorArg::String(p)) = decorator.args.first() {
                    path_str = Some(p.clone());
                    break;
                }
            }
        }
        
        let path_params: Vec<String> = if let Some(ref path) = path_str {
            path.split('/')
                .filter(|s| s.starts_with(':'))
                .map(|s| s[1..].to_string())
                .collect()
        } else {
            Vec::new()
        };
        
        for param in &function.params {
            let param_schema = self.type_to_schema(&param.param_type);
            let location = if path_params.contains(&param.name) {
                "path".to_string()
            } else {
                "query".to_string()
            };
            
            params.push(Parameter {
                name: param.name.clone(),
                location,
                required: param.default.is_none(),
                schema: param_schema,
            });
        }
        
        params
    }

    fn extract_request_body(&self, function: &Function, method: &str) -> Option<RequestBody> {
        if method == "get" || method == "delete" {
            return None;
        }

        // For POST/PUT/PATCH, use first parameter as body
        if let Some(first_param) = function.params.first() {
            let schema = self.type_to_schema(&first_param.param_type);
            let mut content = std::collections::HashMap::new();
            content.insert(
                "application/json".to_string(),
                MediaType { schema },
            );
            Some(RequestBody { content })
        } else {
            None
        }
    }

    fn extract_responses(&self, function: &Function) -> std::collections::HashMap<String, Response> {
        let mut responses = std::collections::HashMap::new();
        
        // Success response (200)
        if let Some(ref return_type) = function.return_type {
            let schema = self.type_to_schema(return_type);
            let mut content = std::collections::HashMap::new();
            content.insert(
                "application/json".to_string(),
                MediaType { schema },
            );
            responses.insert(
                "200".to_string(),
                Response {
                    description: "Success".to_string(),
                    content: Some(content),
                },
            );
        } else {
            responses.insert(
                "200".to_string(),
                Response {
                    description: "Success".to_string(),
                    content: None,
                },
            );
        }
        
        // Error responses (400, 401, 403, 404, 500)
        let error_schema = Schema {
            schema_type: "object".to_string(),
            format: None,
            items: None,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert("error".to_string(), Schema {
                    schema_type: "string".to_string(),
                    format: None,
                    items: None,
                    properties: None,
                    ref_path: None,
                });
                props.insert("errors".to_string(), Schema {
                    schema_type: "array".to_string(),
                    format: None,
                    items: Some(Box::new(Schema {
                        schema_type: "object".to_string(),
                        format: None,
                        items: None,
                        properties: None,
                        ref_path: None,
                    })),
                    properties: None,
                    ref_path: None,
                });
                props
            }),
            ref_path: None,
        };
        
        let error_content = {
            let mut content = std::collections::HashMap::new();
            content.insert("application/json".to_string(), MediaType { schema: error_schema });
            content
        };
        
        responses.insert("400".to_string(), Response {
            description: "Bad Request - Validation Error".to_string(),
            content: Some(error_content.clone()),
        });
        
        responses.insert("401".to_string(), Response {
            description: "Unauthorized".to_string(),
            content: Some(error_content.clone()),
        });
        
        responses.insert("403".to_string(), Response {
            description: "Forbidden".to_string(),
            content: Some(error_content.clone()),
        });
        
        responses.insert("404".to_string(), Response {
            description: "Not Found".to_string(),
            content: Some(error_content.clone()),
        });
        
        responses.insert("500".to_string(), Response {
            description: "Internal Server Error".to_string(),
            content: Some(error_content),
        });
        
        responses
    }

    fn type_to_schema(&self, ty: &Type) -> Schema {
        match ty {
            Type::String => Schema {
                schema_type: "string".to_string(),
                format: None,
                items: None,
                properties: None,
                ref_path: None,
            },
            Type::Number => Schema {
                schema_type: "number".to_string(),
                format: Some("double".to_string()),
                items: None,
                properties: None,
                ref_path: None,
            },
            Type::Boolean => Schema {
                schema_type: "boolean".to_string(),
                format: None,
                items: None,
                properties: None,
                ref_path: None,
            },
            Type::List(item_type) => {
                let item_schema = self.type_to_schema(item_type);
                Schema {
                    schema_type: "array".to_string(),
                    format: None,
                    items: Some(Box::new(item_schema)),
                    properties: None,
                    ref_path: None,
                }
            }
            Type::Named(name) => Schema {
                schema_type: "object".to_string(),
                format: None,
                items: None,
                properties: None,
                ref_path: Some(format!("#/components/schemas/{}", name)),
            },
            Type::Generic { name, params } => {
                if name == "List" && !params.is_empty() {
                    let item_schema = self.type_to_schema(&params[0]);
                    Schema {
                        schema_type: "array".to_string(),
                        format: None,
                        items: Some(Box::new(item_schema)),
                        properties: None,
                        ref_path: None,
                    }
                } else {
                    Schema {
                        schema_type: "object".to_string(),
                        format: None,
                        items: None,
                        properties: None,
                        ref_path: None,
                    }
                }
            }
            _ => Schema {
                schema_type: "object".to_string(),
                format: None,
                items: None,
                properties: None,
                ref_path: None,
            },
        }
    }

    fn generate_json(&mut self, spec: &OpenAPISpec) -> String {
        // Simple JSON generation (in production, use serde_json)
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"openapi\": \"{}\",\n", spec.openapi));
        json.push_str("  \"info\": {\n");
        json.push_str(&format!("    \"title\": \"{}\",\n", spec.info.title));
        json.push_str(&format!("    \"version\": \"{}\",\n", spec.info.version));
        if let Some(ref desc) = spec.info.description {
            json.push_str(&format!("    \"description\": \"{}\",\n", desc));
        }
        json.push_str("  },\n");
        json.push_str("  \"paths\": {\n");
        
        for (path, path_item) in &spec.paths {
            json.push_str(&format!("    \"{}\": {{\n", path));
            
            if let Some(ref op) = path_item.get {
                json.push_str("      \"get\": ");
                json.push_str(&self.operation_to_json(op));
                json.push_str(",\n");
            }
            if let Some(ref op) = path_item.post {
                json.push_str("      \"post\": ");
                json.push_str(&self.operation_to_json(op));
                json.push_str(",\n");
            }
            if let Some(ref op) = path_item.put {
                json.push_str("      \"put\": ");
                json.push_str(&self.operation_to_json(op));
                json.push_str(",\n");
            }
            if let Some(ref op) = path_item.delete {
                json.push_str("      \"delete\": ");
                json.push_str(&self.operation_to_json(op));
                json.push_str(",\n");
            }
            if let Some(ref op) = path_item.patch {
                json.push_str("      \"patch\": ");
                json.push_str(&self.operation_to_json(op));
                json.push_str(",\n");
            }
            
            json.push_str("    },\n");
        }
        
        json.push_str("  },\n");
        
        // Add components section
        if let Some(ref components) = spec.components {
            json.push_str("  \"components\": {\n");
            
            // Security Schemes
            if !components.security_schemes.is_empty() {
                json.push_str("    \"securitySchemes\": {\n");
                for (name, scheme) in &components.security_schemes {
                    json.push_str(&format!("      \"{}\": {{\n", name));
                    json.push_str(&format!("        \"type\": \"{}\",\n", scheme.scheme_type));
                    if let Some(ref s) = scheme.scheme {
                        json.push_str(&format!("        \"scheme\": \"{}\",\n", s));
                    }
                    if let Some(ref bf) = scheme.bearer_format {
                        json.push_str(&format!("        \"bearerFormat\": \"{}\",\n", bf));
                    }
                    json.push_str("      },\n");
                }
                json.push_str("    },\n");
            }
            
            // Schemas
            if !components.schemas.is_empty() {
                json.push_str("    \"schemas\": {\n");
                for (name, schema) in &components.schemas {
                    json.push_str(&format!("      \"{}\": {},\n", name, self.schema_to_json(schema)));
                }
                json.push_str("    }\n");
            }
            
            json.push_str("  }\n");
        }
        
        json.push_str("}\n");
        json
    }
    
    fn schema_to_json(&self, schema: &Schema) -> String {
        if let Some(ref ref_path) = schema.ref_path {
            return format!("{{\"$ref\": \"{}\"}}", ref_path);
        }
        
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("        \"type\": \"{}\",\n", schema.schema_type));
        
        if let Some(ref format) = schema.format {
            json.push_str(&format!("        \"format\": \"{}\",\n", format));
        }
        
        if let Some(ref items) = schema.items {
            json.push_str("        \"items\": ");
            json.push_str(&self.schema_to_json(items));
            json.push_str(",\n");
        }
        
        if let Some(ref properties) = schema.properties {
            json.push_str("        \"properties\": {\n");
            for (name, prop_schema) in properties {
                json.push_str(&format!("          \"{}\": {},\n", name, self.schema_to_json(prop_schema)));
            }
            json.push_str("        },\n");
        }
        
        json.push_str("      }");
        json
    }

    fn operation_to_json(&self, op: &Operation) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        if let Some(ref summary) = op.summary {
            json.push_str(&format!("        \"summary\": \"{}\",\n", summary));
        }
        json.push_str(&format!("        \"operationId\": \"{}\",\n", op.operation_id));
        json.push_str("        \"responses\": {\n");
        json.push_str("          \"200\": {\n");
        json.push_str("            \"description\": \"Success\"\n");
        json.push_str("          }\n");
        json.push_str("        }\n");
        json.push_str("      }");
        json
    }

    fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub location: String,
    pub required: bool,
    pub schema: Schema,
}
