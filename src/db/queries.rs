// queries.rs

use crate::models::{SystemMetrics, GpuInfo};
use chrono::{DateTime, Duration, Utc};
use sqlx::{PgPool, Row, Result};

/// Uloženie systémových metrík do databázy
/// Ukladá kompletnú sadu systémových metrík vrátane GPU informácií
pub async fn save_metrics(pool: &PgPool, metrics: &SystemMetrics, gpu_info: Option<&GpuInfo>) -> Result<i64> {
    let result = sqlx::query!(
        r#"
        INSERT INTO system_metrics 
        (timestamp, cpu_usage, memory_total, memory_used, memory_available, 
         swap_total, swap_used, disk_total, disk_used, disk_available,
         gpu_name, gpu_usage, gpu_memory_total, gpu_memory_used, gpu_temperature,
         network_sent_kbps, network_recv_kbps,
         process_count, system_uptime,
         cpu_temperature, motherboard_temperature, disk_temperature, max_temperature)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
        RETURNING id
        "#,
        metrics.timestamp,              // Časová značka
        metrics.cpu_usage,              // Využitie CPU v %
        metrics.memory_total,           // Celková RAM v bajtoch
        metrics.memory_used,            // Použitá RAM v bajtoch
        metrics.memory_available,       // Dostupné RAM v bajtoch
        metrics.swap_total,             // Celkový swap
        metrics.swap_used,              // Použitý swap
        metrics.disk_total,             // Celková veľkosť disku
        metrics.disk_used,              // Použitý priestor na disku
        metrics.disk_available,         // Dostupné miesto na disku
        gpu_info.map(|g| g.name.clone()),  // Názov GPU
        gpu_info.map(|g| g.usage),      // Využitie GPU v %
        gpu_info.map(|g| g.memory_total as i64),  // Celková GPU pamäť
        gpu_info.map(|g| g.memory_used as i64),   // Použitá GPU pamäť
        gpu_info.and_then(|g| g.temperature),  // Teplota GPU
        metrics.network_sent_kbps,      // Odoslané dáta v KB/s
        metrics.network_recv_kbps,      // Prijaté dáta v KB/s
        metrics.process_count,          // Počet procesov
        metrics.system_uptime,          // Doba behu systému v sekundách
        metrics.cpu_temperature,        // Teplota CPU
        metrics.motherboard_temperature, // Teplota základnej dosky
        metrics.disk_temperature,       // Teplota disku
        metrics.max_temperature        // Maximálna teplota
    )
    .fetch_one(pool)                   // Vykonanie dotazu a získanie jedného riadku
    .await?;                           // Async čakanie na výsledok

    Ok(result.id)                      // Vrátenie ID nového záznamu
}

