// api.rs

use crate::api::{create_router, AppState};
use crate::db::connection::create_pool;
use crate::services::api_monitor::ApiSystemMonitor;  // Import API monitora
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Hlavná funkcia pre spustenie REST API módu
/// Inicializuje API server, databázu a spúšťa background ukladanie metrík
pub async fn run_api_mode(host: String, port: u16, save_metrics: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 System Monitor & API - Starting REST API Mode...");
    println!("────────────────────────────────────────────────────");
    
    // Vytvorenie connection pool pre databázu
    let pool = create_pool().await?;
    println!("✅ Connected to PostgreSQL database");
    
    // Vytvorenie API monitora a stavu aplikácie
    let api_monitor = ApiSystemMonitor::new();  // Nový API monitor
    let app_state = AppState::new(pool.clone(), api_monitor);
    
    // Vytvorenie routera (smerovača) pre API
    let app = create_router(app_state);
    
    // Spustenie background ukladania metrík (ak je povolené)
    if save_metrics {
        start_background_saving(pool.clone()).await?;
    } else {
        // Informácia o vypnutom ukladaní
        println!("⚠️  Background metric saving is disabled");
        println!("   Use --save-metrics flag to enable automatic saving to database");
    }
    
    // Konfigurácia adresy a spustenie servera
    let addr = SocketAddr::from((host.parse::<std::net::Ipv4Addr>()?, port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Informácie o spustenom serveri
    println!("🌐 REST API is ready at http://{}", addr);
    println!("📊 Available endpoints:");
    println!("   • GET  /api/metrics     - System metrics");
    println!("   • GET  /api/processes   - Top processes");
    println!("   • GET  /api/health      - Health check");
    println!("   • GET  /api/gpu         - GPU information");
    println!("✅ Server is ready!");
    println!("🛑 Press Ctrl+C to stop the server");
    
    // Spustenie servera
    axum::serve(listener, app).await?;
    Ok(())
}

/// Spustenie background úlohy pre automatické ukladanie metrík
/// Metriky sa ukladajú každých 60 sekúnd do databázy
async fn start_background_saving(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Vytvorenie monitora v Arc a Mutex pre bezpečný viacvláknový prístup
    let monitor_arc = Arc::new(Mutex::new(ApiSystemMonitor::new())); 
    
    // Spustenie asynchrónnej úlohy
    tokio::spawn(async move {
        println!("⚙️  Background metric saving started (60s interval)...");
        
        // Nekonečný cyklus pre pravidelné ukladanie
        loop {
            // Získanie metrík synchronizovaným prístupom
            let (metrics, gpu_info) = {
                let mut monitor = monitor_arc.lock().await;  // Zámok pre bezpečný prístup
                let metrics = monitor.get_metrics_for_db();   // Získanie metrík
                let gpu_info = monitor.get_gpu_info();        // Získanie GPU informácií
                (metrics, gpu_info)
            };
            
            // Uloženie metrík do databázy
            match crate::db::save_metrics(&pool, &metrics, gpu_info.as_ref()).await {
                Ok(id) => println!("💾 [Auto-Save] Metrics saved to DB (ID: {})", id),  // Úspech
                Err(e) => eprintln!("❌ [Auto-Save] Error saving to DB: {}", e),       // Chyba
            }
            
            // Čakanie 60 sekúnd pred ďalším uložením
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });
    
    Ok(())
}