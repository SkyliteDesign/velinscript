pub struct UrlStdlib;

impl UrlStdlib {
    pub fn generate_parse_code(url_str: &str) -> String {
        format!("url::Url::parse({}).map_err(|e| e.to_string())", url_str)
    }

    pub fn generate_protocol_code(url: &str) -> String {
        format!("{}.scheme().to_string()", url)
    }

    pub fn generate_hostname_code(url: &str) -> String {
        format!(
            "{}.host_str().map(|s| s.to_string()).unwrap_or_else(|| String::new())",
            url
        )
    }

    pub fn generate_port_code(url: &str) -> String {
        format!(
            "{}.port().map(|p| p as i64).unwrap_or_else(|| {{
                let scheme = {}.scheme();
                if scheme == \"https\" {{ 443 }} else if scheme == \"http\" {{ 80 }} else {{ 0 }}
            }})",
            url, url
        )
    }

    pub fn generate_pathname_code(url: &str) -> String {
        format!("{}.path().to_string()", url)
    }

    pub fn generate_search_code(url: &str) -> String {
        format!(
            "{}.query().map(|q| format!(\"?{{}}\", q)).unwrap_or_else(|| String::new())",
            url
        )
    }

    pub fn generate_hash_code(url: &str) -> String {
        format!(
            "{}.fragment().map(|f| format!(\"#{{}}\", f)).unwrap_or_else(|| String::new())",
            url
        )
    }

    pub fn generate_format_code(components: &str) -> String {
        format!(
            "{{
                let comp: serde_json::Value = {};
                let mut url = url::Url::parse(\"http://localhost\").unwrap();
                if let Some(protocol) = comp.get(\"protocol\").and_then(|v| v.as_str()) {{
                    url.set_scheme(protocol.trim_end_matches(':')).unwrap_or_default();
                }}
                if let Some(hostname) = comp.get(\"hostname\").and_then(|v| v.as_str()) {{
                    url.set_host(Some(hostname)).unwrap_or_default();
                }}
                if let Some(port) = comp.get(\"port\").and_then(|v| v.as_u64()) {{
                    url.set_port(Some(port as u16)).unwrap_or_default();
                }}
                if let Some(pathname) = comp.get(\"pathname\").and_then(|v| v.as_str()) {{
                    url.set_path(pathname);
                }}
                if let Some(query) = comp.get(\"query\").and_then(|v| v.as_object()) {{
                    let mut pairs = url.query_pairs_mut();
                    pairs.clear();
                    for (k, v) in query {{
                        if let Some(v_str) = v.as_str() {{
                            pairs.append_pair(k, v_str);
                        }}
                    }}
                }}
                if let Some(hash) = comp.get(\"hash\").and_then(|v| v.as_str()) {{
                    url.set_fragment(Some(hash.trim_start_matches('#')));
                }}
                url.to_string()
            }}",
            components
        )
    }

    pub fn generate_parse_query_code(query_str: &str) -> String {
        format!(
            "{{
                let query = {};
                let mut map = std::collections::HashMap::new();
                for pair in query.trim_start_matches('?').split('&') {{
                    let parts: Vec<&str> = pair.splitn(2, '=').collect();
                    if parts.len() == 2 {{
                        let key = urlencoding::decode(parts[0]).unwrap_or_else(|_| parts[0].to_string());
                        let value = urlencoding::decode(parts[1]).unwrap_or_else(|_| parts[1].to_string());
                        map.insert(key, value);
                    }}
                }}
                serde_json::to_value(map).unwrap_or(serde_json::Value::Null)
            }}",
            query_str
        )
    }

    pub fn generate_stringify_query_code(params: &str) -> String {
        format!(
            "{{
                let params: serde_json::Value = {};
                let mut pairs = Vec::new();
                if let Some(obj) = params.as_object() {{
                    for (k, v) in obj {{
                        let value = if let Some(s) = v.as_str() {{
                            urlencoding::encode(s).to_string()
                        }} else {{
                            v.to_string()
                        }};
                        pairs.push(format!(\"{{}}={{}}\", urlencoding::encode(k), value));
                    }}
                }}
                pairs.join(\"&\")
            }}",
            params
        )
    }
}
