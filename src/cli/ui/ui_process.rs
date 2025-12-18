use ratatui::{
    Frame,
    style::{Style, Color},
    widgets::{Block, Borders, Paragraph, BorderType},
    text::{Line, Span},
};
use crate::cli::app::TuiApp;
use super::ui_widgets::get_cpu_color;

/// Hlavná render funkcia pre detailný pohľad na proces
/// Zobrazuje podrobné informácie o vybranom procese
pub fn render(f: &mut Frame, app: &mut TuiApp) {
    let area = f.area();  // Získanie celej dostupnej plochy frame

    // Vytvorenie bloku (boxu) pre obsah detailov procesu
    let block = Block::default()
        .title("🔍 Process Details")                    // Titulok s emodži
        .borders(Borders::ALL)                          // Všetky okraje
        .border_type(BorderType::Rounded)               // Okrúhle rohy
        .border_style(Style::default().fg(Color::Yellow)); // Žltá farba okrajov

    // Generovanie detailov procesu
    let details = if let Some(index) = app.process_list_state.selected() {
        // Ak je vybratý nejaký proces
        if let Some(proc) = app.top_processes.get(index) {
            // Konverzia pamäte z bajtov na GB
            let memory_gb = proc.memory as f64 / 1024.0 / 1024.0 / 1024.0;

            // Vytvorenie zoznamu informačných riadkov
            vec![
                // Riadok 1: Názov procesu
                Line::from(vec![
                    Span::styled("Process: ", Style::default().fg(Color::Cyan)),  // Tyrkysový štítok
                    Span::styled(&proc.name, Style::default().fg(Color::White)),  // Biely názov procesu
                ]),
                
                // Riadok 2: PID procesu
                Line::from(vec![
                    Span::styled("PID: ", Style::default().fg(Color::Cyan)),      // Tyrkysový štítok
                    Span::styled(proc.pid.to_string(), Style::default().fg(Color::White)),  // Biely PID
                ]),
                
                // Riadok 3: Využitie CPU
                Line::from(vec![
                    Span::styled("CPU Usage: ", Style::default().fg(Color::Cyan)),  // Tyrkysový štítok
                    Span::styled(
                        format!("{:.2}%", proc.cpu_usage),                         // Formátované percento
                        Style::default().fg(get_cpu_color(proc.cpu_usage as f64))  // Farba podľa zaťaženia
                    ),
                ]),
                
                // Riadok 4: Využitie pamäte
                Line::from(vec![
                    Span::styled("Memory: ", Style::default().fg(Color::Cyan)),   // Tyrkysový štítok
                    Span::styled(
                        format!("{:.2} GB", memory_gb),                           // Formátované GB
                        Style::default().fg(Color::Green)                         // Zelená farba
                    ),
                ]),
                
                // Riadok 5: Stav procesu
                Line::from(vec![
                    Span::styled("Status: ", Style::default().fg(Color::Cyan)),   // Tyrkysový štítok
                    Span::styled("Running", Style::default().fg(Color::Green)),   // Zelený "Running"
                ]),
                
                Line::from(""),  // Prázdny riadok pre oddelenie
                
                // Riadok 7: Návod na návrat
                Line::from(Span::styled(
                    "Press [Esc] to go back",                                     // Text nápovedy
                    Style::default().fg(Color::DarkGray)                          // Tmavosivá farba
                )),
            ]
        } else {
            // Chybové hlásenie, ak proces neexistuje
            vec![Line::from("Error: Process not found.")]
        }
    } else {
        // Chybové hlásenie, ak nie je vybratý žiadny proces
        vec![Line::from("Error: No process selected.")]
    };

    // Vytvorenie odstavca (paragraph) s detailmi
    let paragraph = Paragraph::new(details)
        .block(block)                                           // Pridanie bloku
        .alignment(ratatui::layout::Alignment::Left);           // Zarovnanie doľava

    // Vykreslenie widgetu na plochu
    f.render_widget(paragraph, area);
}