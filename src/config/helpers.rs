// helpers.rs

use dotenv;  // Import knižnice pre prácu s .env súbormi

/// Inicializácia prostredia aplikácie
/// Načíta premenné prostredia z .env súboru ak existuje
pub fn init_environment() {
    // Načítanie premenných prostredia z .env súboru
    // .ok() konvertuje Result na Option, ignoruje chyby ak súbor neexistuje
    dotenv::dotenv().ok();
    
    // Potvrdenie úspešnej inicializácie
    println!("✅ Environment initialized");
}

/// Validácia hostname (názvu hostiteľa)
/// Kontroluje, či reťazec nie je prázdny
pub fn validate_host(host: &str) -> bool {
    !host.is_empty()  // Vráti true ak host nie je prázdny reťazec
}

/// Validácia portového čísla
/// Kontroluje, či port je v platnom rozsahu (1-65535)
pub fn validate_port(port: u16) -> bool {
    port > 0 && port <= 65535  // Port musí byť väčší ako 0 a maximálne 65535
}