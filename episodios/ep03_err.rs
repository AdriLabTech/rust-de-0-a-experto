fn main() {
    let edad: i32 = "17";
    println!("{edad}");
}

// ----------------------------------------------------------------------------
// RUST // 03 · tipos incompatibles. Falla A PROPOSITO.
//   error[E0308]: mismatched types   (expected `i32`, found `&str`)
// "17" es texto, no un numero. Rust no convierte tipos por su cuenta.
// Arreglo: let edad: i32 = 17;   o convertir:  "17".parse::<i32>().unwrap()
