use ratatui::{
    Frame,
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph, Table, Row, Cell, BorderType, Gauge, Sparkline},
    text::{Line, Span},
};
use crate::cli::app::TuiApp;
use super::ui_widgets::{truncate_str, get_process_bar};

/// Hlavná render funkcia pre prehľadový pohľad systému
/// Zobrazuje systémové metriky a zoznam procesov
pub fn render(f: &mut Frame, app: &mut TuiApp) {
    let area = f.area();

    // Rozdelenie obrazovky na časti
    let chunks = Layout::default()
        .direction(Direction::Vertical)          // Vertikálne usporiadanie
        .margin(1)                               // Okraj 1 znak
        .constraints([
            Constraint::Length(3),   // Titulok
            Constraint::Length(19),  // Metriky (zväčšené pre teploty)
            Constraint::Min(12),     // Procesy
            Constraint::Length(3),   // Päta
        ])
        .split(area);

    render_title(f, app, chunks[0]);           // Vykreslenie titulku
    render_system_metrics(f, app, chunks[1]);  // Vykreslenie systémových metrík
    render_process_list(f, app, chunks[2]);    // Vykreslenie zoznamu procesov
    render_footer(f, chunks[3]);               // Vykreslenie päty
}

/// Vykreslenie titulku s informáciami o systéme
fn render_title(f: &mut Frame, app: &TuiApp, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)                           // Všetky okraje
        .border_type(BorderType::Rounded)                // Okrúhle rohy
        .border_style(Style::default().fg(Color::LightBlue)); // Svetlomodrá farba okrajov

    // Vytvorenie titulkového obsahu
    let title_content = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("🖥️  SYSTEM MONITOR ", Style::default()
                .fg(Color::Cyan)                      // Tyrkysový text
                .add_modifier(Modifier::BOLD)),       // Tučné písmo
            Span::styled(format!("| {} @ {}", app.system_info.hostname, app.system_info.os_name),
                Style::default().fg(Color::DarkGray)), // Šedý text
        ]),
        Line::from(vec![
            Span::styled("CPU: ", Style::default().fg(Color::Yellow)), // Žltý "CPU:"
            Span::styled(truncate_str(&app.system_info.cpu_name, 40), Style::default().fg(Color::White)), // Biely názov CPU
        ]),
        Line::from(vec![
            Span::styled("GPU: ", Style::default().fg(Color::Magenta)), // Fialový "GPU:"
            Span::styled(truncate_str(&app.system_info.gpu_name, 40), Style::default().fg(Color::White)), // Biely názov GPU
        ]),
    ])
    .block(title_block);  // Pridanie bloku

    f.render_widget(title_content, area);
}

