# MÓDULO 3 — Control de Flujo

# Introducción

El control de flujo permite que un programa tome decisiones y repita tareas automáticamente.

Gracias al control de flujo, un programa puede:

- ejecutar diferentes acciones según condiciones
- repetir instrucciones múltiples veces
- crear lógica compleja
- automatizar procesos

En :contentReference[oaicite:0]{index=0} existen múltiples estructuras para controlar el flujo del programa.

---

# Objetivos del Módulo

Al finalizar este módulo podrás:

- utilizar if y else
- trabajar con match
- crear loops
- usar while y for
- controlar repeticiones
- tomar decisiones dentro de programas
- crear pequeños sistemas interactivos

---

# 1. La Estructura if

La estructura `if` permite ejecutar código solo si una condición es verdadera.

---

# Sintaxis

```rust
if condicion {
    // código
}
```

---

# Ejemplo

```rust
fn main() {

    let edad = 20;

    if edad >= 18 {
        println!("Eres mayor de edad");
    }
}
```

---

# Explicación

## edad >= 18

Es una condición lógica.

---

# Operadores Relacionales

| Operador | Significado |
|---|---|
| > | mayor que |
| < | menor que |
| >= | mayor o igual |
| <= | menor o igual |
| == | igual |
| != | diferente |

---

# 2. if y else

`else` ejecuta código cuando la condición es falsa.

---

# Ejemplo

```rust
fn main() {

    let edad = 15;

    if edad >= 18 {
        println!("Mayor de edad");
    } else {
        println!("Menor de edad");
    }
}
```

---

# Resultado

```text
Menor de edad
```

---

# 3. else if

Permite evaluar múltiples condiciones.

---

# Ejemplo

```rust
fn main() {

    let nota = 85;

    if nota >= 90 {
        println!("Excelente");
    } else if nota >= 70 {
        println!("Aprobado");
    } else {
        println!("Reprobado");
    }
}
```

---

# Explicación

Rust evalúa:
1. primera condición
2. segunda condición
3. si ninguna se cumple → else

---

# 4. Condiciones Booleanas

Las condiciones devuelven:
- true
- false

---

# Ejemplo

```rust
fn main() {

    let activo = true;

    if activo {
        println!("Usuario activo");
    }
}
```

---

# 5. Operadores Lógicos

| Operador | Significado |
|---|---|
| && | AND |
| \|\| | OR |
| ! | NOT |

---

# Ejemplo AND

```rust
fn main() {

    let edad = 20;
    let tiene_id = true;

    if edad >= 18 && tiene_id {
        println!("Puede entrar");
    }
}
```

---

# Ejemplo OR

```rust
fn main() {

    let admin = false;
    let moderador = true;

    if admin || moderador {
        println!("Acceso permitido");
    }
}
```

---

# Ejemplo NOT

```rust
fn main() {

    let activo = false;

    if !activo {
        println!("Cuenta inactiva");
    }
}
```

---

# 6. if Como Expresión

En Rust, `if` puede devolver valores.

---

# Ejemplo

```rust
fn main() {

    let edad = 20;

    let mensaje = if edad >= 18 {
        "Mayor"
    } else {
        "Menor"
    };

    println!("{}", mensaje);
}
```

---

# Explicación

El resultado del `if` se guarda en:
- mensaje

---

# 7. match

`match` es una de las estructuras más poderosas de Rust.

Se usa para comparar múltiples casos.

---

# Ejemplo Básico

```rust
fn main() {

    let numero = 2;

    match numero {

        1 => println!("Uno"),

        2 => println!("Dos"),

        3 => println!("Tres"),

        _ => println!("Otro número")
    }
}
```

---

# Explicación

## _

Representa:
- cualquier otro caso

---

# Ventajas de match

- más limpio
- más seguro
- más legible
- obligatorio cubrir casos

---

# 8. match con Strings

