// OpenAPI 3.0 Generator für VelinScript

use velin_compiler::parser::ast::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAPISpec {
    pub openapi: String,
    pub info: Info,
    pub paths: HashMap<String, PathItem>,
    pub components: Components,
    pub servers: Option<Vec<Server>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    pub operation_id: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
    pub security: Option<Vec<SecurityRequirement>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String, // "query", "path", "header", "cookie"
    pub required: bool,
    pub schema: Schema,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pub required: bool,
    pub content: HashMap<String, MediaType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Schema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    pub content: Option<HashMap<String, MediaType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub ref_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
    pub security_schemes: Option<HashMap<String, SecurityScheme>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: String,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityRequirement {
    #[serde(flatten)]
    pub requirements: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
}

pub struct OpenAPIGenerator;

impl OpenAPIGenerator {
    /// Generiert OpenAPI Spec aus VelinScript Program
    pub fn generate(program: &Program, title: &str, version: &str) -> OpenAPISpec {
        let mut paths = HashMap::new();
        let mut components = Components {
            schemas: HashMap::new(),
            security_schemes: Some(HashMap::new()),
        };

        // Extract endpoints
        for item in &program.items {
            if let Item::Function(f) = item {
                if let Some(endpoint) = Self::extract_endpoint(f) {
                    let path_item = Self::create_path_item(f, &endpoint, &mut components);
                    paths.insert(endpoint.path, path_item);
                }
            }
        }

        // Extract schemas from structs and enums
        for item in &program.items {
            match item {
                Item::Struct(s) => {
                    let schema = Self::struct_to_schema(s, &mut components);
                    components.schemas.insert(s.name.clone(), schema);
                }
                Item::Enum(e) => {
                    let schema = Self::enum_to_schema(e);
                    components.schemas.insert(e.name.clone(), schema);
                }
                _ => {}
            }
        }

        OpenAPISpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: title.to_string(),
                version: version.to_string(),
                description: Some("Auto-generated API documentation from VelinScript".to_string()),
            },
            paths,
            components,
            servers: Some(vec![Server {
                url: "http://localhost:3000".to_string(),
                description: Some("Development server".to_string()),
            }]),
        }
    }

    fn extract_endpoint(function: &Function) -> Option<EndpointInfo> {
        for decorator in &function.decorators {
            if matches!(decorator.name.as_str(), "GET" | "POST" | "PUT" | "DELETE" | "PATCH") {
                if let Some(DecoratorArg::String(route)) = decorator.args.first() {
                    return Some(EndpointInfo {
                        method: decorator.name.clone(),
                        path: route.clone(),
                    });
                }
            }
        }
        None
    }

    fn create_path_item(
        function: &Function,
        endpoint: &EndpointInfo,
        components: &mut Components,
    ) -> PathItem {
        let mut operation = Operation {
            operation_id: Self::to_snake_case(&function.name),
            summary: Some(function.name.clone()),
            description: None,
            parameters: Vec::new(),
            request_body: None,
            responses: HashMap::new(),
            security: None,
            tags: None,
        };

        // Extract parameters
        for param in &function.params {
            let location = if endpoint.path.contains(&format!(":{}", param.name)) {
                "path".to_string()
            } else {
                "query".to_string()
            };

            operation.parameters.push(Parameter {
                name: param.name.clone(),
                location,
                required: param.default.is_none(),
                schema: Self::type_to_schema(&param.param_type, components),
                description: None,
            });
        }

        // Extract return type
        if let Some(ref return_type) = function.return_type {
            let mut responses = HashMap::new();
            responses.insert(
                "200".to_string(),
                Response {
                    description: "Success".to_string(),
                    content: Some({
                        let mut content = HashMap::new();
                        content.insert(
                            "application/json".to_string(),
                            MediaType {
                                schema: Self::type_to_schema(return_type, components),
                            },
                        );
                        content
                    }),
                },
            );
            operation.responses = responses;
        } else {
            operation.responses.insert(
                "200".to_string(),
                Response {
                    description: "Success".to_string(),
                    content: None,
                },
            );
        }

        // Check for security decorators
        let mut has_auth = false;
        for decorator in &function.decorators {
            if decorator.name == "Auth" {
                has_auth = true;
                break;
            }
        }

        if has_auth {
            let mut security_req = HashMap::new();
            security_req.insert("bearerAuth".to_string(), vec![]);
            operation.security = Some(vec![SecurityRequirement {
                requirements: security_req,
            }]);

            // Add security scheme if not exists
            if let Some(ref mut schemes) = components.security_schemes {
                if !schemes.contains_key("bearerAuth") {
                    schemes.insert(
                        "bearerAuth".to_string(),
                        SecurityScheme {
                            scheme_type: "http".to_string(),
                            scheme: Some("bearer".to_string()),
                            bearer_format: Some("JWT".to_string()),
                        },
                    );
                }
            }
        }

        // Create PathItem based on method
        let mut path_item = PathItem {
            get: None,
            post: None,
            put: None,
            delete: None,
            patch: None,
        };

        match endpoint.method.as_str() {
            "GET" => path_item.get = Some(operation),
            "POST" => path_item.post = Some(operation),
            "PUT" => path_item.put = Some(operation),
            "DELETE" => path_item.delete = Some(operation),
            "PATCH" => path_item.patch = Some(operation),
            _ => {}
        }

        path_item
    }

    fn type_to_schema(type_def: &Type, components: &mut Components) -> Schema {
        match type_def {
            Type::String => Schema {
                schema_type: Some("string".to_string()),
                format: None,
                items: None,
                properties: None,
                required: None,
                ref_path: None,
            },
            Type::Number => Schema {
                schema_type: Some("number".to_string()),
                format: Some("double".to_string()),
                items: None,
                properties: None,
                required: None,
                ref_path: None,
            },
            Type::Boolean => Schema {
                schema_type: Some("boolean".to_string()),
                format: None,
                items: None,
                properties: None,
                required: None,
                ref_path: None,
            },
            Type::List(inner) => Schema {
                schema_type: Some("array".to_string()),
                format: None,
                items: Some(Box::new(Self::type_to_schema(inner, components))),
                properties: None,
                required: None,
                ref_path: None,
            },
            Type::Optional(inner) => Self::type_to_schema(inner, components),
            Type::Named(name) => {
                // Check if it's a struct/enum (would be in components.schemas)
                Schema {
                    schema_type: None,
                    format: None,
                    items: None,
                    properties: None,
                    required: None,
                    ref_path: Some(format!("#/components/schemas/{}", name)),
                }
            }
            _ => Schema {
                schema_type: Some("object".to_string()),
                format: None,
                items: None,
                properties: None,
                required: None,
                ref_path: None,
            },
        }
    }

    fn struct_to_schema(struct_def: &Struct, components: &mut Components) -> Schema {
        let mut properties = HashMap::new();
        let mut required = Vec::new();

        for field in &struct_def.fields {
            let field_schema = Self::type_to_schema_simple(&field.field_type, &*components);
            properties.insert(field.name.clone(), field_schema);
            // Assume all fields are required (could be enhanced with Optional detection)
            required.push(field.name.clone());
        }

        Schema {
            schema_type: Some("object".to_string()),
            format: None,
            items: None,
            properties: Some(properties),
            required: Some(required),
            ref_path: None,
        }
    }

    fn enum_to_schema(_enum_def: &Enum) -> Schema {
        // Represent enum as string with enum values
        Schema {
            schema_type: Some("string".to_string()),
            format: None,
            items: None,
            properties: None,
            required: None,
            ref_path: None,
        }
    }

    fn type_to_schema_simple(type_def: &Type, _components: &Components) -> Schema {
        match type_def {
            Type::String => Schema {
                schema_type: Some("string".to_string()),
                format: None,
                items: None,
                properties: None,
                required: None,
                ref_path: None,
            },
            Type::Number => Schema {
                schema_type: Some("number".to_string()),
                format: Some("double".to_string()),
                items: None,
                properties: None,
                required: None,
                ref_path: None,
            },
            Type::Boolean => Schema {
                schema_type: Some("boolean".to_string()),
                format: None,
                items: None,
                properties: None,
                required: None,
                ref_path: None,
            },
            Type::List(inner) => {
                let inner_schema = Self::type_to_schema_simple(inner, _components);
                Schema {
                    schema_type: Some("array".to_string()),
                    format: None,
                    items: Some(Box::new(inner_schema)),
                    properties: None,
                    required: None,
                    ref_path: None,
                }
            }
            Type::Optional(inner) => Self::type_to_schema_simple(inner, _components),
            Type::Named(name) => Schema {
                schema_type: None,
                format: None,
                items: None,
                properties: None,
                required: None,
                ref_path: Some(format!("#/components/schemas/{}", name)),
            },
            _ => Schema {
                schema_type: Some("object".to_string()),
                format: None,
                items: None,
                properties: None,
                required: None,
                ref_path: None,
            },
        }
    }

    fn to_snake_case(s: &str) -> String {
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

#[derive(Debug)]
struct EndpointInfo {
    method: String,
    path: String,
}

/// Generiert OpenAPI Spec
pub fn generate_openapi(program: &Program, title: &str, version: &str) -> OpenAPISpec {
    OpenAPIGenerator::generate(program, title, version)
}

/// Generiert Markdown Dokumentation
pub fn generate_markdown(spec: &OpenAPISpec) -> String {
    let mut md = String::new();
    
    md.push_str(&format!("# {}\n\n", spec.info.title));
    md.push_str(&format!("Version: {}\n\n", spec.info.version));
    
    if let Some(description) = &spec.info.description {
        md.push_str(&format!("{}\n\n", description));
    }
    
    md.push_str("## Endpoints\n\n");
    
    for (path, path_item) in &spec.paths {
        md.push_str(&format!("### {}\n\n", path));
        
        if let Some(ref get) = path_item.get {
            md.push_str(&format!("**GET** `{}`\n\n", path));
            md.push_str(&format!("- Operation ID: `{}`\n", get.operation_id));
            if let Some(ref summary) = get.summary {
                md.push_str(&format!("- Summary: {}\n", summary));
            }
            md.push_str("\n");
        }
        
        if let Some(ref post) = path_item.post {
            md.push_str(&format!("**POST** `{}`\n\n", path));
            md.push_str(&format!("- Operation ID: `{}`\n", post.operation_id));
            if let Some(ref summary) = post.summary {
                md.push_str(&format!("- Summary: {}\n", summary));
            }
            md.push_str("\n");
        }
        
        if let Some(ref put) = path_item.put {
            md.push_str(&format!("**PUT** `{}`\n\n", path));
            md.push_str(&format!("- Operation ID: `{}`\n", put.operation_id));
            md.push_str("\n");
        }
        
        if let Some(ref delete) = path_item.delete {
            md.push_str(&format!("**DELETE** `{}`\n\n", path));
            md.push_str(&format!("- Operation ID: `{}`\n", delete.operation_id));
            md.push_str("\n");
        }
    }
    
    md.push_str("## Schemas\n\n");
    
    for (name, schema) in &spec.components.schemas {
        md.push_str(&format!("### {}\n\n", name));
        if let Some(ref properties) = schema.properties {
            md.push_str("| Field | Type | Required |\n");
            md.push_str("|-------|------|----------|\n");
            for (field_name, field_schema) in properties {
                let required = schema.required.as_ref()
                    .map(|r| r.contains(field_name))
                    .unwrap_or(false);
                let type_str = match &field_schema.schema_type {
                    Some(t) => t.clone(),
                    None => "object".to_string(),
                };
                md.push_str(&format!("| {} | {} | {} |\n", field_name, type_str, if required { "Yes" } else { "No" }));
            }
        }
        md.push_str("\n");
    }
    
    md
}
