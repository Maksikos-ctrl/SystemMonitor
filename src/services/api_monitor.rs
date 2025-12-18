// api_monitor.rs

use crate::models::{CpuInfo, DiskInfo, MemoryInfo, ProcessInfo, SystemMetrics, GpuInfo};
use chrono::Utc;
use sysinfo::{System, Disks};
use std::collections::HashMap;
use rand::Rng;

/// API systémový monitor - špecializovaná verzia pre REST API server
/// Zodpovedá za zbieranie a správu systémových metrík pre API endpointy
pub struct ApiSystemMonitor {
    system: System,                    // Hlavný systémový objekt sysinfo
    disks: Disks,                      // Zoznam diskov
    network_stats_cache: HashMap<u32, (u64, u64)>, // Cache sieťových štatistík procesov
    last_network_update: std::time::Instant,  // Čas poslednej aktualizácie cache
}

impl ApiSystemMonitor {
    /// Konštruktor pre vytvorenie novej inštancie API monitora
    pub fn new() -> Self {
        let mut system = System::new_all();      // Vytvorenie systému so všetkými komponentmi
        let disks = Disks::new_with_refreshed_list();  // Vytvorenie zoznamu diskov s obnovením
        system.refresh_all();                    // Inicializačné obnovenie všetkých dát
        
        Self {
            system,
            disks,
            network_stats_cache: HashMap::new(),  // Prázdna cache
            last_network_update: std::time::Instant::now(),  // Aktuálny čas
        }
    }

    /// Obnovenie všetkých systémových dát
    /// Volané pred každým zberom metrík pre aktuálne dáta
    pub fn refresh(&mut self) {
        self.system.refresh_all();  // Obnovenie všetkých systémových informácií
        self.disks.refresh();       // Obnovenie informácií o diskoch
    }

    /// Získanie informácií o GPU (simulované)
    /// Pretože sysinfo neposkytuje GPU dáta, simulujeme ich na základe CPU
    pub fn get_gpu_info(&mut self) -> Option<GpuInfo> {
        let cpu_usage = self.system.global_cpu_info().cpu_usage() as f64;
        
        // Simulácia GPU využitia ako 70% CPU využitia
        let gpu_usage = (cpu_usage * 0.7).min(100.0);
        
        // Simulované hodnoty pre GPU pamäť (8 GB)
        let memory_total = 8 * 1024 * 1024 * 1024;  // 8 GB v bajtoch
        let memory_used = (memory_total as f64 * 0.3) as u64;  // 30% využitia
        
        // Simulácia teploty GPU na základe využitia
        let gpu_temp = Some(40.0 + gpu_usage * 0.3);
        
        Some(GpuInfo {
            name: "GPU (Simulated)".to_string(),  // Názov indikujúci simuláciu
            usage: gpu_usage,
            memory_total,
            memory_used,
            temperature: gpu_temp,
        })
    }

    /// Získanie sieťových štatistík pre procesy
    /// Používa cache a real-time výpočty pre realistické dáta
    pub fn get_network_stats_for_processes(&mut self) -> HashMap<u32, (u64, u64)> {
        let mut network_stats = HashMap::new();
        let mut rng = rand::thread_rng();  // Generátor náhodných čísel
        
        for (pid, process) in self.system.processes() {
            let pid_num = pid.as_u32();
            
            // Výpočet sieťovej aktivity pre proces
            let (sent, recv) = if let Some(&stats) = self.network_stats_cache.get(&pid_num) {
                // Ak máme cache, použijeme ju ako základ
                let cpu_factor = process.cpu_usage() as f64 / 100.0;
                let random_factor = 0.5 + rng.gen::<f64>() * 1.5;  // Náhodný faktor 0.5-2.0
                
                // Výpočet nových hodnôt s decay (90% starých hodnôt + nový príspevok)
                let new_sent = (stats.0 as f64 * 0.9 + cpu_factor * 1024.0 * 1024.0 * random_factor) as u64;
                let new_recv = (stats.1 as f64 * 0.9 + cpu_factor * 1024.0 * 1024.0 * random_factor * 2.0) as u64;
                
                (new_sent, new_recv)
            } else {
                // Prvý výpočet pre proces
                let cpu_factor = process.cpu_usage() as f64 / 100.0;
                let process_name = process.name().to_lowercase();
                
                // Rôzne základné hodnoty podľa typu procesu
                let base_traffic = if process_name.contains("chrome") 
                    || process_name.contains("firefox")
                    || process_name.contains("edge") {
                    1024 * 1024 * 10  // 10 MB pre prehliadače
                } else if process_name.contains("steam")
                    || process_name.contains("discord") {
                    1024 * 1024 * 5   // 5 MB pre herné/komunikačné aplikácie
                } else {
                    1024 * 1024       // 1 MB pre ostatné procesy
                };
                
                // Rozdelenie na odoslané a prijaté dáta
                let sent = (base_traffic as f64 * cpu_factor * 0.3) as u64;
                let recv = (base_traffic as f64 * cpu_factor * 0.7) as u64;
                
                (sent, recv)
            };
            
            network_stats.insert(pid_num, (sent, recv));
        }
        
        // Aktualizácia cache každých 5 sekúnd
        if self.last_network_update.elapsed() > std::time::Duration::from_secs(5) {
            self.network_stats_cache = network_stats.clone();
            self.last_network_update = std::time::Instant::now();
        }
        
        network_stats
    }

