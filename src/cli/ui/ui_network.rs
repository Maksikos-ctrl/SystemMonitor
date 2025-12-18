use ratatui::{
    Frame,
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph, Table, Row, Cell, BorderType, Sparkline, Gauge},
    text::{Line, Span},
};
use crate::cli::app::{TuiApp, Mode, NetworkConnection};
use crate::models::ProcessInfo;

/// Hlavná render funkcia pre sieťový pohľad
/// Rozhoduje medzi prehľadom a detailným pohľadom procesu
pub fn render(f: &mut Frame, app: &mut TuiApp) {
    match app.network_mode_detail {
        Some(ref process_name) if app.mode == Mode::NetworkView => {
            // Ak sme v detailnom pohľade a máme názov procesu, zobrazíme detail
            render_network_process_detail(f, app, process_name);
        }
        _ => {
            // Inak zobrazíme hlavný prehľad
            render_network_overview(f, app);
        }
    }
}

/// Vykreslenie hlavného prehľadu sieťovej aktivity
fn render_network_overview(f: &mut Frame, app: &mut TuiApp) {
    let area = f.area();
    
    // Rozdelenie obrazovky na časti
    let chunks = Layout::default()
        .direction(Direction::Vertical)          // Vertikálne usporiadanie
        .margin(1)                               // Okraj 1 znak
        .constraints([
            Constraint::Length(3),    // Titulok
            Constraint::Length(8),    // Využitie šírky pásma
            Constraint::Length(4),    // Celkové štatistiky
            Constraint::Min(10),      // Tabuľka procesov (minimálne 10 riadkov)
            Constraint::Length(3),    // Päta
        ])
        .split(area);

    // Vykreslenie jednotlivých sekcií
    render_network_title(f, app, chunks[0]);           // Titulok
    render_bandwidth_usage(f, app, chunks[1]);         // Využitie šírky pásma
    render_network_totals(f, app, chunks[2]);         // Celkové štatistiky
    render_network_process_table(f, app, chunks[3]);  // Tabuľka procesov
    render_network_footer(f, chunks[4]);              // Päta
}

/// Vykreslenie grafu využitia šírky pásma
fn render_bandwidth_usage(f: &mut Frame, app: &TuiApp, area: Rect) {
    let block = Block::default()
        .title("📶 Bandwidth Usage")                    // Titulok s emodži
        .borders(Borders::ALL)                          // Všetky okraje
        .border_type(BorderType::Rounded)               // Okrúhle rohy
        .border_style(Style::default().fg(Color::Cyan)); // Tyrkysová farba okrajov
    
    let inner_area = block.inner(area);                 // Vnútorná plocha bloku
    f.render_widget(block, area);                       // Vykreslenie bloku
    
    // Rozdelenie na popisky a grafy
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Popisky
            Constraint::Length(2),  // Grafy
        ])
        .split(inner_area);
    
    // Aktuálne hodnoty odoslaných a prijatých dát
    let current_sent = app.network_sent_history.last().copied().unwrap_or(0.0);
    let current_recv = app.network_recv_history.last().copied().unwrap_or(0.0);
    
    // Historické maximá pre škálovanie
    let max_historical_sent = app.network_sent_history.iter()
        .copied()
        .reduce(f64::max)
        .unwrap_or(1.0);
    let max_historical_recv = app.network_recv_history.iter()
        .copied()
        .reduce(f64::max)
        .unwrap_or(1.0);
    
    // Celkové maximum pre škálovanie
    let max_value = max_historical_sent.max(max_historical_recv).max(100.0).max(current_sent.max(current_recv));
    
    // Popisky s aktuálnymi hodnotami
    let labels = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("↑ Sent: ", Style::default().fg(Color::Red)),          // Červený odoslané
            Span::styled(format!("{:.1} KB/s", current_sent), Style::default().fg(Color::White)),
            Span::raw("   "),                                                   // Medzera
            Span::styled("↓ Received: ", Style::default().fg(Color::Green)),    // Zelené prijaté
            Span::styled(format!("{:.1} KB/s", current_recv), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Max: ", Style::default().fg(Color::Yellow)),         // Žlté maximum
            Span::styled(format!("{:.1} KB/s", max_value), Style::default().fg(Color::White)),
            Span::raw("   "),                                                  // Medzera
            Span::styled("Scale: 0 - ", Style::default().fg(Color::DarkGray)), // Šedé mierka
            Span::styled(format!("{:.0} KB/s", max_value), Style::default().fg(Color::White)),
        ]),
    ]);
    
    f.render_widget(labels, chunks[0]);  // Vykreslenie popiskov
    
    // Rozdelenie na dva grafy (odoslané a prijaté)
    let gauge_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Odoslané
            Constraint::Length(1),  // Prijaté
        ])
        .split(chunks[1]);
    
    // Gauge pre odoslané dáta
    let sent_percent = (current_sent / max_value.max(1.0) * 100.0).min(100.0);
    let sent_gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))          // Bez okrajov
        .gauge_style(Style::default().fg(Color::Red).bg(Color::DarkGray)) // Červený na šedom
        .percent(sent_percent as u16)                           // Percentuálne vyplnenie
        .label(format!("↑ {:.1} KB/s", current_sent));          // Popisok s hodnotou
    
    // Gauge pre prijaté dáta
    let recv_percent = (current_recv / max_value.max(1.0) * 100.0).min(100.0);
    let recv_gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))          // Bez okrajov
        .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray)) // Zelený na šedom
        .percent(recv_percent as u16)                           // Percentuálne vyplnenie
        .label(format!("↓ {:.1} KB/s", current_recv));          // Popisok s hodnotou
    
    // Vykreslenie oboch grafov
    f.render_widget(sent_gauge, gauge_chunks[0]);
    f.render_widget(recv_gauge, gauge_chunks[1]);
}

