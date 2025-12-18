// Importy pre stav aplikácie a TUI komponenty
use ratatui::widgets::ListState;  // Stav pre zoznamy (selekcia, scrollovanie)
use std::sync::{Arc, Mutex};      // Bezpečné zdieľanie dát medzi vláknami
use crate::services::monitor::SystemMonitor;  // Monitorovací servis
use crate::models::{SystemMetrics, GpuInfo, ProcessInfo as ModelsProcessInfo};  // Dátové modely
use std::collections::HashMap;    // Hash map pre efektívne vyhľadávanie
use std::process::Command;        // Spúšťanie externých príkazov

/// Informácie o systéme zobrazované v TUI
/// Tieto informácie sa získavajú pri štarte aplikácie
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub hostname: String,    // Názov počítača v sieti
    pub os_name: String,     // Názov operačného systému
    pub cpu_name: String,    // Model procesora
    pub gpu_name: String,    // Model grafickej karty
}

/// Reprezentácia sieťového spojenia procesu
/// Obsahuje informácie o lokálnom a vzdialenom konci spojenia
#[derive(Debug, Clone)]
pub struct NetworkConnection {
    pub process_name: String,    // Názov procesu vytvárajúceho spojenie
    pub local_address: String,   // Lokálna IP adresa a port
    pub remote_address: String,  // Vzdialená IP adresa a port
    pub protocol: String,        // Sieťový protokol (TCP/UDP)
    pub state: String,           // Stav spojenia (ESTABLISHED, LISTENING, atď.)
    pub pid: u32,               // PID procesu
}

/// Režimy zobrazenia TUI aplikácie
/// Definuje, ktorá obrazovka sa má renderovať
#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    Overview,        // Hlavný prehľad systému
    ProcessDetail,   // Detailný pohľad na proces
    NetworkView,     // Sieťová aktivita a spojenia
    Help,            // Nápoveda a klávesové skratky
}

/// Veľkosť histórie pre grafy (v počte záznamov)
/// Každý záznam predstavuje jednu sekundu
pub const HISTORY_SIZE: usize = 30;

/// Hlavná štruktúra aplikácie - obsahuje všetok stav TUI
/// Táto štruktúra sa pravidelne aktualizuje a renderuje
pub struct TuiApp {
    // ========== ZÁKLADNÝ STAV ==========
    pub mode: Mode,               // Aktuálny režim zobrazenia
    pub should_quit: bool,        // Príznak pre ukončenie aplikácie
    
    // ========== MONITOROVACÍ SERVIS ==========
    /// Zdieľaný monitor chránený mutexom
    /// Umožňuje bezpečný prístup z viacerých vlákien
    pub monitor: Arc<Mutex<SystemMonitor>>,
    
    // ========== SYSTÉMOVÉ INFORMÁCIE ==========
    pub system_info: SystemInfo,  // Statické informácie o systéme
    pub metrics: Option<SystemMetrics>,  // Aktuálne metriky (CPU, RAM, sieť)
    pub gpu_info: Option<GpuInfo>,       // Informácie o GPU
    
    // ========== PROCESY ==========
    pub top_processes: Vec<ModelsProcessInfo>,  // Zoznam najnáročnejších procesov
    pub process_list_state: ListState,          // Stav navigácie v zozname procesov
    
    // ========== HISTÓRIA PRE GRAFY ==========
    /// Historické dáta pre časové grafy
    /// Každé pole obsahuje HISTORY_SIZE najnovších hodnôt
    pub cpu_history: Vec<u64>,     // História využitia CPU (%)
    pub ram_history: Vec<u64>,     // História využitia RAM (%)
    pub disk_history: Vec<u64>,    // História využitia disku (%)
    pub gpu_history: Vec<u64>,     // História využitia GPU (%)
    