/// Vykreslenie systémových metrík (CPU, RAM, DISK, GPU, teploty)
fn render_system_metrics(f: &mut Frame, app: &mut TuiApp, area: Rect) {
    // Rozdelenie oblasti metrík na podoblasti
    let metric_chunks = Layout::default()
        .direction(Direction::Vertical)  // Vertikálne usporiadanie
        .margin(1)                       // Okraj
        .constraints([
            Constraint::Length(4),  // CPU
            Constraint::Length(4),  // RAM
            Constraint::Length(4),  // DISK
            Constraint::Length(4),  // GPU
            Constraint::Length(3),  // Teploty (NOVÝ RIADOK)
        ])
        .split(area);

    // Získanie metrík aplikácie
    let m = app.metrics.as_ref();
    
    // Výpočet percentuálneho využitia CPU
    let cpu_usage = m.map_or(0.0, |m| m.cpu_usage);
    
    // Výpočet percentuálneho využitia RAM
    let ram_percent = m.map_or(0.0, |m| (m.memory_used as f64 / m.memory_total as f64) * 100.0);
    
    // Výpočet percentuálneho využitia disku
    let disk_percent = if let Some(m) = m {
        if m.disk_total > 0 {
            (m.disk_used as f64 / m.disk_total as f64) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };
    
    // Získanie využitia GPU
    let gpu_percent = app.gpu_info.as_ref().map_or(0.0, |g| g.usage);

    // CPU s teplotou
    let cpu_temp = m.and_then(|m| m.cpu_temperature).unwrap_or(0.0);  // Teplota CPU
    render_metric_with_chart(
        f, metric_chunks[0],              // Plocha
        "CPU", cpu_usage, &app.cpu_history,  // Názov, hodnota, história
        get_temp_color(cpu_temp),          // Farba podľa teploty
        &format!("{:.0}°C", cpu_temp)     // Dodatočné info
    );

    // RAM
    let ram_used_gb = m.map_or(0.0, |m| m.memory_used as f64 / 1024.0 / 1024.0 / 1024.0);    // Použitá RAM v GB
    let ram_total_gb = m.map_or(0.0, |m| m.memory_total as f64 / 1024.0 / 1024.0 / 1024.0);  // Celková RAM v GB
    render_metric_with_chart(
        f, metric_chunks[1],              // Plocha
        "RAM", ram_percent, &app.ram_history,  // Názov, hodnota, história
        Color::Green,                     // Zelená farba
        &format!("{:.1}/{:.1}GB", ram_used_gb, ram_total_gb)  // Info o pamäti
    );

    // DISK s teplotou
    let disk_used_gb = m.map_or(0.0, |m| m.disk_used as f64 / 1024.0 / 1024.0 / 1024.0);    // Použitý disk v GB
    let disk_total_gb = m.map_or(0.0, |m| m.disk_total as f64 / 1024.0 / 1024.0 / 1024.0);  // Celkový disk v GB
    let disk_temp = m.and_then(|m| m.disk_temperature).unwrap_or(0.0);  // Teplota disku
    render_metric_with_chart(
        f, metric_chunks[2],              // Plocha
        "DISK", disk_percent, &app.disk_history,  // Názov, hodnota, história
        get_temp_color(disk_temp),        // Farba podľa teploty
        &format!("{:.1}/{:.1}GB | {:.0}°C", disk_used_gb, disk_total_gb, disk_temp)  // Info o disku a teplote
    );

    // GPU s teplotou
    if let Some(gpu) = &app.gpu_info {
        let gpu_mem_used_gb = gpu.memory_used as f64 / 1024.0 / 1024.0 / 1024.0;    // Použitá GPU pamäť v GB
        let gpu_mem_total_gb = gpu.memory_total as f64 / 1024.0 / 1024.0 / 1024.0;  // Celková GPU pamäť v GB
        let gpu_temp = gpu.temperature.unwrap_or(0.0);  // Teplota GPU
        
        render_metric_with_chart(
            f, metric_chunks[3],              // Plocha
            "GPU", gpu_percent, &app.gpu_history,  // Názov, hodnota, história
            get_temp_color(gpu_temp),         // Farba podľa teploty
            &format!("{:.1}/{:.1}GB | {:.0}°C", gpu_mem_used_gb, gpu_mem_total_gb, gpu_temp)  // Info o GPU
        );
    }

    // Zobrazenie dodatočných teplôt
    if let Some(m) = m {
        let mb_temp = m.motherboard_temperature.unwrap_or(0.0);  // Teplota základnej dosky
        let max_temp = m.max_temperature.unwrap_or(0.0);         // Maximálna teplota
        
        render_temperature_summary(f, metric_chunks[4], mb_temp, max_temp);  // Zobrazenie súhrnu teplôt
    }
}

/// NOVÁ FUNKCIA: Widget metriky s grafom
/// Vytvára kombináciu grafu a gauge s históriou
fn render_metric_with_chart(
    f: &mut Frame,
    area: Rect,
    label: &str,
    value: f64,
    history: &[u64],
    color: Color,
    extra_info: &str
) {
    // Rozdelenie oblasti na popisok a graf
    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)  // Horizontálne usporiadanie
        .constraints([
            Constraint::Length(12),  // Popisok a gauge
            Constraint::Min(10),     // Graf histórie
        ])
        .split(area);

    // Vytvorenie popisku s percentami
    let label_text = format!("{}: {:.0}%", label, value);
    
    // Vytvorenie gauge (ukazovateľa)
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(Color::DarkGray))  // Farba na šedom pozadí
        .label(extra_info)              // Dodatočné informácie
        .percent(value.round() as u16); // Percentuálne vyplnenie

    // Blok pre gauge
    let gauge_block = Block::default()
        .title(label_text)                     // Titulok s názvom metriky
        .borders(Borders::ALL)                 // Všetky okraje
        .border_type(BorderType::Plain);       // Jednoduché okraje

    f.render_widget(gauge.block(gauge_block), inner_chunks[0]);  // Vykreslenie gauge

    // Kontrola, či existuje história
    if !history.is_empty() {
        // Vytvorenie sparkline grafu (mini grafu)
        let sparkline = Sparkline::default()
            .data(history)                            // Dáta histórie
            .max(100)                                 // Maximálna hodnota
            .style(Style::default().fg(color))        // Farba grafu
            .bar_set(ratatui::symbols::bar::NINE_LEVELS);  // Štyl stĺpcov

        // Blok pre graf
        let chart_block = Block::default()
            .title("History")                         // Titulok "History"
            .borders(Borders::ALL)                    // Všetky okraje
            .border_type(BorderType::Plain);          // Jednoduché okraje

        f.render_widget(sparkline.block(chart_block), inner_chunks[1]);  // Vykreslenie grafu
    }
}

