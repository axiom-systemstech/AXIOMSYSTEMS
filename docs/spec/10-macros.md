# 10. MACROS

## 10.1 Macros Declarativas

```rust
// Macro básica
macro_rules! println {
    ($($arg:tt)*) => {
        print!($($arg)*)
        print("\n")
    }
}

// Usar
println("Hola, mundo!")

// Macro con patrones
macro_rules! sum {
    ($a:expr, $b:expr) => {
        $a + $b
    }
    ($a:expr, $b:expr, $c:expr) => {
        $a + $b + $c
    }
}

// Macro con repetición
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp = Vec::new()
            $(temp.push($x))*
            temp
        }
    }
}

// Usar
let v = vec![1, 2, 3, 4]
```

10.2 Macros de Atributo

```rust
// Attribute macro
#[derive(Debug, Clone, Copy)]
struct Point { x: i32, y: i32 }

// Custom attribute
#[my_macro]
fn my_function() { }

// Procedural macro
#[proc_macro]
fn my_macro(input: TokenStream) -> TokenStream {
    // Procesar input y generar output
}
```

10.3 Macros de Función

```rust
// Function-like macro
#[proc_macro]
fn my_macro(input: TokenStream) -> TokenStream {
    // Procesar input
    return input
}

// Usar
my_macro!(some code)
```

10.4 Macros de Derive

```rust
// Derive macro
#[derive(MyTrait)]
struct MyType { /* ... */ }

// Implementación de derive
#[proc_macro_derive(MyTrait)]
fn my_trait_derive(input: TokenStream) -> TokenStream {
    // Generar implementación de MyTrait
}
```

10.5 Macros Built-in

```rust
// println! (imprimir con nueva línea)
println("Hola")

// format! (formatear string)
let s = format("El valor es {}", 42)

// include_str! (incluir archivo como string)
let content = include_str!("archivo.txt")

// include_bytes! (incluir archivo como bytes)
let data = include_bytes!("archivo.bin")

// file! (nombre del archivo actual)
let file = file!()

// line! (número de línea actual)
let line = line!()

// column! (número de columna actual)
let column = column!()

// stringify! (convertir a string)
let s = stringify!(fn main() { /* ... */ })

// concat! (concatenar strings en compilación)
let s = concat!("a", "b", "c")
```

10.6 Macros y Seguridad

```rust
// Macro segura
macro_rules! safe_macro {
    // Solo acepta expresiones seguras
}

// Macro insegura (requiere unsafe)
macro_rules! unsafe_macro {
    ($($arg:tt)*) => {
        unsafe {
            // Código inseguro
        }
    }
}