```rust
fn main() {

    let lenguaje = "Rust";

    match lenguaje {

        "Python" => println!("Lenguaje interpretado"),

        "Rust" => println!("Lenguaje compilado"),

        _ => println!("Otro lenguaje")
    }
}
```

---

# 9. loop

`loop` crea un ciclo infinito.

---

# Ejemplo

```rust
fn main() {

    let mut contador = 0;

    loop {

        contador += 1;

        println!("{}", contador);

        if contador == 5 {
            break;
        }
    }
}
```

---

# Explicación

## break

Detiene el ciclo.

---

# Resultado

```text
1
2
3
4
5
```

---

# 10. while

Ejecuta código mientras la condición sea verdadera.

---

# Ejemplo

```rust
fn main() {

    let mut numero = 1;

    while numero <= 5 {

        println!("{}", numero);

        numero += 1;
    }
}
```

---

# Explicación

Mientras:
- numero <= 5

el ciclo continúa.

---

# 11. for

`for` recorre colecciones o rangos.

---

# Ejemplo con Rangos

```rust
fn main() {

    for numero in 1..6 {

        println!("{}", numero);
    }
}
```

---

# Resultado

```text
1
2
3
4
5
```

---

# Explicación

## 1..6

Empieza en:
- 1

Termina antes de:
- 6

---

# Rango Inclusivo

```rust
1..=6
```

Incluye el 6.

---

# Ejemplo

```rust
fn main() {

    for numero in 1..=5 {

        println!("{}", numero);
    }
}
```

---

# 12. for con Arrays

```rust
fn main() {

    let numeros = [10, 20, 30, 40];

    for numero in numeros {

        println!("{}", numero);
    }
}
```

---

# Explicación

Rust recorre automáticamente:
- cada elemento

---

# 13. continue

`continue` salta una iteración.

---

# Ejemplo

```rust
fn main() {

    for numero in 1..=5 {

        if numero == 3 {
            continue;
        }

        println!("{}", numero);
    }
}
```

---

# Resultado

```text
1
2
4
5
```

---

# 14. break

Finaliza el ciclo inmediatamente.

---

# Ejemplo

```rust
fn main() {

    for numero in 1..=10 {

        if numero == 5 {
            break;
        }

        println!("{}", numero);
    }
}
```

---

# Resultado

```text
1
2
3
4
```

---

# 15. Loops Anidados

```rust
fn main() {

    for fila in 1..=3 {

        for columna in 1..=3 {

            println!("Fila {} Columna {}", fila, columna);
        }
    }
}
```

---

# 16. Ejercicios del Módulo

## Ejercicio 1

Crear un programa que determine:
- si una persona es mayor o menor de edad

---

## Ejercicio 2

Mostrar números del 1 al 10 usando:
- while
- for

---

## Ejercicio 3

Crear un menú usando match.

---

## Ejercicio 4

Mostrar números pares del 1 al 20.

---

# 17. Retos

## Reto 1 — Tabla de Multiplicar

Mostrar la tabla del:
- 5

---

# Ejemplo

```text
5 x 1 = 5
5 x 2 = 10
```

---

## Reto 2 — Contador Inteligente

Mostrar:
- números pares
- impares
- múltiplos de 3

---

# 18. Proyecto del Módulo

# Sistema de Verificación de Usuario

## Objetivo

Crear un programa que:

- valide edad
- valide acceso
- use match
- use loops
- muestre mensajes dinámicos

---

# Ejemplo

```rust
fn main() {

    let usuario = "admin";
    let edad = 20;

    if edad < 18 {

        println!("Acceso denegado");

    } else {

        match usuario {

            "admin" => println!("Bienvenido administrador"),

            "user" => println!("Bienvenido usuario"),

            _ => println!("Usuario desconocido")
        }
    }
}
```

---

# 19. Errores Comunes

## Olvidar llaves

Incorrecto:

```rust
if edad > 18
    println!("Mayor");
```

---

# Correcto