/// NOVÁ FUNKCIA: Súhrn teplôt
/// Zobrazuje teplotu základnej dosky a maximálnu teplotu
fn render_temperature_summary(f: &mut Frame, area: Rect, mb_temp: f64, max_temp: f64) {
    // Rozdelenie oblasti na dve časti
    let temp_chunks = Layout::default()
        .direction(Direction::Horizontal)  // Horizontálne usporiadanie
        .constraints([
            Constraint::Percentage(50),  // Základná doska
            Constraint::Percentage(50),  // Maximálna teplota
        ])
        .split(area);

    // Základná doska
    let mb_block = Block::default()
        .title("Motherboard")                                 // Titulok "Motherboard"
        .borders(Borders::ALL)                               // Všetky okraje
        .border_type(BorderType::Plain)                      // Jednoduché okraje
        .border_style(Style::default().fg(get_temp_color(mb_temp)));  // Farba okrajov podľa teploty
    
    let mb_content = Paragraph::new(format!("{} {:.0}°C", get_temp_icon(mb_temp), mb_temp))
        .style(Style::default().fg(get_temp_color(mb_temp)))  // Farba textu podľa teploty
        .block(mb_block)                                      // Pridanie bloku
        .alignment(ratatui::layout::Alignment::Center);       // Zarovnanie na stred

    // Maximálna teplota
    let max_block = Block::default()
        .title("Max Temperature")                               // Titulok "Max Temperature"
        .borders(Borders::ALL)                                 // Všetky okraje
        .border_type(BorderType::Plain)                        // Jednoduché okraje
        .border_style(Style::default().fg(get_temp_color(max_temp)));  // Farba okrajov podľa teploty
    
    let max_content = Paragraph::new(format!("{} {:.0}°C", get_temp_icon(max_temp), max_temp))
        .style(Style::default().fg(get_temp_color(max_temp)))  // Farba textu podľa teploty
        .block(max_block)                                      // Pridanie bloku
        .alignment(ratatui::layout::Alignment::Center);        // Zarovnanie na stred

    f.render_widget(mb_content, temp_chunks[0]);  // Vykreslenie teploty základnej dosky
    f.render_widget(max_content, temp_chunks[1]); // Vykreslenie maximálnej teploty
}

/// Pomocné funkcie pre teploty

/// Určenie farby podľa teploty
fn get_temp_color(temp: f64) -> Color {
    match temp {
        t if t < 50.0 => Color::Green,     // Zelená - bezpečná teplota
        t if t < 70.0 => Color::Yellow,    // Žltá - stredná teplota
        t if t < 85.0 => Color::Red,       // Červená - vysoká teplota
        _ => Color::Magenta,               // Fialová - kritická teplota
    }
}

/// Určenie ikony podľa teploty
fn get_temp_icon(temp: f64) -> &'static str {
    match temp {
        t if t < 50.0 => "🟢",  // Zelený kruh - bezpečná
        t if t < 70.0 => "🟡",  // Žltý kruh - varovanie
        t if t < 85.0 => "🔴",  // Červený kruh - nebezpečná
        _ => "🔥",              // Oheň - kritická
    }
}

