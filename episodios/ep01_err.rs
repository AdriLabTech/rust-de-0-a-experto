fn main() {
    printn!("Hola, mundo");
}

// ----------------------------------------------------------------------------
// RUST // 01 · tu primer error (esta linea de comentarios va al final para que
// los numeros de linea del error coincidan con los del video).
// Falla A PROPOSITO: la macro se llama println!, no printn!.
//   error: cannot find macro `printn` in this scope
//   help: a macro with a similar name exists   (el compilador sugiere println!)
// Arreglo: escribir println!. Probar:  rustc --edition 2024 episodios/ep01_err.rs
