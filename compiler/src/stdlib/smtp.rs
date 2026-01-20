
pub struct SmtpStdlib;

impl SmtpStdlib {
    pub fn generate_connect_code(config: &str) -> String {
        format!(
            "{{
                let config: serde_json::Value = {};
                let host = config.get(\"host\").and_then(|v| v.as_str()).unwrap_or(\"localhost\");
                let port = config.get(\"port\").and_then(|v| v.as_u64()).unwrap_or(587) as u16;
                let username = config.get(\"username\").and_then(|v| v.as_str());
                let password = config.get(\"password\").and_then(|v| v.as_str());
                let tls = config.get(\"tls\").and_then(|v| v.as_bool()).unwrap_or(false);
                
                async move {{
                    let builder = lettre::SmtpTransport::relay(host).map_err(|e| e.to_string())?;
                    let creds = if let (Some(user), Some(pass)) = (username, password) {{
                        Some(lettre::transport::smtp::authentication::Credentials::new(user.to_string(), pass.to_string()))
                    }} else {{
                        None
                    }};
                    let mailer = if tls {{
                        builder.port(port).credentials(creds).build()
                    }} else {{
                        builder.port(port).credentials(creds).build()
                    }};
                    Ok(mailer)
                }}
            }}",
            config
        )
    }

    pub fn generate_send_code(mailer: &str, email: &str) -> String {
        format!(
            "{{
                let email_data: serde_json::Value = {};
                let from_addr = email_data.get(\"from\").and_then(|v| v.as_str()).unwrap_or(\"noreply@example.com\");
                let to_addrs: Vec<&str> = email_data.get(\"to\").and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(|v| v.as_str()).collect()).unwrap_or_default();
                let subject = email_data.get(\"subject\").and_then(|v| v.as_str()).unwrap_or(\"\");
                let body = email_data.get(\"body\").and_then(|v| v.as_str()).unwrap_or(\"\");
                let html = email_data.get(\"html\").and_then(|v| v.as_str());
                
                let message = lettre::Message::builder()
                    .from(from_addr.parse().map_err(|e| e.to_string())?)
                    .to(to_addrs.first().unwrap_or(&from_addr).parse().map_err(|e| e.to_string())?)
                    .subject(subject)
                    .body(if let Some(html_body) = html {{
                        html_body.to_string()
                    }} else {{
                        body.to_string()
                    }})
                    .map_err(|e| e.to_string())?;
                
                {}.send(&message).map(|_| ()).map_err(|e| e.to_string())
            }}",
            email, mailer
        )
    }

    pub fn generate_template_code(template_path: &str, data: &str) -> String {
        format!(
            "{{
                use std::fs;
                let template = fs::read_to_string({}).map_err(|e| e.to_string())?;
                let data: serde_json::Value = {};
                let mut result = template.clone();
                if let Some(obj) = data.as_object() {{
                    for (key, value) in obj {{
                        let value_str = if let Some(s) = value.as_str() {{
                            s.to_string()
                        }} else {{
                            value.to_string()
                        }};
                        result = result.replace(&format!(\"{{{{{{}}}}\", key), &value_str);
                    }}
                }}
                Ok(result)
            }}",
            template_path, data
        )
    }
}
