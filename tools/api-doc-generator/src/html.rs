// HTML Generator für API-Dokumentation
// Generiert interaktive HTML-Dokumentation mit Swagger UI

use crate::openapi::OpenAPISpec;

pub struct HTMLGenerator;

impl HTMLGenerator {
    /// Generiert HTML-Dokumentation mit Swagger UI
    pub fn generate(spec: &OpenAPISpec) -> String {
        let spec_json = serde_json::to_string(spec).unwrap_or_default();
        let escaped_json = spec_json.replace("</script>", "<\\/script>");
        
        format!(r#"
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - API Dokumentation</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        html {{
            box-sizing: border-box;
            overflow: -moz-scrollbars-vertical;
            overflow-y: scroll;
        }}
        *, *:before, *:after {{
            box-sizing: inherit;
        }}
        body {{
            margin:0;
            background: #fafafa;
        }}
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {{
            const ui = SwaggerUIBundle({{
                spec: {},
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            }});
        }};
    </script>
</body>
</html>
"#, spec.info.title, escaped_json)
    }
    
    /// Generiert einfache HTML-Dokumentation ohne Swagger UI
    pub fn generate_simple(spec: &OpenAPISpec) -> String {
        let mut html = format!(r#"
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - API Dokumentation</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background: #f5f5f5;
        }}
        h1 {{
            color: #333;
            border-bottom: 3px solid #007bff;
            padding-bottom: 10px;
        }}
        h2 {{
            color: #555;
            margin-top: 30px;
            border-bottom: 2px solid #ddd;
            padding-bottom: 5px;
        }}
        h3 {{
            color: #666;
            margin-top: 20px;
        }}
        .endpoint {{
            background: white;
            padding: 15px;
            margin: 15px 0;
            border-radius: 5px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        .method {{
            display: inline-block;
            padding: 5px 10px;
            border-radius: 3px;
            font-weight: bold;
            margin-right: 10px;
        }}
        .method.get {{ background: #61affe; color: white; }}
        .method.post {{ background: #49cc90; color: white; }}
        .method.put {{ background: #fca130; color: white; }}
        .method.delete {{ background: #f93e3e; color: white; }}
        .method.patch {{ background: #50e3c2; color: white; }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 10px 0;
        }}
        th, td {{
            padding: 10px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }}
        th {{
            background: #f8f9fa;
            font-weight: bold;
        }}
        code {{
            background: #f4f4f4;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
        }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <p><strong>Version:</strong> {}</p>
"#, spec.info.title, spec.info.title, spec.info.version);
        
        if let Some(description) = &spec.info.description {
            html.push_str(&format!("<p>{}</p>", description));
        }
        
        html.push_str("<h2>Endpoints</h2>");
        
        for (path, path_item) in &spec.paths {
            html.push_str("<div class=\"endpoint\">");
            html.push_str(&format!("<h3>{}</h3>", path));
            
            if let Some(ref get) = path_item.get {
                html.push_str(&Self::format_operation("GET", get));
            }
            if let Some(ref post) = path_item.post {
                html.push_str(&Self::format_operation("POST", post));
            }
            if let Some(ref put) = path_item.put {
                html.push_str(&Self::format_operation("PUT", put));
            }
            if let Some(ref delete) = path_item.delete {
                html.push_str(&Self::format_operation("DELETE", delete));
            }
            if let Some(ref patch) = path_item.patch {
                html.push_str(&Self::format_operation("PATCH", patch));
            }
            
            html.push_str("</div>");
        }
        
        html.push_str("<h2>Schemas</h2>");
        
        for (name, schema) in &spec.components.schemas {
            html.push_str(&format!("<h3>{}</h3>", name));
            if let Some(ref properties) = schema.properties {
                html.push_str("<table>");
                html.push_str("<tr><th>Field</th><th>Type</th><th>Required</th></tr>");
                for (field_name, field_schema) in properties {
                    let required = schema.required.as_ref()
                        .map(|r| r.contains(field_name))
                        .unwrap_or(false);
                    let type_str = match &field_schema.schema_type {
                        Some(t) => t.clone(),
                        None => "object".to_string(),
                    };
                    html.push_str(&format!(
                        "<tr><td><code>{}</code></td><td>{}</td><td>{}</td></tr>",
                        field_name, type_str, if required { "Yes" } else { "No" }
                    ));
                }
                html.push_str("</table>");
            }
        }
        
        html.push_str("</body></html>");
        html
    }
    
    fn format_operation(method: &str, op: &crate::openapi::Operation) -> String {
        let mut html = format!(
            r#"<div><span class="method {}">{}</span> <code>{}</code></div>"#,
            method.to_lowercase(), method, op.operation_id
        );
        
        if let Some(ref summary) = op.summary {
            html.push_str(&format!("<p><strong>Summary:</strong> {}</p>", summary));
        }
        
        if let Some(ref description) = op.description {
            html.push_str(&format!("<p>{}</p>", description));
        }
        
        if !op.parameters.is_empty() {
            html.push_str("<h4>Parameters</h4><table><tr><th>Name</th><th>Type</th><th>Required</th><th>Description</th></tr>");
            for param in &op.parameters {
                let type_str = match &param.schema.schema_type {
                    Some(t) => t.clone(),
                    None => "object".to_string(),
                };
                html.push_str(&format!(
                    "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    param.name, type_str, if param.required { "Yes" } else { "No" },
                    param.description.as_deref().unwrap_or("")
                ));
            }
            html.push_str("</table>");
        }
        
        html
    }
}
