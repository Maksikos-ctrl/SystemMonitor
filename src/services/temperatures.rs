// temperatures.rs

use crate::models::TemperatureInfo;
use wmi::{COMLibrary, WMIConnection};
use std::collections::HashMap;

/// Monitor teplôt systémových komponentov
/// Používa WMI (Windows Management Instrumentation) pre čítanie teplôt
pub struct TemperatureMonitor {
    wmi_con: Option<WMIConnection>,  // WMI spojenie (len pre Windows)
}

impl TemperatureMonitor {
    /// Konštruktor pre vytvorenie monitora teplôt
    pub fn new() -> Self {
        let wmi_con = match Self::create_wmi_connection() {
            Ok(con) => {
                println!("✅ WMI connection established for temperature monitoring");
                Some(con)
            }
            Err(e) => {
                // Chybové hlásenie ak WMI zlyhá (napr. na Linuxe)
                eprintln!("⚠️  Failed to establish WMI connection: {}", e);
                eprintln!("   Temperature monitoring will be limited");
                None
            }
        };
        
        TemperatureMonitor { wmi_con }
    }
    
    /// Vytvorenie WMI spojenia (len Windows)
    fn create_wmi_connection() -> Result<WMIConnection, wmi::WMIError> {
        let com_con = COMLibrary::new()?;          // Inicializácia COM knižnice
        WMIConnection::new(com_con.into())         // Vytvorenie WMI spojenia
    }
    
    /// Získanie teplôt všetkých komponentov
    pub fn get_temperatures(&self) -> TemperatureInfo {
        let mut temps = TemperatureInfo::new();
        
        // Ak máme WMI spojenie, načítame reálne teploty
        if let Some(wmi_con) = &self.wmi_con {
            temps.cpu_temp = self.get_cpu_temperature(wmi_con);
            temps.gpu_temp = self.get_gpu_temperature(wmi_con);
            temps.motherboard_temp = self.get_motherboard_temperature(wmi_con);
            temps.disk_temp = self.get_disk_temperature(wmi_con);
        }
        
        temps
    }
    
    /// Získanie teploty CPU cez WMI
    fn get_cpu_temperature(&self, wmi_con: &WMIConnection) -> Option<f32> {
        // Prvý pokus: MSAcpi_ThermalZoneTemperature
        let query = "SELECT * FROM MSAcpi_ThermalZoneTemperature";
        if let Ok(results) = wmi_con.raw_query::<HashMap<String, serde_json::Value>>(query) {
            for result in results {
                if let Some(temp_raw) = result.get("CurrentTemperature") {
                    if let serde_json::Value::Number(num) = temp_raw {
                        if let Some(temp_kelvin) = num.as_u64() {
                            // Konverzia z Kelvinov na Celsius
                            let temp_celsius = (temp_kelvin as f32 / 10.0) - 273.15;
                            if temp_celsius > 0.0 && temp_celsius < 150.0 {
                                return Some(temp_celsius);
                            }
                        }
                    }
                }
            }
        }
        
        // Druhý pokus: Win32_TemperatureProbe
        let query = "SELECT * FROM Win32_TemperatureProbe";
        if let Ok(results) = wmi_con.raw_query::<HashMap<String, serde_json::Value>>(query) {
            for result in results {
                if let Some(temp_raw) = result.get("CurrentReading") {
                    if let serde_json::Value::Number(num) = temp_raw {
                        if let Some(temp) = num.as_f64() {
                            let temp_celsius = temp as f32 / 10.0;
                            if temp_celsius > 0.0 && temp_celsius < 150.0 {
                                return Some(temp_celsius);
                            }
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// Získanie teploty GPU (zjednodušené)
    fn get_gpu_temperature(&self, wmi_con: &WMIConnection) -> Option<f32> {
        let query = "SELECT * FROM Win32_VideoController";
        if let Ok(results) = wmi_con.raw_query::<HashMap<String, serde_json::Value>>(query) {
            for result in results {
                if let Some(_adapter_ram) = result.get("AdapterRAM") { 
                    // Ak GPU existuje, vráť odhadovanú teplotu
                    return Some(65.0);
                }
            }
        }
        
        None
    }
    
    /// Získanie teploty základnej dosky
    fn get_motherboard_temperature(&self, wmi_con: &WMIConnection) -> Option<f32> {
        let query = "SELECT * FROM Win32_TemperatureProbe WHERE Name LIKE '%Motherboard%' OR Name LIKE '%System%'";
        if let Ok(results) = wmi_con.raw_query::<HashMap<String, serde_json::Value>>(query) {
            for result in results {
                if let Some(temp_raw) = result.get("CurrentReading") {
                    if let serde_json::Value::Number(num) = temp_raw {
                        if let Some(temp) = num.as_f64() {
                            let temp_celsius = temp as f32 / 10.0;
                            if temp_celsius > 0.0 && temp_celsius < 150.0 {
                                return Some(temp_celsius);
                            }
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// Získanie teploty disku
    fn get_disk_temperature(&self, wmi_con: &WMIConnection) -> Option<f32> {
        let query = "SELECT * FROM MSStorageDriver_ATAPISmartData";
        if let Ok(results) = wmi_con.raw_query::<HashMap<String, serde_json::Value>>(query) {
            for result in results {
                if let Some(vendor_data) = result.get("VendorSpecific") {
                    if let serde_json::Value::Array(bytes) = vendor_data {
                        if bytes.len() > 0 {
                            // Ak SMART dáta existujú, vráť odhadovanú teplotu
                            return Some(45.0);
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// Odhad teplôt na základe využitia CPU
    pub fn get_estimated_temperatures(&self, cpu_usage: f32) -> TemperatureInfo {
        let mut temps = TemperatureInfo::new();
        
        // Odhad teplôt na základe zaťaženia CPU
        temps.cpu_temp = Some(30.0 + (cpu_usage * 0.5));
        temps.gpu_temp = Some(40.0 + (cpu_usage * 0.3));
        temps.motherboard_temp = Some(35.0 + (cpu_usage * 0.2));
        temps.disk_temp = Some(38.0);
        
        temps
    }
    
    /// Získanie teplôt s fallback na odhady ak reálne dáta nie sú dostupné
    pub fn get_temperatures_with_fallback(&self, cpu_usage: f32) -> crate::models::TemperatureInfo {
        let real_temps = self.get_temperatures();
        
        // Kontrola či sme získali nejaké reálne dáta
        if real_temps.cpu_temp.is_some() 
            || real_temps.gpu_temp.is_some()
            || real_temps.motherboard_temp.is_some()
            || real_temps.disk_temp.is_some() {
            return real_temps;  // Vráť reálne dáta
        }
        
        // Ak žiadne reálne dáta, vráť odhady
        self.get_estimated_temperatures(cpu_usage)
    }
}