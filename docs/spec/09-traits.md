# 9. TRAITS E INTERFACES

## 9.1 Definición de Traits

```rust
// Trait básico
trait Printable {
    fn print(&self)
}

// Trait con múltiples métodos
trait Comparable {
    fn compare(&self, other: &Self) -> i32
    fn is_equal(&self, other: &Self) -> bool {
        return self.compare(other) == 0
    }
}

// Trait con tipos asociados
trait Collection {
    type Item
    
    fn add(&mut self, item: Self::Item)
    fn len(&self) -> usize
    fn get(&self, index: usize) -> Option<&Self::Item>
}
```

9.2 Implementación de Traits

```rust
// Implementar trait para tipo
impl Printable for i32 {
    fn print(&self) {
        // Imprimir como i32
    }
}

impl Printable for String {
    fn print(&self) {
        // Imprimir como String
    }
}

// Implementar para struct
struct Point { x: f64, y: f64 }

impl Printable for Point {
    fn print(&self) {
        print("Point(" + self.x + ", " + self.y + ")")
    }
}
```

9.3 Traits y Genéricos

```rust
// Función con trait bound
fn print_value<T: Printable>(value: T) {
    value.print()
}

// Múltiples bounds
fn process<T: Printable + Clone>(value: T) {
    let cloned = value.clone()
    value.print()
    cloned.print()
}

// Where clause
fn process<T, U>(a: T, b: U) 
where
    T: Printable,
    U: Clone + Printable,
{
    a.print()
    let b_clone = b.clone()
    b_clone.print()
}
```

9.4 Traits de Biblioteca

```rust
// Clone
#[derive(Clone)]
struct Point { x: f64, y: f64 }

// Copy (solo para tipos que se copian)
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

// Debug
#[derive(Debug)]
struct Point { x: f64, y: f64 }

// Display
impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> String {
        return "Point(" + self.x + ", " + self.y + ")"
    }
}

// Default
impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
```

9.5 Traits en Objetos

```rust
// Trait object
trait Drawable {
    fn draw(&self)
}

// Usar trait object
fn draw_all(drawables: &[&Drawable]) {
    for d in drawables {
        d.draw()
    }
}

// Box de trait
let drawable: Box<Drawable> = Box::new(Circle::new())
drawable.draw()
```

9.6 Traits Auto

```rust
// Auto-trait (marcador)
trait Send { }    // Se puede enviar entre hilos
trait Sync { }    // Se puede compartir entre hilos
trait Unpin { }   // Se puede mover después de pinned
trait Sized { }   // Tamaño conocido en compilación

// Marcos automáticos para tipos
struct MyType { data: i32 }  // Send + Sync + Unpin + Sized
```

9.7 Traits y Polimorfismo

```rust
// Polimorfismo en tiempo de compilación (generics)
fn process<T: Printable>(value: T) {
    value.print()
}

// Polimorfismo en tiempo de ejecución (trait objects)
fn process(value: &Printable) {
    value.print()
}

// Polimorfismo con Box
let values: Vec<Box<Printable>> = vec![
    Box::new(1),
    Box::new("hello"),
    Box::new(Point { x: 1.0, y: 2.0 }),
]
for v in values {
    v.print()
}
```

9.8 Supertraits

```rust
// Trait que extiende otro
trait Printable {
    fn print(&self)
}

trait Debuggable: Printable {
    fn debug(&self) {
        self.print()
    }
}

// Implementar supertrait
struct MyType

impl Printable for MyType {
    fn print(&self) {
        print("MyType")
    }
}

impl Debuggable for MyType {
    // No necesita implementar debug (tiene implementación por defecto)
}
