fn main() {
    let palabra = String::from("hola");
    let primera = palabra[0];
    println!("{primera}");
}

// ----------------------------------------------------------------------------
// RUST // 05 · indexar un texto. Falla A PROPOSITO.
//   error[E0277]: the type `str` cannot be indexed by `{integer}`
//   = note: you can use `.chars().nth()` or `.bytes().nth()`
// Un texto UTF-8 no se indexa por posicion: un caracter puede ocupar varios bytes.
// Arreglo: palabra.chars().next()   (primera letra) o &palabra[0..1] si sabes que es ASCII
