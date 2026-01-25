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

    pub fn generate_restart_code(_pid: &str) -> String {
        // Restart logic is complex without knowing the original command.
        // For now, we'll return an error or implement a mock.
        format!("Err(\"Restart not supported directly via PID. Use spawn again.\".to_string())")
    }

    pub fn generate_status_code(pid: &str) -> String {
        format!(
             "{{
                #[cfg(unix)]
                {{
                    use std::process::Command;
                    let output = Command::new(\"ps\").args(&[\"-p\", &{}.to_string(), \"-o\", \"pid,stat,comm\"]).output();
                    match output {{
                        Ok(o) if o.status.success() => {{
                            let mut map = std::collections::HashMap::new();
                            map.insert(\"pid\".to_string(), {}.to_string());
                            map.insert(\"status\".to_string(), \"running\".to_string());
                            Ok(map)
                        }},
                        _ => {{
                            let mut map = std::collections::HashMap::new();
                            map.insert(\"pid\".to_string(), {}.to_string());
                            map.insert(\"status\".to_string(), \"stopped\".to_string());
                            Ok(map)
                        }}
                    }}
                }}
                #[cfg(windows)]
                {{
                    use std::process::Command;
                    let output = Command::new(\"tasklist\").args(&[\"/FI\", &format!(\"PID eq {{}}\", {})]).output();
                    match output {{
                        Ok(o) if o.status.success() && !o.stdout.is_empty() => {{
                            let mut map = std::collections::HashMap::new();
                            map.insert(\"pid\".to_string(), {}.to_string());
                            map.insert(\"status\".to_string(), \"running\".to_string());
                            Ok(map)
                        }},
                        _ => {{
                            let mut map = std::collections::HashMap::new();
                            map.insert(\"pid\".to_string(), {}.to_string());
                            map.insert(\"status\".to_string(), \"stopped\".to_string());
                            Ok(map)
                        }}
                    }}
                }}
                #[cfg(not(any(unix, windows)))]
                {{
                    let mut map = std::collections::HashMap::new();
                    map.insert(\"pid\".to_string(), {}.to_string());
                    map.insert(\"status\".to_string(), \"unknown\".to_string());
                    Ok(map)
                }}
             }}",
             pid, pid, pid, pid, pid, pid, pid
        )
    }

    pub fn generate_list_code() -> String {
        // Listing processes requires crate sysinfo, which we might not have.
        // Returning empty list or mock.
        "vec![]".to_string()
    }

    pub fn generate_wait_code(_pid: &str) -> String {
        // We can't wait on arbitrary PID unless we are the parent and have the Child handle.
        // If we only have PID (i32), we can't wait on it in std::process.
        format!("Err(\"Cannot wait on arbitrary PID without Child handle\".to_string())")
    }

    pub fn generate_get_output_code(_pid: &str) -> String {
        format!("Err(\"Cannot get output from arbitrary PID\".to_string())")
    }

    pub fn generate_is_running_code(pid: &str) -> String {
        format!(
             "{{
                #[cfg(unix)]
                {{
                    std::process::Command::new(\"kill\").args(&[\"-0\", &{}.to_string()]).output().map(|o| o.status.success()).unwrap_or(false)
                }}
                #[cfg(windows)]
                {{
                    std::process::Command::new(\"tasklist\").args(&[\"/FI\", &format!(\"PID eq {{}}\", {})]).output().map(|o| o.status.success() && !o.stdout.is_empty()).unwrap_or(false)
                }}
                #[cfg(not(any(unix, windows)))]
                {{
                    false
                }}
             }}",
             pid, pid
         )
    }

    pub fn generate_get_memory_code(pid: &str) -> String {
        format!(
             "{{
                #[cfg(unix)]
                {{
                    use std::process::Command;
                    let output = Command::new(\"ps\").args(&[\"-p\", &{}.to_string(), \"-o\", \"rss=\"]).output();
                    match output {{
                        Ok(o) if o.status.success() => {{
                            let mem_str = String::from_utf8_lossy(&o.stdout).trim().to_string();
                            mem_str.parse::<i64>().map(|kb| kb * 1024).map_err(|e| e.to_string())
                        }},
                        _ => Ok(0)
                    }}
                }}
                #[cfg(windows)]
                {{
                    use std::process::Command;
                    let output = Command::new(\"tasklist\").args(&[\"/FI\", &format!(\"PID eq {{}}\", {}), \"/FO\", \"CSV\"]).output();
                    match output {{
                        Ok(o) if o.status.success() => {{
                            let output_str = String::from_utf8_lossy(&o.stdout);
                            // Parse memory from CSV output (column 5)
                            let lines: Vec<&str> = output_str.lines().collect();
                            if lines.len() > 1 {{
                                let cols: Vec<&str> = lines[1].split(',').collect();
                                if cols.len() > 5 {{
                                    let mem_str = cols[4].trim_matches('\"').replace(\" K\", \"\").replace(\",\", \"\");
                                    mem_str.parse::<i64>().map(|kb| kb * 1024).map_err(|e| e.to_string())
                                }} else {{
                                    Ok(0)
                                }}
                            }} else {{
                                Ok(0)
                            }}
                        }},
                        _ => Ok(0)
                    }}
                }}
                #[cfg(not(any(unix, windows)))]
                {{
                    Ok(0)
                }}
             }}",
             pid, pid
         )
    }
}
