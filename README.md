# 🖥️ System Monitor

## 📋 Prehľad

System Monitor je multifunkčná aplikácia na monitorovanie systému vytvorená v jazyku Rust, ktorá poskytuje terminálové používateľské rozhranie (TUI) aj REST API. Aplikácia ponúka monitorovanie systémových zdrojov v reálnom čase s podporou SQLite databázy pre ukladanie historických metrík.

## 🚀 Funkcie

### Hlavné funkcie

- **Monitorovanie v reálnom čase**: CPU, RAM, disk, sieťové a teplotné metriky
- **Teplotný monitoring**: Sledovanie teplôt CPU, GPU, základnej dosky a diskov s upozorneniami
- **Analýza procesov**: Top procesy podľa využitia CPU a siete
- **Duálne rozhranie**: TUI pre lokálne použitie a REST API pre vzdialený prístup
- **Databázové úložisko**: SQLite integrácia pre historické metriky
- **Asynchrónne operácie**: Tokio runtime pre výkon

### Režimy rozhrania

- 🎨 **TUI režim**: Interaktívne terminálové rozhrainie s farebnými metrikami
- 🌐 **API režim**: RESTful API server s JSON endpointmi
- 📊 **Kombinovaný režim**: TUI + API súčasne

## 🏗️ Architektúra

### Štruktúra projektu

```
system-monitor/
├── src/
│   ├── main.rs              # Vstupný bod aplikácie
│   ├── lib.rs               # Knižnica
│   ├── cli/                 # Command-line interface
│   │   ├── app/             # TUI aplikačná logika
│   │   │   ├── app_staters.rs
│   │   │   └── app_system_info.rs
│   │   └── ui/              # UI komponenty
│   │       ├── ui_help.rs
│   │       ├── ui_network.rs
│   │       ├── ui_overview.rs
│   │       ├── ui_process.rs
│   │       └── ui_widgets.rs
│   ├── services/            # Služby pre monitoring
│   │   ├── api_monitor.rs   # Monitor pre API
│   │   └── monitor.rs       # Hlavný monitor pre TUI
│   ├── models/              # Dátové modely
│   │   ├── metrics.rs       # Systémové metriky
│   │   └── temperatures.rs  # Teplotné dáta
│   ├── db/                  # Databázové operácie
│   │   ├── connection.rs    # SQLite pool
│   │   └── queries.rs       # SQL queries
│   ├── api/                 # REST API implementácia
│   │   ├── handlers.rs      # API handlery
│   │   ├── routes.rs        # Routing
│   │   └── staters.rs       # State management
│   ├── config/              # Konfigurácia
│   │   ├── dirs.rs          # Cesty k súborom
│   │   └── helpers.rs       # Pomocné funkcie
│   └── modes/               # Režimy aplikácie
│       ├── api.rs           # API režim
│       ├── menus.rs         # Interaktívne menu
│       └── tui.rs           # TUI režim
├── Cargo.toml
├── Cargo.lock
├── .env                     # Environment variables
├── build.sh                 # Build script
└── README.md
```

### Kľúčové komponenty

#### 1. CLI modul (`src/cli/`)

**Účel**: Parsovanie argumentov príkazového riadku a routing príkazov

**Kľúčové štruktúry**:
- `Cli`: Hlavná CLI štruktúra s podpríkazmi
- `Commands`: Enum s variantmi Tui, Api, Both

**Použitie**:
```bash
system-monitor              # Interaktívne menu
system-monitor tui          # Spustenie TUI rozhrania
system-monitor api          # Spustenie API servera
system-monitor both         # Spustenie oboch režimov
```

#### 2. UI modul (`src/cli/ui/`)

**Účel**: Renderovanie terminálového používateľského rozhrania pomocou ratatui

**Kľúčové komponenty**:
- `ui_overview.rs`: Hlavný prehľad so systémovými metrikami
- `ui_network.rs`: Monitoring sieťovej šírky pásma
- `ui_process.rs`: Detailný pohľad na procesy
- `ui_help.rs`: Obrazovka pomoci s klávesovými skratkami
- `ui_widgets.rs`: Znovupoužiteľné UI komponenty

#### 3. Services modul (`src/services/`)

**Účel**: Hlavná funkcionalita monitorovania systému