/// Vykreslenie tabuľky sieťových procesov
fn render_network_process_table(f: &mut Frame, app: &mut TuiApp, area: Rect) {
    let block = Block::default()
        .title("🔥 Top Network Processes")                // Titulok
        .borders(Borders::ALL)                           // Všetky okraje
        .border_type(BorderType::Rounded)                // Okrúhle rohy
        .border_style(Style::default().fg(Color::Yellow)); // Žltá farba okrajov
    
    let inner_area = block.inner(area);                  // Vnútorná plocha
    f.render_widget(block, area);                        // Vykreslenie bloku
    
    // Kontrola prázdnych dát
    if app.top_network_processes.is_empty() {
        let no_data = Paragraph::new("No network data available")
            .alignment(Alignment::Center);
        f.render_widget(no_data, inner_area);
        return;
    }
    
    // Validácia dát - kontrola identických hodnôt (môže indikovať bug)
    let first_sent = app.top_network_processes.first()
        .and_then(|p| p.network_sent)
        .unwrap_or(0);
    
    let all_same = app.top_network_processes.iter()
        .all(|p| p.network_sent == Some(first_sent));
    
    if all_same && first_sent > 100_000_000 {  // Ak sú všetky hodnoty identické a vysoké
        // Zobrazenie chybového hlásenia
        let error_msg = Paragraph::new(vec![
            Line::from("⚠️  DATA VALIDATION ERROR"),
            Line::from(""),
            Line::from("All processes show identical network values"),
            Line::from(format!("Value: {} bytes/s", first_sent)),
            Line::from(""),
            Line::from("This indicates a bug in data collection."),
            Line::from("Showing fallback process list..."),
        ]).alignment(Alignment::Center);
        
        f.render_widget(error_msg, inner_area);
        return;
    }
    
    // Vytvorenie riadkov tabuľky
    let rows: Vec<Row> = app.top_network_processes
        .iter()
        .enumerate()
        .map(|(i, proc)| {
            // Kontrola výberu riadku
            let is_selected = app.network_process_state.selected() == Some(i);
            let base_style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::Yellow)  // Žltý text na šedom pozadí
            } else {
                Style::default()
            };
            
            // Farba podľa typu procesu
            let process_color = get_process_color(&proc.name);
            let name_style = base_style.fg(process_color);
            
            // Ikona podľa typu procesu
            let process_icon = get_process_icon(&proc.name);
            let process_name = format!("{} {}", process_icon, truncate_name(&proc.name, 18));
            
            // Konverzia bajtov na KB/s
            let sent_bytes = proc.network_sent.unwrap_or(0);
            let recv_bytes = proc.network_recv.unwrap_or(0);
            
            // Kontrola realistických hodnôt (ochrana proti chybným dátam)
            let max_realistic = 100 * 1024 * 1024; // 100 MB/s
            let sent_kbps = if sent_bytes > max_realistic {
                println!("[UI WARN] Unrealistic sent value for {}: {} bytes", 
                    proc.name, sent_bytes);
                0.0  // Nulovanie nereálnych hodnôt
            } else {
                sent_bytes as f64 / 1024.0
            };
            
            let recv_kbps = if recv_bytes > max_realistic {
                println!("[UI WARN] Unrealistic recv value for {}: {} bytes", 
                    proc.name, recv_bytes);
                0.0  // Nulovanie nereálnych hodnôt
            } else {
                recv_bytes as f64 / 1024.0
            };
            
            let total_kbps = sent_kbps + recv_kbps;
            
            // Počet aktívnych spojení pre proces
            let connection_count = app.network_connections.iter()
                .filter(|conn| conn.pid == proc.pid)
                .count();
            
            // Formátovanie názvu s počtom spojení
            let name_with_connections = if connection_count > 0 {
                format!("{} ({})", truncate_name(&proc.name, 16), connection_count)
            } else {
                truncate_name(&proc.name, 20)
            };
            
            // Vytvorenie riadku tabuľky
            Row::new(vec![
                Cell::from(format!("{:2}", i + 1)).style(base_style),                     // Poradové číslo
                Cell::from(name_with_connections).style(name_style),                     // Názov procesu
                Cell::from(format!("{:>7.1}", sent_kbps))                                // Odoslané KB/s
                    .style(base_style.fg(Color::Red)),                                   // Červená farba
                Cell::from(format!("{:>7.1}", recv_kbps))                                // Prijaté KB/s
                    .style(base_style.fg(Color::Green)),                                 // Zelená farba
                Cell::from(format!("{:>7.1}", total_kbps))                               // Celkom KB/s
                    .style(base_style.fg(Color::Cyan)),                                  // Tyrkysová farba
                Cell::from(get_traffic_bar(total_kbps as u64)).style(base_style),        // Grafický ukazovateľ
            ])
        })
        .collect();
    
    // Šírky stĺpcov
    let widths = [
        Constraint::Length(3),    // Poradové číslo
        Constraint::Length(22),   // Názov procesu
        Constraint::Length(10),   // Odoslané
        Constraint::Length(10),   // Prijaté
        Constraint::Length(10),   // Celkom
        Constraint::Min(10),      // Ukazovateľ (minimálne 10)
    ];
    
    // Vytvorenie tabuľky
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["#", "Process", "Sent KB/s", "Recv KB/s", "Total KB/s", "Usage"])
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))  // Tyrkysový tučný hlavičok
                .bottom_margin(1),  // Spodný okraj
        )
        .column_spacing(1);  // Medzera medzi stĺpcami
    
    f.render_widget(table, inner_area);
}

