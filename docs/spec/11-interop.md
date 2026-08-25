# 11. INTEROPERABILIDAD

## 11.1 FFI con C

```rust
// Declarar función C
extern "C" {
    fn printf(format: *const u8, ...) -> i32
    fn malloc(size: usize) -> *const u8
    fn free(ptr: *const u8)
}

// Llamar a C
fn main() {
    let format = "Hola, mundo!\0".as_ptr()
    unsafe {
        printf(format)
    }
}

// Función C desde AXIOM
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    return a + b
}

// Exportar para C
// En C: int add(int a, int b)
```

11.2 FFI con Rust

```rust
// Llamar a Rust desde AXIOM
extern "Rust" {
    fn rust_function() -> i32
}

// Llamar
let value = unsafe { rust_function() }

// Función Rust exportada
#[no_mangle]
pub fn axiom_function() -> i32 {
    return 42
}
```

11.3 FFI con WebAssembly

```rust
// Exportar para WebAssembly
#[export_name = "add"]
fn add(a: i32, b: i32) -> i32 {
    return a + b
}

// Llamar desde JavaScript
// const result = instance.exports.add(1, 2)
```

11.4 Interfaz con Python

```rust
// Código Python desde AXIOM
extern "Python" {
    fn py_print(msg: String)
}

// Llamar
unsafe { py_print("Hola desde Python") }
```

11.5 Interfaz con C++

```rust
// Declarar C++
extern "C++" {
    fn cpp_function() -> i32
}

// Llamar
let value = unsafe { cpp_function() }
```

11.6 Seguridad en FFI

```rust
// FFI requiere unsafe
unsafe {
    call_external_function()
}

// ABIs soportados
// - C (system)
// - Rust
// - C++
// - Wasm
// - SystemV

// Declaración segura de FFI
extern "C" {
    // Solo funciones seguras
    fn safe_function() -> i32
}

// Funciones inseguras
extern "C" {
    // Requiere manejo manual de memoria
    fn unsafe_function(ptr: *const u8)
}
