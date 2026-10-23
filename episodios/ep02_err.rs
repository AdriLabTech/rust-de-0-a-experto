fn main() {
    let racha = 0;
    racha += 1;
    println!("{racha}");
}

// ----------------------------------------------------------------------------
// RUST // 02 · el error de mutabilidad. Falla A PROPOSITO.
//   error[E0384]: cannot assign twice to immutable variable `racha`
//   help: consider making this binding mutable   (sugiere: let mut racha = 0;)
// Por defecto las variables son inmutables: asi un valor no cambia sin que te enteres.
// Arreglo: let mut racha = 0;  (mut solo cuando de verdad haga falta)
