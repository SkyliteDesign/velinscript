
pub struct CsvStdlib;

impl CsvStdlib {
    pub fn generate_read_code(path: &str, has_header: bool) -> String {
        format!(
            "{{
                use std::fs::File;
                use std::io::BufReader;
                let mut reader = csv::Reader::from_reader(BufReader::new(File::open({}).map_err(|e| e.to_string())?));
                let mut records = Vec::new();
                let headers = if {} {{
                    reader.headers().map(|h| h.iter().map(|s| s.to_string()).collect::<Vec<_>>()).ok()
                }} else {{
                    None
                }};
                for result in reader.records() {{
                    if let Ok(record) = result {{
                        let mut map = std::collections::HashMap::new();
                        if let Some(ref hdrs) = headers {{
                            for (i, field) in record.iter().enumerate() {{
                                if i < hdrs.len() {{
                                    map.insert(hdrs[i].clone(), field.to_string());
                                }}
                            }}
                        }} else {{
                            for (i, field) in record.iter().enumerate() {{
                                map.insert(i.to_string(), field.to_string());
                            }}
                        }}
                        records.push(map);
                    }}
                }}
                Ok(records)
            }}",
            path, has_header
        )
    }

    pub fn generate_write_code(path: &str, rows: &str, headers: Option<&str>) -> String {
        if let Some(hdrs) = headers {
            format!(
                "{{
                    use std::fs::File;
                    use std::io::Write;
                    let mut writer = csv::Writer::from_writer(File::create({}).map_err(|e| e.to_string())?);
                    let headers: Vec<String> = {};
                    writer.write_record(&headers).map_err(|e| e.to_string())?;
                    let rows: Vec<std::collections::HashMap<String, String>> = {};
                    for row in rows {{
                        let values: Vec<String> = headers.iter().map(|h| row.get(h).cloned().unwrap_or_default()).collect();
                        writer.write_record(&values).map_err(|e| e.to_string())?;
                    }}
                    writer.flush().map_err(|e| e.to_string())?;
                    Ok(())
                }}",
                path, hdrs, rows
            )
        } else {
            format!(
                "{{
                    use std::fs::File;
                    use std::io::Write;
                    let mut writer = csv::Writer::from_writer(File::create({}).map_err(|e| e.to_string())?);
                    let rows: Vec<std::collections::HashMap<String, String>> = {};
                    if let Some(first_row) = rows.first() {{
                        let headers: Vec<String> = first_row.keys().cloned().collect();
                        writer.write_record(&headers).map_err(|e| e.to_string())?;
                        for row in &rows {{
                            let values: Vec<String> = headers.iter().map(|h| row.get(h).cloned().unwrap_or_default()).collect();
                            writer.write_record(&values).map_err(|e| e.to_string())?;
                        }}
                    }}
                    writer.flush().map_err(|e| e.to_string())?;
                    Ok(())
                }}",
                path, rows
            )
        }
    }

    pub fn generate_parse_code(csv_string: &str) -> String {
        format!(
            "{{
                let mut reader = csv::Reader::from_reader({}.as_bytes());
                let mut records = Vec::new();
                for result in reader.records() {{
                    if let Ok(record) = result {{
                        records.push(record.iter().map(|s| s.to_string()).collect::<Vec<String>>());
                    }}
                }}
                records
            }}",
            csv_string
        )
    }

    pub fn generate_stringify_code(rows: &str, headers: &str) -> String {
        format!(
            "{{
                let mut output = Vec::new();
                let headers: Vec<String> = {};
                output.push(headers.join(\",\"));
                let rows: Vec<Vec<String>> = {};
                for row in rows {{
                    output.push(row.join(\",\"));
                }}
                output.join(\"\\n\")
            }}",
            headers, rows
        )
    }

    pub fn generate_validate_code(path: &str, schema: &str) -> String {
        format!(
            "{{
                use std::fs::File;
                use std::io::BufReader;
                let mut reader = csv::Reader::from_reader(BufReader::new(File::open({}).map_err(|e| e.to_string())?));
                let schema: serde_json::Value = {};
                let headers = reader.headers().map_err(|e| e.to_string())?;
                let expected_headers: Vec<String> = schema.get(\"headers\").and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()).unwrap_or_default();
                if expected_headers.is_empty() || headers.iter().collect::<Vec<_>>() == expected_headers.iter().map(|s| s.as_str()).collect::<Vec<_>>() {{
                    Ok(true)
                }} else {{
                    Err(\"CSV headers do not match schema\".to_string())
                }}
            }}",
            path, schema
        )
    }
}
