// DAP (Debug Adapter Protocol) Server Implementation

use crate::debugger::Debugger;
use crate::breakpoints::BreakpointManager;
use crate::variables::VariableInspector;
use crate::stack::CallStack;
use serde_json::{Value, Map};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

pub struct DAPServer {
    port: u16,
    debugger: Arc<Mutex<Debugger>>,
    breakpoints: Arc<Mutex<BreakpointManager>>,
    variables: Arc<Mutex<VariableInspector>>,
    call_stack: Arc<Mutex<CallStack>>,
}

impl DAPServer {
    pub fn new(port: u16) -> Self {
        DAPServer {
            port,
            debugger: Arc::new(Mutex::new(Debugger::new())),
            breakpoints: Arc::new(Mutex::new(BreakpointManager::new())),
            variables: Arc::new(Mutex::new(VariableInspector::new())),
            call_stack: Arc::new(Mutex::new(CallStack::new())),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        println!("✓ DAP Server listening on port {}", self.port);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("📡 New connection from {}", addr);
                    let debugger = Arc::clone(&self.debugger);
                    let breakpoints = Arc::clone(&self.breakpoints);
                    let variables = Arc::clone(&self.variables);
                    let call_stack = Arc::clone(&self.call_stack);
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_client(stream, debugger, breakpoints, variables, call_stack).await {
                            eprintln!("Error handling client: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    async fn handle_client(
        mut stream: TcpStream,
        debugger: Arc<Mutex<Debugger>>,
        breakpoints: Arc<Mutex<BreakpointManager>>,
        variables: Arc<Mutex<VariableInspector>>,
        call_stack: Arc<Mutex<CallStack>>,
    ) -> Result<(), Box<dyn std::error::Error + Send>> {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
        let (reader, mut writer) = stream.split();
        let mut buf_reader = tokio::io::BufReader::new(reader);
        let mut line_buf = String::new();

        loop {
            line_buf.clear();
            
            // Read headers line by line
            let mut content_length = 0;
            loop {
                let bytes_read = buf_reader.read_line(&mut line_buf).await?;
                if bytes_read == 0 {
                    return Ok(()); // Connection closed
                }
                
                let line = line_buf.trim();
                if line.is_empty() {
                    break; // End of headers
                }
                
                if line.starts_with("Content-Length:") {
                    if let Some(len_str) = line.split(':').nth(1) {
                        content_length = len_str.trim().parse().unwrap_or(0);
                    }
                }
                
                line_buf.clear();
            }

            if content_length == 0 {
                continue;
            }

            // Read message body
            let mut body = vec![0u8; content_length];
            buf_reader.read_exact(&mut body).await?;
            let message_str = String::from_utf8(body)?;

            // Parse JSON message
            let message: Value = serde_json::from_str(&message_str)?;

            // Handle DAP request
            let response = Self::handle_request(&message, &debugger, &breakpoints, &variables, &call_stack).await?;

            // Send response
            let response_json = serde_json::to_string(&response)?;
            let response_header = format!("Content-Length: {}\r\n\r\n", response_json.len());
            writer.write_all(response_header.as_bytes()).await?;
            writer.write_all(response_json.as_bytes()).await?;
            writer.flush().await?;
        }
    }

    async fn handle_request(
        request: &Value,
        _debugger: &Arc<Mutex<Debugger>>,
        breakpoints: &Arc<Mutex<BreakpointManager>>,
        variables: &Arc<Mutex<VariableInspector>>,
        call_stack: &Arc<Mutex<CallStack>>,
    ) -> Result<Value, Box<dyn std::error::Error + Send>> {
        let method = request.get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let request_seq = request.get("seq").and_then(|v| v.as_u64()).unwrap_or(0);

        let mut response = Map::new();
        response.insert("type".to_string(), Value::String("response".to_string()));
        response.insert("request_seq".to_string(), Value::Number(request_seq.into()));
        response.insert("success".to_string(), Value::Bool(true));
        response.insert("command".to_string(), Value::String(method.to_string()));

        match method {
            "initialize" => {
                let body = Map::new();
                response.insert("body".to_string(), Value::Object(body));
            }
            "setBreakpoints" => {
                if let Some(args) = request.get("arguments") {
                    let mut breakpoints_manager = breakpoints.lock().unwrap();
                    let result = breakpoints_manager.set_breakpoints(args);
                    response.insert("body".to_string(), result);
                }
            }
            "threads" => {
                let mut body = Map::new();
                body.insert("threads".to_string(), Value::Array(vec![
                    Value::Object({
                        let mut thread = Map::new();
                        thread.insert("id".to_string(), Value::Number(1.into()));
                        thread.insert("name".to_string(), Value::String("main".to_string()));
                        thread
                    })
                ]));
                response.insert("body".to_string(), Value::Object(body));
            }
            "stackTrace" => {
                let call_stack_manager = call_stack.lock().unwrap();
                let stack_frames = call_stack_manager.get_frames();
                let mut body = Map::new();
                body.insert("stackFrames".to_string(), Value::Array(stack_frames));
                response.insert("body".to_string(), Value::Object(body));
            }
            "variables" => {
                if let Some(args) = request.get("arguments") {
                    let variables_manager = variables.lock().unwrap();
                    let vars = variables_manager.get_variables(args);
                    let mut body = Map::new();
                    body.insert("variables".to_string(), vars);
                    response.insert("body".to_string(), Value::Object(body));
                }
            }
            "continue" => {
                let mut body = Map::new();
                body.insert("allThreadsContinued".to_string(), Value::Bool(true));
                response.insert("body".to_string(), Value::Object(body));
            }
            "next" => {
                let body = Map::new();
                response.insert("body".to_string(), Value::Object(body));
            }
            "stepIn" => {
                let body = Map::new();
                response.insert("body".to_string(), Value::Object(body));
            }
            "stepOut" => {
                let body = Map::new();
                response.insert("body".to_string(), Value::Object(body));
            }
            "pause" => {
                let body = Map::new();
                response.insert("body".to_string(), Value::Object(body));
            }
            "evaluate" => {
                if let Some(args) = request.get("arguments") {
                    let expression = args.get("expression")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    
                    // Use args to get additional parameters if needed
                    let _context = args.get("context").and_then(|v| v.as_str()).unwrap_or("watch");
                    
                    let mut body = Map::new();
                    body.insert("result".to_string(), Value::String(format!("Evaluated: {}", expression)));
                    body.insert("variablesReference".to_string(), Value::Number(0.into()));
                    response.insert("body".to_string(), Value::Object(body));
                }
            }
            _ => {
                response.insert("success".to_string(), Value::Bool(false));
                response.insert("message".to_string(), Value::String(format!("Unknown command: {}", method)));
            }
        }

        Ok(Value::Object(response))
    }
}
