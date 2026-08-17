
pub struct SandboxStdlib;

impl SandboxStdlib {
    pub fn generate_build_code(project_path: &str) -> String {
        format!(
            "std::process::Command::new(\"velin\").arg(\"compile\").arg(\"-i\").arg({}).output().map(|_| ()).map_err(|e| e.to_string())",
            project_path
        )
    }

    pub fn generate_test_code(project_path: &str) -> String {
        format!(
            "std::process::Command::new(\"velin\").arg(\"test\").current_dir({}).output().map(|_| ()).map_err(|e| e.to_string())",
            project_path
        )
    }

    pub fn generate_validate_code(code: &str) -> String {
        // Create temp file and check
        format!(
            "{{
                use std::io::Write;
                let mut file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
                file.write_all({}.as_bytes()).map_err(|e| e.to_string())?;
                std::process::Command::new(\"velin\").arg(\"check\").arg(\"-i\").arg(file.path()).output().map(|_| ()).map_err(|e| e.to_string())
            }}",
            code
        )
    }

    pub fn generate_run_code(_code: &str) -> String {
        format!(
             "{{
                // Not strictly safe 'sandbox', but executes code
                Err(\"Sandbox run not implemented yet\".to_string())
             }}"
        )
    }

    pub fn generate_lint_code(code: &str) -> String {
        format!(
            "{{
                use std::io::Write;
                let mut file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
                file.write_all({}.as_bytes()).map_err(|e| e.to_string())?;
                let output = std::process::Command::new(\"velin\").arg(\"lint\").arg(\"-i\").arg(file.path()).output().map_err(|e| e.to_string())?;
                if output.status.success() {{
                    Ok(vec![])
                }} else {{
                    let errors: Vec<String> = String::from_utf8_lossy(&output.stderr)
                        .lines()
                        .filter(|l| l.contains(\"error\") || l.contains(\"warning\"))
                        .map(|s| s.to_string())
                        .collect();
                    Ok(errors)
                }}
            }}",
            code
        )
    }

    pub fn generate_format_code(code: &str) -> String {
        format!(
            "{{
                use std::io::Write;
                let mut file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
                file.write_all({}.as_bytes()).map_err(|e| e.to_string())?;
                let output = std::process::Command::new(\"velin\").arg(\"format\").arg(\"-i\").arg(file.path()).output().map_err(|e| e.to_string())?;
                if output.status.success() {{
                    std::fs::read_to_string(file.path()).map_err(|e| e.to_string())
                }} else {{
                    Ok({}.to_string())
                }}
            }}",
            code, code
        )
    }

    pub fn generate_check_types_code(code: &str) -> String {
        format!(
            "{{
                use std::io::Write;
                let mut file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
                file.write_all({}.as_bytes()).map_err(|e| e.to_string())?;
                let output = std::process::Command::new(\"velin\").arg(\"check\").arg(\"-i\").arg(file.path()).output().map_err(|e| e.to_string())?;
                if output.status.success() {{
                    Ok(())
                }} else {{
                    Err(format!(\"Type check failed: {{}}\", String::from_utf8_lossy(&output.stderr)))
                }}
            }}",
            code
        )
    }

    pub fn generate_optimize_code(code: &str) -> String {
        format!(
            "{{
                use std::io::Write;
                let mut file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
                file.write_all({}.as_bytes()).map_err(|e| e.to_string())?;
                let output = std::process::Command::new(\"velin\").arg(\"compile\").arg(\"-i\").arg(file.path()).arg(\"--optimize\").output().map_err(|e| e.to_string())?;
                if output.status.success() {{
                    // Read optimized code from output or return original
                    Ok({}.to_string())
                }} else {{
                    Ok({}.to_string())
                }}
            }}",
            code, code, code
        )
    }
}
