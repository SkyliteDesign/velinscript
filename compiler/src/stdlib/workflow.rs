pub struct WorkflowStdlib;

impl WorkflowStdlib {
    pub fn generate_create_code(definition: &str) -> String {
        format!(
            "{{
                let def: serde_json::Value = {};
                let workflow_id = uuid::Uuid::new_v4().to_string();
                let name = def.get(\"name\").and_then(|v| v.as_str()).unwrap_or(\"workflow\");
                let steps = def.get(\"steps\").and_then(|v| v.as_array()).cloned().unwrap_or_default();
                serde_json::json!({{
                    \"id\": workflow_id,
                    \"name\": name,
                    \"steps\": steps,
                    \"status\": \"pending\",
                    \"created_at\": chrono::Utc::now().to_rfc3339()
                }})
            }}",
            definition
        )
    }

    pub fn generate_start_code(workflow: &str) -> String {
        format!(
            "{{
                let mut wf: serde_json::Value = {};
                wf[\"status\"] = serde_json::json!(\"running\");
                wf[\"started_at\"] = serde_json::json!(chrono::Utc::now().to_rfc3339());
                wf
            }}",
            workflow
        )
    }

    pub fn generate_execute_step_code(workflow: &str, step_id: &str) -> String {
        format!(
            "{{
                let mut wf: serde_json::Value = {};
                let step_id_str = {};
                if let Some(steps) = wf.get_mut(\"steps\").and_then(|v| v.as_array_mut()) {{
                    for step in steps {{
                        if step.get(\"id\").and_then(|v| v.as_str()) == Some(step_id_str) {{
                            step[\"status\"] = serde_json::json!(\"completed\");
                            step[\"completed_at\"] = serde_json::json!(chrono::Utc::now().to_rfc3339());
                        }}
                    }}
                }}
                wf
            }}",
            workflow, step_id
        )
    }

    pub fn generate_get_status_code(workflow: &str) -> String {
        format!(
            "{}.get(\"status\").and_then(|v| v.as_str()).unwrap_or(\"unknown\")",
            workflow
        )
    }

    pub fn generate_get_history_code(workflow: &str) -> String {
        format!(
            "{{
                let wf: serde_json::Value = {};
                let mut history = Vec::new();
                if let Some(steps) = wf.get(\"steps\").and_then(|v| v.as_array()) {{
                    for step in steps {{
                        history.push(serde_json::json!({{
                            \"step_id\": step.get(\"id\"),
                            \"name\": step.get(\"name\"),
                            \"status\": step.get(\"status\"),
                            \"completed_at\": step.get(\"completed_at\")
                        }}));
                    }}
                }}
                history
            }}",
            workflow
        )
    }

    pub fn generate_complete_code(workflow: &str) -> String {
        format!(
            "{{
                let mut wf: serde_json::Value = {};
                wf[\"status\"] = serde_json::json!(\"completed\");
                wf[\"completed_at\"] = serde_json::json!(chrono::Utc::now().to_rfc3339());
                wf
            }}",
            workflow
        )
    }

    pub fn generate_fail_code(workflow: &str, error: &str) -> String {
        format!(
            "{{
                let mut wf: serde_json::Value = {};
                wf[\"status\"] = serde_json::json!(\"failed\");
                wf[\"error\"] = serde_json::json!({});
                wf[\"failed_at\"] = serde_json::json!(chrono::Utc::now().to_rfc3339());
                wf
            }}",
            workflow, error
        )
    }
}
