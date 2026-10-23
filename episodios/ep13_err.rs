enum Comando { Listar, Salir, Borrar }

fn main() {
    let c = Comando::Listar;
    match c {
        Comando::Listar => println!("listar"),
        Comando::Salir => println!("salir"),
    }
}

// ----------------------------------------------------------------------------
// RUST // 13 · match que no cubre todas las variantes. Falla A PROPOSITO.
//   error[E0004]: non-exhaustive patterns: `Comando::Borrar` not covered
// Si manana anades una variante al enum, el compilador te senala cada match que falta por actualizar.
// Arreglo: anadir la rama  Comando::Borrar => ...,   o un comodin  _ => ...,  si de verdad da igual.
