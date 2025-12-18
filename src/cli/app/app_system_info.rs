/// Komplexné informácie o systéme
/// Obsahuje všetky statické informácie, ktoré sa nemenia počas behu aplikácie
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub cpu_name: String,      // Model a frekvencia procesora
    pub gpu_name: String,      // Model grafickej karty
    pub ram_total_gb: u64,     // Celková RAM v GB
    pub disk_total_gb: u64,    // Celková kapacita disku v GB
    pub os_name: String,       // Názov a verzia OS
    pub hostname: String,      // Názov počítača v sieti
}

/// Hlavná funkcia pre získanie všetkých systémových informácií
/// Získava informácie z rôznych zdrojov podľa platformy
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        cpu_name: get_cpu_name(),
        gpu_name: get_gpu_name(),
        ram_total_gb: get_total_ram_gb(),
        disk_total_gb: get_total_ram_gb(),  // TODO: Opraviť na get_total_disk_gb()
        os_name: get_os_name(),
        hostname: get_hostname(),
    }
}

// ==================== FUNKCIE PRE ZÍSKANIE ŠPECIFICKÝCH INFORMÁCIÍ ====================

/// Získa názov procesora s podporou pre rôzne platformy
fn get_cpu_name() -> String {
    // Platformovo špecifická implementácia
    
    #[cfg(target_os = "linux")]
    {
        // Linux: Čítanie z /proc/cpuinfo
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("model name") {
                    return line.split(':').nth(1).unwrap_or("Unknown CPU").trim().to_string();
                }
            }
        }
        "Unknown CPU".to_string()
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // Rôzne príkazy pre získanie CPU informácií vo Windows
        let commands = vec![
            ("wmic", vec!["cpu", "get", "name", "/format:list"]),
            ("powershell", vec!["-Command", "Get-WmiObject Win32_Processor | Select-Object -ExpandProperty Name"]),
            ("cmd", vec!["/C", "wmic cpu get name"]),
        ];

        // Skúšanie príkazov v poradí
        for (cmd, args) in commands {
            if let Ok(output) = Command::new(cmd).args(&args).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.lines().collect();

                // Parsovanie výstupu
                for line in lines {
                    if line.contains("Name=") {
                        if let Some(name) = line.split('=').nth(1) {
                            let trimmed = name.trim();
                            if !trimmed.is_empty() {
                                return trimmed.to_string();
                            }
                        }
                    } else if !line.is_empty() && !line.contains("Name") && !line.contains("wmic") {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            return trimmed.to_string();
                        }
                    }
                }
            }
        }

        // Fallback pomocou sysinfo knižnice
        let sys = sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::new().with_cpu(sysinfo::CpuRefreshKind::everything())
        );
        for cpu in sys.cpus() {
            let name = cpu.brand().to_string();
            if !name.is_empty() {
                return name;
            }
        }

        "Intel/AMD CPU".to_string()  // Generický fallback
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS: Použitie sysctl
        if let Ok(output) = std::process::Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
        {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !name.is_empty() {
                return name;
            }
        }
        "Apple Silicon".to_string()  // Generický názov pre Apple procesory
    }
}

/// Získa názov grafickej karty
fn get_gpu_name() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // Príkazy pre získanie GPU informácií vo Windows
        let commands = vec![
            ("wmic", vec!["path", "win32_videocontroller", "get", "name", "/format:list"]),
            ("powershell", vec!["-Command", "Get-WmiObject Win32_VideoController | Select-Object -ExpandProperty Name"]),
        ];

        for (cmd, args) in commands {
            if let Ok(output) = Command::new(cmd).args(&args).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.lines().collect();

                for line in lines {
                    if line.contains("Name=") {
                        if let Some(name) = line.split('=').nth(1) {
                            let trimmed = name.trim();
                            if !trimmed.is_empty() {
                                return trimmed.to_string();
                            }
                        }
                    } else if !line.is_empty() && !line.contains("Name") {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            return trimmed.to_string();
                        }
                    }
                }
            }
        }
    }

    // Unix-like systémy (Linux, macOS)
    #[cfg(target_family = "unix")]
    {
        // Použitie lspci na získanie informácií o GPU
        if let Ok(output) = std::process::Command::new("lspci")
            .arg("-v")
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                // Hľadanie GPU v výstupe lspci
                if line.contains("VGA compatible controller") || line.contains("3D controller") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() > 2 {
                        return parts[2].trim().to_string();
                    }
                }
            }
        }
    }

    "Graphics Card".to_string()  // Generický fallback
}

/// Získa celkové množstvo RAM v GB
fn get_total_ram_gb() -> u64 {
    // Platformovo špecifická implementácia
    
    #[cfg(target_os = "linux")]
    {
        // Linux: Použitie sys_info knižnice
        if let Ok(meminfo) = sys_info::mem_info() {
            return (meminfo.total / (1024 * 1024)) as u64;  // Konverzia z KB na GB
        }
        return 16;  // Predvolená hodnota
    }
    
    #[cfg(target_os = "windows")]
    {
        // Windows: Použitie sysinfo knižnice
        let sys = sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::new().with_memory(sysinfo::MemoryRefreshKind::everything())
        );
        return (sys.total_memory() / (1024 * 1024 * 1024)) as u64;  // Konverzia z B na GB
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS: Použitie sysctl
        if let Ok(output) = std::process::Command::new("sysctl")
            .arg("-n")
            .arg("hw.memsize")
            .output()
        {
            if let Ok(size_str) = String::from_utf8(output.stdout) {
                if let Ok(size_bytes) = size_str.trim().parse::<u64>() {
                    return size_bytes / (1024 * 1024 * 1024);  // Konverzia z B na GB
                }
            }
        }
        return 16;  // Predvolená hodnota
    }
    
    // Ostatné platformy
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        16  // Predvolená hodnota
    }
}

/// Získa názov operačného systému
fn get_os_name() -> String {
    #[cfg(target_os = "linux")]
    {
        // Linux: Čítanie z /etc/os-release
        if let Ok(release) = std::fs::read_to_string("/etc/os-release") {
            for line in release.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return line.split('=').nth(1).unwrap_or("Linux")
                        .trim_matches('"')
                        .to_string();
                }
            }
        }
        "Linux".to_string()
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // Windows: Použitie wmic
        if let Ok(output) = Command::new("cmd")
            .args(["/C", "wmic os get caption"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            for line in lines {
                if !line.is_empty() && !line.contains("Caption") {
                    return line.trim().to_string();
                }
            }
        }

        "Windows".to_string()  // Fallback
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS: Použitie sw_vers
        if let Ok(output) = std::process::Command::new("sw_vers")
            .arg("-productName")
            .output()
        {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !name.is_empty() {
                return name;
            }
        }
        "macOS".to_string()  // Fallback
    }
}

/// Získa hostname počítača
fn get_hostname() -> String {
    // Použitie hostname knižnice s ošetrením chýb
    hostname::get()
        .unwrap_or_else(|_| "localhost".into())
        .to_string_lossy()
        .to_string()
}