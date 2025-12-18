use crate::api::handlers;  // Obslužné funkcie endpointov
use crate::api::state::AppState;  // Stav aplikácie
use axum::{                // Webový framework
    routing::get,          // GET metóda smerovania
    Router,                // Hlavný router
};

/// Vytvorí a nakonfiguruje router API s všetkými endpointmi
///
/// # Argumenty
/// * `state` - Globálny stav aplikácie, ktorý bude zdieľaný všetkými handlerami
///
/// # Návratová hodnota
/// Nakonfigurovaný `Router` s definovanými cestami a handlerami
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // ========== HEALTH CHECK ==========
        // GET /health - Kontrola dostupnosti služby
        .route("/health", get(handlers::health_check))
        
        // ========== METRIKY ==========
        // GET /api/metrics/current - Aktuálne metriky
        .route("/api/metrics/current", get(handlers::get_current_metrics))
        // GET /api/metrics/latest - N najnovších metrík
        .route("/api/metrics/latest", get(handlers::get_latest_metrics))
        // GET /api/metrics/history - Metriky za časové obdobie
        .route("/api/metrics/history", get(handlers::get_metrics_history))
        
        // ========== SYSTÉMOVÉ INFORMÁCIE ==========
        // GET /api/cpu - Informácie o procesore
        .route("/api/cpu", get(handlers::get_cpu_info))
        // GET /api/memory - Informácie o pamäti
        .route("/api/memory", get(handlers::get_memory_info))
        // GET /api/disk - Informácie o diskoch
        .route("/api/disk", get(handlers::get_disk_info))
        // GET /api/processes/top - Najnáročnejšie procesy
        .route("/api/processes/top", get(handlers::get_top_processes))
        
        // ========== ŠTATISTIKY ==========
        // GET /api/stats - Agregované štatistiky
        .route("/api/stats", get(handlers::get_stats))
        
        // Pripojenie globálneho stavu k routeru
        // Tento stav bude automaticky injektovaný do všetkých handlerov
        .with_state(state)
}