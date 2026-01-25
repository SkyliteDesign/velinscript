pub struct EncodingStdlib;

impl EncodingStdlib {
    pub fn generate_base64_encode_code(input: &str) -> String {
        format!(
            "base64::engine::general_purpose::STANDARD.encode({}.as_bytes())",
            input
        )
    }

    pub fn generate_base64_decode_code(input: &str) -> String {
        format!(
            "base64::engine::general_purpose::STANDARD.decode({}).map(|v| String::from_utf8_lossy(&v).to_string()).map_err(|e| e.to_string())",
            input
        )
    }

    pub fn generate_url_encode_code(input: &str) -> String {
        format!("urlencoding::encode({}).to_string()", input)
    }

    pub fn generate_url_decode_code(input: &str) -> String {
        format!(
            "urlencoding::decode({}).map(|s| s.to_string()).unwrap_or_else(|_| String::new())",
            input
        )
    }

    pub fn generate_hex_encode_code(input: &str) -> String {
        format!("hex::encode({}.as_bytes())", input)
    }

    pub fn generate_hex_decode_code(input: &str) -> String {
        format!(
            "hex::decode({}).map(|v| String::from_utf8_lossy(&v).to_string()).map_err(|e| e.to_string())",
            input
        )
    }

    pub fn generate_is_valid_utf8_code(bytes: &str) -> String {
        format!("String::from_utf8({}.clone()).is_ok()", bytes)
    }

    pub fn generate_fix_utf8_code(bytes: &str) -> String {
        format!(
            "{{
                let mut fixed = Vec::new();
                let mut iter = {}.iter();
                while let Some(&byte) = iter.next() {{
                    if byte < 0x80 {{
                        fixed.push(byte);
                    }} else if byte < 0xE0 {{
                        if let Some(&b2) = iter.next() {{
                            fixed.push(byte);
                            fixed.push(b2);
                        }}
                    }} else if byte < 0xF0 {{
                        if let (Some(&b2), Some(&b3)) = (iter.next(), iter.next()) {{
                            fixed.push(byte);
                            fixed.push(b2);
                            fixed.push(b3);
                        }}
                    }}
                }}
                fixed
            }}",
            bytes
        )
    }
}
