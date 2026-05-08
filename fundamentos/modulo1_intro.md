# MÓDULO 1 — Introducción a Rust
## Capítulo 1 — ¿Qué es Rust?

---
1. *Introducción*

Rust es un lenguaje de programación moderno enfocado en:

 - Alto rendimiento
 - Seguridad de memoria
 - Concurrencia segura
 - Velocidad comparable con C y C++
 - Prevención de errores comunes

Fue creado originalmente por Graydon Hoare y posteriormente desarrollado por Mozilla Foundation.


Actualmente Rust es utilizado en:

 - Sistemas operativos
 - Motores de videojuegos
 - Backend de alto rendimiento
 - Blockchain
 - Herramientas de seguridad
 - WebAssembly
 - Inteligencia artificial

---
2. ## ¿Por Qué Rust es Tan Popular?
Problemas de otros lenguajes

Muchos lenguajes rápidos como:

 - C
 - C++

permiten errores peligrosos:

 - acceso inválido a memoria
 - punteros nulos
 - fugas de memoria
 - data races

Rust evita estos problemas SIN usar recolector de basura (Garbage Collector).

---

# Filosofía de Rust

### Rust busca:

*Seguridad*

Evitar errores críticos en tiempo de compilación.

*Velocidad*

Generar programas extremadamente rápidos.

*Concurrencia segura*

Permitir múltiples tareas simultáneas sin errores de memoria.

*Control total*

El programador controla memoria y rendimiento.

---

# 4. Comparación con Otros Lenguajes
| Lenguaje | Velocidad | Seguridad Memoria | Fácil de Aprender |
|---       |---        |---                |---                |
| Python   | Media     | Alta              | Muy fácil         |
| JavaScript| Media    | Alta              | Fácil             |
| C++      | Muy alta  | Baja              | Difícil           |
| Rust     | Muy alta  | Muy alta          | Intermedia        |

--- 

# 5. Instalación de Rust
*Windows*
**Descargar:**
 - Rustup
**Desde:**
 - [Rust](https://rust-lang.org/es/)

---

### Linux / macOS
Abrir terminal: Usa `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

---

# 6. Verificar Instalación
Abrir terminal y escribir: `rustc --version`

Debe aparecer algo parecido a: `rustc 1.xx.x`

---

### 7. ¿Qué es Cargo?

Cargo es el sistema oficial de Rust para:

 - crear proyectos
 - compilar
 - instalar librerías
 - ejecutar programas
 - manejar dependencias
Es una de las herramientas más poderosas del ecosistema Rust.

---
# 8. Crear Tu Primer Proyecto
**Crear proyecto**
 `cargo new nombre_proyecto`

**Entrar a la carpeta**
`cd hola_rust`

**Ejecutar el programa**
`cargo run`

---

# 9. Estructura del Proyecto
Cuando creas un proyecto aparece:
```
hola_rust/
│
├── Cargo.toml
└── src/
    └── main.rs
```

---
# 10. Explicación de Archivos
*Cargo.toml*

*Contiene:*
 - nombre del proyecto
 - versión
 - dependencias

*Ejemplo:*
```
[package]
name = "hola_rust"
version = "0.1.0"
edition = "2021"
```
*main.rs*
Archivo principal del programa.

---

# 11. Tu Primer Programa

*Código:*
```
fn main() {
    println!("Hola mundo");
}
```
---
# 12. Explicación Detallada
*fn*

Significa:

 - function (función)

*main*

Es la función principal del programa.

Todo programa Rust inicia aquí.

*println!*

Sirve para imprimir texto en pantalla.

*"Hola mundo"*

Cadena de texto (string literal).

---
# 13. Compilación

Rust es compilado.
Proceso:
```
Código Rust
↓
Compilador
↓
Ejecutable
```

Ventajas:

 - programas rápidos
 - menos errores en ejecución


