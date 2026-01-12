// VelinScript Language Server
// LSP Implementation für IDE-Support

use tower_lsp::{LspService, Server};

mod server;
mod handlers;
mod document;

use server::VelinLanguageServer;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    
    let (service, socket) = LspService::new(|client| VelinLanguageServer::new(client));
    Server::new(stdin, stdout, socket).serve().await;
}
