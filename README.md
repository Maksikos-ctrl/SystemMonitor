# 🖥️ System Monitor

## 📋 Prehľad

**System Monitor** je multifunkčná aplikácia na monitorovanie systému vytvorená v jazyku Rust, ktorá poskytuje terminálové používateľské rozhranie (TUI) aj REST API. Aplikácia ponúka monitorovanie systémových zdrojov v reálnom čase s podporou **PostgreSQL** databázy pre ukladanie historických metrík.

---

## 🚀 Funkcie

### Hlavné funkcie

- **Monitorovanie v reálnom čase**: CPU, RAM, disk, sieť a GPU metriky
- **Teplotný monitoring**: Sledovanie teplôt CPU, GPU, základnej dosky a diskov s vizuálnymi upozorneniami
- **Analýza procesov**: Top procesy podľa využitia CPU a sieťovej aktivity
- **Duálne rozhranie**: TUI pre lokálne použitie a REST API pre vzdialený prístup
- **Databázové úložisko**: **PostgreSQL** integrácia pre historické metriky a analýzy
- **Cross-platform**: Podpora pre Windows (s WMI) a Linux

### Režimy rozhrania

1. **🎨 TUI režim**: Interaktívne terminálové rozhranie s farebnými metrikami
2. **🌐 API režim**: RESTful API server s JSON endpointmi
3. **📊 Kombinovaný režim**: Súčasné spustenie TUI aj API servera

---

## 🏗️ Architektúra

### Štruktúra projektu

```
system-monitor/
├── src/
│   ├── main.rs                 # Vstupný bod aplikácie
│   ├── lib.rs                  # Hlavná knižnica
│   ├── cli/                    # Command-line interface
│   │   ├── app.rs              # Hlavná CLI logika (runner, app state)
│   │   └── ui/                 # UI komponenty pre TUI
│   │       ├── mod.rs
│   │       ├── ui_help.rs
│   │       ├── ui_network.rs
│   │       ├── ui_overview.rs
│   │       ├── ui_process.rs
│   │       └── ui_widgets.rs
│   ├── services/               # Služby pre monitoring
│   │   ├── mod.rs
│   │   ├── api_monitor.rs      # Monitor pre API server
│   │   ├── monitor.rs          # Hlavný monitor pre TUI
│   │   └── temperatures.rs     # Monitor teplôt (WMI pre Windows)
│   ├── models/                 # Dátové modely
│   │   ├── mod.rs
│   │   ├── metrics.rs          # Systémové metriky
│   │   └── temperatures.rs     # Teplotné modely a varovania
│   ├── db/                     # Databázové operácie
│   │   ├── mod.rs
│   │   ├── connection.rs       # PostgreSQL pool a inicializácia
│   │   └── queries.rs          # SQL queries pre metriky
│   ├── api/                    # REST API implementácia
│   │   ├── mod.rs
│   │   ├── handlers.rs         # API handlery
│   │   ├── routes.rs           # API routing
│   │   └── state.rs            # Aplikačný state pre API
│   ├── modes/                  # Režimy aplikácie (TUI, API, Menu)
│   │   ├── mod.rs
│   │   ├── api.rs              # Spustenie API módu
│   │   ├── menu.rs             # Interaktívne textové menu
│   │   └── tui.rs              # Spustenie TUI módu
│   └── helpers/                # Pomocné funkcie a validácia
│       ├── mod.rs
│       └── helpers.rs
├── Cargo.toml
├── Cargo.lock
├── .env.example                # Príklad premenných prostredia
├── build.rs
└── README.md
```

### Kľúčové komponenty

| Komponent | Účel | Hlavné súbory |
|-----------|------|---------------|
| **CLI & UI** | Parsovanie argumentov a renderovanie TUI | `cli/app.rs`, `cli/ui/*.rs` |
| **Services** | Zber systémových metrík a teplôt | `services/monitor.rs`, `services/temperatures.rs` |
| **Models** | Dátové štruktúry pre metriky a procesy | `models/metrics.rs` |
| **Database** | **PostgreSQL** spojenie a ukladanie metrík | `db/connection.rs`, `db/queries.rs` |
| **API** | REST API server s endpointmi | `api/routes.rs`, `api/handlers.rs` |
| **Modes** | Spúšťanie rôznych režimov aplikácie | `modes/tui.rs`, `modes/api.rs` |