**Kľúčové komponenty**:
- `monitor.rs`: Hlavný systémový monitor pre TUI (s podporou teplôt)
- `api_monitor.rs`: API-špecifický monitor (lightweight, pre background úlohy)

#### 4. Models modul (`src/models/`)

**Účel**: Dátové štruktúry pre systémové metriky

**Kľúčové štruktúry**:
- `SystemMetrics`: Kompletné systémové metriky vrátane teplôt
- `ProcessInfo`: Informácie o jednotlivých procesoch
- `TemperatureInfo`: Teploty komponentov s úrovňami upozornení
- `GpuInfo`: GPU-špecifické metriky

#### 5. Database modul (`src/db/`)

**Účel**: SQLite databázové operácie

**Kľúčové komponenty**:
- `connection.rs`: Pooling databázových spojení a inicializácia tabuliek
- `queries.rs`: SQL queries pre ukladanie a získavanie metrík

#### 6. API modul (`src/api/`)

**Účel**: Implementácia REST API servera pomocou axum

**Kľúčové komponenty**:
- **Endpointy**:
  - `GET /api/metrics` - Systémové metriky
  - `GET /api/processes` - Top procesy
  - `GET /api/health` - Health check
  - `GET /api/gpu` - GPU informácie
  - `GET /api/history` - Historické dáta
- **Funkcie**: Background ukladanie metrík, pooling spojení, JSON odpovede

## 🔧 Inštalácia a nastavenie

### Predpoklady

```bash
# Rust (1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# SQLite (zvyčajne už nainštalovaný)
# Ubuntu/Debian
sudo apt-get install sqlite3 libsqlite3-dev

# Windows - SQLite je included
```

### Build zo zdrojového kódu

```bash
# Klonovanie repozitára
git clone <repository-url>
cd system-monitor

# Build v release režime
cargo build --release

# Binárka bude v: ./target/release/system-monitor
```

### Konfigurácia

Vytvorte `.env` súbor v koreňovom adresári projektu:

```env
DATABASE_URL=sqlite://./data/metrics.db
API_HOST=127.0.0.1
API_PORT=3000
REFRESH_INTERVAL_MS=2000
```

Alebo použite konfiguračný súbor `config/settings.toml`:

```toml
[api]
host = "127.0.0.1"
port = 3000

[database]
path = "data/metrics.db"
max_connections = 5

[monitoring]
interval_ms = 2000
history_retention_days = 7
```

## 📖 Príklady použitia

### Interaktívny menu režim

```bash
system-monitor
```

Zobrazí interaktívne menu pre výber medzi TUI, API alebo pomocou.

### TUI režim

```bash
system-monitor tui
```

Spustí terminálové rozhranie s nasledujúcimi klávesovými skratkami:

| Klávesa | Akcia |
|---------|-------|
| `Q` | Ukončenie aplikácie |
| `H` | Zobrazenie/skrytie pomoci |
| `R` | Obnovenie dát |
| `N` | Prepnutie na sieťový pohľad |
| `Tab` | Prepínanie medzi pohľadmi |
| `↑/↓` | Navigácia v zozname procesov |
| `Enter` | Zobrazenie detailov procesu |
| `Esc` | Návrat späť/ukončenie |

### API režim

```bash
# Spustenie s predvolenými nastaveniami
system-monitor api

# Spustenie s vlastnými nastaveniami
system-monitor api --host 0.0.0.0 --port 8080
```

### Kombinovaný režim

```bash
system-monitor both
```

Spustí TUI aj API server súčasne.

## 🌐 API endpointy

### GET /api/metrics

Vracia kompletné systémové metriky.

**Príklad odpovede**:
```json
{
  "timestamp": "2025-12-18T10:30:00Z",
  "cpu_usage": 45.2,
  "memory_total": 17179869184,
  "memory_used": 8589934592,
  "memory_available": 8589934592,
  "swap_total": 4294967296,
  "swap_used": 1073741824,
  "cpu_temperature": 65.0,
  "gpu_temperature": 70.0,
  "network_sent_kbps": 1250.5,
  "network_recv_kbps": 3450.2,
  "disk_total": 500000000000,
  "disk_used": 250000000000,
  "process_count": 156,
  "system_uptime": 86400
}
```

### GET /api/processes

Vracia top procesy zoradené podľa kombinovaného využitia CPU a siete.

