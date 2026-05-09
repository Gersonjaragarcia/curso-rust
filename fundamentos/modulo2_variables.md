# MÓDULO 2 — Variables y Tipos de Datos

# Introducción

En este módulo aprenderás los fundamentos de las variables y los tipos de datos en :contentReference[oaicite:0]{index=0}. Estos conceptos son esenciales porque permiten almacenar, modificar y manipular información dentro de un programa.

Rust utiliza un sistema de tipos estático y seguro, lo que ayuda a prevenir muchos errores comunes durante la compilación.

---

# Objetivos del Módulo

Al finalizar este módulo podrás:

- Crear variables
- Entender mutabilidad e inmutabilidad
- Trabajar con distintos tipos de datos
- Utilizar strings, arrays y tuplas
- Comprender el tipado estático
- Realizar conversiones de tipos
- Crear pequeños programas usando datos

---

# 1. Variables en Rust

Las variables permiten almacenar información en memoria.

En Rust se crean utilizando la palabra:

```rust
let
```

---

# Ejemplo Básico

```rust
fn main() {
    let nombre = "Carlos";

    println!("{}", nombre);
}
```

---

# Explicación

## let

Crea una variable.

---

## nombre

Es el identificador de la variable.

---

## "Carlos"

Valor almacenado.

---

## println!

Imprime información en pantalla.

---

# 2. Variables Inmutables

Por defecto, las variables en Rust son inmutables.

Esto significa que NO pueden cambiar su valor.

---

# Ejemplo

```rust
fn main() {
    let edad = 18;

    println!("{}", edad);
}
```

---

# Intentar Modificar una Variable

```rust
fn main() {
    let edad = 18;

    edad = 20;
}
```

Esto producirá un error.

---

# ¿Por Qué Rust Hace Esto?

Rust prioriza:
- seguridad
- estabilidad
- prevención de errores

Las variables inmutables ayudan a evitar cambios accidentales.

---

# 3. Variables Mutables

Si deseas modificar una variable debes usar:

```rust
mut
```

---

# Ejemplo

```rust
fn main() {
    let mut edad = 18;

    edad = 20;

    println!("{}", edad);
}
```

---

# Explicación

## mut

Permite cambiar el valor de una variable.

---

# 4. Constantes

Las constantes nunca cambian.

Se crean usando:

```rust
const
```

---

# Ejemplo

```rust
const PI: f64 = 3.1416;

fn main() {
    println!("{}", PI);
}
```

---

# Explicación

## const

Define constantes.

---

## f64

Tipo numérico decimal.

---

# Diferencia Entre let y const

| Característica | let | const |
|---|---|---|
| Puede cambiar | Sí, con mut | No |
| Tipo obligatorio | No | Sí |
| Disponible globalmente | No | Sí |

---

# 5. Shadowing

Rust permite redeclarar variables.

Esto se llama:

## Shadowing

---

# Ejemplo

```rust
fn main() {
    let numero = 10;

    let numero = numero + 5;

    println!("{}", numero);
}
```

Resultado:

```text
15
```

---

# Diferencia Entre mut y Shadowing

| mut | Shadowing |
|---|---|
| Modifica el valor | Crea nueva variable |
| Mismo tipo normalmente | Puede cambiar tipo |

---

# 6. Tipos de Datos

Rust tiene múltiples tipos de datos.

Se dividen en:

- escalares
- compuestos

---

# 7. Tipos Escalares

Los tipos escalares almacenan un único valor.

---

# Enteros

## Ejemplo

```rust
fn main() {
    let numero: i32 = 25;

    println!("{}", numero);
}
```

---

# Tipos Enteros Comunes

| Tipo | Tamaño |
|---|---|
| i8 | 8 bits |
| i16 | 16 bits |
| i32 | 32 bits |
| i64 | 64 bits |

---

# Explicación

La letra:
- `i` significa integer con signo
- `u` significa unsigned

---

# 8. Números Decimales

```rust
fn main() {
    let precio: f64 = 19.99;

    println!("{}", precio);
}
```

---

# Tipos Float

| Tipo | Tamaño |
|---|---|
| f32 | 32 bits |
| f64 | 64 bits |

---

# 9. Booleanos

Solo pueden tener:
- true
- false

