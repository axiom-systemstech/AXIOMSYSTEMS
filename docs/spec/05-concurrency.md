# 5. CONCURRENCIA EN AXIOM

## 5.1 Filosofía de Concurrencia

AXIOM usa un modelo de **actores** combinado con **async/await**:

- **Actores**: Unidades de concurrencia aisladas
- **Async/Await**: Programación asíncrona ligera
- **Canales**: Comunicación entre actores
- **Sin data races**: Garantizado por el sistema de tipos

## 5.2 Async/Await

```rust
// Función asíncrona
async fn fetch_data(url: String) -> String {
    let response = await http::get(url)
    return response.body
}

// Llamada asíncrona
fn main() {
    let data = await fetch_data("https://api.axion")
    print(data)
}

// Múltiples tareas asíncronas
let tasks = [
    fetch_data("https://api1.axion"),
    fetch_data("https://api2.axion"),
]
let results = await join_all(tasks)
```

5.3 Actores

```rust
// Definir un actor
actor Counter {
    value: i32,
    
    fn new() -> Self {
        Self { value: 0 }
    }
    
    // Mensajes (métodos async)
    async fn increment(&mut self) {
        self.value += 1
    }
    
    async fn get_value(&self) -> i32 {
        return self.value
    }
}

// Usar el actor
fn main() {
    let counter = Counter::new()
    
    // Enviar mensajes
    await counter.increment()
    await counter.increment()
    
    let value = await counter.get_value()
    print(value)  // 2
}
```

5.4 Canales

```rust
// Crear canal
let (sender, receiver) = channel()

// Enviar mensaje
sender.send(42)

// Recibir mensaje
let value = receiver.recv()  // 42

// Canal con buffer
let (sender, receiver) = channel(10)  // buffer de 10

// Canal de múltiples productores
let sender1 = sender.clone()
let sender2 = sender.clone()

// Select (esperar múltiples canales)
select! {
    val = receiver1.recv() => print(val),
    val = receiver2.recv() => print(val),
    timeout(1000) => print("Timeout"),
}
```

5.5 Tareas (Tasks)

```rust
// Spawn de tarea
let task = spawn(async {
    // Código en paralelo
    return 42
})

// Esperar tarea
let result = await task  // 42

// Tarea en background
spawn(async {
    loop {
        await sleep(1000)
        print("Tick")
    }
})
```

5.6 Sincronización

```rust
// Mutex (exclusión mutua)
let mutex = Mutex::new(0)

{
    let mut guard = mutex.lock()
    *guard += 1
}

// RwLock (lectura-escritura)
let rwlock = RwLock::new(0)

{
    let read_guard = rwlock.read()  // Múltiples lectores
    let value = *read_guard
}

{
    let mut write_guard = rwlock.write()  // Un solo escritor
    *write_guard += 1
}

// Barrier (sincronización de hilos)
let barrier = Barrier::new(5)
barrier.wait()  // Esperar a 5 hilos

// Condvar (variable de condición)
let (lock, cond) = (Mutex::new(false), Condvar::new())
cond.wait_while(lock.lock(), |ready| !*ready)
```

5.7 Atomicos

```rust
// Operaciones atómicas
let counter = AtomicI32::new(0)
counter.fetch_add(1, Ordering::SeqCst)

// Orderings
Ordering::Relaxed  // Sin barreras de memoria
Ordering::Acquire  // Barrier de carga
Ordering::Release  // Barrier de almacenamiento
Ordering::AcqRel   // Ambas
Ordering::SeqCst   // Secuencialmente consistente
```

5.8 Threads

```rust
// Thread nativo
thread::spawn(|| {
    // Código en otro hilo
    print("Hilo ejecutándose")
})

// Thread con nombre
thread::Builder::new()
    .name("mi-hilo".to_string())
    .spawn(|| { /* ... */ })

// Esperar hilos
let handle = thread::spawn(|| 42)
let result = handle.join()  // 42
```

5.9 Concurrencia Segura

```rust
// Send (se puede enviar entre hilos)
// Sync (se puede compartir entre hilos)

// Tipos que son Send + Sync
i32  // Sí
String  // Sí
Rc<T>  // No (no es Send)
Arc<T>  // Sí (es Send + Sync)

// Garantizar Send
struct MyType {
    data: Arc<i32>,  // Thread-safe
}

// Garantizar Sync
struct MyType {
    data: Mutex<i32>,  // Thread-safe
}
