/// Moduly pre jednotlivé obrazovky používateľského rozhrania
pub mod ui_widgets;    // Spoločné widgety a komponenty
pub mod ui_overview;   // Hlavná obrazovka s prehľadom systému
pub mod ui_process;    // Zobrazenie procesov a ich detailov
pub mod ui_network;    // Sieťová aktivita a spojenia
pub mod ui_help;       // Obrazovka s pomocníkom a klávesovými skratkami

// Importy pre rendering
use ratatui::Frame;
use crate::cli::app::{TuiApp, Mode};

/// Hlavná renderovacia funkcia - smeruje rendering podľa aktuálneho režimu
///
/// # Argumenty
/// * `f` - Frame pre rendering
/// * `app` - Hlavná aplikácia so stavom
///
/// # Funkcionalita
/// * Analýza aktuálneho režimu aplikácie
/// * Volanie príslušného renderovacieho modulu
/// * Zabezpečuje jednotný renderingový pipeline pre celú aplikáciu
pub fn render(f: &mut Frame, app: &mut TuiApp) {
    match app.mode {
        // Prehľadový režim - základná obrazovka
        Mode::Overview => ui_overview::render(f, app),
        // Detailný režim procesu
        Mode::ProcessDetail => ui_process::render(f, app),
        // Sieťový režim
        Mode::NetworkView => ui_network::render(f, app),
        // Režim pomocníka
        Mode::Help => ui_help::render(f, app),
    }
}