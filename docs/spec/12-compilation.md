# 12. COMPILACIÓN

## 12.1 Flujo de Compilación

```

Código AXIOM (.ax)
↓
Lexer
↓
Parser (AST)
↓
Análisis Semántico
↓
Optimización (IR)
↓
Generación de Código
↓
Código Nativo / Bytecode
↓
Ejecutable (.axexe)

```

## 12.2 Modos de Compilación

```rust
// Debug (desarrollo)
axiom build --debug
// - Sin optimizaciones
// - Con símbolos de debug
// - Más rápido de compilar

// Release (producción)
axiom build --release
// - Optimizaciones agresivas
// - Sin símbolos de debug
// - Más lento de compilar pero más rápido de ejecutar
```

12.3 Targets

```rust
// Targets soportados
--target x86_64-unknown-linux-gnu     // Linux x64
--target x86_64-apple-darwin          // macOS x64
--target arm64-apple-darwin           // macOS ARM
--target x86_64-pc-windows-msvc      // Windows x64
--target wasm32-unknown-unknown       // WebAssembly

// Cross-compilation
axiom build --target arm64-apple-darwin --arch arm64
```

12.4 Optimizaciones

```rust
// Nivel de optimización
-O0  // Sin optimización (debug)
-O1  // Optimización básica
-O2  // Optimización media (default)
-O3  // Optimización máxima
-Os  // Optimizar para tamaño
-Oz  // Optimizar para tamaño extremo

// Optimizaciones específicas
--lto                    // Link-Time Optimization
--codegen-units 1        // Una sola unidad de compilación
--strip                 // Eliminar símbolos
--debug-assertions      // Mantener assertions en release
```

12.5 Bytecode vs Nativo

```rust
// Compilar a bytecode (VM)
axiom build --bytecode
// Genera archivo .axbc (bytecode AXIOM)

// Compilar a nativo
axiom build --native
// Genera ejecutable nativo

// Híbrido (bytecode + JIT)
axiom build --hybrid
// Genera bytecode con compilación JIT
```

12.6 Compilación Incremental

```rust
// Compilación incremental
axiom build
// Solo recompila archivos modificados

// Limpiar build
axiom clean

// Forzar recompilación completa
axiom build --force
```

12.7 Perfilado

```rust
// Perfilado de tiempo
axiom build --timing
// Muestra tiempos de compilación

// Perfilado de memoria
axiom build --memory
// Muestra uso de memoria

// Perfilado de código generado
axiom build --emit-asm
// Genera archivo .asm con ensamblador
```

12.8 Linting

```rust
// Analizar código
axiom check

// Lint con corrección automática
axiom fix

// Lint específico
axiom check --warnings
axiom check --no-warnings
