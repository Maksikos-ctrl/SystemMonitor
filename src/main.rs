mod api;
mod db;
mod models;
mod services;
mod cli;
mod modes;
mod config;

use clap::Parser;
use config::{Cli, Commands, init_environment};
use modes::{run_tui_mode, run_api_mode, show_interactive_menu};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    init_environment();
    
   
    let cli = Cli::parse();
    
   
    match cli.command {
        Some(Commands::Tui) => {
            run_tui_mode()?; 
            Ok(()) 
        }
        Some(Commands::Api { host, port, save_metrics }) => {
            run_api_mode(host, port, save_metrics).await
        }
        None => {
            show_interactive_menu().await
        }
    }
}