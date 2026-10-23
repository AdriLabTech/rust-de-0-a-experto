fn main() {
    let t = "ñ";
    println!("{} {}", t.len(), t.chars().count());
}

// ---------------------------------------------------------------------------
// VS // 01 · Un texto en Rust siempre es UTF-8: "ñ" ocupa 2 bytes pero es 1 caracter.
// Ejecutar:  rustc --edition 2024 utf8.rs -o /tmp/utf8 && /tmp/utf8   -> "2 1"   (len = bytes; chars().count() = caracteres)
// Por eso Rust te obliga a elegir si recorres bytes o caracteres.