```rust
if edad > 18 {
    println!("Mayor");
}
```

---

## Confundir = con ==

Incorrecto:

```rust
if edad = 18
```

---

# Correcto

```rust
if edad == 18
```

---

# 20. Resumen del Módulo

Aprendiste:

- if
- else
- else if
- operadores lógicos
- match
- loop
- while
- for
- break
- continue
- rangos
- loops anidados

---

# Próximo Módulo

# MÓDULO 4 — Ownership y Memoria

Aprenderás:

- ownership
- borrowing
- references
- move semantics
- slices
- heap y stack
- memoria segura

---
# Profundizando en match en Rust

# ¿Qué es match?

`match` es una estructura de control de flujo extremadamente poderosa en :contentReference[oaicite:0]{index=0}.

Permite:

- comparar valores
- ejecutar código según casos
- manejar múltiples condiciones
- trabajar con patrones
- evitar errores lógicos

---

# Idea Principal

`match` compara un valor contra múltiples posibilidades.

---

# Sintaxis Básica

```rust
match valor {

    patron => accion,

    patron => accion,

    _ => accion_final
}
```

---

# Ejemplo Básico

```rust
fn main() {

    let numero = 2;

    match numero {

        1 => println!("Uno"),

        2 => println!("Dos"),

        3 => println!("Tres"),

        _ => println!("Otro número")
    }
}
```

---

# Explicación Paso a Paso

## match numero

Rust analiza el valor de:
- numero

---

# Caso 1

```rust
1 => println!("Uno")
```

Si:
- numero == 1

---

# Caso 2

```rust
2 => println!("Dos")
```

Si:
- numero == 2

---

# _

```rust
_ => println!("Otro número")
```

Significa:

```text
"Cualquier otro caso"
```

---

# Resultado

Como:
- numero = 2

Rust imprime:

```text
Dos
```

---

# ¿Por Qué match es Especial?

En muchos lenguajes existe:
- switch

Pero `match` es mucho más avanzado.

---

# Diferencias con switch

| switch | match |
|---|---|
| Más limitado | Muy poderoso |
| Menos seguro | Muy seguro |
| Puede olvidar casos | Rust obliga a cubrirlos |
| Menos flexible | Pattern matching avanzado |

---

# match Debe Ser Exhaustivo

Esto es MUY importante.

Rust obliga a manejar:
- todos los posibles casos

---

# Ejemplo Incorrecto

```rust
fn main() {

    let numero = 5;

    match numero {

        1 => println!("Uno"),

        2 => println!("Dos")
    }
}
```

---

# Error

Porque faltan:
- 3
- 4
- 5
- etc.

---

# Solución

```rust
fn main() {

    let numero = 5;

    match numero {

        1 => println!("Uno"),

        2 => println!("Dos"),

        _ => println!("Otro número")
    }
}
```

---

# El _ (Wildcard)

El `_` significa:

```text
"todo lo demás"
```

---

# Es MUY usado en Rust

Porque evita errores.

---

# match con Strings

```rust
fn main() {

    let lenguaje = "Rust";

    match lenguaje {

        "Python" => println!("Interpretado"),

        "Rust" => println!("Compilado"),

        "JavaScript" => println!("Web"),

        _ => println!("Otro lenguaje")
    }
}
```

---

# Resultado

```text
Compilado
```

---

# match con Booleanos

```rust
fn main() {

    let activo = true;

    match activo {

        true => println!("Usuario activo"),

        false => println!("Usuario inactivo")
    }
}
```

---

# match con Múltiples Casos

Puedes unir casos usando:

```rust
|
```

---

# Ejemplo

```rust
fn main() {

    let numero = 2;

    match numero {

        1 | 2 | 3 => println!("Número pequeño"),

        4 | 5 | 6 => println!("Número mediano"),

        _ => println!("Número grande")
    }
}
```

---

# Explicación

## 1 | 2 | 3

