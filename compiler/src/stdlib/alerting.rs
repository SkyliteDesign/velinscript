
pub struct AlertingStdlib;

impl AlertingStdlib {
    pub fn generate_create_rule_code(rule: &str) -> String {
        format!(
            "{{
                let rule_data: serde_json::Value = {};
                let rule_id = rule_data.get(\"id\").and_then(|v| v.as_str()).unwrap_or(&uuid::Uuid::new_v4().to_string());
                let condition = rule_data.get(\"condition\").and_then(|v| v.as_str()).unwrap_or(\"\");
                let threshold = rule_data.get(\"threshold\").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let action = rule_data.get(\"action\").and_then(|v| v.as_str()).unwrap_or(\"notify\");
                serde_json::json!({{
                    \"id\": rule_id,
                    \"condition\": condition,
                    \"threshold\": threshold,
                    \"action\": action,
                    \"enabled\": true,
                    \"created_at\": chrono::Utc::now().to_rfc3339()
                }})
            }}",
            rule
        )
    }

    pub fn generate_check_code(metric: &str, value: &str, rules: &str) -> String {
        format!(
            "{{
                let metric_name = {};
                let metric_value: f64 = {};
                let rules: Vec<serde_json::Value> = {};
                let mut triggered = Vec::new();
                for rule in rules {{
                    if let Some(enabled) = rule.get(\"enabled\").and_then(|v| v.as_bool()) {{
                        if enabled {{
                            if let Some(threshold) = rule.get(\"threshold\").and_then(|v| v.as_f64()) {{
                                let condition = rule.get(\"condition\").and_then(|v| v.as_str()).unwrap_or(\"\");
                                let should_trigger = match condition {{
                                    \">\" => metric_value > threshold,
                                    \">=\" => metric_value >= threshold,
                                    \"<\" => metric_value < threshold,
                                    \"<=\" => metric_value <= threshold,
                                    \"==\" => (metric_value - threshold).abs() < 0.001,
                                    _ => false
                                }};
                                if should_trigger {{
                                    triggered.push(rule.clone());
                                }}
                            }}
                        }}
                    }}
                }}
                triggered
            }}",
            metric, value, rules
        )
    }

    pub fn generate_trigger_code(alert: &str) -> String {
        format!(
            "{{
                use std::fs::OpenOptions;
                use std::io::Write;
                let alert_data: serde_json::Value = {};
                let alert_entry = serde_json::json!({{
                    \"id\": uuid::Uuid::new_v4().to_string(),
                    \"rule_id\": alert_data.get(\"rule_id\").and_then(|v| v.as_str()),
                    \"metric\": alert_data.get(\"metric\").and_then(|v| v.as_str()),
                    \"value\": alert_data.get(\"value\").and_then(|v| v.as_f64()),
                    \"timestamp\": chrono::Utc::now().to_rfc3339(),
                    \"status\": \"triggered\"
                }});
                let alert_file = std::env::var(\"ALERT_LOG_FILE\").unwrap_or_else(|_| \"alerts.log\".to_string());
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&alert_file)
                    .map_err(|e| e.to_string())?;
                writeln!(file, \"{{}}\", serde_json::to_string(&alert_entry).unwrap_or_default()).map_err(|e| e.to_string())?;
                Ok(())
            }}",
            alert
        )
    }

    pub fn generate_history_code(filters: &str) -> String {
        format!(
            "{{
                use std::fs::File;
                use std::io::{BufRead, BufReader};
                let filters: serde_json::Value = {};
                let alert_file = std::env::var(\"ALERT_LOG_FILE\").unwrap_or_else(|_| \"alerts.log\".to_string());
                let file = File::open(&alert_file).map_err(|e| e.to_string())?;
                let reader = BufReader::new(file);
                let mut results = Vec::new();
                for line in reader.lines() {{
                    if let Ok(line_str) = line {{
                        if let Ok(entry) = serde_json::from_str::<serde_json::Value>(&line_str) {{
                            let mut matches = true;
                            if let Some(rule_id) = filters.get(\"rule_id\").and_then(|v| v.as_str()) {{
                                if entry.get(\"rule_id\").and_then(|v| v.as_str()) != Some(rule_id) {{
                                    matches = false;
                                }}
                            }}
                            if let Some(metric) = filters.get(\"metric\").and_then(|v| v.as_str()) {{
                                if entry.get(\"metric\").and_then(|v| v.as_str()) != Some(metric) {{
                                    matches = false;
                                }}
                            }}
                            if matches {{
                                results.push(entry);
                            }}
                        }}
                    }}
                }}
                Ok(results)
            }}",
            filters
        )
    }
}