/// Získanie aktuálnych metrík z databázy
/// Vráti posledný uložený záznam systémových metrík
pub async fn get_current_metrics(pool: &PgPool) -> Result<Option<SystemMetrics>> {
    let row = sqlx::query(
        r#"SELECT id, timestamp, cpu_usage, memory_total, memory_used, 
           memory_available, swap_total, swap_used, disk_total, disk_used, 
           disk_available, 
           gpu_name, gpu_usage, gpu_memory_total, gpu_memory_used, gpu_temperature,
           network_sent_kbps, network_recv_kbps,
           process_count, system_uptime,
           cpu_temperature, motherboard_temperature, disk_temperature, max_temperature
           FROM system_metrics 
           ORDER BY timestamp DESC LIMIT 1"#  // Zoradenie podľa času, najnovší prvý
    )
    .fetch_optional(pool)              // Možný výsledok (môže byť None)
    .await?;

    match row {
        Some(row) => Ok(Some(SystemMetrics {
            id: row.try_get("id")?,                    // ID záznamu
            timestamp: row.try_get("timestamp")?,      // Časová značka
            cpu_usage: row.try_get("cpu_usage")?,      // Využitie CPU
            memory_total: row.try_get("memory_total")?, // Celková pamäť
            memory_used: row.try_get("memory_used")?,  // Použitá pamäť
            memory_available: row.try_get("memory_available")?, // Dostupné pamäť
            swap_total: row.try_get("swap_total")?,    // Celkový swap
            swap_used: row.try_get("swap_used")?,      // Použitý swap
            disk_total: row.try_get("disk_total")?,    // Celkový disk
            disk_used: row.try_get("disk_used")?,      // Použitý disk
            disk_available: row.try_get("disk_available")?, // Dostupné miesto
            gpu_name: row.try_get("gpu_name")?,        // Názov GPU
            gpu_usage: row.try_get("gpu_usage")?,      // Využitie GPU
            gpu_memory_total: row.try_get("gpu_memory_total")?, // GPU pamäť celkovo
            gpu_memory_used: row.try_get("gpu_memory_used")?,   // Použitá GPU pamäť
            gpu_temperature: row.try_get("gpu_temperature")?,   // Teplota GPU
            network_sent_kbps: row.try_get("network_sent_kbps")?, // Odoslané dáta
            network_recv_kbps: row.try_get("network_recv_kbps")?, // Prijaté dáta
            process_count: row.try_get("process_count")?,       // Počet procesov
            system_uptime: row.try_get("system_uptime")?,       // Doba behu systému
            cpu_temperature: row.try_get("cpu_temperature")?,   // Teplota CPU
            motherboard_temperature: row.try_get("motherboard_temperature")?, // Teplota základnej dosky
            disk_temperature: row.try_get("disk_temperature")?, // Teplota disku
            max_temperature: row.try_get("max_temperature")?,   // Maximálna teplota
        })),
        None => Ok(None),  // Ak neexistujú žiadne záznamy
    }
}

/// Získanie posledných N metrík z databázy
/// Používa sa pre históriu alebo pre zobrazenie posledných meraní
pub async fn get_latest_metrics(pool: &PgPool, limit: i64) -> Result<Vec<SystemMetrics>> {
    let rows = sqlx::query(
        r#"SELECT id, timestamp, cpu_usage, memory_total, memory_used, 
           memory_available, swap_total, swap_used, disk_total, disk_used, 
           disk_available,
           gpu_name, gpu_usage, gpu_memory_total, gpu_memory_used, gpu_temperature,
           network_sent_kbps, network_recv_kbps,
           process_count, system_uptime,
           cpu_temperature, motherboard_temperature, disk_temperature, max_temperature
           FROM system_metrics 
           ORDER BY timestamp DESC LIMIT $1"#  // Limit počtu záznamov
    )
    .bind(limit)                           // Parameter pre limit
    .fetch_all(pool)                       // Získanie všetkých riadkov
    .await?;

    let mut metrics = Vec::new();          // Vytvorenie vektora pre výsledky
    for row in rows {
        metrics.push(SystemMetrics {       // Konverzia každého riadku na SystemMetrics
            id: row.try_get("id")?,
            timestamp: row.try_get("timestamp")?,
            cpu_usage: row.try_get("cpu_usage")?,
            memory_total: row.try_get("memory_total")?,
            memory_used: row.try_get("memory_used")?,
            memory_available: row.try_get("memory_available")?,
            swap_total: row.try_get("swap_total")?,
            swap_used: row.try_get("swap_used")?,
            disk_total: row.try_get("disk_total")?,
            disk_used: row.try_get("disk_used")?,
            disk_available: row.try_get("disk_available")?,
            gpu_name: row.try_get("gpu_name")?,
            gpu_usage: row.try_get("gpu_usage")?,
            gpu_memory_total: row.try_get("gpu_memory_total")?,
            gpu_memory_used: row.try_get("gpu_memory_used")?,
            gpu_temperature: row.try_get("gpu_temperature")?,
            network_sent_kbps: row.try_get("network_sent_kbps")?,
            network_recv_kbps: row.try_get("network_recv_kbps")?,
            process_count: row.try_get("process_count")?,
            system_uptime: row.try_get("system_uptime")?,
            cpu_temperature: row.try_get("cpu_temperature")?,
            motherboard_temperature: row.try_get("motherboard_temperature")?,
            disk_temperature: row.try_get("disk_temperature")?,
            max_temperature: row.try_get("max_temperature")?,
        });
    }

    Ok(metrics)  // Vrátenie vektora metrík
}