/// Vykreslenie zoznamu procesov
fn render_process_list(f: &mut Frame, app: &mut TuiApp, area: Rect) {
    let block = Block::default()
        .title("🔥 Top Processes")                // Titulok s emodži
        .borders(Borders::ALL)                   // Všetky okraje
        .border_type(BorderType::Rounded)        // Okrúhle rohy
        .border_style(Style::default().fg(Color::Yellow));  // Žltá farba okrajov

    let inner_area = block.inner(area);          // Vnútorná plocha bloku
    f.render_widget(block, area);                // Vykreslenie bloku

    // Kontrola prázdneho zoznamu procesov
    if app.top_processes.is_empty() {
        let no_processes = Paragraph::new("No processes found")  // Správa "Žiadne procesy"
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(no_processes, inner_area);
        return;
    }

    // Vytvorenie riadkov tabuľky procesov
    let rows: Vec<Row> = app.top_processes
        .iter()
        .enumerate()
        .map(|(i, proc)| {
            // Kontrola výberu riadku
            let is_selected = app.process_list_state.selected() == Some(i);
            let style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::Yellow)  // Žltý text na šedom pozadí
            } else {
                Style::default()
            };

            // Vytvorenie riadku s informáciami o procese
            Row::new(vec![
                Cell::from(format!("{:3}", i + 1)).style(style),  // Poradové číslo
                Cell::from(truncate_str(&proc.name, 20)).style(style),  // Názov procesu (skrátený)
                Cell::from(format!("{:5.1}%", proc.cpu_usage)).style(style),  // Využitie CPU
                Cell::from(format!("{:6.1} MB", proc.memory as f64 / 1024.0 / 1024.0)).style(style),  // Pamäť
                Cell::from(get_process_bar(proc.cpu_usage as u8)).style(style),  // Grafický ukazovateľ
            ])
        })
        .collect();

    // Šírky stĺpcov tabuľky
    let widths = [
        Constraint::Length(4),    // Poradové číslo
        Constraint::Length(22),   // Názov procesu
        Constraint::Length(8),    // CPU
        Constraint::Length(10),   // Pamäť
        Constraint::Min(10),      // Grafický ukazovateľ
    ];

    // Vytvorenie tabuľky
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["#", "Process", "CPU", "Memory", "Usage"])  // Hlavička tabuľky
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))  // Tyrkysová tučná
                .bottom_margin(1),  // Spodný okraj hlavičky
        )
        .column_spacing(1);  // Medzera medzi stĺpcami

    f.render_widget(table, inner_area);  // Vykreslenie tabuľky
}

/// Vykreslenie päty s klávesovými skratkami
fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text = vec![
        Line::from(vec![
            Span::styled("[H] ", Style::default().fg(Color::Yellow)),      // Žltý H
            Span::styled("Help", Style::default().fg(Color::DarkGray)),    // Šedá nápoveda
            Span::styled("  [R] ", Style::default().fg(Color::Green)),     // Zelený R
            Span::styled("Refresh", Style::default().fg(Color::DarkGray)), // Šedé obnovenie
            Span::styled("  [Q] ", Style::default().fg(Color::Red)),       // Červený Q
            Span::styled("Quit", Style::default().fg(Color::DarkGray)),    // Šedé ukončenie
            Span::styled("  [N] ", Style::default().fg(Color::Blue)),      // Modrý N
            Span::styled("Network", Style::default().fg(Color::DarkGray)), // Šedá sieť
            Span::styled("  [↑↓] ", Style::default().fg(Color::Cyan)),     // Tyrkysové šípky
            Span::styled("Navigate", Style::default().fg(Color::DarkGray)), // Šedá navigácia
            Span::styled("  [Enter] ", Style::default().fg(Color::Magenta)), // Fialový Enter
            Span::styled("Details", Style::default().fg(Color::DarkGray)), // Šedé detaily
        ])
    ];

    let footer = Paragraph::new(footer_text)
        .block(Block::default()
            .borders(Borders::ALL)                      // Všetky okraje
            .border_type(BorderType::Rounded))          // Okrúhle rohy
        .alignment(ratatui::layout::Alignment::Center); // Zarovnanie na stred

    f.render_widget(footer, area);  // Vykreslenie päty
}