    // ========== SIETOVÉ DÁTA ==========
    pub network_sent_history: Vec<f64>,     // História odoslaných dát (KB/s)
    pub network_recv_history: Vec<f64>,     // História prijatých dát (KB/s)
    pub network_sent_total: f64,            // Celkové odoslané dáta (KB/s)
    pub network_recv_total: f64,            // Celkové prijaté dáta (KB/s)
    pub top_network_processes: Vec<ModelsProcessInfo>,  // Procesy so sieťovou aktivitou
    pub network_connections: Vec<NetworkConnection>,     // Aktívne sieťové spojenia
    pub network_process_state: ListState,               // Stav navigácie v sieťových procesoch
    pub network_mode_detail: Option<String>,            // Detailný pohľad na sieťový proces
}

impl TuiApp {
    /// Vytvorí novú inštanciu TUI aplikácie
    ///
    /// # Argumenty
    /// * `monitor` - Zdieľaný monitorovací servis
    ///
    /// # Inicializácia
    /// * Nastaví základný stav aplikácie
    /// * Získa statické informácie o systéme
    /// * Inicializuje prázdne histórie
    pub fn new(monitor: Arc<Mutex<SystemMonitor>>) -> Self {
        use whoami::fallible;
        
        // Získanie hostname s ošetrením chýb
        let hostname = fallible::hostname()
            .unwrap_or_else(|_| "Unknown".to_string());
        
        // Základné informácie o systéme
        let system_info = SystemInfo {
            hostname,
            os_name: format!("{} {}", whoami::platform(), whoami::arch()),
            cpu_name: "Unknown CPU".to_string(),
            gpu_name: "Unknown GPU".to_string(),
        };
        
        // Konštrukcia aplikácie s predvolenými hodnotami
        Self {
            mode: Mode::Overview,
            should_quit: false,
            monitor,
            system_info,
            metrics: None,
            gpu_info: None,
            top_processes: Vec::new(),
            process_list_state: ListState::default(),
            cpu_history: Vec::with_capacity(HISTORY_SIZE),
            ram_history: Vec::with_capacity(HISTORY_SIZE),
            disk_history: Vec::with_capacity(HISTORY_SIZE),
            gpu_history: Vec::with_capacity(HISTORY_SIZE),
            
            network_sent_history: Vec::with_capacity(HISTORY_SIZE),
            network_recv_history: Vec::with_capacity(HISTORY_SIZE),
            network_sent_total: 0.0,
            network_recv_total: 0.0,
            top_network_processes: Vec::new(),
            network_connections: Vec::new(),
            network_process_state: ListState::default(),
            network_mode_detail: None,
        }
    }
    
