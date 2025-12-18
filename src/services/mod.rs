// mod.rs

/// Hlavný modul služieb - obsahuje všetky monitorovacie služby
pub mod api_monitor;      // API monitor pre REST API server
pub mod monitor;          // Hlavný systémový monitor pre TUI
pub mod temperatures;     // Monitor teplôt komponentov

/// Re-export hlavných štruktúr pre jednoduchší import
pub use api_monitor::ApiSystemMonitor;  // API monitor
pub use monitor::SystemMonitor;         // Hlavný monitor
pub use temperatures::TemperatureMonitor; // Monitor teplôt