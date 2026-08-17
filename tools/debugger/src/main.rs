// VelinScript Debugger - DAP Server Implementation

mod dap_server;
mod debugger;
mod breakpoints;
mod variables;
mod stack;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "velin-debugger")]
#[command(about = "VelinScript Debugger (DAP Server)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Startet den DAP Server
    Start {
        /// Port für den DAP Server
        #[arg(short, long, default_value = "4711")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { port } => {
            println!("🚀 VelinScript Debugger (DAP Server)");
            println!("📡 Listening on port {}", port);
            
            let server = dap_server::DAPServer::new(port);
            server.run().await?;
        }
    }

    Ok(())
}
