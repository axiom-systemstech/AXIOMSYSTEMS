// src/main.rs - Punto de entrada de AXIOM CLI (AÑADIR PARSE)

// ... (código existente) ...
use colored::*;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("parse") => parse_file(&args),
        Some("version") | Some("-v") | Some("--version") => {
            println!("AXIOM SYSTEMS v{}", axiom_core::VERSION);
        }
        _ => print_help(),
    }
}

fn parse_file(args: &[String]) {
    let Some(filename) = args.get(2) else {
        eprintln!("{} Especifica un archivo para parsear", "✗".red());
        eprintln!("  Uso: axiom parse <archivo.ax>");
        return;
    };

    match fs::read_to_string(filename) {
        Ok(content) => {
            let mut parser = axiom_core::parser::Parser::new(&content);
            match parser.parse_program() {
                Ok(program) => {
                    println!("AST generado correctamente");
                    println!("{} items encontrados", program.items.len());
                    println!("{:#?}", program);
                }
                Err(error) => {
                    eprintln!("Error en línea {}: {}", error.span.line, error.message);
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            eprintln!("{} Error al leer el archivo: {}", "✗".red(), error);
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("AXIOM SYSTEMS");
    println!("  parse <archivo.ax>  Muestra el AST");
}