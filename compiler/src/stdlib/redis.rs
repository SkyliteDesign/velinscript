
pub struct RedisStdlib;

impl RedisStdlib {
    pub fn generate_connect_code(url: &str) -> String {
        format!(
            "redis::Client::open({}).map_err(|e| e.to_string()).and_then(|client| client.get_connection().map_err(|e| e.to_string()))",
            url
        )
    }

    pub fn generate_set_code(client: &str, key: &str, value: &str, ttl: Option<&str>) -> String {
        if let Some(ttl_val) = ttl {
            format!(
                "redis::cmd(\"SET\").arg({}).arg({}).arg(\"EX\").arg({}).query::<_, String>({}).map_err(|e| e.to_string())",
                key, value, ttl_val, client
            )
        } else {
            format!(
                "redis::cmd(\"SET\").arg({}).arg({}).query::<_, String>({}).map_err(|e| e.to_string())",
                key, value, client
            )
        }
    }

    pub fn generate_get_code(client: &str, key: &str) -> String {
        format!(
            "redis::cmd(\"GET\").arg({}).query::<_, Option<String>>({}).map_err(|e| e.to_string())",
            key, client
        )
    }

    pub fn generate_delete_code(client: &str, key: &str) -> String {
        format!(
            "redis::cmd(\"DEL\").arg({}).query::<_, i64>({}).map(|n| n > 0).map_err(|e| e.to_string())",
            key, client
        )
    }

    pub fn generate_hset_code(client: &str, hash: &str, field: &str, value: &str) -> String {
        format!(
            "redis::cmd(\"HSET\").arg({}).arg({}).arg({}).query::<_, i64>({}).map(|_| ()).map_err(|e| e.to_string())",
            hash, field, value, client
        )
    }

    pub fn generate_hget_code(client: &str, hash: &str, field: &str) -> String {
        format!(
            "redis::cmd(\"HGET\").arg({}).arg({}).query::<_, Option<String>>({}).map_err(|e| e.to_string())",
            hash, field, client
        )
    }

    pub fn generate_hgetall_code(client: &str, hash: &str) -> String {
        format!(
            "redis::cmd(\"HGETALL\").arg({}).query::<_, Vec<String>>({}).map(|v| {{
                let mut map = std::collections::HashMap::new();
                for chunk in v.chunks(2) {{
                    if chunk.len() == 2 {{
                        map.insert(chunk[0].clone(), chunk[1].clone());
                    }}
                }}
                map
            }}).map_err(|e| e.to_string())",
            hash, client
        )
    }

    pub fn generate_lpush_code(client: &str, list: &str, value: &str) -> String {
        format!(
            "redis::cmd(\"LPUSH\").arg({}).arg({}).query::<_, i64>({}).map(|_| ()).map_err(|e| e.to_string())",
            list, value, client
        )
    }

    pub fn generate_rpush_code(client: &str, list: &str, value: &str) -> String {
        format!(
            "redis::cmd(\"RPUSH\").arg({}).arg({}).query::<_, i64>({}).map(|_| ()).map_err(|e| e.to_string())",
            list, value, client
        )
    }

    pub fn generate_lpop_code(client: &str, list: &str) -> String {
        format!(
            "redis::cmd(\"LPOP\").arg({}).query::<_, Option<String>>({}).map_err(|e| e.to_string())",
            list, client
        )
    }

    pub fn generate_llen_code(client: &str, list: &str) -> String {
        format!(
            "redis::cmd(\"LLEN\").arg({}).query::<_, i64>({}).map_err(|e| e.to_string())",
            list, client
        )
    }

    pub fn generate_sadd_code(client: &str, set: &str, member: &str) -> String {
        format!(
            "redis::cmd(\"SADD\").arg({}).arg({}).query::<_, i64>({}).map(|_| ()).map_err(|e| e.to_string())",
            set, member, client
        )
    }

    pub fn generate_sismember_code(client: &str, set: &str, member: &str) -> String {
        format!(
            "redis::cmd(\"SISMEMBER\").arg({}).arg({}).query::<_, i64>({}).map(|n| n > 0).map_err(|e| e.to_string())",
            set, member, client
        )
    }

    pub fn generate_smembers_code(client: &str, set: &str) -> String {
        format!(
            "redis::cmd(\"SMEMBERS\").arg({}).query::<_, Vec<String>>({}).map(|v| v.into_iter().collect::<std::collections::HashSet<String>>()).map_err(|e| e.to_string())",
            set, client
        )
    }

    pub fn generate_publish_code(client: &str, channel: &str, message: &str) -> String {
        format!(
            "redis::cmd(\"PUBLISH\").arg({}).arg({}).query::<_, i64>({}).map(|_| ()).map_err(|e| e.to_string())",
            channel, message, client
        )
    }
}
