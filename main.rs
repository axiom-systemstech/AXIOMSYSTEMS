// src/main.rs - Punto de entrada de AXIOM

use axiom_core::lexer::Lexer;
use colored::*;
use std::fs;

fn main() {
    axiom_core::init();
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "run" => run_file(&args),
        "lex" => lex_file(&args),
        "help" | "-h" | "--help" => print_help(),
        "version" | "-v" | "--version" => print_version(),
        _ => {
            println!("{} Comando desconocido: {}", "❌".red(), args[1].red());
            print_help();
        }
    }
}

fn run_file(args: &[String]) {
    if args.len() < 3 {
        println!("{} Especifica un archivo para ejecutar", "❌".red());
        return;
    }
    
    let filename = &args[2];
    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("{} Ejecutando: {}", "▶️".green(), filename.cyan());
            println!("{}", "─".repeat(50).dimmed());
            
            // Aquí ejecutaremos el código más adelante
            println!("{}", content);
            
            println!("{}", "─".repeat(50).dimmed());
            println!("{} Programa ejecutado correctamente", "✅".green());
        }
        Err(e) => {
            println!("{} Error al leer el archivo: {}", "❌".red(), e);
        }
    }
}

fn lex_file(args: &[String]) {
    if args.len() < 3 {
        println!("{} Especifica un archivo para analizar", "❌".red());
        return;
    }
    
    let filename = &args[2];
    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("{} Tokenizando: {}", "🔍".cyan(), filename.cyan());
            println!("{}", "─".repeat(50).dimmed());
            
            let lexer = Lexer::new(&content);
            for token in lexer {
                println!("{:?}", token);
            }
            
            println!("{}", "─".repeat(50).dimmed());
            println!("✅ Tokenización completada");
        }
        Err(e) => {
            println!("{} Error al leer el archivo: {}", "❌".red(), e);
        }
    }
}

fn print_help() {
    println!();
    println!("{}", "AXIOM SYSTEMS - Lenguaje de Programación".bright_cyan().bold());
    println!("{}", "╔════════════════════════════════════════════════════════════╗".dimmed());
    println!("║  {}  {}  {}  {}  ║", 
        "Comando".bold(),
        "Descripción".bold(),
        "Uso".bold(),
        "Ejemplo".bold()
    );
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  {}  {}  {}  {} ║", 
        "run".green(),
        "Ejecuta un programa".dimmed(),
        "axion run <archivo>".dimmed(),
        "axion run main.ax".dimmed()
    );
    println!("║  {}  {}  {}  {} ║", 
        "lex".green(),
        "Muestra los tokens".dimmed(),
        "axion lex <archivo>".dimmed(),
        "axion lex main.ax".dimmed()
    );
    println!("║  {}  {}  {}  {} ║", 
        "help".green(),
        "Muestra esta ayuda".dimmed(),
        "axion help".dimmed(),
        "axion help".dimmed()
    );
    println!("║  {}  {}  {}  {} ║", 
        "version".green(),
        "Muestra la versión".dimmed(),
        "axion version".dimmed(),
        "axion version".dimmed()
    );
    println!("{}", "╚════════════════════════════════════════════════════════════╝".dimmed());
}

fn print_version() {
    println!("{} v{}", "AXION SYSTEMS".bright_cyan(), env!("CARGO_PKG_VERSION"));
    println!("{} El futuro de la programación", "🚀".green());
}
