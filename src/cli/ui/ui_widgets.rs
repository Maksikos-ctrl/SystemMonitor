use ratatui::{
    Frame,
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Color},
    widgets::{Block, Borders, Gauge, Sparkline, BorderType},
};
use unicode_width::UnicodeWidthStr;


/// Pomocná funkcia na skrátenie reťazca s ohľadom na unicode šírku znakov
/// Táto funkcia je inteligentnejšia ako štandardné skracovanie, pretože berie do úvahy
/// šírku znakov (napr. emodži majú väčšiu šírku ako bežné znaky)
pub fn truncate_str(s: &str, max_len: usize) -> String {
    // Kontrola nulovej dĺžky
    if max_len == 0 {
        return String::new();  // Vrátenie prázdneho reťazca
    }

    // Výpočet šírky reťazca v terminálových stĺpcoch
    let width = s.width();
    
    // Ak sa reťazec zmestí, vráti sa nezmenený
    if width <= max_len {
        s.to_string()
    } else {
        // Výpočet bezpečnej dĺžky s ohľadom na "..."
        let safe_len = max_len.saturating_sub(3);  // "saturating_sub" zabraňuje podtečeniu
        
        // Ak je bezpečná dĺžka 0, vráti sa iba "..."
        if safe_len == 0 {
            return "...".to_string();
        }

        let mut result = String::new();  // Výsledný skrátený reťazec
        let mut current_len = 0;         // Aktuálna šírka v terminálových stĺpcoch

        // Iterácia cez znaky reťazca s ohľadom na unicode šírku
        for ch in s.chars() {
            // Výpočet šírky aktuálneho znaku
            let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            
            // Kontrola, či sa znak zmestí do bezpečnej dĺžky
            if current_len + ch_width <= safe_len {
                result.push(ch);            // Pridanie znaku
                current_len += ch_width;    // Aktualizácia aktuálnej šírky
            } else {
                break;  // Ak sa už nezmestí, prerušiť cyklus
            }
        }

        format!("{}...", result)  // Pridanie "..." na koniec
    }
}

/// Pomocná funkcia na vytvorenie grafického ukazovateľa pre proces
/// Vracia reťazec s vizuálnym indikátorom zaťaženia (napr. "██████░░░░░░░░░░░░░░")
pub fn get_process_bar(percent: u8) -> String {
    let width = 20;                      // Šírka ukazovateľa v znakoch
    let percent = percent.min(100);      // Obmedzenie na maximálne 100%
    let filled = (percent as usize * width) / 100;  // Počet vyplnených znakov
    let empty = width.saturating_sub(filled);       // Počet prázdnych znakov

    // Výber znaku podľa intenzity
    let filled_char = match percent {
        0..=30 => "░",     // Nízka intenzita - svetlý znak
        31..=60 => "▒",    // Stredná intenzita - stredne tmavý znak
        61..=80 => "▓",    // Vysoká intenzita - tmavý znak
        _ => "█",          // Maximálna intenzita - plný znak
    };

    // Vytvorenie reťazca s vyplnenými a prázdnymi znakmi
    filled_char.repeat(filled) + &" ".repeat(empty)
}

/// Pomocná funkcia na získanie farby pre indikáciu využitia CPU
/// Farba sa mení podľa zaťaženia CPU (zelená → žltá → červená)
#[allow(dead_code)]  // Povolenie nepoužívanej funkcie
pub fn get_cpu_color(usage: f64) -> Color {
    match usage {
        x if x < 30.0 => Color::Green,    // Nízke zaťaženie - zelená
        x if x < 70.0 => Color::Yellow,   // Stredné zaťaženie - žltá
        _ => Color::Red,                  // Vysoké zaťaženie - červená
    }
}