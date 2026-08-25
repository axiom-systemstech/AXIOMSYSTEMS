# 4. MEMORIA Y OWNERSHIP

## 4.1 Filosofía de Memoria

AXIOM usa un sistema de **ownership** inspirado en Rust:

- Cada valor tiene un **dueño** (owner)
- Solo un dueño a la vez
- Cuando el dueño sale de ámbito, el valor se libera
- Sin garbage collector
- Sin data races

## 4.2 Ownership

```rust
// Cada variable es dueña de su valor
let x = 10        // x es dueño de 10
let y = x         // x se mueve a y, x ya no es válido

// Los tipos primitivos se copian (Copy)
let x = 10        // x es dueño de 10
let y = x         // y es dueño de una copia de 10
// x sigue siendo válido

// Los tipos compuestos se mueven (Move)
let s = String("hello")  // s es dueño del string
let t = s                // s se mueve a t
// s ya no es válido
// println(s)  // ERROR: s fue movido
```

4.3 Préstamos (Borrowing)

```rust
// Préstamo inmutable
fn use_reference(x: &i32) {
    print(x)  // Solo lectura
}

let x = 10
use_reference(&x)  // Prestar x
// x sigue siendo dueño del valor

// Préstamo mutable
fn modify_reference(x: &mut i32) {
    *x += 1  // Modificar el valor
}

let mut x = 10
modify_reference(&mut x)  // Prestar mutablemente
// x ahora es 11

// Reglas de préstamo:
// 1. Puedes tener múltiples préstamos inmutables
// 2. O puedes tener un solo préstamo mutable
// 3. No puedes tener ambos al mismo tiempo
```

4.4 Ciclo de Vida (Lifetimes)

```rust
// Función con lifetimes explícitos
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct con lifetimes
struct Ref<'a, T> {
    data: &'a T,
}

// Elision de lifetimes (reglas automáticas)
fn first(x: &str, y: &str) -> &str {  // El compilador infiere
    if x.len() > 0 { x } else { y }
}
```

4.5 Mutabilidad

```rust
// Variables inmutables (por defecto)
let x = 10
// x = 20  // ERROR: no se puede mutar

// Variables mutables
mut let y = 10
y = 20  // OK

// Mutabilidad en estructuras
struct Point {
    x: f64,
    y: f64,
}

let p = Point { x: 1.0, y: 2.0 }
// p.x = 3.0  // ERROR

mut let p2 = Point { x: 1.0, y: 2.0 }
p2.x = 3.0  // OK
```

4.6 Smart Pointers

```rust
// Box (heap allocation)
let x = Box::new(10)
// x está en el heap

// Rc (Reference Counted)
let x = Rc::new(10)
let y = Rc::clone(&x)  // Incrementar contador

// Arc (Atomic Reference Counted)
let x = Arc::new(10)
let y = Arc::clone(&x)  // Thread-safe

// RefCell (mutabilidad interior)
let x = RefCell::new(10)
*x.borrow_mut() += 1

// Weak (referencia débil)
let x = Rc::new(10)
let y = Rc::downgrade(&x)  // Referencia débil
```

4.7 Gestión de Memoria Manual

```rust
// Asignación manual
let ptr = unsafe {
    allocate(1024)  // Asignar 1KB
}

// Liberación manual
unsafe {
    deallocate(ptr)
}

// Scope de memoria
use std::mem::scope;
scope! {
    // Memoria limitada a este scope
    let x = 10
    // x se libera al final del scope
}
```

4.8 Copy vs Move

```rust
// Tipos Copy (se copian automáticamente)
// Todos los primitivos
// Tuples de primitivos
// Arrays de primitivos
// Structs con todos los campos Copy

// Tipos Move (se mueven)
// String
// Vec
// Structs con campos que no son Copy
// Enums

// Copy explícito
#[derive(Copy)]
struct Point { x: i32, y: i32 }

// Clone explícito
#[derive(Clone)]
struct MyType { data: String }
```

4.9 Drop

```rust
// Drop (destructor)
impl Drop for MyType {
    fn drop(&mut self) {
        // Limpiar recursos
        println!("Liberando recursos")
    }
}

// Drop temprano
let x = MyType::new()
drop(x)  // Liberar antes del final del scope
