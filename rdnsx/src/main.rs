//! RDNSx CLI - Fast and multi-purpose DNS toolkit

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

mod cli;
mod commands;
mod output_writer;

use cli::Cli;

fn print_banner() {
    println!(r#"
██████╗ ██████╗ ███╗   ██╗███████╗██╗  ██╗
██╔══██╗██╔══██╗████╗  ██║██╔════╝╚██╗██╔╝
██████╔╝██║  ██║██╔██╗ ██║███████╗ ╚███╔╝
██╔══██╗██║  ██║██║╚██╗██║╚════██║ ██╔██╗
██║  ██║██████╔╝██║ ╚████║███████║██╔╝ ██╗
╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝

          Quinas Project by RFS
    Fast and multi-purpose DNS toolkit
"#);
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments first to check for silent mode
    let cli = Cli::parse();

    // Print banner unless silent mode is enabled
    if !cli.silent {
        print_banner();
        println!();
    }

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    cli.run().await
}
