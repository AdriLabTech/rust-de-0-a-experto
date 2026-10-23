# Episodios de «Rust de 0 a experto»

Un programa por episodio. Para ejecutar uno, desde la raíz del repositorio:

```bash
rustc --edition 2024 episodios/ep03.rs -o /tmp/ep03 && /tmp/ep03
```

Cada `epNN.rs` empieza con un comentario que dice qué enseña. Cada `epNN_err.rs` es el fallo del episodio y termina con
un comentario que explica el error y cómo se arregla. Solo biblioteca estándar.

## Bloque 1 · Fundamentos de programación (episodios 1 a 9)

| Ep. | Tema | Fichero | Qué se ve | Fallo didáctico |
|---|---|---|---|---|
| 01 | Instalar Rust y Cargo | [`ep01.rs`](ep01.rs) | `fn main`, `println!` | [`ep01_err.rs`](ep01_err.rs): macro mal escrita (`printn!`) |
| 02 | Variables y constantes | [`ep02.rs`](ep02.rs) | `let`, `mut`, `const`, sombreado, ámbito | [`ep02_err.rs`](ep02_err.rs): `E0384`, variable inmutable |
| 03 | Tipos de datos | [`ep03.rs`](ep03.rs) | enteros, decimales, `bool`, `char`, `as`, desbordamiento | [`ep03_err.rs`](ep03_err.rs): `E0308`, tipos distintos |
| 04 | Operadores | [`ep04.rs`](ep04.rs) | aritméticos, lógicos, precedencia, `0.1 + 0.2` | [`ep04_err.rs`](ep04_err.rs): `E0277`, `i32 * f64` |
| 05 | Texto y entrada de usuario | [`ep05.rs`](ep05.rs) | `&str` y `String`, bytes y caracteres, `read_line` | [`ep05_err.rs`](ep05_err.rs): `E0277`, indexar un texto |
| 06 | Condicionales | [`ep06.rs`](ep06.rs) | `if` como expresión, `match` con rangos | [`ep06_err.rs`](ep06_err.rs): `E0308`, condición que no es `bool` |
| 07 | Bucles | [`ep07.rs`](ep07.rs) | `for`, `while`, `loop`, `break` con valor, etiquetas | — |
| 08 | Funciones | [`ep08.rs`](ep08.rs) | parámetros, valor devuelto, tuplas, slices, recursión | [`ep08_err.rs`](ep08_err.rs): `E0308`, el `;` que se come el valor |
| 09 | Arrays, tuplas y `Vec` | [`ep09.rs`](ep09.rs) | tamaño fijo y dinámico, `get`, `sort` | [`ep09_err.rs`](ep09_err.rs): **compila**, pánico por índice fuera de rango (código 101) |

## Bloque 2 · Lo propio de Rust (episodios 10 a 14)

| Ep. | Tema | Fichero | Qué se ve | Fallo didáctico |
|---|---|---|---|---|
| 10 | Propiedad (*ownership*) | [`ep10.rs`](ep10.rs) | `Copy` frente a *move*, `clone` | [`ep10_err.rs`](ep10_err.rs): `E0382`, usar un valor movido |
| 11 | Préstamo | [`ep11.rs`](ep11.rs) | `&` y `&mut`, muchos lectores o un escritor | [`ep11_err.rs`](ep11_err.rs): `E0502`, leer y escribir a la vez |
| 12 | Structs y métodos | [`ep12.rs`](ep12.rs) | `impl`, `&self`, `&mut self`, `self`, struct tupla | [`ep12_err.rs`](ep12_err.rs): `E0594`, struct inmutable |
| 13 | Enums y `match` | [`ep13.rs`](ep13.rs) | variantes con datos, `match` exhaustivo | [`ep13_err.rs`](ep13_err.rs): `E0004`, variante sin tratar |
| 14 | `Option` y `Result` | [`ep14.rs`](ep14.rs) | error propio, `Display`, `From`, operador `?` | [`ep14_err.rs`](ep14_err.rs): `E0369`, `Option` usado como número |

## Bloque 3 · Proyecto y abstracción (episodios 15 a 17)

| Ep. | Tema | Fichero | Qué se ve | Fallo didáctico |
|---|---|---|---|---|
| 15 | Iteradores y `HashMap` | [`ep15.rs`](ep15.rs) | `map`, `filter`, `collect`, closures, totales por clave | — |
| 16 | Ficheros y proyecto final | [`ep16.rs`](ep16.rs) y [`ep16_habitos.rs`](ep16_habitos.rs) | leer y escribir ficheros, y el gestor de hábitos completo | — |
| 17 | Traits y genéricos | [`ep17.rs`](ep17.rs) | `trait`, *trait bounds*, `Box<dyn …>` | [`ep17_err.rs`](ep17_err.rs): `E0369`, genérico sin `PartialOrd` |

## El proyecto final (`ep16_habitos.rs`)

Un gestor de hábitos por terminal que junta lo de todos los episodios. Comandos: `add <hábito> <cantidad>`, `list`,
`totals`, `del <índice>` y `quit`. Guarda los datos en un fichero de texto con una línea por registro (`agua;8`).

```bash
rustc --edition 2024 episodios/ep16_habitos.rs -o /tmp/hab
printf 'add agua 8\nadd leer 30\nadd agua 2\nlist\ntotals\nquit\n' | /tmp/hab /tmp/h.txt
```

Salida (comprobada): tras los tres `add`, `list` muestra `0: agua x8`, `1: leer x30`, `2: agua x2` y `totals` muestra
`leer: 30` y `agua: 10`. Las entradas erróneas (`foo`, `add x`, `del 9`) dan un mensaje en vez de un pánico.

## Qué errores se ven y cuándo

`E0384` (ep02) · `E0308` (ep03, 06 y 08) · `E0277` (ep04 y 05) · `E0382` (ep10) · `E0502` (ep11) · `E0594` (ep12) ·
`E0004` (ep13) · `E0369` (ep14 y 17). Puedes buscar cualquier código con `rustc --explain E0382`.
