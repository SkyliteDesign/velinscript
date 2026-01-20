
pub struct TemplateStdlib;

impl TemplateStdlib {
    pub fn generate_render_code(template: &str, data: &str) -> String {
        format!(
            "{{
                let template_str = {};
                let data_obj: serde_json::Value = {};
                let mut result = template_str.clone();
                if let Some(obj) = data_obj.as_object() {{
                    for (key, value) in obj {{
                        let value_str = if let Some(s) = value.as_str() {{
                            s.to_string()
                        }} else {{
                            value.to_string()
                        }};
                        result = result.replace(&format!(\"{{{{{{}}}}\", key), &value_str);
                        result = result.replace(&format!(\"{{{{{{}}}}\", key), &value_str);
                    }}
                }}
                result
            }}",
            template, data
        )
    }

    pub fn generate_render_file_code(path: &str, data: &str) -> String {
        format!(
            "{{
                use std::fs;
                let template = fs::read_to_string({}).map_err(|e| e.to_string())?;
                let data_obj: serde_json::Value = {};
                let mut result = template.clone();
                if let Some(obj) = data_obj.as_object() {{
                    for (key, value) in obj {{
                        let value_str = if let Some(s) = value.as_str() {{
                            s.to_string()
                        }} else {{
                            value.to_string()
                        }};
                        result = result.replace(&format!(\"{{{{{{}}}}\", key), &value_str);
                        result = result.replace(&format!(\"{{{{{{}}}}\", key), &value_str);
                    }}
                }}
                Ok(result)
            }}",
            path, data
        )
    }

    pub fn generate_partial_code(partial_path: &str, data: &str) -> String {
        format!(
            "{{
                use std::fs;
                let partial = fs::read_to_string({}).map_err(|e| e.to_string())?;
                let data_obj: serde_json::Value = {};
                let mut result = partial.clone();
                if let Some(obj) = data_obj.as_object() {{
                    for (key, value) in obj {{
                        let value_str = if let Some(s) = value.as_str() {{
                            s.to_string()
                        }} else {{
                            value.to_string()
                        }};
                        result = result.replace(&format!(\"{{{{{{}}}}\", key), &value_str);
                        result = result.replace(&format!(\"{{{{{{}}}}\", key), &value_str);
                    }}
                }}
                Ok(result)
            }}",
            partial_path, data
        )
    }

    pub fn generate_cache_code(template: &str, cache_key: &str) -> String {
        format!(
            "{{
                use std::collections::HashMap;
                use std::sync::{Arc, Mutex};
                static TEMPLATE_CACHE: once_cell::sync::Lazy<Arc<Mutex<HashMap<String, String>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
                let key = {};
                let template_str = {};
                let mut cache = TEMPLATE_CACHE.lock().unwrap();
                if let Some(cached) = cache.get(&key) {{
                    cached.clone()
                }} else {{
                    cache.insert(key.clone(), template_str.clone());
                    template_str
                }}
            }}",
            cache_key, template
        )
    }
}
