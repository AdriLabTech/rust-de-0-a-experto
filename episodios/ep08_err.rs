fn cuadrado(x: i32) -> i32 {
    x * x;
}

fn main() {
    println!("{}", cuadrado(7));
}

// ----------------------------------------------------------------------------
// RUST // 08 · el punto y coma que se come el valor. Falla A PROPOSITO.
//   error[E0308]: mismatched types   (expected `i32`, found `()`)
// `x * x;` con punto y coma es una instruccion: no devuelve nada (el tipo vacio `()`).
// Arreglo: quitar el ;  ->  x * x
