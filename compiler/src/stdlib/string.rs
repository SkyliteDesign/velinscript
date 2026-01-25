pub struct StringStdlib;

impl StringStdlib {
    pub fn generate_split_code(text: &str, delimiter: &str) -> String {
        format!(
            "{}.split({}).map(|s| s.to_string()).collect::<Vec<String>>()",
            text, delimiter
        )
    }

    pub fn generate_join_code(list: &str, delimiter: &str) -> String {
        format!("{}.join({})", list, delimiter)
    }

    pub fn generate_replace_code(text: &str, old: &str, new: &str) -> String {
        format!("{}.replace({}, {})", text, old, new)
    }

    pub fn generate_trim_code(text: &str) -> String {
        format!("{}.trim().to_string()", text)
    }

    pub fn generate_slugify_code(text: &str) -> String {
        format!(
            "{{
                {}.to_lowercase()
                  .chars()
                  .map(|c| if c.is_alphanumeric() {{ c }} else {{ '-' }})
                  .collect::<String>()
                  .split('-')
                  .filter(|s| !s.is_empty())
                  .collect::<Vec<_>>()
                  .join(\"-\")
            }}",
            text
        )
    }

    pub fn generate_to_int_code(text: &str) -> String {
        format!("{}.parse::<i64>().map_err(|e| e.to_string())", text)
    }

    pub fn generate_to_float_code(text: &str) -> String {
        format!("{}.parse::<f64>().map_err(|e| e.to_string())", text)
    }

    pub fn generate_capitalize_code(text: &str) -> String {
        format!(
            "{{
                let mut c = {}.chars();
                match c.next() {{
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }}
            }}",
            text
        )
    }

    pub fn generate_lowercase_code(text: &str) -> String {
        format!("{}.to_lowercase()", text)
    }

    pub fn generate_uppercase_code(text: &str) -> String {
        format!("{}.to_uppercase()", text)
    }

    pub fn generate_starts_with_code(text: &str, prefix: &str) -> String {
        format!("{}.starts_with({})", text, prefix)
    }

    pub fn generate_ends_with_code(text: &str, suffix: &str) -> String {
        format!("{}.ends_with({})", text, suffix)
    }
}
