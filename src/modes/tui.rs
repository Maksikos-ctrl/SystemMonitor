// tui.rs

use crate::services::monitor::SystemMonitor;
use crate::cli::runner::run_tui;

/// Hlavná funkcia pre spustenie TUI (Terminal User Interface) módu
/// Inicializuje systémový monitor a spustí TUI rozhranie
pub fn run_tui_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 System Monitor - Starting TUI Mode...");
    println!("───────────────────────────────────────");
    
    // Vytvorenie nového inštancie systémového monitora
    let monitor = SystemMonitor::new();
    
    // Spustenie TUI rozhrania s monitorom
    run_tui(monitor)
}