    /// Aktualizuje všetky dáta aplikácie
    /// Táto metóda sa volá pravidelne každú sekundu
    ///
    /// # Tok dát
    /// 1. Získanie dát z monitora
    /// 2. Aktualizácia histórie
    /// 3. Získanie sieťových spojení
    /// 4. Výpočet sieťových štatistík
    pub fn update(&mut self) {
        // ========== ZÍSKANIE DÁT Z MONITORA ==========
        // Synchronizovaný prístup k monitoru cez mutex
        let (metrics_result, top_processes_result, gpu_info_result, network_stats) = {
            if let Ok(mut monitor) = self.monitor.lock() {
                let metrics = Some(monitor.get_metrics_for_db());
                let processes = monitor.get_top_processes(20);
                let gpu_info = monitor.get_gpu_info();
                let network_stats = monitor.get_network_stats_for_processes();
                
                (metrics, processes, gpu_info, network_stats)
            } else {
                // Fallback ak sa nepodarí získať zámok
                (None, Vec::new(), None, HashMap::new())
            }
        };
        
        // ========== AKTUALIZÁCIA ZÁKLADNÝCH DÁT ==========
        self.metrics = metrics_result;
        self.top_processes = top_processes_result.clone();
        self.gpu_info = gpu_info_result;
        
        // ========== ZÍSKANIE SIETOVÝCH SPOJENÍ ==========
        self.network_connections = self.get_real_network_connections(&top_processes_result);
        
        // ========== AKTUALIZÁCIA HISTÓRIE ==========
        if let Some(metrics) = &self.metrics {
            // CPU história - priame percento
            self.cpu_history.push(metrics.cpu_usage as u64);
            
            // RAM história - výpočet percenta z celkovej pamäte
            self.ram_history.push(((metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0) as u64);
            
            // Disk história - ak je dostupná informácia o disku
            if metrics.disk_total > 0 {
                let disk_percent = (metrics.disk_used as f64 / metrics.disk_total as f64) * 100.0;
                self.disk_history.push(disk_percent as u64);
            }
            
            // Orezanie histórie na maximálnu veľkosť
            if self.cpu_history.len() > HISTORY_SIZE { self.cpu_history.remove(0); }
            if self.ram_history.len() > HISTORY_SIZE { self.ram_history.remove(0); }
            if self.disk_history.len() > HISTORY_SIZE { self.disk_history.remove(0); }
        }
        
        // ========== GPU HISTÓRIA ==========
        if let Some(gpu_info) = &self.gpu_info {
            self.gpu_history.push(gpu_info.usage as u64);
            if self.gpu_history.len() > HISTORY_SIZE { self.gpu_history.remove(0); }
        }
        
        // ========== SIETOVÉ DÁTA ==========
        self.update_network_data(network_stats);
    }
    
    /// Získa reálne sieťové spojenia procesov
    /// Implementácia je špecifická pre jednotlivé OS
    ///
    /// # Argumenty
    /// * `processes` - Zoznam procesov na spárovanie so spojeniami
    ///
    /// # Platformy
    /// - Windows: Používa `netstat -ano`
    /// - Linux: Používa `ss -tuna` alebo `netstat -tuna`
    /// - macOS: Podobné ako Linux
    fn get_real_network_connections(&self, processes: &[ModelsProcessInfo]) -> Vec<NetworkConnection> {
        let mut connections = Vec::new();
        
        // Platformovo špecifická implementácia
        #[cfg(target_os = "windows")]
        {
            connections = self.get_windows_connections(processes);
        }
        
        #[cfg(target_os = "linux")]
        {
            connections = self.get_linux_connections(processes);
        }
        
        #[cfg(target_os = "macos")]
        {
            connections = self.get_macos_connections(processes);
        }
        
        // Fallback ak sa nepodarilo získať reálne spojenia
        if connections.is_empty() {
            self.get_fallback_connections(processes)
        } else {
            connections
        }
    }
    
    /// Získa sieťové spojenia na Windows pomocou netstat
    fn get_windows_connections(&self, processes: &[ModelsProcessInfo]) -> Vec<NetworkConnection> {
        let mut connections = Vec::new();
        
        // Spustenie netstat na získanie TCP spojení s PID
        match Command::new("netstat")
            .args(&["-ano", "-p", "TCP"])
            .output() 
        {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                
                // Parsovanie výstupu riadok po riadku
                for line in output_str.lines() {
                    if line.contains("TCP") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 5 {
                            // Extrakcia PID z piatého stĺpca
                            if let Ok(pid_str) = parts[4].parse::<u32>() {
                                // Nájdenie procesu podľa PID
                                if let Some(process) = processes.iter().find(|p| p.pid == pid_str) {
                                    let local_addr = parts[1].to_string();
                                    let remote_addr = parts[2].to_string();
                                    let state = parts[3].to_string();
                                    
                                    // Filtrovanie pasívnych spojení
                                    if state != "LISTENING" && remote_addr != "[::]:0" {
                                        connections.push(NetworkConnection {
                                            process_name: process.name.clone(),
                                            local_address: local_addr,
                                            remote_address: remote_addr,
                                            protocol: "TCP".to_string(),
                                            state,
                                            pid: pid_str,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // netstat nie je dostupný - použije sa fallback
            }
        }
        
        connections
    }
    
    /// Získa sieťové spojenia na Linux pomocou ss alebo netstat
    fn get_linux_connections(&self, processes: &[ModelsProcessInfo]) -> Vec<NetworkConnection> {
        let mut connections = Vec::new();
        
        // Možné príkazy v poradí pokusov
        let commands = vec!["ss -tuna", "netstat -tuna"];
        
        for cmd in commands {
            if let Ok(output) = Command::new("sh")
                .arg("-c")
                .arg(format!("{} 2>/dev/null", cmd))
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                
                // Preskočenie hlavičky
                for line in output_str.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 6 {
                        let state = parts[0];
                        let local_addr = parts[4];
                        let remote_addr = parts[5];
                        
                        // Filtrovanie pasívnych spojení
                        if state != "LISTEN" && !remote_addr.ends_with(":*") {
                            // Použitie lsof na získanie PID pre spojenie
                            if let Ok(lsof_output) = Command::new("lsof")
                                .args(&["-i", &format!("@{}", remote_addr.split(':').next().unwrap_or(""))])
                                .output()
                            {
                                let lsof_str = String::from_utf8_lossy(&lsof_output.stdout);
                                for lsof_line in lsof_str.lines().skip(1) {
                                    let lsof_parts: Vec<&str> = lsof_line.split_whitespace().collect();
                                    if lsof_parts.len() >= 2 {
                                        if let (Ok(pid), process_name) = (lsof_parts[1].parse::<u32>(), lsof_parts[0]) {
                                            if let Some(process) = processes.iter().find(|p| p.pid == pid) {
                                                connections.push(NetworkConnection {
                                                    process_name: process.name.clone(),
                                                    local_address: local_addr.to_string(),
                                                    remote_address: remote_addr.to_string(),
                                                    protocol: "TCP".to_string(),
                                                    state: state.to_string(),
                                                    pid,
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Ak sme našli spojenia, ukončíme hľadanie
                if !connections.is_empty() {
                    break;
                }
            }
        }
        
        connections
    }
    
    /// Získa sieťové spojenia na macOS (podobné ako Linux)
    fn get_macos_connections(&self, processes: &[ModelsProcessInfo]) -> Vec<NetworkConnection> {
        self.get_linux_connections(processes)
    }
    
    /// Fallback metóda pre získanie sieťových spojení
    /// Používa sa ak OS-špecifické metódy zlyhajú
    fn get_fallback_connections(&self, processes: &[ModelsProcessInfo]) -> Vec<NetworkConnection> {
        let mut connections = Vec::new();
        
        // Zostavenie spojení z procesov so sieťovou aktivitou
        for proc in processes.iter().take(10) {
            // Kontrola sieťovej aktivity procesu
            if proc.network_sent.unwrap_or(0) > 100 || proc.network_recv.unwrap_or(0) > 100 {
                connections.push(NetworkConnection {
                    process_name: proc.name.clone(),
                    local_address: format!("PID:{}", proc.pid),
                    remote_address: "Network activity detected".to_string(),
                    protocol: "DATA".to_string(),
                    state: "ACTIVE".to_string(),
                    pid: proc.pid,
                });
            }
        }
        
        // Informačná správa ak neboli nájdené žiadne spojenia
        if connections.is_empty() {
            connections.push(NetworkConnection {
                process_name: "System".to_string(),
                local_address: "N/A".to_string(),
                remote_address: "Real connections require elevated privileges".to_string(),
                protocol: "INFO".to_string(),
                state: "UNAVAILABLE".to_string(),
                pid: 0,
            });
        }
        
        connections
    }
    
    /// Aktualizuje sieťové dáta a štatistiky
    ///
    /// # Argumenty
    /// * `network_stats` - Mapa PID -> (odoslané dáta, prijaté dáta)
    fn update_network_data(&mut self, network_stats: HashMap<u32, (u64, u64)>) {
        // ========== HISTÓRIA SIETOVEJ AKTIVITY ==========
        if let Some(metrics) = &self.metrics {
            self.network_sent_history.push(metrics.network_sent_kbps.unwrap_or(0.0));
            self.network_recv_history.push(metrics.network_recv_kbps.unwrap_or(0.0));
            
            // Orezanie histórie
            if self.network_sent_history.len() > HISTORY_SIZE { self.network_sent_history.remove(0); }
            if self.network_recv_history.len() > HISTORY_SIZE { self.network_recv_history.remove(0); }
        }
        
        // ========== TOP SIETOVÉ PROCESY ==========
        // Klonovanie a triedenie procesov podľa celkovej sieťovej aktivity
        let mut network_procs: Vec<ModelsProcessInfo> = self.top_processes.clone();
        network_procs.sort_by(|a, b| {
            let a_total = a.network_sent.unwrap_or(0) + a.network_recv.unwrap_or(0);
            let b_total = b.network_sent.unwrap_or(0) + b.network_recv.unwrap_or(0);
            b_total.cmp(&a_total)  // Zostupné triedenie
        });
        
        // Výber 15 najaktívnejších procesov
        self.top_network_processes = network_procs.into_iter().take(15).collect();
        
        // ========== CELKOVÉ SIETOVÉ ŠTATISTIKY ==========
        let total_sent: u64 = network_stats.values().map(|&(sent, _)| sent).sum();
        let total_recv: u64 = network_stats.values().map(|&(_, recv)| recv).sum();
        
        // Konverzia na KB/s
        self.network_sent_total = total_sent as f64 / 1024.0;
        self.network_recv_total = total_recv as f64 / 1024.0;
    }
    
    // ========== PUBLICKÉ METÓDY PRE OVLÁDANIE APLIKÁCIE ==========
    
    /// Nastaví príznak pre ukončenie aplikácie
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
    /// Ručná aktualizácia dát (rovnaká ako automatická)
    pub fn refresh(&mut self) {
        self.update();
    }
    
    /// Prepne aplikáciu do sieťového režimu
    pub fn enter_network_mode(&mut self) {
        self.mode = Mode::NetworkView;
    }
    
    // ========== NAVIGÁCIA V PROCESOCH ==========
    
    /// Posunie výber v zozname procesov o jeden krok nahor
    /// Cyklická navigácia - z posledného na prvý
    pub fn previous_process(&mut self) {
        if !self.top_processes.is_empty() {
            let current = self.process_list_state.selected();
            let new_index = current.map_or(0, |i| {
                if i == 0 { self.top_processes.len() - 1 } else { i - 1 }
            });
            self.process_list_state.select(Some(new_index));
        }
    }
    
    /// Posunie výber v zozname procesov o jeden krok nadol
    /// Cyklická navigácia - z prvého na posledný
    pub fn next_process(&mut self) {
        if !self.top_processes.is_empty() {
            let current = self.process_list_state.selected();
            let new_index = current.map_or(0, |i| {
                if i >= self.top_processes.len() - 1 { 0 } else { i + 1 }
            });
            self.process_list_state.select(Some(new_index));
        }
    }
    
    /// Prepne do detailného režimu vybraného procesu
    pub fn enter_detail_mode(&mut self) {
        self.mode = Mode::ProcessDetail;
    }
    
    /// Návrat z detailného režimu do prehľadu
    pub fn exit_detail_mode(&mut self) {
        self.mode = Mode::Overview;
    }
    
    // ========== NAVIGÁCIA V SIETOVÝCH PROCESOCH ==========
    
    /// Posunie výber v zozname sieťových procesov nahor
    pub fn previous_network_process(&mut self) {
        if !self.top_network_processes.is_empty() {
            let current = self.network_process_state.selected();
            let new_index = current.map_or(0, |i| {
                if i == 0 { self.top_network_processes.len() - 1 } else { i - 1 }
            });
            self.network_process_state.select(Some(new_index));
        }
    }
    
    /// Posunie výber v zozname sieťových procesov nadol
    pub fn next_network_process(&mut self) {
        if !self.top_network_processes.is_empty() {
            let current = self.network_process_state.selected();
            let new_index = current.map_or(0, |i| {
                if i >= self.top_network_processes.len() - 1 { 0 } else { i + 1 }
            });
            self.network_process_state.select(Some(new_index));
        }
    }
}