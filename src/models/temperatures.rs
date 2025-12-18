// temperatures.rs

use serde::{Deserialize, Serialize};

/// Štruktúra pre zber teplôt komponentov
/// Centralizované ukladanie teplôt rôznych systémových komponentov
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureInfo {
    pub cpu_temp: Option<f32>,              // Teplota CPU v °C
    pub gpu_temp: Option<f32>,              // Teplota GPU v °C
    pub motherboard_temp: Option<f32>,      // Teplota základnej dosky v °C
    pub disk_temp: Option<f32>,             // Teplota disku v °C
}

/// Default implementácia pre TemperatureInfo
/// Vytvára prázdnu inštanciu so všetkými hodnotami None
impl Default for TemperatureInfo {
    fn default() -> Self {
        TemperatureInfo {
            cpu_temp: None,
            gpu_temp: None,
            motherboard_temp: None,
            disk_temp: None,
        }
    }
}

/// Implementácia metód pre TemperatureInfo
impl TemperatureInfo {
    /// Konštruktor pre vytvorenie novej inštancie
    pub fn new() -> Self {
        Self::default()  // Použitie default hodnot
    }

    /// Výpočet maximálnej teploty zo všetkých komponentov
    /// Vráti None ak nie sú dostupné žiadne teploty
    pub fn get_max_temp(&self) -> Option<f32> {
        // Zoznam všetkých teplôt
        let temps = [
            self.cpu_temp,
            self.gpu_temp,
            self.motherboard_temp,
            self.disk_temp,
        ];
        
        // Filtrovanie None hodnôt a nájdenie maxima
        temps.iter()
            .filter_map(|&t| t)                    // Odstránenie None hodnôt
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))  // Nájdenie maxima
    }

    /// Určenie úrovne varovania podľa maximálnej teploty
    /// Používa sa pre vizuálnu indikáciu teplotného stavu systému
    pub fn get_warning_level(&self) -> TemperatureWarning {
        if let Some(max_temp) = self.get_max_temp() {
            // Rozdelenie podľa teplotných prahov
            if max_temp > 85.0 {
                TemperatureWarning::Critical  // Kritická teplota (>85°C)
            } else if max_temp > 75.0 {
                TemperatureWarning::High      // Vysoká teplota (75-85°C)
            } else if max_temp > 65.0 {
                TemperatureWarning::Medium    // Stredná teplota (65-75°C)
            } else {
                TemperatureWarning::Normal    // Normálna teplota (<65°C)
            }
        } else {
            TemperatureWarning::Unknown       // Neznáma teplota (žiadne dáta)
        }
    }
}

/// Enum pre úrovne teplotných varovaní
/// Používa sa pre farebnú a vizuálnu indikáciu
#[derive(Debug, Clone, PartialEq)]
pub enum TemperatureWarning {
    Normal,     // Normálna teplota - zelená
    Medium,     // Stredná teplota - žltá/oranžová
    High,       // Vysoká teplota - oranžová
    Critical,   // Kritická teplota - červená
    Unknown,    // Neznámy stav - šedá
}