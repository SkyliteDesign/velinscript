// Code Generation für Boilerplate

pub struct BoilerplateGenerator;

impl BoilerplateGenerator {
    pub fn new() -> Self {
        BoilerplateGenerator
    }
    
    pub fn generate_api(&self, name: &str, path: Option<&str>) -> String {
        let api_name = name;
        let default_path = format!("/api/{}", api_name.to_lowercase());
        let api_path = path.unwrap_or(&default_path);
        
        format!(
            r#"// Auto-generated API: {}

@GET("{}")
fn get{}(): string {{
    return "Hello from {} API";
}}

@POST("{}")
fn create{}(data: string): string {{
    return "Created: " + data;
}}

@GET("{}/:id")
fn get{}ById(id: string): string {{
    return "Get {} by ID: " + id;
}}

@PUT("{}/:id")
fn update{}(id: string, data: string): string {{
    return "Updated {}: " + id;
}}

@DELETE("{}/:id")
fn delete{}(id: string): boolean {{
    return true;
}}
"#,
            api_name,
            api_path,
            api_name,
            api_name,
            api_path,
            api_name,
            api_path,
            api_name,
            api_name,
            api_path,
            api_name,
            api_name,
            api_path,
            api_name
        )
    }
    
    pub fn generate_crud(&self, model_name: &str, fields: &str) -> String {
        // Parse fields: "id:string,name:string,email:string"
        let field_list: Vec<&str> = fields.split(',').collect();
        let struct_fields: Vec<String> = field_list
            .iter()
            .map(|f| {
                let parts: Vec<&str> = f.split(':').collect();
                if parts.len() == 2 {
                    format!("    {}: {},", parts[0].trim(), parts[1].trim())
                } else {
                    format!("    {}: string,", f.trim())
                }
            })
            .collect();
        
        let struct_fields_str = struct_fields.join("\n");
        let model_lower = model_name.to_lowercase();
        let api_path = format!("/api/{}", model_lower);
        
        format!(
            r#"// Auto-generated CRUD for {}

struct {} {{
{}
}}

@GET("{}")
fn get{}s(): List<{}> {{
    return db.findAll({});
}}

@GET("{}/:id")
fn get{}(id: string): {} {{
    return db.find({}, id);
}}

@POST("{}")
fn create{}({}: {}): {} {{
    return db.save({});
}}

@PUT("{}/:id")
fn update{}(id: string, {}: {}): {} {{
    return db.save({});
}}

@DELETE("{}/:id")
fn delete{}(id: string): boolean {{
    return db.delete({}, id);
}}
"#,
            model_name,
            model_name,
            struct_fields_str,
            api_path,
            model_name,
            model_name,
            model_name,
            api_path,
            model_name,
            model_name,
            model_name,
            api_path,
            model_name,
            model_lower,
            model_name,
            model_name,
            model_lower,
            api_path,
            model_name,
            model_lower,
            model_name,
            model_name,
            model_lower,
            api_path,
            model_name,
            model_name
        )
    }
    
    pub fn generate_test(&self, function_name: &str) -> String {
        // Convert function_name to camelCase for test function name: test + FunctionName
        let test_fn_name = if function_name.is_empty() {
            "test".to_string()
        } else {
            format!("test{}", function_name)
        };
        
        format!(
            r#"// Auto-generated test for {}

@test
fn {}() {{
    // Test generated automatically. 
    // Please add specific assertions relevant to the function logic.
    let result = {}(/* params */);
    assert(result != null); 
}}
"#,
            function_name,
            test_fn_name,
            function_name
        )
    }
    
    pub fn generate_responses_module(&self) -> String {
        include_str!("../../../tools/vscode-extension/templates/responses.velin").to_string()
    }
    
    pub fn generate_errors_module(&self) -> String {
        include_str!("../../../tools/vscode-extension/templates/errors.velin").to_string()
    }
    
    pub fn generate_logging_module(&self) -> String {
        include_str!("../../../tools/vscode-extension/templates/logging.velin").to_string()
    }
    
    pub fn generate_cache_module(&self) -> String {
        include_str!("../../../tools/vscode-extension/templates/cache.velin").to_string()
    }
    
    pub fn generate_health_module(&self) -> String {
        include_str!("../../../tools/vscode-extension/templates/health.velin").to_string()
    }
    
    pub fn generate_async_module(&self) -> String {
        include_str!("../../../tools/vscode-extension/templates/async.velin").to_string()
    }
    
    pub fn generate_security_module(&self) -> String {
        include_str!("../../../tools/vscode-extension/templates/security.velin").to_string()
    }
}
