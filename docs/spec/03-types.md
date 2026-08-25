# 3. SISTEMA DE TIPOS DE AXIOM

## 3.1 Tipos Primitivos

```rust
// Enteros con signo
i8   // -128 a 127
i16  // -32768 a 32767
i32  // -2147483648 a 2147483647
i64  // -9223372036854775808 a 9223372036854775807
i128 // -1.7e38 a 1.7e38
isize // Depende de la arquitectura (32/64 bits)

// Enteros sin signo
u8   // 0 a 255
u16  // 0 a 65535
u32  // 0 a 4294967295
u64  // 0 a 18446744073709551615
u128 // 0 a 3.4e38
usize // Depende de la arquitectura (32/64 bits)

// Flotantes
f32  // IEEE 754 single precision
f64  // IEEE 754 double precision

// Booleanos
bool // true o false

// Caracteres
char // Unicode (32 bits)

// Strings
String // Cadena de texto UTF-8
```

3.2 Tipos Compuestos

```rust
// Arrays (tamaño fijo)
let arr: [i32; 5] = [1, 2, 3, 4, 5]

// Slices (referencia a arrays)
let slice: &[i32] = &arr[1..4]

// Tuples
let tuple: (i32, String, f64) = (42, "hello", 3.14)

// Structs
struct Point {
    x: f64,
    y: f64,
}

// Enums
enum Color {
    Red,
    Green,
    Blue,
}
```

3.3 Tipos Especiales

```rust
// Option (valor opcional)
Option<T> = Some(T) | None

// Result (resultado con error)
Result<T, E> = Ok(T) | Err(E)

// Never (nunca retorna)
!   // Tipo de funciones que no retornan

// Any (cualquier tipo)
Any   // Tipo dinámico

// Unit (tipo vacío)
()   // Similar a void en otros lenguajes
```

3.4 Inferencia de Tipos

```rust
// El compilador infiere el tipo
let x = 10           // x es i32
let y = 3.14         // y es f64
let s = "hello"      // s es String
let b = true         // b es bool

// Inferencia en funciones
fn add(a, b) {       // Los tipos se infieren de los usos
    return a + b
}

// Inferencia de retorno
fn multiply(a: i32, b: i32) {
    a * b   // Se infiere que retorna i32
}
```

3.5 Conversión de Tipos

```rust
// Conversión explícita (as)
let x = 10.0
let y = x as i32  // 10

// Conversión segura (into)
let x: i32 = 10
let y: f64 = x.into()

// Parseo de strings
let x: i32 = "42".parse()  // 42
let y: f64 = "3.14".parse()  // 3.14

// Verificación de tipos
if x is i32 {
    // x es i32
}

// Downcasting
if let Some(value) = x.downcast<f64>() {
    // value es f64
}
```

3.6 Tipos Personalizados

```rust
// Struct con genéricos
struct Container<T> {
    value: T,
}

// Enum con genéricos
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Tipo alias
type MyInt = i32
type MyResult = Result<i32, String>

// Nuevo tipo (newtype)
struct Age(i32)  // Age es un nuevo tipo, no es i32
```

3.7 Tipos Recursivos

```rust
// Lista enlazada
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// Árbol binario
struct Node<T> {
    value: T,
    left: Box<Node<T>>,
    right: Box<Node<T>>,
}
```

3.8 Tipos Dependientes

```rust
// Array con tamaño dependiente de tipo
fn create_array<T: Sized>(size: usize) -> [T; size] {
    // ...
}

// Tipos dependientes de const
struct Array<T, const N: usize> {
    data: [T; N],
}
```

3.9 Sistema de Tipos Gradual

AXIOM soporta tipado gradual:

```rust
// Tipado estático (por defecto)
let x: i32 = 10

// Tipado dinámico (opt-in)
let y: dynamic = 10
y = "hello"  // Válido
y = 3.14    // Válido

// Chequeo en tiempo de ejecución
if y is String {
    print(y as String)
}
