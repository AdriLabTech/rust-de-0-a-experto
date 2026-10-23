fn main() {
    let vasos = 3;
    if vasos {
        println!("has bebido");
    }
}

// ----------------------------------------------------------------------------
// RUST // 06 · condicion que no es bool. Falla A PROPOSITO.
//   error[E0308]: mismatched types   (expected `bool`, found integer)
// En Rust la condicion de un if debe ser un bool: un numero no vale como "verdadero".
// Arreglo: if vasos > 0 { ... }
