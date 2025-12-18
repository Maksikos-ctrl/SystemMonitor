use sqlx::PgPool;           // Pool spojení s PostgreSQL databázou
use std::sync::Arc;         // Atomický reference counter pre bezpečné zdieľanie
use tokio::sync::Mutex;     // Asynchrónny mutex pre vzájomné vylúčenie
use crate::services::api_monitor::ApiSystemMonitor;  // Monitorovací servis

/// Globálny stav aplikácie zdieľaný medzi všetkými API endpointami
/// Tento stav je bezpečný pre konkurentný prístup z viacerých vlákien
#[derive(Clone)]
pub struct AppState {
    /// Pool databázových spojení - zdieľaný medzi všetkými požiadavkami
    pub db_pool: Arc<PgPool>,
    
    /// Monitorovací servis chránený mutexom - umožňuje bezpečný prístup
    /// z viacerých asynchrónnych úloh súčasne
    pub system_monitor: Arc<Mutex<ApiSystemMonitor>>,
}

impl AppState {
    /// Vytvorí novú inštanciu stavu aplikácie
    ///
    /// # Argumenty
    /// * `pool` - Pool databázových spojení
    /// * `monitor` - Inštancia monitorovacieho servisu
    ///
    /// # Návratová hodnota
    /// Nová inštancia `AppState` s obalom pre bezpečné zdieľanie
    pub fn new(pool: PgPool, monitor: ApiSystemMonitor) -> Self {
        Self {
            db_pool: Arc::new(pool),  // Zabalíme pool do Arc pre zdieľanie
            system_monitor: Arc::new(Mutex::new(monitor)),  // Zabalíme monitor do Arc+Mutex
        }
    }
}