---

## 🔧 Inštalácia a nastavenie

### Predpoklady

1. **Rust toolchain** (1.70+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **PostgreSQL databáza**:
   ```bash
   # Ubuntu/Debian
   sudo apt install postgresql postgresql-contrib
   sudo systemctl start postgresql

   # Vytvorenie databázy a užívateľa
   sudo -u postgres psql -c "CREATE DATABASE system_monitor;"
   sudo -u postgres psql -c "CREATE USER monitor_user WITH PASSWORD 'strong_password';"
   sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE system_monitor TO monitor_user;"
   ```

### Build zo zdrojového kódu

```bash
# Klonovanie a build
git clone <https://github.com/Maksikos-ctrl/SystemMonitor>
cd system-monitor
cargo build --release

# Binárka bude v: ./target/release/system-monitor
# Na Linuxe: ./target/release/system-monitor
# Na Windows: ./target/release/system-monitor.exe
```

### Konfigurácia

Vytvorte súbor `.env` v koreňovom adresári projektu:

```env
# Povinné: PostgreSQL spojenie
DATABASE_URL=postgres://monitor_user:strong_password@localhost/system_monitor

# Voliteľné: Nastavenia API servera
API_HOST=127.0.0.1
API_PORT=3000
SAVE_METRICS=true  # Povoliť automatické ukladanie metrík každých 60s
```

---

## 📖 Príklady použitia

### Interaktívne menu (predvolený režim)

Spustí menu na výber režimu.

```bash
system-monitor
# alebo na Linuxe: ./system-monitor
```

### TUI režim

Spustí grafické terminálové rozhranie.

```bash
system-monitor tui
```

#### Klávesové skratky v TUI:

| Klávesa | Akcia |
|---------|-------|
| `Q` | Ukončenie aplikácie |
| `H` | Zobrazenie/skrytie obrazovky pomoci |
| `R` | Okamžité obnovenie dát |
| `N` | Prepnutie na sieťový pohľad |
| `Tab` | Prepínanie medzi hlavnými pohľadmi |
| `↑/↓` | Navigácia v zozname procesov |
| `Enter` | Zobrazenie detailov vybraného procesu |
| `Esc` | Návrat späť (z detailov) alebo ukončenie |

### API režim

Spustí REST API server. Metriky sa automaticky ukladajú do DB, ak je `SAVE_METRICS=true`.

```bash
# Predvolené nastavenia (host: 127.0.0.1, port: 3000)
system-monitor api

# Vlastné nastavenia
system-monitor api --host 0.0.0.0 --port 8080
```

---

## 🌐 API referenčný prehľad

Server poskytuje nasledujúce JSON endpointy:

| Endpoint | Metóda | Popis |
|----------|--------|-------|
| `/api/metrics` | GET | Aktuálne systémové metriky vrátane teplôt |
| `/api/processes` | GET | Zoznam top procesov (param. `?limit=10`) |
| `/api/health` | GET | Health check stav servera a DB |
| `/api/gpu` | GET | Informácie o GPU (simulované/odhadované) |
| `/api/history?hours=24` | GET | Historické metriky za posledných N hodín |

### Príklad: Získanie metrík

```bash
curl http://localhost:3000/api/metrics | jq .
```

**Odpoveď:**

```json
{
  "timestamp": "2025-12-18T10:30:00Z",
  "cpu_usage": 45.2,
  "memory_used": 8589934592,
  "memory_total": 17179869184,
  "cpu_temperature": 65.0,
  "gpu_temperature": 70.0,
  "network_sent_kbps": 1250.5,
  "network_recv_kbps": 3450.2,
  "process_count": 156
}
```

---

## 🗄️ Databázová schéma (PostgreSQL)

Aplikácia automaticky vytvorí potrebné tabuľky pri prvom spojení.

### Hlavná tabuľka `system_metrics`:

```sql
CREATE TABLE IF NOT EXISTS system_metrics (
    id BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    cpu_usage DOUBLE PRECISION NOT NULL,
    memory_total BIGINT NOT NULL,
    memory_used BIGINT NOT NULL,
    -- ... (ďalšie polia podľa modelu SystemMetrics)
    cpu_temperature DOUBLE PRECISION,
    gpu_temperature DOUBLE PRECISION,
    motherboard_temperature DOUBLE PRECISION,
    disk_temperature DOUBLE PRECISION,
    max_temperature DOUBLE PRECISION
);
```

Vytvoria sa aj indexy pre rýchle vyhľadávanie podľa času (`idx_metrics_timestamp`) a GPU (`idx_metrics_gpu`).

---

## 🌡️ Teplotný monitoring

Aplikácia sa snaží získať čo najpresnejšie teploty, s fallback mechanizmom.

| Zdroj teplôt | Platforma | Popis |
|--------------|-----------|-------|
| **WMI (Windows)** | Windows | Priamy dotaz na systémové senzory |
| **SysFS (Linux)** | Linux | Čítanie z `/sys/class/thermal/` |
| **Odhad (Fallback)** | Všetky | Odhad na základe aktuálneho zaťaženia CPU |

### Úrovne varovaní

Vizuálne indikované farbou a ikonou:

- **Normálna** (< 65°C): 🟢
- **Stredná** (65-75°C): 🟡
- **Vysoká** (75-85°C): 🟠
- **Kritická** (> 85°C): 🔴

---

## 🚀 Rýchly štart

### Možnosť 1: Stiahnutie predkompilovanej binárky

Pre jednoduchšie testovanie môžete použiť priamo skompilované súbory.

**Windows (x64):**
- Stiahnuť `system-monitor`[📥system-monitor.exe](https://drive.google.com/file/d/1bQvI8uQ8mqYtOfsQ3YPLvQl7l6IcHD9C/view?usp=sharing)

<!-- **🐧 Linux (x64):**
- 📥 Stiahnuť `system-monitor` -->

Po stiahnutí:

```bash
# Windows (v PowerShell alebo CMD)
.\system-monitor.exe --help

# Linux / macOS (v termináli)
# 1. Udeľte súboru práva na spustenie:
chmod +x system-monitor
# 2. Spustite aplikáciu:
./system-monitor tui
```

### Možnosť 2: Build a spustenie zo zdrojov

Toto je preferovaný spôsob pre vývoj a plnú funkcionalitu.

```bash
# 1. Klonovanie a build
git clone https://github.com/Maksikos-ctrl/system-monitor.git
cd system-monitor
cargo build --release

# 2. Nastavenie databázy (pozri vyššie "Predpoklady") a .env súboru

# 3. Spustenie v požadovanom režime
# TUI režim:
./target/release/system-monitor tui
# API režim:
./target/release/system-monitor api --port 3000

# 4. Overenie funkčnosti API
curl http://localhost:3000/api/health
```

---

## 📦 Použité knižnice (Dependencies)

| Kategória | Knižnica | Použitie v projekte |
|-----------|----------|---------------------|
| **Async Runtime** | `tokio` | Asynchrónny runtime pre API server a DB operácie |
| **Web Framework** | `axum` | Jednoduchý a výkonný framework pre REST API |
| **Databáza** | `sqlx` | Asynchrónny, type-safe PostgreSQL driver |
| **TUI Framework** | `ratatui` | Moderné knižnica pre vytvorenie terminálového UI |
| **Systémové info** | `sysinfo` | Získavanie metrík CPU, pamäte, procesov, diskov |
| **WMI (Windows)** | `wmi` | Monitorovanie teplôt na Windows |
| **CLI Parsing** | `clap` | Parsovanie argumentov príkazového riadku |
| **Konfigurácia** | `dotenv` | Načítanie premenných prostredia z `.env` súboru |

Úplný zoznam nájdete v súbore `Cargo.toml`.

---

## 📞 Kontakt a podpora

- 👨‍💻 **Autor**: Maksym Chernikov
- 📧 **Email**: maksikos973@gmail.com
- 📚 **Predmet**: Jazyk Rust
- 🏫 **Vysoká škola**: FRI UNIZA
- 📅 **Akademický rok**: 2025/2026

---

## ⭐ Semestrálna práca - Jazyk Rust 2025

Built with ❤️ using Rust 🦀