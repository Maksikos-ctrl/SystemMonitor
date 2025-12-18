use crate::api::state::AppState;  // Stav aplikácie
use crate::db;                    // Databázové funkcie
use axum::{                       // Webový framework
    extract::{Query, State},      // Extrakcia parametrov z požiadaviek
    http::StatusCode,             // HTTP status kódy
    Json,                         // JSON serializácia
};
use serde::{Deserialize, Serialize};  // Serializácia/deserializácia
use serde_json::{json, Value};        // Práca s JSON hodnotami

/// Query parameter pre obmedzenie počtu výsledkov
/// Používa sa napr. v `/api/metrics/latest?limit=10`
#[derive(Debug, Deserialize)]
pub struct LimitQuery {
    #[serde(default = "default_limit")]  // Predvolená hodnota 10 ak nie je zadané
    pub limit: i64,
}

/// Predvolená hodnota pre limit výsledkov
fn default_limit() -> i64 {
    10
}

/// Query parameter pre časový rozsah v hodinách
/// Používa sa napr. v `/api/metrics/history?hours=24`
#[derive(Debug, Deserialize)]
pub struct HoursQuery {
    #[serde(default = "default_hours")]  // Predvolená hodnota 24 hodín
    pub hours: i64,
}

/// Predvolená hodnota pre časový rozsah
fn default_hours() -> i64 {
    24
}

// ==================== HANDLERE PRE METRIKY ====================

/// GET /api/metrics/current
/// Vráti aktuálne metriky systému (posledne uložené v databáze)
///
/// # Parametre
/// - `state`: Globálny stav aplikácie
///
/// # Návratová hodnota
/// - `Ok(Json)`: JSON s aktuálnymi metrikami
/// - `Err(StatusCode)`: 500 ak nastane chyba
pub async fn get_current_metrics(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    // Načítanie aktuálnych metrík z databázy
    let metrics = db::get_current_metrics(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  // Konvertovanie chyby na 500

    match metrics {
        Some(m) => Ok(Json(json!({
            "success": true,
            "data": m
        }))),
        None => Ok(Json(json!({
            "success": false,
            "message": "No metrics available yet"  // Žiadne metriky ešte nie sú dostupné
        }))),
    }
}

/// GET /api/metrics/latest?limit=10
/// Vráti X najnovších metrík (podľa parametra limit)
///
/// # Parametre
/// - `state`: Globálny stav aplikácie
/// - `params`: Query parametre (limit)
///
/// # Návratová hodnota
/// - `Ok(Json)`: JSON so zoznamom metrík
pub async fn get_latest_metrics(
    State(state): State<AppState>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Value>, StatusCode> {
    // Načítanie N najnovších metrík z databázy
    let metrics = db::get_latest_metrics(&state.db_pool, params.limit)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": true,
        "count": metrics.len(),  // Skutočný počet vrátených záznamov
        "data": metrics
    })))
}

/// GET /api/metrics/history?hours=24
/// Vráti metriky za posledných X hodín
///
/// # Parametre
/// - `state`: Globálny stav aplikácie
/// - `params`: Query parametre (hours)
///
/// # Návratová hodnota
/// - `Ok(Json)`: JSON s históriou metrík
pub async fn get_metrics_history(
    State(state): State<AppState>,
    Query(params): Query<HoursQuery>,
) -> Result<Json<Value>, StatusCode> {
    // Načítanie metrík za posledných N hodín
    let metrics = db::get_metrics_since(&state.db_pool, params.hours)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": true,
        "count": metrics.len(),
        "hours": params.hours,  // Vrátime späť počet požadovaných hodín
        "data": metrics
    })))
}

// ==================== HANDLERE PRE SYSTÉMOVÉ INFORMÁCIE ====================

/// GET /api/cpu
/// Vráti informácie o procesore
///
/// # Poznámka
/// Tento endpoint momentálne volá `get_gpu_info()` - pravdepodobne chyba v implementácii
/// Malo by volať `get_cpu_info()` alebo podobnú metódu
pub async fn get_cpu_info(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    // Získanie zámku na monitor (aby sa predišlo súbežným prístupom)
    let mut monitor = state.system_monitor.lock().await;
    let cpu_info = monitor.get_gpu_info();  // TODO: Opraviť na get_cpu_info()

    Ok(Json(json!({
        "success": true,
        "cpu_count": cpu_info.as_ref().map(|c| c.name.len()).unwrap_or(0),  // Počet CPU jadier
        "data": cpu_info
    })))
}

/// GET /api/memory
/// Vráti informácie o pamäti
///
/// # Poznámka
/// Tento endpoint tiež volá `get_gpu_info()` - potrebuje opraviť
pub async fn get_memory_info(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let mut monitor = state.system_monitor.lock().await;
    let memory_info = monitor.get_gpu_info();  // TODO: Opraviť na get_memory_info()

    Ok(Json(json!({
        "success": true,
        "data": memory_info
    })))
}

/// GET /api/disk
/// Vráti informácie o diskoch
///
/// # Poznámka
/// Tento endpoint tiež volá `get_gpu_info()` - potrebuje opraviť
pub async fn get_disk_info(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let mut monitor = state.system_monitor.lock().await;
    let disk_info = monitor.get_gpu_info();  // TODO: Opraviť na get_disk_info()

    Ok(Json(json!({
        "success": true,
        "count": disk_info.as_ref().map(|d| d.name.len()).unwrap_or(0),  // Počet diskov
        "data": disk_info
    })))
}

/// GET /api/processes/top?limit=10
/// Vráti X najnáročnejších procesov podľa využitia zdrojov
///
/// # Parametre
/// - `state`: Globálny stav aplikácie
/// - `params`: Query parametre (limit)
pub async fn get_top_processes(
    State(state): State<AppState>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Value>, StatusCode> {
    let mut monitor = state.system_monitor.lock().await;
    let processes = monitor.get_top_processes(params.limit as usize);  // Konverzia na usize

    Ok(Json(json!({
        "success": true,
        "count": processes.len(),
        "data": processes
    })))
}

// ==================== HANDLERE PRE ŠTATISTIKY ====================

/// GET /api/stats
/// Vráti agregované štatistiky o metrikách
///
/// # Vrátené štatistiky
/// - `total_metrics`: Celkový počet uložených metrík
/// - `average_cpu_1h`: Priemerné využitie CPU za poslednú hodinu
/// - `average_cpu_24h`: Priemerné využitie CPU za posledných 24 hodín
pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    // Asynchrónne načítanie viacerých štatistík súčasne
    let avg_cpu_1h = db::get_average_cpu(&state.db_pool, 1).await.unwrap_or(0.0);
    let avg_cpu_24h = db::get_average_cpu(&state.db_pool, 24).await.unwrap_or(0.0);
    let total_metrics = db::count_metrics(&state.db_pool).await.unwrap_or(0);

    Ok(Json(json!({
        "success": true,
        "stats": {
            "total_metrics": total_metrics,
            "average_cpu_1h": avg_cpu_1h,
            "average_cpu_24h": avg_cpu_24h
        }
    })))
}

// ==================== HEALTH CHECK ====================

/// GET /health
/// Health check endpoint pre monitorovanie stavu služby
/// Používa sa napr. kubernetes, docker swarm, load balancermi
///
/// # Návratová hodnota
/// Vždy vráti `200 OK` so základnými informáciami o službe
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "system-monitor",
        "timestamp": chrono::Utc::now().to_rfc3339()  // Časová pečiatka odpovede
    }))
}