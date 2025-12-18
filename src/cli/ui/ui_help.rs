use ratatui::{
    Frame,
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph, BorderType, Wrap},
    text::{Line, Span},
};
use crate::cli::app::TuiApp;

/// Render funkcia pre zobrazenie obrazovky pomoci
/// Zobrazuje klávesové skratky a popis dostupných pohľadov
pub fn render(f: &mut Frame, _app: &mut TuiApp) {
    // Získanie celej dostupnej plochy frame
    let area = f.area();

    // Vytvorenie bloku (boxu) pre obsah pomoci
    let block = Block::default()
        .title("❓ Help & Shortcuts")                    // Titulok s emodži
        .borders(Borders::ALL)                          // Všetky okraje
        .border_type(BorderType::Rounded)               // Okrúhle rohy
        .border_style(Style::default().fg(Color::Cyan)); // Tyrkysová farba okrajov

    // Definovanie obsahu pomoci - zoznam riadkov
    let help_content = vec![
        // Nadpis sekcie klávesových skratiek
        Line::from(vec![
            Span::styled("Keyboard Shortcuts:", Style::default()
                .fg(Color::Yellow)                     // Žltý text
                .add_modifier(Modifier::BOLD)),        // Tučné písmo
        ]),
        Line::from(""), // Prázdny riadok
        
        // Skratka Q - ukončenie aplikácie
        Line::from(vec![
            Span::styled("[Q] ", Style::default().fg(Color::Red)), // Červené [Q]
            Span::styled("Quit application", Style::default().fg(Color::White)),
        ]),
        
        // Skratka H - zobrazenie/skrytie pomoci
        Line::from(vec![
            Span::styled("[H] ", Style::default().fg(Color::Yellow)), // Žlté [H]
            Span::styled("Show/hide this help screen", Style::default().fg(Color::White)),
        ]),
        
        // Skratka R - vynútené obnovenie dát
        Line::from(vec![
            Span::styled("[R] ", Style::default().fg(Color::Green)), // Zelené [R]
            Span::styled("Force refresh data", Style::default().fg(Color::White)),
        ]),
        
        // Skratka N - prepnutie na sieťový pohľad
        Line::from(vec![
            Span::styled("[N] ", Style::default().fg(Color::Blue)), // Modré [N]
            Span::styled("Switch to Network view", Style::default().fg(Color::White)),
        ]),
        
        // Skratka Tab - prepínanie medzi pohľadmi
        Line::from(vec![
            Span::styled("[Tab] ", Style::default().fg(Color::Magenta)), // Fialové [Tab]
            Span::styled("Toggle between views", Style::default().fg(Color::White)),
        ]),
        
        // Šípky hore/dole - navigácia v zozname procesov
        Line::from(vec![
            Span::styled("[↑↓] ", Style::default().fg(Color::Cyan)), // Tyrkysové šípky
            Span::styled("Navigate process list", Style::default().fg(Color::White)),
        ]),
        
        // Enter - zobrazenie detailov procesu
        Line::from(vec![
            Span::styled("[Enter] ", Style::default().fg(Color::Magenta)), // Fialový Enter
            Span::styled("View process details", Style::default().fg(Color::White)),
        ]),
        
        // Esc - návrat/ukončenie
        Line::from(vec![
            Span::styled("[Esc] ", Style::default().fg(Color::Red)), // Červený Esc
            Span::styled("Go back/Exit", Style::default().fg(Color::White)),
        ]),
        
        Line::from(""), // Prázdny riadok
        
        // Nadpis sekcie pohľadov
        Line::from(vec![
            Span::styled("Views:", Style::default()
                .fg(Color::Yellow)                     // Žltý text
                .add_modifier(Modifier::BOLD)),        // Tučné písmo
        ]),
        Line::from(""), // Prázdny riadok
        
        // Zoznam dostupných pohľadov
        Line::from("• Overview: System metrics and top processes"),
        Line::from("• Network: Bandwidth usage and network processes"),
        Line::from("• Process Details: Detailed info about selected process"),
        Line::from(""), // Prázdny riadok
        
        // Inštrukcia pre návrat
        Line::from(Span::styled("Press [H] or [Esc] to go back",
            Style::default().fg(Color::DarkGray))), // Tmavosivý text
    ];

    // Vytvorenie odstavca (paragraph) s obsahom pomoci
    let paragraph = Paragraph::new(help_content)
        .block(block)                                           // Pridanie bloku
        .alignment(ratatui::layout::Alignment::Left)            // Zarovnanie doľava
        .wrap(Wrap { trim: true });                             // Zalomovanie textu

    // Vykreslenie widgetu na plochu
    f.render_widget(paragraph, area);
}