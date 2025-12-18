// Importy knižníc pre prácu s terminálom a TUI
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::io;
use std::sync::{Arc, Mutex};
use crate::services::monitor::SystemMonitor;
use super::{app::{TuiApp, Mode}, ui, Result};

/// Hlavná funkcia pre spustenie TUI aplikácie
/// Inicializuje terminál, spustí hlavnú slučku a spravuje životný cyklus aplikácie
///
/// # Argumenty
/// * `monitor` - Inštancia systémového monitora pre získavanie dát
///
/// # Návratová hodnota
/// * `Result<()>` - Úspech alebo chyba počas behu aplikácie
///
/// # Chybové stavy
/// * Chyby pri inicializácii terminálu (raw mode, alternate screen)
/// * Chyby pri čítaní vstupu z klávesnice
/// * Chyby pri renderingu UI
pub fn run_tui(monitor: SystemMonitor) -> Result<()> {
    // ========== INICIALIZÁCIA TERMINÁLU ==========
    // Povolenie raw módu - priamy prístup k terminálu bez buffrovania
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    // Prechod do alternatívneho screenu (celoobrazovkový režim)
    execute!(stdout, EnterAlternateScreen)?;
    
    // Vytvorenie backendu pre Ratatui
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ========== INICIALIZÁCIA APLIKÁCIE ==========
    // Zdieľaná inštancia monitora (pre viacvláknový prístup)
    let monitor_arc = Arc::new(Mutex::new(monitor));
    // Hlavná aplikácia
    let mut app = TuiApp::new(Arc::clone(&monitor_arc));
    
    // Prvá aktualizácia dát
    app.update();
    
    // Časovač pre pravidelné aktualizácie
    let mut last_tick = std::time::Instant::now();
    let tick_rate = std::time::Duration::from_millis(1000);  // Aktualizácia každú sekundu

    // ========== HLAVNÁ SLOČKA APLIKÁCIE ==========
    loop {
        // Výpočet timeoutu pre poll udalostí
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        // Čítanie vstupu z klávesnice s timeoutom
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Ignorovanie opakovaných stlačení (len prvý press)
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                
                // Smerovanie kláves podľa aktuálneho režimu
                match app.mode {
                    Mode::Overview => handle_overview_keys(&mut app, key.code),
                    Mode::NetworkView => handle_network_keys(&mut app, key.code),
                    Mode::ProcessDetail => handle_process_detail_keys(&mut app, key.code),
                    Mode::Help => handle_help_keys(&mut app, key.code),
                }
            }
        }

        // Pravidelná aktualizácia dát (každú sekundu)
        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = std::time::Instant::now();
        }

        // Renderovanie UI
        terminal.draw(|f| ui::render(f, &mut app))?;
        
        // Kontrola ukončenia aplikácie
        if app.should_quit {
            break;
        }
    }

    // ========== UKONČENIE A ČIŠTENIE ==========
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

// ==================== OBSLUHA KLÁVES PRE JEDNOTLIVÉ REŽIMY ====================

/// Spracovanie klávesových vstupov v režime prehľadu (Overview)
///
/// # Argumenty
/// * `app` - Referencia na aplikáciu
/// * `key_code` - Stlačený kláves
fn handle_overview_keys(app: &mut TuiApp, key_code: KeyCode) {
    match key_code {
        // ========== VŠEOBECNÉ KLAVESY ==========
        // Ukončenie aplikácie
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            app.quit();
        }
        // Prechod na pomocníka
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.mode = Mode::Help;
        }
        // Ručná aktualizácia dát
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.refresh();
        }
        // Prechod do sieťového režimu
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.enter_network_mode();
        }
        
        // ========== NAVIGÁCIA V PROCESOCH ==========
        // Pohyb nahor v zozname procesov
        KeyCode::Up => {
            app.previous_process();
        }
        // Pohyb nadol v zozname procesov
        KeyCode::Down => {
            app.next_process();
        }
        // Vstup do detailu vybraného procesu
        KeyCode::Enter => {
            app.enter_detail_mode();
        }
        // Rýchly prechod do sieťového režimu (Tab)
        KeyCode::Tab => {
            app.enter_network_mode();
        }
        
        // Ignorovanie ostatných klávesov
        _ => {}
    }
}

/// Spracovanie klávesových vstupov v sieťovom režime (NetworkView)
///
/// # Argumenty
/// * `app` - Referencia na aplikáciu
/// * `key_code` - Stlačený kláves
fn handle_network_keys(app: &mut TuiApp, key_code: KeyCode) {
    match key_code {
        // ========== UKONČENIE DETAILNÉHO ZOBRAZENIA ==========
        // Esc v detailnom zobrazení procesu - návrat do zoznamu
        KeyCode::Esc if app.network_mode_detail.is_some() => {
            app.network_mode_detail = None;
        }
        
        // ========== VŠEOBECNÉ KLAVESY ==========
        // Návrat do prehľadového režimu
        KeyCode::Esc => {
            app.mode = Mode::Overview;
        }
        // Ukončenie aplikácie
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
        }
        // Ručná aktualizácia dát
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.refresh();
        }
        
        // ========== NAVIGÁCIA V SIEŤOVÝCH PROCESOCH ==========
        // Pohyb nahor v zozname sieťových procesov
        KeyCode::Up => {
            app.previous_network_process();
        }
        // Pohyb nadol v zozname sieťových procesov
        KeyCode::Down => {
            app.next_network_process();
        }
        // Vstup do detailu vybraného sieťového procesu
        KeyCode::Enter => {
            if let Some(selected) = app.network_process_state.selected() {
                if let Some(process) = app.top_network_processes.get(selected) {
                    app.network_mode_detail = Some(process.name.clone());
                }
            }
        }
        
        // ========== PREPÍNANIE MEDZI REŽIMAMI ==========
        // Prepnutie do prehľadového režimu (Tab)
        KeyCode::Tab => {
            app.mode = Mode::Overview;
        }
        // Prepnutie do pomocníka
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.mode = Mode::Help;
        }
        
        // Ignorovanie ostatných klávesov
        _ => {}
    }
}

/// Spracovanie klávesových vstupov v detailnom zobrazení procesu
///
/// # Argumenty
/// * `app` - Referencia na aplikáciu
/// * `key_code` - Stlačený kláves
fn handle_process_detail_keys(app: &mut TuiApp, key_code: KeyCode) {
    match key_code {
        // Návrat z detailu do zoznamu procesov
        KeyCode::Esc => {
            app.exit_detail_mode();
        }
        // Ukončenie aplikácie aj z detailného zobrazenia
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
        }
        // Ignorovanie ostatných klávesov
        _ => {}
    }
}

/// Spracovanie klávesových vstupov v režime pomocníka (Help)
///
/// # Argumenty
/// * `app` - Referencia na aplikáciu
/// * `key_code` - Stlačený kláves
fn handle_help_keys(app: &mut TuiApp, key_code: KeyCode) {
    match key_code {
        // Návrat z pomocníka do prehľadového režimu
        KeyCode::Esc | KeyCode::Char('h') | KeyCode::Char('H') => {
            app.mode = Mode::Overview;
        }
        // Ukončenie aplikácie aj z pomocníka
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
        }
        // Ignorovanie ostatných klávesov
        _ => {}
    }
}