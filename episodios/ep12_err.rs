struct Registro {
    habito: String,
    cantidad: u32,
}

fn main() {
    let r = Registro { habito: String::from("agua"), cantidad: 8 };
    r.cantidad = 9;
    println!("{} {}", r.habito, r.cantidad);
}

// ----------------------------------------------------------------------------
// RUST // 12 · modificar un campo de un struct inmutable. Falla A PROPOSITO.
//   error[E0594]: cannot assign to `r.cantidad`, as `r` is not declared as mutable
//   help: consider changing this to be mutable
// La mutabilidad es de la variable entera, no de cada campo.
// Arreglo: let mut r = Registro { ... };
