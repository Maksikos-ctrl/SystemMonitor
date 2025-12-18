// mod.rs

/// Databázový modul - obsahuje funkcionalitu pre prácu s databázou
pub mod connection;  // Modul pre pripojenie k databáze
pub mod queries;     // Modul pre databázové dotazy

/// Export dôležitých funkcií pre jednoduchší import
pub use connection::create_pool;  // Export funkcie na vytvorenie connection pool
pub use queries::{                // Export všetkých dotazových funkcií
    save_metrics,           // Uloženie metrík
    get_current_metrics,    // Získanie aktuálnych metrík
    get_latest_metrics,     // Získanie posledných metrík
    get_metrics_since,      // Získanie metrík od určitého času
    get_average_cpu,        // Výpočet priemerného CPU
    count_metrics,          // Spočítanie metrík
    cleanup_old_metrics,    // Vyčistenie starých metrík
};