/// Vykreslenie detailného pohľadu na sieťovú aktivitu procesu
fn render_network_process_detail(f: &mut Frame, app: &TuiApp, process_name: &str) {
    let area = f.area();
    
    // Rozdelenie obrazovky detailu
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)                                // Väčší okraj
        .constraints([
            Constraint::Length(3),    // Titulok
            Constraint::Length(6),    // Informácie o procese
            Constraint::Min(12),      // Zoznam spojení
            Constraint::Length(3),    // Päta
        ])
        .split(area);
    
    render_detail_title(f, process_name, chunks[0]);              // Titulok detailu
    render_process_info(f, app, process_name, chunks[1]);         // Info o procese
    render_real_connections(f, app, process_name, chunks[2]);     // Aktívne spojenia
    render_detail_footer(f, chunks[3]);                           // Päta detailu
}

/// Vykreslenie titulku detailného pohľadu
fn render_detail_title(f: &mut Frame, process_name: &str, area: Rect) {
    let title = format!("🔍 {} - NETWORK DETAILS", truncate_name(process_name, 30));
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Magenta));  // Fialová farba okrajov
    
    let content = Paragraph::new("")
        .block(block)
        .alignment(Alignment::Center);
    
    f.render_widget(content, area);
}

/// Vykreslenie informácií o procese v detailnom pohľade
fn render_process_info(f: &mut Frame, app: &TuiApp, process_name: &str, area: Rect) {
    // Nájdenie procesu podľa názvu
    let proc_info = app.top_network_processes.iter()
        .find(|p| p.name == process_name);
    
    if let Some(proc) = proc_info {
        // Konverzia bajtov na KB/s
        let sent_kb = proc.network_sent.unwrap_or(0) as f64 / 1024.0;
        let recv_kb = proc.network_recv.unwrap_or(0) as f64 / 1024.0;
        let total_kb = sent_kb + recv_kb;
        
        // Získanie reálnych spojení pre proces
        let real_connections: Vec<&NetworkConnection> = app.network_connections
            .iter()
            .filter(|conn| conn.pid == proc.pid)
            .collect();
        
        // Formátovanie informácie o spojeniach
        let connection_info = if !real_connections.is_empty() {
            format!("{} active", real_connections.len())
        } else {
            "No connections".to_string()
        };
        
        // Vytvorenie informačných riadkov
        let lines = vec![
            Line::from(vec![
                Span::styled("• PID: ", Style::default().fg(Color::Yellow)),               // Žltý PID
                Span::styled(format!("{}", proc.pid), Style::default().fg(Color::White)),
                Span::styled("   • CPU: ", Style::default().fg(Color::Yellow)),           // Žltý CPU
                Span::styled(format!("{:.1}%", proc.cpu_usage), Style::default().fg(Color::White)),
                Span::styled("   • Memory: ", Style::default().fg(Color::Yellow)),        // Žltá pamäť
                Span::styled(format!("{:.1} MB", proc.memory as f64 / 1024.0 / 1024.0), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("• Sent: ", Style::default().fg(Color::Red)),                // Červené odoslané
                Span::styled(format!("{:.1} KB/s", sent_kb), Style::default().fg(Color::White)),
                Span::styled("   • Received: ", Style::default().fg(Color::Green)),      // Zelené prijaté
                Span::styled(format!("{:.1} KB/s", recv_kb), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("• Total: ", Style::default().fg(Color::Cyan)),              // Tyrkysové celkom
                Span::styled(format!("{:.1} KB/s", total_kb), Style::default().fg(Color::White)),
                Span::styled("   • Connections: ", Style::default().fg(Color::Yellow)),  // Žlté spojenia
                Span::styled(connection_info, Style::default().fg(Color::White)),
            ]),
        ];
        
        let info_block = Block::default()
            .borders(Borders::NONE);  // Bez okrajov
        
        let info_paragraph = Paragraph::new(lines)
            .block(info_block);
        
        f.render_widget(info_paragraph, area);
    } else {
        // Chybové hlásenie ak proces nebol nájdený
        let error_text = Paragraph::new("Process information not available")
            .alignment(Alignment::Center);
        f.render_widget(error_text, area);
    }
}

/// Vykreslenie reálnych sieťových spojení procesu
fn render_real_connections(f: &mut Frame, app: &TuiApp, process_name: &str, area: Rect) {
    let block = Block::default()
        .title("🌐 Real Network Connections")  // Titulok s emodži
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);      // Jednoduché okraje
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);
    
    // Nájdenie PID procesu
    let pid = app.top_network_processes.iter()
        .find(|p| p.name == process_name)
        .map(|p| p.pid)
        .unwrap_or(0);
    
    // Filtrovanie spojení podľa PID
    let connections: Vec<&NetworkConnection> = app.network_connections
        .iter()
        .filter(|conn| conn.pid == pid)
        .collect();
    
    // Ak nie sú žiadne spojenia
    if connections.is_empty() {
        let no_conn = Paragraph::new(vec![
            Line::from("No active network connections detected"),
            Line::from(""),
            Line::from("Possible reasons:"),
            Line::from("• Application is not currently transmitting data"),
            Line::from("• Elevated privileges required to view connections"),
            Line::from("• Network filtering/security software"),
        ])
        .alignment(Alignment::Center);
        
        f.render_widget(no_conn, inner_area);
        return;
    }
    
    // Vytvorenie riadkov tabuľky spojení
    let rows: Vec<Row> = connections.iter()
        .enumerate()
        .map(|(i, conn)| {
            // Striedavé farby pozadia pre lepšiu čitateľnosť
            let row_style = if i % 2 == 0 {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };
            
            Row::new(vec![
                Cell::from(truncate_str(&conn.local_address, 20)).style(row_style),      // Lokálna adresa
                Cell::from(truncate_str(&conn.remote_address, 25)).style(row_style),     // Vzdialená adresa
                Cell::from(truncate_str(&conn.protocol, 8)).style(row_style),            // Protokol
                Cell::from(format!("{:12}", conn.state)).style(row_style),               // Stav spojenia
            ])
        })
        .collect();
    
    // Šírky stĺpcov
    let widths = [
        Constraint::Length(22),   // Lokálna adresa
        Constraint::Length(27),   // Vzdialená adresa
        Constraint::Length(10),   // Protokol
        Constraint::Length(14),   // Stav
    ];
    
    // Vytvorenie tabuľky
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Local Address", "Remote Address", "Protocol", "State"])
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))  // Tyrkysový tučný hlavičok
        )
        .column_spacing(1);  // Medzera medzi stĺpcami
    
    f.render_widget(table, inner_area);
}

