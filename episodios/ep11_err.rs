fn main() {
    let mut frase = String::from("beber agua");
    let lectura = &frase;
    frase.push('!');
    println!("{lectura}");
}

// ----------------------------------------------------------------------------
// RUST // 11 · leer y escribir a la vez. Falla A PROPOSITO.
//   error[E0502]: cannot borrow `frase` as mutable because it is also borrowed as immutable
// `lectura` sigue viva (se usa en el println!) mientras frase.push modifica el texto.
// Arreglo: usar `lectura` antes de modificar, o mover el println! antes del push.
