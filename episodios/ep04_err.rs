fn main() {
    let unidades: i32 = 3;
    let precio: f64 = 2.5;
    println!("{}", unidades * precio);
}

// ----------------------------------------------------------------------------
// RUST // 04 · mezclar enteros y decimales. Falla A PROPOSITO.
//   error[E0277]: cannot multiply `i32` by `f64`
// Rust nunca convierte entre i32 y f64 por su cuenta.
// Arreglo: unidades as f64 * precio   (conversion explicita)