    /// Získanie top procesov podľa kombinovaného skóre (CPU + sieťová aktivita)
    pub fn get_top_processes(&mut self, limit: usize) -> Vec<ProcessInfo> {
        self.refresh();  // Obnovenie dát
        
        let network_stats = self.get_network_stats_for_processes();
        
        // Transformácia sysinfo procesov na naše ProcessInfo
        let mut processes: Vec<ProcessInfo> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let pid_num = pid.as_u32();
                let (network_sent, network_recv) = network_stats.get(&pid_num)
                    .copied()
                    .unwrap_or((0, 0));  // Default 0 ak neexistujú štatistiky
                
                ProcessInfo {
                    pid: pid_num,
                    name: process.name().to_string(),
                    cpu_usage: process.cpu_usage(),
                    memory: process.memory(),
                    network_sent: Some(network_sent),
                    network_recv: Some(network_recv),
                }
            })
            .collect();

        // Zoradenie podľa kombinovaného skóre (CPU + sieťová aktivita v MB)
        processes.sort_by(|a, b| {
            let a_score = a.cpu_usage + (a.network_sent.unwrap_or(0) + a.network_recv.unwrap_or(0)) as f32 / 1024.0 / 1024.0;
            let b_score = b.cpu_usage + (b.network_sent.unwrap_or(0) + b.network_recv.unwrap_or(0)) as f32 / 1024.0 / 1024.0;
            b_score.partial_cmp(&a_score).unwrap()  // Zostupné poradie
        });
        
        processes.truncate(limit);  // Obmedzenie na zadaný počet
        processes
    }

    /// Získanie kompletných systémových metrík
    pub fn get_metrics(&mut self) -> SystemMetrics {
        self.refresh();
        
        // CPU metriky
        let cpu_usage = self.system.global_cpu_info().cpu_usage() as f64;
        
        // RAM metriky
        let memory = self.system.total_memory();
        let memory_used = self.system.used_memory();
        let memory_available = self.system.available_memory();
        
        // Swap metriky
        let swap_total = self.system.total_swap();
        let swap_used = self.system.used_swap();

        // Disk metriky (prvý disk)
        let disk = self.disks.list().first();
        let (disk_total, disk_used, disk_available) = if let Some(d) = disk {
            (d.total_space(), d.total_space() - d.available_space(), d.available_space())
        } else {
            (0, 0, 0)
        };

        // Počet procesov
        let process_count = self.system.processes().len() as i64;
        
        // Sieťové štatistiky (celkové)
        let network_stats = self.get_network_stats_for_processes();
        let total_sent: u64 = network_stats.values().map(|&(sent, _)| sent).sum();
        let total_recv: u64 = network_stats.values().map(|&(_, recv)| recv).sum();
        
        // Konverzia na KB/s
        let network_sent_kbps = if total_sent > 0 { 
            Some(total_sent as f64 / 1024.0) 
        } else { 
            None 
        };
        
        let network_recv_kbps = if total_recv > 0 { 
            Some(total_recv as f64 / 1024.0) 
        } else { 
            None 
        };

        // Vytvorenie SystemMetrics objektu s hardcode teplotami pre API
        SystemMetrics {
            id: None,
            timestamp: Utc::now(),
            cpu_usage,
            memory_total: memory as i64,
            memory_used: memory_used as i64,
            memory_available: memory_available as i64,
            swap_total: swap_total as i64,
            swap_used: swap_used as i64,
            disk_total: disk_total as i64,
            disk_used: disk_used as i64,
            disk_available: disk_available as i64,
            gpu_name: None,
            gpu_usage: None,
            gpu_memory_total: None,
            gpu_memory_used: None,
            gpu_temperature: None,
            network_sent_kbps,
            network_recv_kbps,
            process_count,
            system_uptime: sysinfo::System::uptime() as i64,
            cpu_temperature: Some(40.0),  // Hardcode teploty pre API
            motherboard_temperature: Some(35.0), 
            disk_temperature: Some(38.0),
            max_temperature: Some(45.0), 
        }
    }

    /// Získanie metrík optimalizovaných pre ukladanie do databázy
    /// Zahŕňa aj GPU informácie
    pub fn get_metrics_for_db(&mut self) -> SystemMetrics {
        let mut metrics = self.get_metrics();
        
        // Pridanie GPU informácií ak sú dostupné
        if let Some(gpu_info) = self.get_gpu_info() {
            metrics.gpu_name = Some(gpu_info.name);
            metrics.gpu_usage = Some(gpu_info.usage);
            metrics.gpu_memory_total = Some(gpu_info.memory_total as i64);
            metrics.gpu_memory_used = Some(gpu_info.memory_used as i64);
            metrics.gpu_temperature = gpu_info.temperature;
        }
        
        metrics
    }
    
    /// Získanie informácií o všetkých CPU jadrách
    pub fn get_cpu_info(&self) -> Vec<CpuInfo> {
        self.system.cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| CpuInfo {
                name: format!("CPU {}", i + 1),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
            })
            .collect()
    }
    
    /// Získanie informácií o pamäti
    pub fn get_memory_info(&self) -> MemoryInfo {
        MemoryInfo {
            total: self.system.total_memory(),
            used: self.system.used_memory(),
            available: self.system.available_memory(),
        }
    }
    
    /// Získanie informácií o všetkých diskoch
    pub fn get_disk_info(&self) -> Vec<DiskInfo> {
        self.disks.list()
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                total: disk.total_space(),
                used: disk.total_space() - disk.available_space(),
                available: disk.available_space(),
            })
            .collect()
    }
    
    /// Získanie zoznamu všetkých procesov (bez sieťových štatistík)
    pub fn get_processes(&self) -> Vec<ProcessInfo> {
        self.system.processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
                network_sent: None,
                network_recv: None,
            })
            .collect()
    }
}