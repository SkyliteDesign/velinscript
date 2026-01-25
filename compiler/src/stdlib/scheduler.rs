pub struct SchedulerStdlib;

impl SchedulerStdlib {
    pub fn generate_schedule_code(task: &str, cron: &str) -> String {
        format!(
            "{{
                let task_data: serde_json::Value = {};
                let cron_expr = {};
                let task_id = uuid::Uuid::new_v4().to_string();
                serde_json::json!({{
                    \"id\": task_id,
                    \"task\": task_data,
                    \"cron\": cron_expr,
                    \"enabled\": true,
                    \"next_run\": chrono::Utc::now().to_rfc3339(),
                    \"created_at\": chrono::Utc::now().to_rfc3339()
                }})
            }}",
            task, cron
        )
    }

    pub fn generate_schedule_interval_code(task: &str, interval: &str) -> String {
        format!(
            "{{
                let task_data: serde_json::Value = {};
                let interval_ms = {} as u64;
                let task_id = uuid::Uuid::new_v4().to_string();
                serde_json::json!({{
                    \"id\": task_id,
                    \"task\": task_data,
                    \"interval_ms\": interval_ms,
                    \"enabled\": true,
                    \"next_run\": chrono::Utc::now().to_rfc3339(),
                    \"created_at\": chrono::Utc::now().to_rfc3339()
                }})
            }}",
            task, interval
        )
    }

    pub fn generate_cancel_code(task_id: &str) -> String {
        format!(
            "{{
                let id = {};
                // In a real implementation, this would remove the task from the scheduler
                serde_json::json!({{
                    \"id\": id,
                    \"cancelled\": true,
                    \"cancelled_at\": chrono::Utc::now().to_rfc3339()
                }})
            }}",
            task_id
        )
    }

    pub fn generate_list_code() -> String {
        format!(
            "{{
                // In a real implementation, this would return all scheduled tasks
                Vec::<serde_json::Value>::new()
            }}"
        )
    }

    pub fn generate_get_code(task_id: &str) -> String {
        format!(
            "{{
                let id = {};
                // In a real implementation, this would return the task with the given ID
                serde_json::json!({{
                    \"id\": id,
                    \"status\": \"unknown\"
                }})
            }}",
            task_id
        )
    }

    pub fn generate_enable_code(task_id: &str) -> String {
        format!(
            "{{
                let id = {};
                serde_json::json!({{
                    \"id\": id,
                    \"enabled\": true
                }})
            }}",
            task_id
        )
    }

    pub fn generate_disable_code(task_id: &str) -> String {
        format!(
            "{{
                let id = {};
                serde_json::json!({{
                    \"id\": id,
                    \"enabled\": false
                }})
            }}",
            task_id
        )
    }
}