/// Získanie metrík od určitého času
/// Používa sa pre získanie historických dát za posledných N hodín
pub async fn get_metrics_since(pool: &PgPool, hours: i64) -> Result<Vec<SystemMetrics>> {
    let since = Utc::now() - Duration::hours(hours);  // Výpočet časového limitu
    
    let rows = sqlx::query(
        r#"SELECT id, timestamp, cpu_usage, memory_total, memory_used, 
           memory_available, swap_total, swap_used, disk_total, disk_used, 
           disk_available,
           gpu_name, gpu_usage, gpu_memory_total, gpu_memory_used, gpu_temperature,
           network_sent_kbps, network_recv_kbps,
           process_count, system_uptime,
           cpu_temperature, motherboard_temperature, disk_temperature, max_temperature
           FROM system_metrics 
           WHERE timestamp > $1 
           ORDER BY timestamp ASC"#  // Chronologické zoradenie
    )
    .bind(since)                     // Parameter pre časový limit
    .fetch_all(pool)
    .await?;

    let mut metrics = Vec::new();
    for row in rows {
        metrics.push(SystemMetrics {
            id: row.try_get("id")?,
            timestamp: row.try_get("timestamp")?,
            cpu_usage: row.try_get("cpu_usage")?,
            memory_total: row.try_get("memory_total")?,
            memory_used: row.try_get("memory_used")?,
            memory_available: row.try_get("memory_available")?,
            swap_total: row.try_get("swap_total")?,
            swap_used: row.try_get("swap_used")?,
            disk_total: row.try_get("disk_total")?,
            disk_used: row.try_get("disk_used")?,
            disk_available: row.try_get("disk_available")?,
            gpu_name: row.try_get("gpu_name")?,
            gpu_usage: row.try_get("gpu_usage")?,
            gpu_memory_total: row.try_get("gpu_memory_total")?,
            gpu_memory_used: row.try_get("gpu_memory_used")?,
            gpu_temperature: row.try_get("gpu_temperature")?,
            network_sent_kbps: row.try_get("network_sent_kbps")?,
            network_recv_kbps: row.try_get("network_recv_kbps")?,
            process_count: row.try_get("process_count")?,
            system_uptime: row.try_get("system_uptime")?,
            cpu_temperature: row.try_get("cpu_temperature")?,
            motherboard_temperature: row.try_get("motherboard_temperature")?,
            disk_temperature: row.try_get("disk_temperature")?,
            max_temperature: row.try_get("max_temperature")?,
        });
    }

    Ok(metrics)
}

/// Výpočet priemerného využitia CPU za posledných N hodín
/// Používa sa pre dlhodobé štatistiky a analýzy
pub async fn get_average_cpu(pool: &PgPool, hours: i64) -> Result<f64> {
    let since = Utc::now() - Duration::hours(hours);
    
    let result = sqlx::query!(
        "SELECT AVG(cpu_usage) as avg_cpu FROM system_metrics WHERE timestamp > $1",
        since
    )
    .fetch_one(pool)
    .await?;

    Ok(result.avg_cpu.unwrap_or(0.0))  // Vrátenie priemeru alebo 0.0 ak žiadne dáta
}

/// Spočítanie celkového počtu metrík v databáze
/// Používa sa pre monitorovanie veľkosti databázy
pub async fn count_metrics(pool: &PgPool) -> Result<i64> {
    let row = sqlx::query!("SELECT COUNT(*) as count FROM system_metrics")
        .fetch_one(pool)
        .await?;
    Ok(row.count.unwrap_or(0))  // Vrátenie počtu alebo 0
}

/// Vyčistenie starých metrík z databázy
/// Odstraňuje záznamy staršie ako N dní (archivácia)
pub async fn cleanup_old_metrics(pool: &PgPool, days: i64) -> Result<u64> {
    let cutoff = Utc::now() - Duration::days(days);  // Výpočet časového limitu
    
    let result = sqlx::query!(
        "DELETE FROM system_metrics WHERE timestamp < $1",
        cutoff
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())  // Vrátenie počtu odstránených záznamov
}