---

# Ejemplo

```rust
fn main() {
    let activo = true;

    println!("{}", activo);
}
```

---

# 10. Caracteres

Se representan con comillas simples.

```rust
fn main() {
    let letra = 'A';

    println!("{}", letra);
}
```

---

# 11. Strings

Las cadenas de texto usan comillas dobles.

```rust
fn main() {
    let mensaje = "Hola Rust";

    println!("{}", mensaje);
}
```

---

# Diferencia Importante

| Tipo | Usa |
|---|---|
| char | 'A' |
| string | "Hola" |

---

# 12. Tuplas

Las tuplas agrupan múltiples valores.

---

# Ejemplo

```rust
fn main() {
    let persona = ("Carlos", 20, true);

    println!("{}", persona.0);
}
```

---

# Acceso a Datos

```text
persona.0
persona.1
persona.2
```

---

# 13. Arrays

Los arrays almacenan múltiples elementos del mismo tipo.

---

# Ejemplo

```rust
fn main() {
    let numeros = [1, 2, 3, 4, 5];

    println!("{}", numeros[0]);
}
```

---

# Explicación

## numeros[0]

Accede al primer elemento.

---

# 14. Tipado Estático

Rust conoce el tipo de cada variable en tiempo de compilación.

---

# Ejemplo

```rust
let edad: i32 = 20;
```

---

# Ventajas

- más seguridad
- menos errores
- mejor rendimiento

---

# 15. Conversión de Tipos

A veces necesitamos convertir tipos.

---

# Ejemplo

```rust
fn main() {
    let numero = 10;

    let decimal = numero as f64;

    println!("{}", decimal);
}
```

---

# Explicación

## as

Convierte un tipo en otro.

---

# 16. Operaciones Matemáticas

```rust
fn main() {
    let suma = 10 + 5;
    let resta = 10 - 5;
    let multiplicacion = 10 * 5;
    let division = 10 / 5;

    println!("{}", suma);
    println!("{}", resta);
    println!("{}", multiplicacion);
    println!("{}", division);
}
```

---

# 17. Ejercicios del Módulo

## Ejercicio 1

Crear variables:
- nombre
- edad
- altura

E imprimirlas.

---

## Ejercicio 2

Crear una variable mutable y cambiar su valor.

---

## Ejercicio 3

Crear:
- una tupla
- un array

Mostrar sus datos.

---

## Ejercicio 4

Realizar operaciones matemáticas básicas.

---

# 18. Retos

## Reto 1 — Perfil Completo

Crear un programa que muestre:
- nombre
- edad
- país
- altura
- si estudia programación

---

## Reto 2 — Mini Calculadora

Crear variables numéricas y mostrar:
- suma
- resta
- multiplicación
- división

---

# 19. Proyecto del Módulo

# Sistema de Información Personal

## Objetivo

Crear un programa que almacene y muestre información completa de una persona usando:
- variables
- arrays
- tuplas
- strings
- booleanos

---

# Ejemplo

```rust
fn main() {
    let nombre = "Ana";
    let edad = 22;
    let altura = 1.68;
    let estudiante = true;

    let hobbies = ["Leer", "Programar", "Videojuegos"];

    println!("===== PERFIL =====");
    println!("Nombre: {}", nombre);
    println!("Edad: {}", edad);
    println!("Altura: {}", altura);
    println!("Estudiante: {}", estudiante);

    println!("Hobby favorito: {}", hobbies[0]);
}
```

---

# 20. Errores Comunes

## Modificar variable inmutable

Incorrecto:

```rust
let edad = 20;

edad = 25;
```

---

# Correcto

```rust
let mut edad = 20;

edad = 25;
```

---

## Usar tipos incompatibles

Incorrecto:

```rust
let edad = "20" + 5;
```

---

# 21. Resumen del Módulo

Aprendiste:

- Variables
- Mutabilidad
- Constantes
- Shadowing
- Enteros
- Floats
- Booleanos
- Strings
- Arrays
- Tuplas
- Conversión de tipos
- Operaciones matemáticas

---

# Próximo Módulo

# MÓDULO 3 — Control de Flujo

Aprenderás:
- if
- else
- match
- loops
- while
- for
- toma de decisiones
- repetición de tareas