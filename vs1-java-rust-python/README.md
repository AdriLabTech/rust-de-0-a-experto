# VS // 01 · Java vs Rust vs Python

Los mismos cuatro ejercicios de clase resueltos con el **mismo algoritmo** en los tres lenguajes, y comparados por
legibilidad, simplicidad y velocidad. El factorial de 27 da **tres resultados distintos** según el lenguaje: esa es la
sorpresa del vídeo.

Lista de reproducción: [VS // Java vs Rust vs Python](https://www.youtube.com/playlist?list=PLELnkvMRWDYI).

Cada fichero termina con un comentario que explica qué hace y qué resultado esperar.

| Ejercicio | Python | Java | Rust |
|---|---|---|---|
| FizzBuzz | [`fizzbuzz.py`](fizzbuzz.py) | [`FizzBuzz.java`](FizzBuzz.java) | [`fizzbuzz.rs`](fizzbuzz.rs) (y [`fizzbuzz_incompleto.rs`](fizzbuzz_incompleto.rs), que no compila a propósito) |
| Factorial de 27 | [`fact.py`](fact.py) | [`Fact.java`](Fact.java) | [`fact.rs`](fact.rs) |
| Contar primos | [`primos.py`](primos.py) | [`Primos.java`](Primos.java) | [`primos.rs`](primos.rs) |
| Palíndromos | [`pal.py`](pal.py) | [`Pal.java`](Pal.java) | [`pal.rs`](pal.rs) (y [`utf8.rs`](utf8.rs): bytes frente a caracteres) |

## El resultado que no coincide: factorial de 27

| Lenguaje | Resultado | Por qué |
|---|---|---|
| Python | `10888869450418352160768000000` | Los enteros no tienen límite (correcto) |
| Java (`long`) | `-5483646897237262336` | Desbordamiento **silencioso**: ni error ni aviso |
| Java (`BigInteger`) | `10888869450418352160768000000` | Correcto, pero más pesado |
| Java (`Math.multiplyExact`) | `ArithmeticException: long overflow` | Falla, que es lo que quieres |
| Rust, depuración | pánico: `attempt to multiply with overflow` (código 101) | Comprobación activa por defecto en depuración |
| Rust, `-O` / `--release` | `12963097176472289280` | **Resultado equivocado sin error**: comportamiento documentado |
| Rust (`checked_mul`) | `None` | Pides la comprobación y te avisa |
| Rust (`u128`) | `10888869450418352160768000000` | Correcto, cabe en 128 bits |

Lección: los tres lenguajes permiten el fallo. Lo que cambia es **cuándo te enteras**: nunca, al ejecutar, o solo si lo pides.

## Cómo ejecutar

```bash
# Python
python3 fizzbuzz.py
python3 primos.py 30000

# Java (JDK)
javac FizzBuzz.java && java FizzBuzz
javac Primos.java && java Primos 30000

# Rust (desde esta carpeta)
rustc --edition 2024 fizzbuzz.rs -o /tmp/fb && /tmp/fb
rustc --edition 2024 -O primos.rs -o /tmp/primos && /tmp/primos 30000
rustc --edition 2024 fact.rs -o /tmp/fact && /tmp/fact        # entra en pánico al final: es la demostración
rustc --edition 2024 -O fact.rs -o /tmp/fact_o && /tmp/fact_o  # con optimización: resultado equivocado sin aviso
```

## Cómo se midió la velocidad

- Ejercicio: contar los primos por debajo de 30000 con el algoritmo ingenuo (probar todos los divisores hasta *n* − 1,
  como en clase). Los tres dan la misma cuenta: **3245**.
- Tres ejecuciones de cada lenguaje y se toma la **mediana**. Rust se mide compilado con optimización (`-O`).
- Versiones: Python 3.14, Java 27, Rust 1.99. Una sola máquina, un procesador Intel de 13.ª generación.
- Es **un algoritmo y un tamaño**: no mide memoria, arranque ni otros problemas. Mide tu caso, no el mío: los tiempos
  de `primos.*` dependen de tu equipo (el programa imprime los milisegundos que ha tardado).
- Orden de magnitud visto en el vídeo: Python es mucho más lento en este bucle de enteros; Java y Rust (optimizado)
  quedan prácticamente empatados; Rust sin optimizar tarda varias veces más que optimizado.

## Notas

- `Pal.java` y `pal.rs` imprimen además las líneas `3: Fizz`, `5: Buzz`… antes del resultado del palíndromo;
  `pal.py` no lo hace. El resultado del palíndromo es el mismo en los tres: `true false`.
- `utf8.rs` muestra `2 1`: «ñ» ocupa 2 bytes pero es 1 carácter, por eso Rust te obliga a elegir si recorres bytes o caracteres.
