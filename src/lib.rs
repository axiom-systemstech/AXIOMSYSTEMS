// src/lib.rs - AXIOM SYSTEMS Core Library

#![deny(unsafe_code)]
#![allow(missing_docs)]
#![deny(rust_2018_idioms)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! # AXIOM SYSTEMS
//! 
//! El primer ecosistema tecnológico completo construido desde cero.

pub mod lexer;
pub mod parser;

/// Versión del sistema
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Nombre del sistema
pub const NAME: &str = "AXIOM SYSTEMS";

/// Inicialización del sistema
pub fn init() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                                                          ║");
    println!("║    █████  ██   ██  █████  ██████  ███    ███            ║");
    println!("║   ██   ██  ██ ██  ██   ██ ██   ██ ████  ████            ║");
    println!("║   ███████   ███   ███████ ██████  ██ ████ ██            ║");
    println!("║   ██   ██  ██ ██  ██   ██ ██   ██ ██  ██  ██            ║");
    println!("║   ██   ██ ██   ██ ██   ██ ██   ██ ██      ██            ║");
    println!("║                                                          ║");
    println!("║   {} v{}                                      ║", NAME, VERSION);
    println!("║   El futuro de la programación                          ║");
    println!("║                                                          ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
}