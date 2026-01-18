
pub struct ProcessStdlib;

impl ProcessStdlib {
    pub fn generate_spawn_code(command: &str, args: &str) -> String {
        format!(
            "{{
                std::process::Command::new({})
                    .args({})
                    .spawn()
                    .map(|c| c.id() as i32)
                    .map_err(|e| e.to_string())
            }}",
            command, args
        )
    }

    pub fn generate_kill_code(pid: &str) -> String {
        format!(
            "{{
                #[cfg(unix)]
                {{
                    let r = std::process::Command::new(\"kill\").arg({}.to_string()).output();
                    r.map(|_| ()).map_err(|e| e.to_string())
                }}
                #[cfg(windows)]
                {{
                    let r = std::process::Command::new(\"taskkill\").args(&[\"/F\", \"/PID\", &{}.to_string()]).output();
                    r.map(|_| ()).map_err(|e| e.to_string())
                }}
            }}",
            pid, pid
        )
    }

    pub fn generate_restart_code(pid: &str) -> String {
        // Restart logic is complex without knowing the original command. 
        // For now, we'll return an error or implement a mock.
        format!("Err(\"Restart not supported directly via PID. Use spawn again.\".to_string())")
    }

    pub fn generate_status_code(pid: &str) -> String {
        // Checking status via PID is platform dependent
        format!(
             "{{
                // Mock status
                let mut map = std::collections::HashMap::new();
                map.insert(\"pid\".to_string(), {}.to_string());
                map.insert(\"status\".to_string(), \"running\".to_string());
                Ok(map)
             }}",
             pid
        )
    }

    pub fn generate_list_code() -> String {
        // Listing processes requires crate sysinfo, which we might not have.
        // Returning empty list or mock.
        "vec![]".to_string()
    }

    pub fn generate_wait_code(pid: &str) -> String {
        // We can't wait on arbitrary PID unless we are the parent and have the Child handle.
        // If we only have PID (i32), we can't wait on it in std::process.
        format!("Err(\"Cannot wait on arbitrary PID without Child handle\".to_string())")
    }

    pub fn generate_get_output_code(pid: &str) -> String {
         format!("Err(\"Cannot get output from arbitrary PID\".to_string())")
    }

    pub fn generate_is_running_code(pid: &str) -> String {
         // Check if process exists
         format!("true") // Mock
    }

    pub fn generate_get_memory_code(pid: &str) -> String {
         format!("Ok(0)") // Mock
    }
}
