// src/main.rs - Punto de entrada de AXIOM CLI

use axiom_core::lexer::Lexer;
use colored::*;
use std::fs;
use std::path::Path;

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
        "check" => check_file(&args),
        "new" => new_project(&args),
        "init" => init_project(),
        "help" | "-h" | "--help" => print_help(),
        "version" | "-v" | "--version" => print_version(),
        _ => {
            println!("{} Comando desconocido: '{}'", "✗".red(), args[1].red());
            println!();
            print_help();
        }
    }
}

/// Ejecuta un archivo AXIOM
fn run_file(args: &[String]) {
    if args.len() < 3 {
        println!("{} Especifica un archivo para ejecutar", "✗".red());
        println!("  Uso: axiom run <archivo.ax>");
        return;
    }
    
    let filename = &args[2];
    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("{} Ejecutando: {}", "▶".green(), filename.cyan());
            println!("{}", "─".repeat(50).dimmed());
            
            // Por ahora solo mostramos el contenido
            // Más adelante ejecutaremos el código
            println!("{}", content);
            
            println!("{}", "─".repeat(50).dimmed());
            println!("{} Programa ejecutado correctamente", "✔".green());
        }
        Err(e) => {
            println!("{} Error al leer el archivo: {}", "✗".red(), e);
        }
    }
}

/// Muestra los tokens de un archivo
fn lex_file(args: &[String]) {
    if args.len() < 3 {
        println!("{} Especifica un archivo para analizar", "✗".red());
        println!("  Uso: axiom lex <archivo.ax>");
        return;
    }
    
    let filename = &args[2];
    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("{} Tokenizando: {}", "🔍".cyan(), filename.cyan());
            println!("{}", "─".repeat(50).dimmed());
            
            let lexer = Lexer::new(&content);
            let mut token_count = 0;
            
            for token in lexer {
                token_count += 1;
                let line = token.span.line;
                let col = token.span.column;
                println!("[{:>4}:{:<3}] {:?}", line, col, token.token);
            }
            
            println!("{}", "─".repeat(50).dimmed());
            println!("{} {} tokens encontrados", "✔".green(), token_count);
        }
        Err(e) => {
            println!("{} Error al leer el archivo: {}", "✗".red(), e);
        }
    }
}

/// Verifica la sintaxis de un archivo sin ejecutarlo
fn check_file(args: &[String]) {
    if args.len() < 3 {
        println!("{} Especifica un archivo para verificar", "✗".red());
        println!("  Uso: axiom check <archivo.ax>");
        return;
    }
    
    let filename = &args[2];
    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("{} Verificando: {}", "🔍".cyan(), filename.cyan());
            println!("{}", "─".repeat(50).dimmed());
            
            let lexer = Lexer::new(&content);
            let mut has_error = false;
            
            for token in lexer {
                if let axiom_core::lexer::Token::Error(msg) = token.token {
                    has_error = true;
                    println!("{} Error en línea {}: {}", "✗".red(), token.span.line, msg.red());
                }
            }
            
            println!("{}", "─".repeat(50).dimmed());
            if has_error {
                println!("{} Verificación fallida", "✗".red());
            } else {
                println!("{} Verificación exitosa", "✔".green());
            }
        }
        Err(e) => {
            println!("{} Error al leer el archivo: {}", "✗".red(), e);
        }
    }
}

/// Crea un nuevo proyecto
fn new_project(args: &[String]) {
    if args.len() < 3 {
        println!("{} Especifica un nombre para el proyecto", "✗".red());
        println!("  Uso: axiom new <nombre>");
        return;
    }
    
    let name = &args[2];
    let path = Path::new(name);
    
    if path.exists() {
        println!("{} El directorio '{}' ya existe", "✗".red(), name);
        return;
    }
    
    // Crear estructura del proyecto
    std::fs::create_dir_all(path).unwrap();
    std::fs::create_dir_all(path.join("src")).unwrap();
    
    // Crear main.ax
    let main_content = r#"// main.ax - Programa principal
fn main() {
    let message = "¡Hola, mundo!"
    println(message)
}
"#;
    std::fs::write(path.join("src/main.ax"), main_content).unwrap();
    
    // Crear Cargo.toml
    let cargo_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#, name);
    std::fs::write(path.join("Cargo.toml"), cargo_content).unwrap();
    
    // Crear README.md
    let readme_content = format!(r#"# {}

Un proyecto en AXIOM.

## Ejecutar

```bash
axiom run src/main.ax
```

Compilar

```bash
axiom build
```

"#, name);
std::fs::write(path.join("README.md"), readme_content).unwrap();

}

/// Inicializa un proyecto en el directorio actual
fn init_project() {
    let path = Path::new(".");
    std::fs::create_dir_all(path.join("src")).unwrap();

    let main_content = r#"// main.ax - Programa principal
fn main() {
    let message = "¡Hola, mundo!"
    println(message)
}
"#;
    std::fs::write(path.join("src/main.ax"), main_content).unwrap();

    let cargo_content = r#"[package]
name = "mi-proyecto"
version = "0.1.0"
edition = "2024"

[dependencies]
"#;
    std::fs::write(path.join("Cargo.toml"), cargo_content).unwrap();
    println!("Proyecto inicializado correctamente");

}

/// Muestra la ayuda
fn print_help() {
println!();
println!("{}", "AXIOM SYSTEMS - Lenguaje de Programación".bright_cyan().bold());
println!("{}", "╔════════════════════════════════════════════════════════════╗".dimmed());
println!("║  {}  {}  {}  ║", 
"COMANDO".bold(),
"DESCRIPCIÓN".bold(),
"EJEMPLO".bold()
);
println!("╠════════════════════════════════════════════════════════════╣");
println!("║  {}  {}  {} ║", 
"run".green(),
"Ejecuta un programa".dimmed(),
"axiom run main.ax".dimmed()
);
println!("║  {}  {}  {} ║", 
"lex".green(),
"Muestra los tokens".dimmed(),
"axiom lex main.ax".dimmed()
);
println!("║  {}  {}  {} ║", 
"check".green(),
"Verifica la sintaxis".dimmed(),
"axiom check main.ax".dimmed()
);
println!("║  {}  {}  {} ║", 
"new".green(),
"Crea un nuevo proyecto".dimmed(),
"axiom new mi-proyecto".dimmed()
);
println!("║  {}  {}  {} ║", 
"init".green(),
"Inicializa proyecto".dimmed(),
"axiom init".dimmed()
);
println!("║  {}  {}  {} ║", 
"help".green(),
"Muestra esta ayuda".dimmed(),
"axiom help".dimmed()
);
println!("║  {}  {}  {} ║", 
"version".green(),
"Muestra la versión".dimmed(),
"axiom version".dimmed()
);
println!("{}", "╚════════════════════════════════════════════════════════════╝".dimmed());
}

/// Muestra la versión
fn print_version() {
println!("{} v{}", "AXIOM SYSTEMS".bright_cyan().bold(), env!("CARGO_PKG_VERSION"));
println!("{} El futuro de la programación", "🚀".green());
println!();
println!("  GitHub: https://github.com/AXIOM-SYSTEMSTECH/axiom-systems");
}
