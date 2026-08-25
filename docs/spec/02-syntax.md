# 2. SINTAXIS DE AXIOM

## 2.1 Comentarios

```rust
// Comentario de una línea

/// Comentario de documentación (para funciones, tipos, etc.)

/* Comentario 
   de múltiples
   líneas */
```

2.2 Variables

```rust
// Declaración de variables
let x = 10              // Inmutable (por defecto)
mut y = 20              // Mutable
const PI = 3.14159      // Constante en tiempo de compilación

// Tipos explícitos
let x: Int = 10
let y: Float = 3.14
let name: String = "AXIOM"

// Múltiples variables
let (a, b) = (1, 2)

// Shadowing
let x = 10
let x = x + 5   // x ahora es 15
```

2.3 Funciones

```rust
// Función básica
fn add(a: Int, b: Int) -> Int {
    return a + b
}

// Sin valor de retorno (función que no retorna nada)
fn print_message() {
    print("Hola")
}

// Función con parámetros por defecto
fn greet(name: String = "Mundo") {
    print("Hola, " + name)
}

// Función con retorno implícito
fn add(a: Int, b: Int) -> Int {
    a + b   // Sin 'return', la última expresión es el retorno
}

// Función con múltiples retornos
fn divide(a: Int, b: Int) -> (Int, Int) {
    return (a / b, a % b)
}

// Función lambda
let add = |a, b| a + b
let multiply = |a, b| { return a * b }

// Función como parámetro
fn apply(f: fn(Int) -> Int, x: Int) -> Int {
    return f(x)
}
```

2.4 Estructuras de Control

If/Else

```rust
if x > 0 {
    print("Positivo")
} else if x < 0 {
    print("Negativo")
} else {
    print("Cero")
}

// If como expresión
let result = if x > 0 { "positivo" } else { "negativo" }
```

While

```rust
let mut i = 0
while i < 10 {
    print(i)
    i += 1
}
```

For

```rust
// Rango
for i in 0..10 {
    print(i)
}

// Iterador
for item in collection {
    print(item)
}

// Con índice
for (i, item) in collection.enumerate() {
    print(i, item)
}
```

Loop

```rust
loop {
    // Bucle infinito
    if condition {
        break
    }
}
```

Match (Pattern Matching)

```rust
match value {
    0 => print("Cero"),
    1 => print("Uno"),
    2..=10 => print("Dos a diez"),
    _ => print("Otro valor"),
}
```

2.5 Estructuras (Structs)

```rust
// Definición
struct Point {
    x: Float,
    y: Float,
}

// Instanciación
let point = Point { x: 10.0, y: 20.0 }

// Acceso
print(point.x)

// Métodos
impl Point {
    fn distance(&self, other: Point) -> Float {
        return ((self.x - other.x).pow(2) + 
                (self.y - other.y).pow(2)).sqrt()
    }
    
    fn new(x: Float, y: Float) -> Self {
        return Self { x, y }
    }
}

// Tupla Struct
struct Color(u8, u8, u8)
let color = Color(255, 0, 0)

// Unit Struct
struct Unit
```

2.6 Enums

```rust
// Definición
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Uso
let result = Result::Ok(42)
let error = Result::Err("Algo salió mal")

// Enum con valores asociados
enum Status {
    Active,
    Inactive,
    Pending(String),  // Con datos
}

// Métodos en enum
impl Status {
    fn is_active(&self) -> Bool {
        match self {
            Status::Active => true,
            _ => false,
        }
    }
}
```

2.7 Unions

```rust
// Unión (como C)
union Value {
    int: Int,
    float: Float,
    string: String,
}
```

2.8 Arrays y Slices

```rust
// Array de tamaño fijo
let arr: [Int; 5] = [1, 2, 3, 4, 5]

// Array con valor por defecto
let arr = [0; 10]   // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

// Slice (referencia a parte de un array)
let slice = &arr[1..4]  // [2, 3, 4]

// Acceso
let first = arr[0]
arr[0] = 10  // Solo si es mutable
```

2.9 Tuples

```rust
let tuple = (1, "hello", 3.14)
let (a, b, c) = tuple
let first = tuple.0
```

2.10 Strings

```rust
// String literal
let s = "Hola, mundo!"

// String con interpolación
let name = "AXIOM"
let greeting = "Hola, \(name)"  // "Hola, AXIOM"

// Strings multilínea
let multiline = """
    Línea 1
    Línea 2
    Línea 3
"""

// Raw strings
let raw = r"C:\path\to\file"
```

2.11 Operadores

```rust
// Aritméticos
+   -   *   /   %

// Comparación
==  !=  <  >  <=  >=

// Lógicos
&&  ||  !

// Bitwise
&   |   ^   <<   >>

// Asignación
=   +=  -=  *=  /=  %=

// Rango
..  ..=  (exclusivo e inclusivo)

// Acceso
.   ::  (struct y namespace)

// Tipo
:   (declaración de tipo)
```

2.12 Precedencia de Operadores

```rust
// Mayor precedencia a menor
()  // Agrupación
.  ::  // Acceso
-  !  // Unario
*  /  %  // Multiplicativo
+  -  // Aditivo
<<  >>  // Desplazamiento
&  // Bitwise AND
^  // Bitwise XOR
|  // Bitwise OR
==  !=  <  >  <=  >=  // Comparación
&&  // Lógico AND
||  // Lógico OR
=  +=  -=  // Asignación
```

2.13 Palabras Clave

```rust
// Declaraciones
fn, let, mut, const, struct, enum, union, trait, impl, type, mod, use

// Control de flujo
if, else, while, for, loop, match, return, break, continue

// Concurrencia
async, await, spawn, channel, select

// Visibilidad
pub, pub(crate), pub(super), pub(in path)

// Especiales
self, super, crate, extern, unsafe, move, static, abstract, final, override

// Literales
true, false, null

// Manejo de errores
try, catch, throw, finally
```

2.14 Convenciones de Nomenclatura

```rust
// Variables y funciones: snake_case
let my_variable = 10
fn my_function() { }

// Tipos y traits: PascalCase
struct MyStruct
enum MyEnum
trait MyTrait

// Constantes: SCREAMING_SNAKE_CASE
const MAX_SIZE: Int = 100

// Módulos: snake_case
mod my_module

// Variables privadas: _prefijo
let _private = 10

// Atributos: #[attribute]
#[derive(Debug)]
struct Point { ... }
