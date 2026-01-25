pub struct FsStdlib;

impl FsStdlib {
    pub fn generate_read_json_code(path: &str) -> String {
        format!(
            "std::fs::read_to_string({}).map_err(|e| e.to_string()).and_then(|s| serde_json::from_str::<serde_json::Value>(&s).map_err(|e| e.to_string()))",
            path
        )
    }

    pub fn generate_write_json_code(path: &str, value: &str) -> String {
        format!(
            "serde_json::to_string_pretty(&{}).map_err(|e| e.to_string()).and_then(|s| std::fs::write({}, s).map_err(|e| e.to_string()))",
            value, path
        )
    }

    pub fn generate_exists_code(path: &str) -> String {
        format!("std::path::Path::new({}).exists()", path)
    }

    pub fn generate_mkdir_code(path: &str) -> String {
        format!(
            "std::fs::create_dir_all({}).map_err(|e| e.to_string())",
            path
        )
    }

    pub fn generate_copy_code(source: &str, dest: &str) -> String {
        format!(
            "std::fs::copy({}, {}).map(|_| ()).map_err(|e| e.to_string())",
            source, dest
        )
    }

    pub fn generate_move_file_code(source: &str, dest: &str) -> String {
        format!(
            "std::fs::rename({}, {}).map_err(|e| e.to_string())",
            source, dest
        )
    }

    pub fn generate_get_size_code(path: &str) -> String {
        format!(
            "std::fs::metadata({}).map(|m| m.len()).map_err(|e| e.to_string())",
            path
        )
    }

    pub fn generate_list_files_code(path: &str) -> String {
        format!(
            "std::fs::read_dir({}).map(|entries| entries.filter_map(|e| e.ok()).map(|e| e.path().display().to_string()).collect::<Vec<_>>()).map_err(|e| e.to_string())",
            path
        )
    }

    pub fn generate_is_empty_code(path: &str) -> String {
        format!(
            "std::fs::read_dir({}).map(|mut i| i.next().is_none()).unwrap_or(false)",
            path
        )
    }
}
