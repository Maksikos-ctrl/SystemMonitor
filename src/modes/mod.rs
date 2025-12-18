// mod.rs

/// Hlavný modul pre rôzne módy aplikácie
/// Organizuje TUI, API a interaktívne menu
pub mod tui;   // Terminal User Interface mód
pub mod api;   // REST API mód
pub mod menu;  // Interaktívne menu

/// Re-export hlavných funkcií pre jednoduchší import
pub use tui::run_tui_mode;            // Export TUI spúšťacej funkcie
pub use api::run_api_mode;            // Export API spúšťacej funkcie
pub use menu::show_interactive_menu;  // Export funkcie na zobrazenie menu