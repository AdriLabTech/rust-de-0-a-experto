fn main() {
    let v = vec![1, 2, 3];
    let x = v.get(5) + 1;
    println!("{x}");
}

// ----------------------------------------------------------------------------
// RUST // 14 · usar un Option como si fuera un numero. Falla A PROPOSITO.
//   error[E0369]: cannot add `{integer}` to `Option<&{integer}>`
// v.get(5) devuelve Option: puede ser Some(valor) o None. Hay que decidir que pasa si no existe.
// Arreglos: if let Some(n) = v.get(5) { n + 1 }   o   v.get(5).map(|n| n + 1)   o   v.get(5).unwrap_or(&0) + 1
