/// Hlavný modul API - obsahuje všetky komponenty REST API
pub mod state;      // Štruktúry pre správu stavu aplikácie
pub mod routes;     // Definície API endpointov
pub mod handlers;   // Obsluha HTTP požiadaviek

/// Reexporty pre jednoduchší prístup z iných modulov
pub use state::AppState;
pub use routes::create_router;