Significa:

```text
1 O 2 O 3
```

---

# match con Rangos

Rust permite usar rangos.

---

# Ejemplo

```rust
fn main() {

    let edad = 20;

    match edad {

        0..=12 => println!("Niño"),

        13..=17 => println!("Adolescente"),

        18..=59 => println!("Adulto"),

        _ => println!("Adulto mayor")
    }
}
```

---

# Explicación

## 0..=12

Incluye:
- 0 hasta 12

---

# Resultado

```text
Adulto
```

---

# match Como Expresión

`match` puede devolver valores.

---

# Ejemplo

```rust
fn main() {

    let numero = 2;

    let mensaje = match numero {

        1 => "Uno",

        2 => "Dos",

        _ => "Otro"
    };

    println!("{}", mensaje);
}
```

---

# Explicación

El resultado del `match` se guarda en:
- mensaje

---

# Esto es MUY importante

Porque en Rust:
- casi todo es una expresión

---

# match con Bloques de Código

Cada caso puede ejecutar múltiples líneas.

---

# Ejemplo

```rust
fn main() {

    let numero = 1;

    match numero {

        1 => {

            println!("Número uno");

            println!("Caso especial");
        }

        _ => {

            println!("Otro número");
        }
    }
}
```

---

# match con Tuplas

Aquí comienza el verdadero poder de Rust.

---

# Ejemplo

```rust
fn main() {

    let persona = ("Carlos", 20);

    match persona {

        ("Carlos", edad) => println!("Carlos tiene {} años", edad),

        _ => println!("Otra persona")
    }
}
```

---

# Explicación

Rust:
- desestructura la tupla
- extrae valores

---

# match con Arrays

```rust
fn main() {

    let numeros = [1, 2, 3];

    match numeros {

        [1, 2, 3] => println!("Array correcto"),

        _ => println!("Otro array")
    }
}
```

---

# match con Condiciones Extras

Puedes usar:

```rust
if
```

dentro de match.

---

# Ejemplo

```rust
fn main() {

    let numero = 8;

    match numero {

        x if x % 2 == 0 => println!("Número par"),

        _ => println!("Número impar")
    }
}
```

---

# Explicación

## x if x % 2 == 0

Significa:

```text
"si x es divisible entre 2"
```

---

# match vs if

| if | match |
|---|---|
| Bueno para pocas condiciones | Excelente para muchos casos |
| Más flexible | Más estructurado |
| Menos seguro | Más seguro |

---

# ¿Cuándo usar if?

Cuando:
- solo hay una o pocas condiciones

---

# ¿Cuándo usar match?

Cuando:
- hay muchos casos
- categorías
- menús
- patrones
- múltiples posibilidades

---

# Ejemplo Real — Menú

```rust
fn main() {

    let opcion = 2;

    match opcion {

        1 => println!("Iniciar sesión"),

        2 => println!("Configuración"),

        3 => println!("Salir"),

        _ => println!("Opción inválida")
    }
}
```

---

# Ventajas de match

## Más legible

---

## Más seguro

---

## Menos errores

---

## Muy poderoso

---

## Excelente para pattern matching

---

# Errores Comunes

## Olvidar _

Incorrecto:

```rust
match numero {

    1 => println!("Uno")
}
```

---

# Correcto

```rust
match numero {

    1 => println!("Uno"),

    _ => println!("Otro")
}
```

---

# Confundir = con =>

Incorrecto:

```rust
1 = println!("Uno")
```

---

# Correcto

```rust
1 => println!("Uno")
```

---

# Resumen

Aprendiste:

- qué es match
- wildcard _
- múltiples casos
- rangos
- match como expresión
- match con tuplas
- match con arrays
- guards
- diferencias entre if y match

---

# Idea Fundamental

`match` es uno de los pilares más importantes de Rust.

Muchos programas profesionales en Rust utilizan:
- match
- pattern matching
- desestructuración

constantemente.