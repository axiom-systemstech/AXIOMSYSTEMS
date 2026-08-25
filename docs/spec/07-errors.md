# 7. MANEJO DE ERRORES

## 7.1 Filosofía

AXIOM distingue entre:

- **Errores recuperables**: Usando `Result<T, E>`
- **Errores irrecuperables**: Usando `panic!()`

## 7.2 Result

```rust
// Definición de Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Uso de Result
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("División por cero".to_string())
    }
    return Ok(a / b)
}

// Manejo de Result
let result = divide(10, 2)
match result {
    Ok(value) => print(value),
    Err(error) => print("Error: " + error),
}

// Unwrap (peligroso)
let value = result.unwrap()  // Panic si es Err

// Unwrap con mensaje
let value = result.expect("División falló")

// Propagación de errores (Try)
fn process() -> Result<i32, String> {
    let value = try divide(10, 2)?  // Si es Err, propaga
    return Ok(value + 1)
}
```

7.3 Option

```rust
// Definición de Option
enum Option<T> {
    Some(T),
    None,
}

// Uso de Option
fn find_user(id: i32) -> Option<String> {
    if id == 1 {
        return Some("Usuario 1".to_string())
    }
    return None
}

// Manejo de Option
let user = find_user(1)
match user {
    Some(name) => print(name),
    None => print("Usuario no encontrado"),
}

// Unwrap (peligroso)
let name = user.unwrap()  // Panic si es None

// Unwrap con mensaje
let name = user.expect("Usuario no encontrado")
```

7.4 Try Operator

```rust
// Con try (?)
fn process() -> Result<i32, String> {
    let a = try divide(10, 2)?  // Propaga error
    let b = try divide(a, 0)?   // Propaga error
    return Ok(a + b)
}

// Con Option
fn process() -> Option<i32> {
    let a = try find_user(1)?  // Propaga None
    let b = try find_user(2)?  // Propaga None
    return Some(a.len() + b.len())
}
```

7.5 Panic

```rust
// Panic explícito
panic("Algo salió mal")

// Panic con formato
panic("Error en {}: {}", file, line)

// Unwrap que hace panic
let value = result.unwrap()

// Assertions
assert(condition)
assert_eq(a, b)
assert_ne(a, b)
```

7.6 Errores Personalizados

```rust
// Definir error personalizado
enum MyError {
    IoError(String),
    ParseError(String),
    NetworkError(String),
}

// Implementar Error
impl Error for MyError {
    fn description(&self) -> String {
        match self {
            MyError::IoError(msg) => "IO Error: ".to_string() + msg,
            MyError::ParseError(msg) => "Parse Error: ".to_string() + msg,
            MyError::NetworkError(msg) => "Network Error: ".to_string() + msg,
        }
    }
}

// Uso
fn my_function() -> Result<i32, MyError> {
    // ...
}
```

7.7 Errores Encadenados

```rust
// Encadenar errores
fn process() -> Result<i32, Box<Error>> {
    let a = try divide(10, 2)
        .map_err(|e| Box::new(e) as Box<Error>)?  // Convertir
    // ...
}

// Contexto de error
fn process() -> Result<i32, Error> {
    let a = try divide(10, 2)
        .context("Dividiendo 10 entre 2")?  // Añadir contexto
    // ...
}
```

7.8 Errores en Concurrencia

```rust
// Errores en tareas
let result = spawn(async {
    try do_something()?
})

// Manejar error
match await result {
    Ok(value) => print(value),
    Err(e) => print("Error en tarea: " + e.to_string()),
}
