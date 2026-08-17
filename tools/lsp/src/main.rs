//! Thin binary wrapper

use tower_lsp::{LspService, Server};
use velin_lsp::server::VelinLanguageServer;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    
    let (service, socket) = LspService::new(|client| VelinLanguageServer::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
