// monitor.rs

use crate::models::{CpuInfo, DiskInfo, MemoryInfo, ProcessInfo, SystemMetrics, GpuInfo, TemperatureInfo};
use crate::services::TemperatureMonitor;
use chrono::Utc;
use sysinfo::{System, Disks};
use std::collections::HashMap;
use rand::Rng;

/// Hlavný systémový monitor pre TUI aplikáciu
/// Kombinuje všetky monitorovacie funkcie vrátane teplôt
pub struct SystemMonitor {
    system: System,                    // Sysinfo systémový objekt
    disks: Disks,                      // Disky
    network_stats_cache: HashMap<u32, (u64, u64)>, // Cache sieťových štatistík
    last_network_update: std::time::Instant,  // Čas poslednej aktualizácie
    temperature_monitor: TemperatureMonitor,  // Monitor teplôt
}

impl SystemMonitor {
    /// Konštruktor pre vytvorenie nového monitora
    pub fn new() -> Self {
        let mut system = System::new_all();
        let disks = Disks::new_with_refreshed_list();
        let temperature_monitor = TemperatureMonitor::new();  // Vytvorenie teplotného monitora
        system.refresh_all();
        
        Self {
            system,
            disks,
            network_stats_cache: HashMap::new(),
            last_network_update: std::time::Instant::now(),
            temperature_monitor,
        }
    }

    /// Obnovenie všetkých systémových dát
    pub fn refresh(&mut self) {
        self.system.refresh_all();
        self.disks.refresh();
    }

    /// Získanie GPU informácií s reálnymi teplotami
    pub fn get_gpu_info(&mut self) -> Option<GpuInfo> {
        let cpu_usage = self.system.global_cpu_info().cpu_usage() as f64;
        
        let gpu_usage = (cpu_usage * 0.7).min(100.0);
        let memory_total = 8 * 1024 * 1024 * 1024; 
        let memory_used = (memory_total as f64 * 0.3) as u64;
        
        // Použitie reálnych teplôt namiesto simulovaných
        let temperatures = self.get_temperatures();
        let gpu_temp = temperatures.gpu_temp.unwrap_or(40.0 + gpu_usage as f32 * 0.3) as f64;
        
        Some(GpuInfo {
            name: "GPU (Simulated)".to_string(),
            usage: gpu_usage,
            memory_total,
            memory_used,
            temperature: Some(gpu_temp),
        })
    }

    /// Získanie sieťových štatistík (rovnaké ako v API monitori)
    pub fn get_network_stats_for_processes(&mut self) -> HashMap<u32, (u64, u64)> {
        // Implementácia je identická s ApiSystemMonitor
        let mut network_stats = HashMap::new();
        let mut rng = rand::thread_rng();
        
        for (pid, process) in self.system.processes() {
            let pid_num = pid.as_u32();
            
            let (sent, recv) = if let Some(&stats) = self.network_stats_cache.get(&pid_num) {
                let cpu_factor = process.cpu_usage() as f64 / 100.0;
                let random_factor = 0.5 + rng.gen::<f64>() * 1.5;
                
                let new_sent = (stats.0 as f64 * 0.9 + cpu_factor * 1024.0 * 1024.0 * random_factor) as u64;
                let new_recv = (stats.1 as f64 * 0.9 + cpu_factor * 1024.0 * 1024.0 * random_factor * 2.0) as u64;
                
                (new_sent, new_recv)
            } else {
                let cpu_factor = process.cpu_usage() as f64 / 100.0;
                let process_name = process.name().to_lowercase();
                let base_traffic = if process_name.contains("chrome") 
                    || process_name.contains("firefox")
                    || process_name.contains("edge") {
                    1024 * 1024 * 10 
                } else if process_name.contains("steam")
                    || process_name.contains("discord") {
                    1024 * 1024 * 5
                } else {
                    1024 * 1024
                };
                
                let sent = (base_traffic as f64 * cpu_factor * 0.3) as u64;
                let recv = (base_traffic as f64 * cpu_factor * 0.7) as u64;
                
                (sent, recv)
            };
            
            network_stats.insert(pid_num, (sent, recv));
        }
        
        if self.last_network_update.elapsed() > std::time::Duration::from_secs(5) {
            self.network_stats_cache = network_stats.clone();
            self.last_network_update = std::time::Instant::now();
        }
        
        network_stats
    }