/// Vykreslenie hlavného titulku sieťového pohľadu
fn render_network_title(f: &mut Frame, app: &TuiApp, area: Rect) {
    let title = format!("🌐 Network Bandwidth View | {}", app.system_info.hostname);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue));  // Modrá farba okrajov
    
    f.render_widget(block, area);
}

/// Vykreslenie celkových štatistík siete
fn render_network_totals(f: &mut Frame, app: &TuiApp, area: Rect) {
    let sent_kbps = app.network_sent_total;
    let recv_kbps = app.network_recv_total;
    let sent_mb = sent_kbps as f64 / 1024.0;
    let recv_mb = recv_kbps as f64 / 1024.0;
    
    // Formátovanie textu s celkovými štatistikami
    let text = format!(
        "📊 Network Totals: ↑ {:.1} KB/s ({:.1} MB total) | ↓ {:.1} KB/s ({:.1} MB total)",
        sent_kbps,
        sent_mb,
        recv_kbps,
        recv_mb
    );
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);  // Jednoduché okraje
    
    let para = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(para, area);
}

/// Vykreslenie päty hlavného sieťového pohľadu
fn render_network_footer(f: &mut Frame, area: Rect) {
    let footer_text = vec![
        Line::from(vec![
            Span::styled("[Esc] ", Style::default().fg(Color::Yellow)),     // Žltý Esc
            Span::styled("Back", Style::default().fg(Color::DarkGray)),
            Span::styled("  [R] ", Style::default().fg(Color::Green)),      // Zelený R
            Span::styled("Refresh", Style::default().fg(Color::DarkGray)),
            Span::styled("  [Q] ", Style::default().fg(Color::Red)),        // Červený Q
            Span::styled("Quit", Style::default().fg(Color::DarkGray)),
            Span::styled("  [Enter] ", Style::default().fg(Color::Magenta)), // Fialový Enter
            Span::styled("Details", Style::default().fg(Color::DarkGray)),
        ])
    ];
    
    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .alignment(Alignment::Center);
    
    f.render_widget(footer, area);
}

