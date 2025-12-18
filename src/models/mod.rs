// mod.rs

/// Hlavný modul pre dátové modely aplikácie
/// Organizuje modely do logických skupín
pub mod metrics;       // Modul pre systémové metriky
pub mod temperatures;  // Modul pre teplotné dáta

/// Re-export dôležitých štruktúr pre jednoduchší import
pub use metrics::{SystemMetrics, CpuInfo, MemoryInfo, DiskInfo, ProcessInfo, GpuInfo};
pub use temperatures::{TemperatureInfo, TemperatureWarning};