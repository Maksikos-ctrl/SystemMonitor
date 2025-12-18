// metrics.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Hlavná štruktúra pre systémové metriky
/// Obsahuje všetky kľúčové metriky systému vrátane teplôt
/// Serializácia a deserializácia pre JSON a SQL podporu
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SystemMetrics {
    pub id: Option<i64>,                    // Databázové ID (voliteľné pre nové záznamy)
    pub timestamp: DateTime<Utc>,           // Časová značka merania
    pub cpu_usage: f64,                     // Využitie CPU v percentách
    pub memory_total: i64,                  // Celková RAM v bajtoch
    pub memory_used: i64,                   // Použitá RAM v bajtoch
    pub memory_available: i64,              // Dostupné RAM v bajtoch
    pub swap_total: i64,                    // Celkový swap priestor
    pub swap_used: i64,                     // Použitý swap priestor
    pub disk_total: i64,                    // Celková veľkosť disku
    pub disk_used: i64,                     // Použitý priestor na disku
    pub disk_available: i64,                // Dostupné miesto na disku
    
    // GPU metriky (voliteľné)
    pub gpu_name: Option<String>,           // Názov GPU zariadenia
    pub gpu_usage: Option<f64>,             // Využitie GPU v percentách
    pub gpu_memory_total: Option<i64>,      // Celková GPU pamäť v bajtoch
    pub gpu_memory_used: Option<i64>,       // Použitá GPU pamäť v bajtoch
    pub gpu_temperature: Option<f64>,       // Teplota GPU v °C (LEN RAZ!)
    
    // Sieťové metriky (voliteľné)
    pub network_sent_kbps: Option<f64>,     // Odoslané dáta v KB/s
    pub network_recv_kbps: Option<f64>,     // Prijaté dáta v KB/s
    
    // Všeobecné systémové informácie
    pub process_count: i64,                 // Počet aktívnych procesov
    pub system_uptime: i64,                 // Doba behu systému v sekundách
    
    // Teplotné metriky (nové polia)
    pub cpu_temperature: Option<f64>,       // Teplota CPU v °C
    pub motherboard_temperature: Option<f64>, // Teplota základnej dosky v °C
    pub disk_temperature: Option<f64>,      // Teplota disku v °C
    pub max_temperature: Option<f64>,       // Maximálna nameraná teplota v °C
    
    // POZOR: gpu_temperature už existuje vyššie - NEOPAKOVAŤ!
}

/// Informácie o procese
/// Obsahuje základné metriky jednotlivého procesu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,                           // ID procesu
    pub name: String,                       // Názov procesu
    pub memory: u64,                        // Použitá pamäť v bajtoch
    pub cpu_usage: f32,                     // Využitie CPU v percentách
    pub network_sent: Option<u64>,          // Odoslané sieťové dáta v bajtoch
    pub network_recv: Option<u64>,          // Prijaté sieťové dáta v bajtoch
}

/// Informácie o CPU
/// Špecifické metriky pre procesor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,                       // Názov procesora
    pub usage: f32,                         // Celkové využitie v percentách
    pub frequency: u64,                     // Frekvencia v Hz
}

/// Informácie o pamäti
/// Štatistiky RAM a swap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: u64,                         // Celková RAM v bajtoch
    pub used: u64,                          // Použitá RAM v bajtoch
    pub available: u64,                     // Dostupné RAM v bajtoch
}

/// Informácie o disku
/// Štatistiky úložného priestoru
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,                       // Názov disku alebo oddielu
    pub total: u64,                         // Celková veľkosť v bajtoch
    pub used: u64,                          // Použitý priestor v bajtoch
    pub available: u64,                     // Dostupné miesto v bajtoch
}

/// Informácie o GPU
/// Špecifické metriky pre grafický procesor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,                       // Názov grafického procesora
    pub usage: f64,                         // Využitie GPU v percentách
    pub memory_total: u64,                  // Celková GPU pamäť v bajtoch
    pub memory_used: u64,                   // Použitá GPU pamäť v bajtoch
    pub temperature: Option<f64>,           // Teplota GPU v °C
}