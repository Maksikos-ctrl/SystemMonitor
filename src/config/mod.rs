// mod.rs

/// Hlavný modul s exportovanými komponentami CLI
pub mod cli;       // Modul pre CLI (Command Line Interface)
pub mod helpers;   // Modul pre pomocné funkcie

/// Re-export dôležitých typov pre jednoduchší import v iných moduloch
pub use cli::{Cli, Commands};  // Export CLI štruktúr a príkazov
pub use helpers::*;            // Export všetkých pomocných funkcií