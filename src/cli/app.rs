/// Moduly pre správu stavu aplikácie a systémových informácií
mod app_state;          // Stav aplikácie a hlavné dátové štruktúry
mod app_system_info;    // Získavanie a reprezentácia systémových informácií

/// Reexporty pre jednoduchší prístup z iných modulov
// Hlavné typy z modulu stavu aplikácie
pub use app_state::{TuiApp, Mode, NetworkConnection, HISTORY_SIZE};
// Systémové informácie
pub use app_system_info::{SystemInfo, get_system_info};
// Reexporty typov z models modulu pre konzistentný prístup
pub use crate::models::{GpuInfo, ProcessInfo};