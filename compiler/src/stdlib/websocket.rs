
pub struct WebSocketStdlib;

impl WebSocketStdlib {
    pub fn generate_connect_code(url: &str) -> String {
        // Requires tokio-tungstenite and futures
        format!(
            "{{
                use tokio_tungstenite::connect_async;
                use futures_util::StreamExt;
                connect_async({}).await.map(|(ws_stream, _)| ws_stream).map_err(|e| e.to_string())
            }}",
            url
        )
    }

    pub fn generate_send_code(ws: &str, message: &str) -> String {
        format!(
            "{{
                use futures_util::SinkExt;
                use tokio_tungstenite::tungstenite::Message;
                {}.send(Message::Text({}.to_string())).await.map_err(|e| e.to_string())
            }}",
            ws, message
        )
    }

    pub fn generate_receive_code(ws: &str) -> String {
        format!(
            "{{
                use futures_util::StreamExt;
                match {}.next().await {{
                    Some(Ok(msg)) => Ok(msg.to_string()),
                    Some(Err(e)) => Err(e.to_string()),
                    None => Err(\"Connection closed\".to_string()),
                }}
            }}",
            ws
        )
    }

    pub fn generate_close_code(ws: &str) -> String {
        format!(
            "{{
                use futures_util::SinkExt;
                {}.close(None).await.map_err(|e| e.to_string())
            }}",
            ws
        )
    }

    pub fn generate_is_connected_code(ws: &str) -> String {
        // tokio-tungstenite stream doesn't expose is_connected easily without polling
        "true".to_string() 
    }

    pub fn generate_ping_code(ws: &str) -> String {
         format!(
            "{{
                use futures_util::SinkExt;
                use tokio_tungstenite::tungstenite::Message;
                {}.send(Message::Ping(vec![])).await.map_err(|e| e.to_string())
            }}",
            ws
        )
    }

    pub fn generate_subscribe_code(ws: &str, topic: &str) -> String {
        // Application level subscription, just send a message
        Self::generate_send_code(ws, &format!("\"subscribe: \" + {}", topic))
    }

    pub fn generate_on_message_code(ws: &str, callback: &str) -> String {
        // This would require an event loop or spawning a task.
        // For generated code, we might return a stream handler or similar.
        // For now, mock or simple implementation
        format!("Ok(())")
    }
}