/// Vykreslenie päty detailného pohľadu
fn render_detail_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new("[Esc] Back to Network View")
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));  // Žltý text
    
    f.render_widget(footer, area);
}

/// Pomocná funkcia - získanie farby podľa názvu procesu
fn get_process_color(process_name: &str) -> Color {
    let name_lower = process_name.to_lowercase();
    
    if name_lower.contains("chrome") || name_lower.contains("firefox") || name_lower.contains("edge") {
        Color::Green      // Prehliadače - zelená
    } else if name_lower.contains("steam") || name_lower.contains("discord") || name_lower.contains("zoom") {
        Color::Blue       // Herné/komunikačné - modrá
    } else if name_lower.contains("torrent") || name_lower.contains("bittorrent") {
        Color::Red        // P2P - červená
    } else if name_lower.contains("update") || name_lower.contains("windows") {
        Color::Yellow     // Aktualizácie - žltá
    } else if name_lower.contains("code") || name_lower.contains("vscode") {
        Color::Magenta    // Vývojové prostredia - fialová
    } else {
        Color::Gray       // Ostatné - šedá
    }
}

/// Pomocná funkcia - získanie ikony podľa názvu procesu
fn get_process_icon(process_name: &str) -> &'static str {
    let name_lower = process_name.to_lowercase();
    
    if name_lower.contains("chrome") {
        "🌐"      // Chrome - zemeguľa
    } else if name_lower.contains("firefox") {
        "🦊"      // Firefox - líška
    } else if name_lower.contains("edge") {
        "🧭"      // Edge - kompas
    } else if name_lower.contains("steam") {
        "🎮"      // Steam - ovládač
    } else if name_lower.contains("discord") {
        "💬"      // Discord - rečňa
    } else if name_lower.contains("zoom") {
        "📹"      // Zoom - kamera
    } else if name_lower.contains("torrent") {
        "🌀"      // Torrent - vír
    } else if name_lower.contains("code") {
        "👨‍💻"     // VS Code - programátor
    } else if name_lower.contains("windows") {
        "🪟"      // Windows - okno
    } else {
        "📄"      // Ostatné - stránka
    }
}

