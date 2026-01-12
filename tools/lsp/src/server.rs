// LSP Server Implementation

use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use crate::document::DocumentCache;
use crate::handlers::{get_completions, get_hover, find_definition, format_document};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct VelinLanguageServer {
    client: Client,
    documents: Arc<RwLock<DocumentCache>>,
}

impl VelinLanguageServer {
    pub fn new(client: Client) -> Self {
        VelinLanguageServer {
            client,
            documents: Arc::new(RwLock::new(DocumentCache::new())),
        }
    }
    
    fn extract_word_at_position(text: &str, line: usize, character: usize) -> String {
        let lines: Vec<&str> = text.split('\n').collect();
        if line >= lines.len() {
            return String::new();
        }
        
        let line_text = lines[line];
        let chars: Vec<char> = line_text.chars().collect();
        
        if character >= chars.len() {
            return String::new();
        }
        
        // Find word boundaries
        let mut start = character;
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }
        
        let mut end = character;
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }
        
        if start < end {
            chars[start..end].iter().collect()
        } else {
            String::new()
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for VelinLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "velin-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), "@".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "VelinScript Language Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;
        
        let mut documents = self.documents.write().await;
        documents.update(uri.clone(), text);
        
        self.client
            .log_message(MessageType::INFO, format!("Document opened: {}", uri))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        
        // Get the changed text
        if let Some(change) = params.content_changes.first() {
            let text = change.text.clone();
            
            let mut documents = self.documents.write().await;
            documents.update(uri.clone(), text);
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;
        
        let documents = self.documents.read().await;
        if let Some(doc) = documents.get(&uri) {
            if let Some(program) = &doc.program {
                let completions = get_completions(program, position);
                return Ok(Some(CompletionResponse::Array(completions)));
            }
        }
        
        Ok(None)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let position = params.text_document_position_params.position;
        
        let documents = self.documents.read().await;
        if let Some(doc) = documents.get(&uri) {
            let word = Self::extract_word_at_position(
                &doc.text,
                position.line as usize,
                position.character as usize,
            );
            
            if !word.is_empty() {
                if let Some(program) = &doc.program {
                    return Ok(get_hover(program, position, &word));
                }
            }
        }
        
        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let position = params.text_document_position_params.position;
        
        let documents = self.documents.read().await;
        if let Some(doc) = documents.get(&uri) {
            let word = Self::extract_word_at_position(
                &doc.text,
                position.line as usize,
                position.character as usize,
            );
            
            if !word.is_empty() {
                if let Some(program) = &doc.program {
                    if let Some(location) = find_definition(program, &word, &doc.text) {
                        // Update URI with actual document URI
                        let location = Location {
                            uri: uri.clone(),
                            range: location.range,
                        };
                        return Ok(Some(GotoDefinitionResponse::Scalar(location)));
                    }
                }
            }
        }
        
        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri.to_string();
        
        let documents = self.documents.read().await;
        if let Some(doc) = documents.get(&uri) {
            if let Some(edits) = format_document(&doc.text) {
                return Ok(Some(edits));
            }
        }
        
        Ok(None)
    }
}
