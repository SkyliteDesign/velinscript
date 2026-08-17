// Call Stack Management

use serde_json::{Value, Map};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub id: usize,
    pub name: String,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

pub struct CallStack {
    frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        CallStack {
            frames: Vec::new(),
        }
    }

    pub fn get_frames(&self) -> Vec<Value> {
        self.frames.iter().map(|frame| {
            let mut frame_obj = Map::new();
            frame_obj.insert("id".to_string(), Value::Number(frame.id.into()));
            frame_obj.insert("name".to_string(), Value::String(frame.name.clone()));
            frame_obj.insert("source".to_string(), Value::Object({
                let mut source = Map::new();
                source.insert("path".to_string(), Value::String(frame.file.to_string_lossy().to_string()));
                source
            }));
            frame_obj.insert("line".to_string(), Value::Number(frame.line.into()));
            frame_obj.insert("column".to_string(), Value::Number(frame.column.into()));
            Value::Object(frame_obj)
        }).collect()
    }

    pub fn push_frame(&mut self, name: String, file: PathBuf, line: usize, column: usize) {
        let id = self.frames.len();
        self.frames.push(StackFrame {
            id,
            name,
            file,
            line,
            column,
        });
    }

    pub fn pop_frame(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }
}

impl Default for CallStack {
    fn default() -> Self {
        Self::new()
    }
}
