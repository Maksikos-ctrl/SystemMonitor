/// Hlavný modul CLI (Command Line Interface) - obsahuje všetky komponenty
/// pre terminálové používateľské rozhranie (TUI)
pub mod app;       // Hlavná aplikácia a jej stav
pub mod ui;        // Používateľské rozhranie (rendering)
pub mod runner;    // Hlavná slučka aplikácie a spracovanie vstupu

/// Alias pre výsledok operácií CLI
/// Používa sa pre jednotný spôsob spracovania chýb v celom CLI module
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;