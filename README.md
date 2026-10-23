# Rust de 0 a experto

Código de la lista **«Rust de 0 a experto»** del canal [Low Noise](https://www.youtube.com/@Low_Noise-n4e) y del vídeo
**VS // 01 · Java vs Rust vs Python**. Cada ejemplo compila y se ejecuta; está comentado línea a línea y pensado para
que lo abras junto al vídeo.

> Comunidad, dudas y votaciones del próximo vídeo: [Discord de Low Noise](https://discord.gg/5huPV8cCVF)

## Qué hay aquí

| Carpeta | Contenido | Más información |
|---|---|---|
| [`episodios/`](episodios/) | Un programa por episodio (`ep01` … `ep17`), el error que enseña el compilador en cada uno (`epNN_err.rs`) y el proyecto final (`ep16_habitos.rs`) | [`episodios/README.md`](episodios/README.md) |
| [`vs1-java-rust-python/`](vs1-java-rust-python/) | Los mismos ejercicios (FizzBuzz, factorial, primos, palíndromos) en Java, Python y Rust, con las medidas del vídeo | [`vs1-java-rust-python/README.md`](vs1-java-rust-python/README.md) |
| [`verificar.py`](verificar.py) | Compila y ejecuta todo y comprueba que cada ejemplo hace lo que dice | ver abajo |

## Cómo usarlo

Necesitas solo [Rust](https://rustup.rs) (`rustc` 1.99 o posterior, edición 2024). No hay dependencias: todo usa la
biblioteca estándar. Para la carpeta `vs1-java-rust-python/` también Python 3 y un JDK (se usó Java 27).

```bash
# ejecutar un episodio
rustc --edition 2024 episodios/ep01.rs -o /tmp/ep01 && /tmp/ep01

# ver el error que enseña el compilador (falla a propósito)
rustc --edition 2024 episodios/ep02_err.rs -o /tmp/x

# o con Cargo, como en el episodio 1
cargo new habitos && cd habitos && cargo run
```

## Convenciones

- `epNN.rs` es el programa del episodio `RUST // NN`. Va con un comentario inicial que dice qué enseña y cómo ejecutarlo.
- `epNN_err.rs` es el fallo del episodio. **Casi todos no compilan a propósito**; `ep09_err.rs` compila y entra en
  pánico al ejecutar. La explicación del error está en un comentario **al final** del fichero, para que los números de
  línea de los mensajes del compilador coincidan con los que se ven en el vídeo.
- Los episodios 07, 15 y 16 no tienen fallo didáctico.
- Hilo conductor de la lista: un registro de hábitos por terminal, que se completa en el episodio 16.

## Verificación

```bash
python3 verificar.py
```

Compila cada ejemplo con `-D warnings` (los avisos cuentan como error), ejecuta los que deben funcionar y comprueba
que los `_err` fallan con el código de error documentado. Comprobado el 2026-10-24 con rustc 1.99.0, Python 3.14.8 y
Java 27.