/// Pomocná funkcia - získanie typu sieťovej aktivity
fn get_traffic_type(process_name: &str) -> &'static str {
    let name_lower = process_name.to_lowercase();
    
    if name_lower.contains("chrome") || name_lower.contains("firefox") || name_lower.contains("edge") {
        "Web Browsing"       // Prehliadanie webu
    } else if name_lower.contains("steam") {
        "Gaming"             // Hranie hier
    } else if name_lower.contains("discord") || name_lower.contains("zoom") {
        "Communication"      // Komunikácia
    } else if name_lower.contains("torrent") {
        "P2P"                // Peer-to-peer
    } else if name_lower.contains("update") {
        "Updates"            // Aktualizácie
    } else if name_lower.contains("code") {
        "Development"        // Vývoj
    } else {
        "Other"              // Ostatné
    }
}

/// Pomocná funkcia - vytvorenie grafického ukazovateľa sieťovej aktivity
fn get_traffic_bar(value: u64) -> String {
    let width = 15;          // Šírka ukazovateľa
    let max_value = 5000;    // Maximálna hodnota pre škálovanie
    
    // Výpočet vyplnených a prázdnych častí
    let scaled_value = (value as f64 * width as f64 / max_value as f64) as usize;
    let filled = scaled_value.min(width);
    let empty = width - filled;
    
    // Výber znaku podľa intenzity
    let filled_char = match value {
        0..=1000 => "░",     // Nízka aktivita
        1001..=3000 => "▒",  // Stredná aktivita
        3001..=4500 => "▓",  // Vysoká aktivita
        _ => "█",            // Maximalná aktivita
    };
    
    // Vytvorenie reťazca
    filled_char.repeat(filled) + &" ".repeat(empty)
}

/// Pomocná funkcia - skrátenie dlhého názvu
fn truncate_name(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()                          // Ak sa zmestí, ponechať
    } else {
        format!("{}...", &s[..max_len-3])      // Inak skrátiť a pridať "..."
    }
}

/// Alias pre truncate_name (pre konzistentnosť)
fn truncate_str(s: &str, max_len: usize) -> String {
    truncate_name(s, max_len)
}