**Query parametre**:
- `limit`: Počet procesov na vrátenie (predvolené: 10)

**Príklad odpovede**:
```json
[
  {
    "pid": 1234,
    "name": "chrome.exe",
    "cpu_usage": 25.5,
    "memory": 524288000,
    "network_sent": 1048576,
    "network_recv": 2097152
  }
]
```

### GET /api/health

Health check endpoint.

**Odpoveď**:
```json
{
  "status": "healthy",
  "timestamp": "2025-12-18T10:30:00Z",
  "version": "1.0.0"
}
```

### GET /api/history

Vracia historické metriky z databázy.

**Query parametre**:
- `metric`: Typ metriky (cpu, memory, temperature)
- `limit`: Počet záznamov (predvolené: 100)
- `from`: Začiatočný timestamp
- `to`: Koncový timestamp

**Príklad**:
```bash
curl "http://localhost:3000/api/history?metric=cpu&limit=50"
```

## 🗄️ Databázová schéma

### Tabuľka system_metrics

```sql
CREATE TABLE IF NOT EXISTS system_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    cpu_usage REAL NOT NULL,
    memory_total INTEGER NOT NULL,
    memory_used INTEGER NOT NULL,
    memory_available INTEGER NOT NULL,
    swap_total INTEGER NOT NULL,
    swap_used INTEGER NOT NULL,
    disk_total INTEGER NOT NULL,
    disk_used INTEGER NOT NULL,
    disk_available INTEGER NOT NULL,
    
    -- GPU metriky
    gpu_name TEXT,
    gpu_usage REAL,
    gpu_memory_total INTEGER,
    gpu_memory_used INTEGER,
    gpu_temperature REAL,
    
    -- Sieťové štatistiky
    network_sent_kbps REAL,
    network_recv_kbps REAL,
    
    -- Všeobecné informácie
    process_count INTEGER NOT NULL,
    system_uptime INTEGER NOT NULL,
    
    -- Teplotné metriky
    cpu_temperature REAL,
    motherboard_temperature REAL,
    disk_temperature REAL,
    max_temperature REAL
);
```

### Indexy

```sql
CREATE INDEX IF NOT EXISTS idx_metrics_timestamp 
ON system_metrics(timestamp DESC);

CREATE INDEX IF NOT EXISTS idx_metrics_cpu 
ON system_metrics(cpu_usage, timestamp DESC);
```

## 🌡️ Teplotný monitoring

### Úrovne upozornení na teplotu

| Teplota | Úroveň | Indikátor |
|---------|--------|-----------|
| < 65°C | Normálna | 🟢 |
| 65-75°C | Stredná | 🟡 |
| 75-85°C | Vysoká | 🟠 |
| > 85°C | Kritická | 🔴 |

### Zdroje teplôt

- **Windows**: WMI queries pre reálne teploty
- **Linux**: Čítanie z `/sys/class/thermal/` a `/sys/class/hwmon/`
- **Fallback**: Odhadované teploty na základe využitia CPU
- **Simulácia**: Predvolené hodnoty, keď nie sú dostupné senzory

## 📦 Použité knižnice

### Core knižnice

| Knižnica | Verzia | Použitie |
|----------|--------|----------|
| **tokio** | 1.x | Asynchrónny runtime pre všetky async operácie |
| **axum** | 0.7.x | Web framework pre REST API server |
| **sqlx** | 0.7.x | Asynchrónna databáza (SQLite) s type-safe queries |
| **ratatui** | 0.26.x | TUI framework pre terminálové rozhranie |
| **crossterm** | 0.27.x | Cross-platform terminal manipulation |

### Monitoring knižnice

| Knižnica | Použitie |
|----------|----------|
| **sysinfo** | Získavanie CPU, RAM, disk, network metrík |

### Serializácia & Konfigurácia

| Knižnica | Použitie |
|----------|----------|
| **serde** + **serde_json** | JSON serializácia pre API odpovede |
| **toml** | Parsovanie konfiguračných súborov |

### Utility knižnice

| Knižnica | Použitie |
|----------|----------|
| **chrono** | Práca s časom a timestampmi |
| **clap** | Parsovanie CLI argumentov |
| **anyhow** | Ergonomický error handling |
| **thiserror** | Vlastné error typy |
| **tracing** + **tracing-subscriber** | Strukturované logovanie |

