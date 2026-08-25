# 6. MÓDULOS Y PAQUETES

## 6.1 Módulos

```rust
// Definir módulo
mod math {
    fn add(a: i32, b: i32) -> i32 {
        return a + b
    }
    
    fn sub(a: i32, b: i32) -> i32 {
        return a - b
    }
}

// Módulo en archivo separado
// math.ax
fn add(a: i32, b: i32) -> i32 {
    return a + b
}

// main.ax
mod math  // Importa math.ax

// Módulo con sub-módulos
mod utils {
    mod strings {
        fn to_uppercase(s: String) -> String { ... }
    }
    
    mod numbers {
        fn parse_int(s: String) -> i32 { ... }
    }
}
```

6.2 Visibilidad

```rust
// Público (accesible desde fuera)
pub fn public_function() { }

// Privado (solo dentro del módulo)
fn private_function() { }

// Público dentro del crate
pub(crate) fn crate_function() { }

// Público dentro del módulo padre
pub(super) fn super_function() { }

// Público dentro de un path específico
pub(in path::to::module) fn specific_function() { }
```

6.3 Importaciones

```rust
// Importar función
use math::add

// Importar múltiples
use math::{add, sub, mul}

// Importar todo
use math::*

// Renombrar
use math::add as add_numbers

// Importar desde path
use std::io::print

// Import anidado
use std::io::{self, print, println}
```

6.4 Paquetes

```toml
# Cargo.toml (package manifest)
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"
authors = ["Tu Nombre <email@domain.com>"]
description = "Mi proyecto en AXIOM"

[dependencies]
http = "1.0.0"
json = "0.5"
```

6.5 Dependencias

```rust
// En el código
use http::Client
use json::parse

fn main() {
    let client = Client::new()
    let response = client.get("https://api.axion")
    let data = parse(response.body)
    print(data)
}
```

6.6 Workspace

```toml
# Cargo.toml (workspace)
[workspace]
members = [
    "project1",
    "project2",
    "libs/lib1",
    "libs/lib2",
]
```

6.7 Registro de Paquetes

```bash
# Publicar paquete
axiom publish

# Añadir dependencia
axiom add http

# Actualizar dependencias
axiom update

# Instalar dependencias
axiom install
