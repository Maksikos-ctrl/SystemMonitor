// menu.rs

use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use crate::modes::{run_tui_mode, run_api_mode};

/// Zobrazenie interaktívneho menu pre výber režimu aplikácie
/// Užívateľ vyberá medzi TUI, API alebo nápovedou
pub async fn show_interactive_menu() -> Result<(), Box<dyn std::error::Error>> {
    // Grafická hlavička menu
    println!("╔═══════════════════════════════════════════╗");
    println!("║     🖥️  SYSTEM MONITOR v1.0               ║");
    println!("╠═══════════════════════════════════════════╣");
    println!("║ Select operation mode:                    ║");
    println!("╚═══════════════════════════════════════════╝");
    println!();
    
    // Možnosti v menu
    let choices = vec![
        "🎨 TUI Interface (Graphical Monitor)",  // Grafické TUI rozhranie
        "🌐 REST API Server",                    // REST API server
        "📖 Show Help",                          // Nápoveda
        "❌ Exit",                               // Ukončenie
    ];
    
    // Interaktívny výber s farebnou tému
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option (use ↑↓ arrows, Enter to select)")  // Inštrukcie
        .default(0)                     // Predvolená možnosť
        .items(&choices)                // Zoznam možností
        .interact()                     // Čakanie na užívateľský vstup
        .unwrap();
    
    match selection {
        0 => {
            // Spustenie TUI módu
            println!();
            run_tui_mode()  // Táto funkcia vracia Result
        }
        1 => {
            // Spustenie API módu s podmenu
            println!();
            show_api_submenu().await
        }
        2 => {
            // Zobrazenie nápovedy a rekurzívny návrat do menu
            show_help()?;
            
            // Riešenie pre rekurziu - používame cyklus namiesto rekurzie
            loop {
                let result = show_interactive_menu_once().await;
                if result.is_ok() {
                    return result;
                }
            }
        }
        3 => {
            // Ukončenie aplikácie
            println!("\n👋 Goodbye!");
            std::process::exit(0);
        }
        _ => unreachable!(),  // Nikdy by sa nemalo stať
    }
}

/// Pomocná funkcia bez rekurzie pre jedno zobrazenie menu
/// Používa sa pre vyhnutie sa stack overflow pri rekurzívnych volaniach
async fn show_interactive_menu_once() -> Result<(), Box<dyn std::error::Error>> {
    // Opätovné zobrazenie menu (rovnaké ako hlavná funkcia)
    println!("╔═══════════════════════════════════════════╗");
    println!("║     🖥️  SYSTEM MONITOR v1.0               ║");
    println!("╠═══════════════════════════════════════════╣");
    println!("║ Select operation mode:                    ║");
    println!("╚═══════════════════════════════════════════╝");
    println!();
    
    let choices = vec![
        "🎨 TUI Interface (Graphical Monitor)",
        "🌐 REST API Server",
        "📖 Show Help",
        "❌ Exit",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option (use ↑↓ arrows, Enter to select)")
        .default(0)
        .items(&choices)
        .interact()
        .unwrap();
    
    match selection {
        0 => {
            // TUI režim
            println!();
            run_tui_mode()
        }
        1 => {
            // API režim
            println!();
            show_api_submenu().await
        }
        2 => {
            // Nápoveda - vráti sa do cyklu
            show_help()?;
            Ok(())  // Návrat do cyklu
        }
        3 => {
            // Ukončenie
            println!("\n👋 Goodbye!");
            std::process::exit(0);
        }
        _ => unreachable!(),
    }
}

/// Podmenu pre konfiguráciu API
/// Umožňuje rýchle spustenie alebo vlastné nastavenia
async fn show_api_submenu() -> Result<(), Box<dyn std::error::Error>> {
    let api_choices = vec![
        "🚀 Start API with default settings (127.0.0.1:3000)",  // Rýchle spustenie
        "⚙️  Start API with custom settings",                   // Vlastné nastavenia
        "⬅️  Back to main menu",                                // Návrat do hlavného menu
    ];
    
    let api_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("API configuration")  // Konfigurácia API
        .default(0)                        // Predvolené nastavenia
        .items(&api_choices)
        .interact()
        .unwrap();
    
    match api_selection {
        0 => {
            // Spustenie s predvolenými nastaveniami
            run_api_mode("127.0.0.1".to_string(), 3000, true).await
        }
        1 => {
            // Vlastné nastavenia - interaktívne zadávanie
            let host: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter host address")            // Zadanie hostname
                .default("127.0.0.1".to_string())             // Predvolený localhost
                .interact_text()?;                            // Čítanie textového vstupu
            
            let port: u16 = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter port number")             // Zadanie portu
                .default(3000)                                // Predvolený port 3000
                .validate_with(|input: &u16| {                // Validácia vstupu
                    if *input > 0 && *input <= 65535 {
                        Ok(())
                    } else {
                        Err("Port must be between 1 and 65535")  // Chybová správa
                    }
                })
                .interact_text()?;
            
            // Výber či ukladať metriky do databázy
            let save_metrics = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Enable background metric saving to database?")  // Otázka
                .default(true)                                                // Predvolená hodnota
                .interact()?;
            
            // Spustenie s vlastnými nastaveniami
            run_api_mode(host, port, save_metrics).await
        }
        2 => {
            // Návrat do hlavného menu
            Ok(())  // Jednoduchý návrat - volajúci rozhodne čo ďalej
        }
        _ => unreachable!(),
    }
}

/// Zobrazenie nápovedy s inštrukciami na používanie aplikácie
fn show_help() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("╔═══════════════════════════════════════════╗");
    println!("║              SYSTEM MONITOR HELP          ║");
    println!("╠═══════════════════════════════════════════╣");
    println!("║ Usage:                                    ║");
    println!("║                                           ║");
    println!("║   system-monitor                          ║");
    println!("║     - Show interactive menu               ║");
    println!("║                                           ║");
    println!("║   system-monitor tui                      ║");
    println!("║     - Start TUI interface                 ║");
    println!("║                                           ║");
    println!("║   system-monitor api                      ║");
    println!("║     - Start REST API server               ║");
    println!("║                                           ║");
    println!("║   system-monitor api --host 0.0.0.0 --port 8080 --save-metrics");
    println!("║     - Start API with custom settings      ║");
    println!("╚═══════════════════════════════════════════╝");
    println!();
    
    // Čakanie na stlačenie Enter pre pokračovanie
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    Ok(())
}