pub struct FixturesStdlib;

impl FixturesStdlib {
    pub fn generate_create_code(template: &str) -> String {
        format!(
            "{{
                let template_data: serde_json::Value = {};
                let mut fixture = template_data.clone();
                // Generate random values for placeholders
                if let Some(obj) = fixture.as_object_mut() {{
                    for (key, value) in obj.iter_mut() {{
                        if let Some(str_val) = value.as_str() {{
                            if str_val == \"{{random_string}}\" {{
                                *value = serde_json::json!(uuid::Uuid::new_v4().to_string());
                            }} else if str_val == \"{{random_number}}\" {{
                                *value = serde_json::json!(rand::random::<i64>());
                            }} else if str_val == \"{{random_email}}\" {{
                                *value = serde_json::json!(format!(\"{{}}@example.com\", uuid::Uuid::new_v4().to_string()));
                            }}
                        }}
                    }}
                }}
                fixture
            }}",
            template
        )
    }

    pub fn generate_create_many_code(template: &str, count: &str) -> String {
        format!(
            "{{
                let template_data: serde_json::Value = {};
                let count_num = {} as usize;
                let mut fixtures = Vec::new();
                for _ in 0..count_num {{
                    let mut fixture = template_data.clone();
                    if let Some(obj) = fixture.as_object_mut() {{
                        for (key, value) in obj.iter_mut() {{
                            if let Some(str_val) = value.as_str() {{
                                if str_val == \"{{random_string}}\" {{
                                    *value = serde_json::json!(uuid::Uuid::new_v4().to_string());
                                }} else if str_val == \"{{random_number}}\" {{
                                    *value = serde_json::json!(rand::random::<i64>());
                                }} else if str_val == \"{{random_email}}\" {{
                                    *value = serde_json::json!(format!(\"{{}}@example.com\", uuid::Uuid::new_v4().to_string()));
                                }}
                            }}
                        }}
                    }}
                    fixtures.push(fixture);
                }}
                fixtures
            }}",
            template, count
        )
    }

    pub fn generate_factory_code(name: &str, builder: &str) -> String {
        format!(
            "{{
                let factory_name = {};
                let builder_fn = {};
                // In a real implementation, this would register a factory
                serde_json::json!({{
                    \"name\": factory_name,
                    \"registered\": true
                }})
            }}",
            name, builder
        )
    }

    pub fn generate_build_code(factory: &str, overrides: &str) -> String {
        format!(
            "{{
                let factory_data: serde_json::Value = {};
                let overrides_data: serde_json::Value = {};
                let mut fixture = serde_json::json!({{
                    \"id\": uuid::Uuid::new_v4().to_string(),
                    \"created_at\": chrono::Utc::now().to_rfc3339()
                }});
                // Apply overrides
                if let (Some(factory_obj), Some(overrides_obj)) = (factory_data.as_object(), overrides_data.as_object()) {{
                    if let Some(fixture_obj) = fixture.as_object_mut() {{
                        for (key, value) in overrides_obj {{
                            fixture_obj.insert(key.clone(), value.clone());
                        }}
                    }}
                }}
                fixture
            }}",
            factory, overrides
        )
    }
}
