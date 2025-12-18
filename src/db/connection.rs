// connection.rs

use sqlx::{postgres::PgPoolOptions, PgPool, Result};

/// Vytvorenie a inicializácia PostgreSQL connection pool
/// Spravuje pripojenia k databáze a vytvára potrebné tabuľky
pub async fn create_pool() -> Result<PgPool> {
    // Získanie databázového URL z premenných prostredia
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/system_monitor".to_string());
    
    println!("🔌 Connecting to: {}", database_url);
    
    // Vytvorenie connection pool s obmedzením počtu pripojení
    let pool = PgPoolOptions::new()
        .max_connections(5)                     // Maximálne 5 súbežných pripojení
        .connect(&database_url)                 // Pripojenie k databáze
        .await?;

    // Vytvorenie tabuľky pre systémové metriky (ak neexistuje)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS system_metrics (
            id BIGSERIAL PRIMARY KEY,                    // Primárny kľúč s auto increment
            timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(), // Časová značka s časovou zónou
            cpu_usage DOUBLE PRECISION NOT NULL,         // Využitie CPU v percentách
            memory_total BIGINT NOT NULL,                // Celková RAM v bajtoch
            memory_used BIGINT NOT NULL,                 // Použitá RAM v bajtoch
            memory_available BIGINT NOT NULL,            // Dostupné RAM v bajtoch
            swap_total BIGINT NOT NULL,                  // Celkový swap
            swap_used BIGINT NOT NULL,                   // Použitý swap
            disk_total BIGINT NOT NULL,                  // Celková veľkosť disku
            disk_used BIGINT NOT NULL,                   // Použitý priestor na disku
            disk_available BIGINT NOT NULL,              // Dostupné miesto na disku
            
            -- GPU metriky (voliteľné)
            gpu_name TEXT,                               // Názov GPU
            gpu_usage DOUBLE PRECISION,                  // Využitie GPU v %
            gpu_memory_total BIGINT,                     // Celková GPU pamäť
            gpu_memory_used BIGINT,                      // Použitá GPU pamäť
            gpu_temperature DOUBLE PRECISION,            // Teplota GPU
            
            -- Sieťová štatistika
            network_sent_kbps DOUBLE PRECISION,          // Odoslané dáta v KB/s
            network_recv_kbps DOUBLE PRECISION,          // Prijaté dáta v KB/s
            
            -- Všeobecné informácie
            process_count INTEGER NOT NULL,              // Počet procesov
            system_uptime BIGINT NOT NULL                // Doba behu systému v sekundách
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Vytvorenie indexov pre rýchlejší prístup k dátam
    // Index pre rýchle zoradenie podľa času
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON system_metrics(timestamp DESC)"
    )
    .execute(&pool)
    .await?;
    
    // Index pre rýchle vyhľadávanie podľa GPU
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_metrics_gpu ON system_metrics(gpu_name, timestamp DESC)"
    )
    .execute(&pool)
    .await?;

    println!("✅ PostgreSQL database connected and initialized with GPU support!");
    Ok(pool)  // Vrátenie connection pool
}