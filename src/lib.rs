// src/lib.rs - AXIOM SYSTEMS Core Library

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]

//! AXIOM SYSTEMS - El primer ecosistema tecnológico completo
//! 
//! Este es el núcleo de AXIOM, un lenguaje de programación diseñado para
//! construir todo tipo de software: desde aplicaciones hasta sistemas operativos.

pub mod lexer;
pub mod parser;
pub mod compiler;
pub mod vm;
pub mod cli;
pub mod std;

/// Versión del sistema
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Inicialización del sistema
pub fn init() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║              AXIOM SYSTEMS v{}                           ║", VERSION);
    println!("║              El futuro de la programación               ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}
