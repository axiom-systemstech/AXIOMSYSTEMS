# 13. EJEMPLOS COMPLETOS

## 13.1 Hello World

```rust
// Archivo: hello.ax
fn main() {
    println("¡Hola, mundo!")
}
```

13.2 Fibonacci

```rust
// Archivo: fibonacci.ax
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let n = 10
    let result = fibonacci(n)
    println("Fibonacci(" + n.to_string() + ") = " + result.to_string())
}
```

13.3 Factorial

```rust
// Archivo: factorial.ax
fn factorial(n: i32) -> i32 {
    let mut result = 1
    for i in 1..=n {
        result *= i
    }
    return result
}

fn main() {
    let n = 5
    let result = factorial(n)
    println(n.to_string() + "! = " + result.to_string())
}
```

13.4 Estructuras

```rust
// Archivo: point.ax
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    fn distance(&self, other: Point) -> f64 {
        return ((self.x - other.x).pow(2) + 
                (self.y - other.y).pow(2)).sqrt()
    }
}

fn main() {
    let p1 = Point::new(0.0, 0.0)
    let p2 = Point::new(3.0, 4.0)
    let dist = p1.distance(p2)
    println("Distancia: " + dist.to_string())
}
```

13.5 Enums y Match

```rust
// Archivo: status.ax
enum Status {
    Active,
    Inactive,
    Pending(String),
}

fn main() {
    let status = Status::Pending("Esperando aprobación".to_string())
    
    match status {
        Status::Active => println("Activo"),
        Status::Inactive => println("Inactivo"),
        Status::Pending(reason) => println("Pendiente: " + reason),
    }
}
```

13.6 Concurrencia

```rust
// Archivo: concurrent.ax
async fn fetch_data(url: String) -> String {
    // Simular descarga
    return "Datos de " + url
}

fn main() {
    let tasks = [
        fetch_data("https://api1.axion"),
        fetch_data("https://api2.axion"),
    ]
    
    let results = await join_all(tasks)
    for result in results {
        println(result)
    }
}
```

13.7 Errores

```rust
// Archivo: errors.ax
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("División por cero".to_string())
    }
    return Ok(a / b)
}

fn main() {
    let result = divide(10, 2)
    match result {
        Ok(value) => println("Resultado: " + value.to_string()),
        Err(error) => println("Error: " + error),
    }
}
```

13.8 Generics

```rust
// Archivo: generics.ax
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
    
    fn get_value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let c1 = Container::new(10)
    let c2 = Container::new("Hola")
    
    println("c1: " + c1.get_value().to_string())
    println("c2: " + c2.get_value())
}
```

13.9 Traits

```rust
// Archivo: traits.ax
trait Printable {
    fn print(&self)
}

impl Printable for i32 {
    fn print(&self) {
        println(self.to_string())
    }
}

impl Printable for String {
    fn print(&self) {
        println(self)
    }
}

fn print_all<T: Printable>(items: &[T]) {
    for item in items {
        item.print()
    }
}

fn main() {
    let numbers = [1, 2, 3]
    let strings = ["a", "b", "c"]
    
    print_all(&numbers)
    print_all(&strings)
}
```

13.10 Módulos

```rust
// Archivo: math.ax
pub fn add(a: i32, b: i32) -> i32 {
    return a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    return a - b
}

// Archivo: main.ax
mod math

fn main() {
    let sum = math::add(10, 20)
    let diff = math::sub(20, 10)
    println("Suma: " + sum.to_string())
    println("Resta: " + diff.to_string())
}