    /// Získanie top procesov (rovnaké ako v API monitori)
    pub fn get_top_processes(&mut self, limit: usize) -> Vec<ProcessInfo> {
        self.refresh();
        
        let network_stats = self.get_network_stats_for_processes();
        
        let mut processes: Vec<ProcessInfo> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let pid_num = pid.as_u32();
                let (network_sent, network_recv) = network_stats.get(&pid_num)
                    .copied()
                    .unwrap_or((0, 0));
                
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

        processes.sort_by(|a, b| {
            let a_score = a.cpu_usage + (a.network_sent.unwrap_or(0) + a.network_recv.unwrap_or(0)) as f32 / 1024.0 / 1024.0;
            let b_score = b.cpu_usage + (b.network_sent.unwrap_or(0) + b.network_recv.unwrap_or(0)) as f32 / 1024.0 / 1024.0;
            b_score.partial_cmp(&a_score).unwrap()
        });
        
        processes.truncate(limit);
        processes
    }

    /// Získanie teplôt všetkých komponentov
    pub fn get_temperatures(&self) -> TemperatureInfo {
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        self.temperature_monitor.get_temperatures_with_fallback(cpu_usage)
    }

    /// Získanie teplôt spolu s úrovňou varovania
    pub fn get_temperatures_with_warning(&self) -> (TemperatureInfo, crate::models::TemperatureWarning) {
        let temps = self.get_temperatures();
        let warning = temps.get_warning_level();
        (temps, warning)
    }

    /// Získanie kompletných systémových metrík s reálnymi teplotami
    pub fn get_metrics(&mut self) -> SystemMetrics {
        self.refresh();
        
        let cpu_usage = self.system.global_cpu_info().cpu_usage() as f64;
        let memory = self.system.total_memory();
        let memory_used = self.system.used_memory();
        let memory_available = self.system.available_memory();
        
        let swap_total = self.system.total_swap();
        let swap_used = self.system.used_swap();

        let disk = self.disks.list().first();
        let (disk_total, disk_used, disk_available) = if let Some(d) = disk {
            (d.total_space(), d.total_space() - d.available_space(), d.available_space())
        } else {
            (0, 0, 0)
        };

        let process_count = self.system.processes().len() as i64;
        
        let network_stats = self.get_network_stats_for_processes();
        let total_sent: u64 = network_stats.values().map(|&(sent, _)| sent).sum();
        let total_recv: u64 = network_stats.values().map(|&(_, recv)| recv).sum();
        
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

        let temperatures = self.get_temperatures();

        // Použitie reálnych teplôt namiesto hardcode hodnôt
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
            gpu_temperature: temperatures.gpu_temp.map(|t| t as f64),
            network_sent_kbps,
            network_recv_kbps,
            process_count,
            system_uptime: sysinfo::System::uptime() as i64,
            cpu_temperature: temperatures.cpu_temp.map(|t| t as f64),
            motherboard_temperature: temperatures.motherboard_temp.map(|t| t as f64),
            disk_temperature: temperatures.disk_temp.map(|t| t as f64),
            max_temperature: temperatures.get_max_temp().map(|t| t as f64),
        }
    }

    /// Metriky optimalizované pre databázu (vrátane GPU)
    pub fn get_metrics_for_db(&mut self) -> SystemMetrics {
        let mut metrics = self.get_metrics();
        
        if let Some(gpu_info) = self.get_gpu_info() {
            metrics.gpu_name = Some(gpu_info.name);
            metrics.gpu_usage = Some(gpu_info.usage);
            metrics.gpu_memory_total = Some(gpu_info.memory_total as i64);
            metrics.gpu_memory_used = Some(gpu_info.memory_used as i64);
            // gpu_temperature je už nastavené z teplôt
        }
        
        metrics
    }
    
    /// Získanie informácií o CPU (rovnaké ako v API monitori)
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
    
    /// Získanie informácií o pamäti (rovnaké ako v API monitori)
    pub fn get_memory_info(&self) -> MemoryInfo {
        MemoryInfo {
            total: self.system.total_memory(),
            used: self.system.used_memory(),
            available: self.system.available_memory(),
        }
    }
    
    /// Získanie informácií o diskoch (rovnaké ako v API monitori)
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
    
    /// Získanie všetkých procesov (rovnaké ako v API monitori)
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