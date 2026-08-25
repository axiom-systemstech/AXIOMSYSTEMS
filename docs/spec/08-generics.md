# 8. GENÉRICOS

## 8.1 Funciones Genéricas

```rust
// Función genérica básica
fn identity<T>(value: T) -> T {
    return value
}

// Usar
let x = identity(10)        // T = i32
let y = identity("hello")   // T = &str

// Con múltiples parámetros de tipo
fn swap<T, U>(a: T, b: U) -> (U, T) {
    return (b, a)
}

// Con bounds
fn print_value<T: Printable>(value: T) {
    value.print()
}
```

8.2 Structs Genéricos

```rust
// Struct genérico
struct Container<T> {
    value: T,
}

// Usar
let container = Container { value: 10 }
let container2 = Container { value: "hello" }

// Struct con múltiples genéricos
struct Pair<T, U> {
    first: T,
    second: U,
}

// Usar
let pair = Pair { first: 10, second: "hello" }
```

8.3 Enums Genéricos

```rust
// Enum genérico
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Usar
let result: Result<i32, String> = Result::Ok(42)
let error: Result<i32, String> = Result::Err("Error".to_string())
```

8.4 Impl Genéricos

```rust
// Implementación genérica
impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
    
    fn get_value(&self) -> &T {
        &self.value
    }
}

// Implementación específica
impl Container<i32> {
    fn double(&self) -> i32 {
        self.value * 2
    }
}
```

8.5 Traits con Genéricos

```rust
// Trait genérico
trait Convert<T> {
    fn convert(&self) -> T
}

// Implementación
impl Convert<f64> for i32 {
    fn convert(&self) -> f64 {
        *self as f64
    }
}

impl Convert<i32> for f64 {
    fn convert(&self) -> i32 {
        *self as i32
    }
}
```

8.6 Bounds de Tipo

```rust
// Trait bounds
fn process<T: Printable + Serializable>(value: T) {
    value.print()
    let data = value.serialize()
}

// Where clauses
fn process<T, U>(a: T, b: U) 
where
    T: Printable,
    U: Serializable,
{
    // ...
}

// Lifetime bounds
fn longest<'a, T: ?Sized + 'a>(x: &'a T, y: &'a T) -> &'a T {
    if x.len() > y.len() { x } else { y }
}
```

8.7 Const Generics

```rust
// Constante como tipo
struct Array<T, const N: usize> {
    data: [T; N],
}

// Usar
let arr = Array::<i32, 5> { data: [1, 2, 3, 4, 5] }

// Función con const
fn create_array<T, const N: usize>(value: T) -> [T; N] {
    [value; N]
}
```

8.8 Asociados de Tipo

```rust
// Trait con tipos asociados
trait Container {
    type Item
    
    fn add(&mut self, item: Self::Item)
    fn get(&self, index: usize) -> Option<&Self::Item>
}

// Implementación
struct List<T> {
    data: Vec<T>,
}

impl<T> Container for List<T> {
    type Item = T
    
    fn add(&mut self, item: T) {
        self.data.push(item)
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}
