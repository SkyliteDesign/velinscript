pub struct PathStdlib;

impl PathStdlib {
    pub fn generate_join_code(parts: &str) -> String {
        format!(
            "{{
                let parts: Vec<&str> = {};
                parts.iter().collect::<std::path::PathBuf>().to_string_lossy().to_string()
            }}",
            parts
        )
    }

    pub fn generate_dirname_code(path: &str) -> String {
        format!(
            "std::path::Path::new({}).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|| String::new())",
            path
        )
    }

    pub fn generate_basename_code(path: &str) -> String {
        format!(
            "std::path::Path::new({}).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| String::new())",
            path
        )
    }

    pub fn generate_extname_code(path: &str) -> String {
        format!(
            "std::path::Path::new({}).extension().map(|e| format!(\".{{}}\", e.to_string_lossy())).unwrap_or_else(|| String::new())",
            path
        )
    }

    pub fn generate_normalize_code(path: &str) -> String {
        format!(
            "std::path::Path::new({}).canonicalize().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| {}.to_string())",
            path, path
        )
    }

    pub fn generate_resolve_code(path: &str) -> String {
        format!(
            "std::path::Path::new({}).canonicalize().map(|p| p.to_string_lossy().to_string()).map_err(|e| e.to_string())",
            path
        )
    }

    pub fn generate_relative_code(from: &str, to: &str) -> String {
        format!(
            "{{
                let from_path = std::path::Path::new({});
                let to_path = std::path::Path::new({});
                pathdiff::diff_paths(to_path, from_path).map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|| String::new())
            }}",
            from, to
        )
    }

    pub fn generate_is_absolute_code(path: &str) -> String {
        format!("std::path::Path::new({}).is_absolute()", path)
    }

    pub fn generate_separator_code() -> String {
        "std::path::MAIN_SEPARATOR.to_string()".to_string()
    }
}
