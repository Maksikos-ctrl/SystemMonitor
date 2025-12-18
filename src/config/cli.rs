// cli.rs

use clap::{Parser, Subcommand};  // Import knižnice pre CLI parsovanie

/// Hlavná CLI štruktúra aplikácie
/// Definuje základné nastavenia a príkazy
#[derive(Parser)]  // Automatická derivácia CLI parsera
#[command(name = "system-monitor")]        // Názov aplikácie
#[command(about = "🖥️ System Monitor - TUI and REST API")]  // Popis aplikácie
#[command(version = "1.0")]               // Verzia aplikácie
pub struct Cli {
    /// Podpríkazy aplikácie
    #[command(subcommand)]
    pub command: Option<Commands>,  // Možné príkazy (optional)
}

/// Enum definujúci dostupné príkazy aplikácie
#[derive(Subcommand)]  // Automatická derivácia podpríkazov
pub enum Commands {
    /// Spustenie TUI (Terminal User Interface) módu
    /// Grafické rozhranie v termináli
    Tui,
    
    /// Spustenie REST API módu
    /// Webové rozhranie pre vzdialený prístup
    Api {
        /// Hostname pre API server (štandardne localhost)
        #[arg(short = 'H', long, default_value = "127.0.0.1")]  // Skratka -H alebo --host
        host: String,
        
        /// Port pre API server (štandardne 3000)
        #[arg(short, long, default_value = "3000")]  // Skratka -p alebo --port
        port: u16,
      
        /// Prepínač pre ukladanie metrík do databázy
        #[arg(short, long)]  // Skratka -s alebo --save-metrics
        save_metrics: bool,  // Boolean hodnota - true/false
    },
}