## 🛠️ Vývoj

### Pridanie nových metrík

1. Pridajte pole do `SystemMetrics` v `models/metrics.rs`
2. Aktualizujte databázovú schému v `db/connection.rs`
3. Implementujte zber v príslušnej monitor službe
4. Aktualizujte UI komponenty podľa potreby

### Pridanie nových UI pohľadov

1. Vytvorte nový súbor v `ui/` adresári
2. Implementujte `render()` funkciu
3. Pridajte do UI routingu v hlavnej aplikácii
4. Aktualizujte help screen s novými skratkami

### Testovanie

```bash
# Spustenie testov
cargo test

# Testy s výstupom
cargo test -- --nocapture

# Integračné testy
cargo test --test integration

# Špecifický test
cargo test test_monitor_service
```

### Code formátovanie a linting

```bash
# Formátovanie kódu
cargo fmt

# Linting
cargo clippy

# Lint s opravami
cargo clippy --fix
```

## 📊 Výkonové úvahy

### Využitie pamäte

- **TUI režim**: ~10-20 MB
- **API režim**: ~20-30 MB (s background ukladaním)
- **Databázové spojenia**: Pool 5 spojení

### Frekvencia aktualizácie

- **TUI refresh**: Každé 2 sekundy
- **API zber metrík**: On-demand
- **Background ukladanie**: Každých 60 sekúnd (ak povolené)

## 🔐 Bezpečnostné úvahy

### API bezpečnosť

- API je navrhnuté pre použitie v lokálnej sieti
- Nie je implementovaná autentifikácia (určené pre dôveryhodné siete)
- Pre produkčné použitie zvážte pridanie autentifikácie

### Ochrana dát

- Zbierajú sa iba systémové metriky
- Neukládajú sa žiadne osobné údaje alebo informácie o užívateľoch
- Názvy procesov sa zbierajú, ale nie užívateľské dáta

## 📄 Dokumentácia

### Generovanie programátorskej dokumentácie

```bash
# Vygeneruje HTML dokumentáciu
cargo doc --no-deps --open

# S private items
cargo doc --no-deps --document-private-items --open
```

Dokumentácia bude dostupná v `target/doc/system_monitor/index.html`

### Obsah dokumentácie

- 📄 **Zadanie semestrálnej práce** - Kompletné zadanie projektu
- 🏗️ **UML diagramy** - Class diagram, Component diagram, Sequence diagram
- 📖 **Používateľská príručka** - Inštalácia, konfigurácia, ovládanie, funkcionality
- 👨‍💻 **Programátorská príručka** - API dokumentácia, moduly, typy, funkcie
- 📋 **Zoznam knižníc** - Použité dependencies s detailným popisom použitia



## 🚀 Quick Start

### Možnosť 1: Stiahnutie predkompilovaného súboru

**Windows (x64):**
- [📥 Stiahnuť system-monitor.exe](https://github.com/Maksikos-ctrl/system-monitor/target/x86_64-pc-windows-msvc/release/system-monitor.exe)

**Linux (x64):**
- [📥 Stiahnuť system-monitor](https://github.com/Maksikos-ctrl/system-monitor/target/x86_64-pc-windows-msvc/release/system-monitor.exe)

Po stiahnutí:
```bash
# Windows
system-monitor.exe tui

# Linux/macOS
chmod +x system-monitor
./system-monitor tui
```

### Možnosť 2: Build zo zdrojového kódu

```bash
# 1. Klonovanie a build
git clone <repo-url> && cd system-monitor
cargo build --release

# 2. Spustenie TUI
./target/release/system-monitor tui

# 3. Alebo spustenie API
./target/release/system-monitor api --port 3000

# 4. Test API
curl http://localhost:3000/api/metrics
```

## 📞 Kontakt a podpora

**Autor**: [Maksym Chernikov]  
**Študent ID**: [563141]  
**Email**: [maksikos973@gmail.com]  
**Akademický rok**: 2025/2026  
**Predmet**: Jazyk Rust

---

## 📝 Licencia

Tento projekt je vytvorený pre akademické účely v rámci predmetu Jazyk Rust na [FRI UNIZA].

---

⭐ **Semestrálna práca - Jazyk Rust 2025**

*Built with ❤️ using Rust 🦀*