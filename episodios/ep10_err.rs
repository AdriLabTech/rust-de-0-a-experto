fn despedir(nombre: String) {
    println!("adios, {nombre}");
}

fn main() {
    let s1 = String::from("correr");
    despedir(s1);
    println!("{s1}");
}

// ----------------------------------------------------------------------------
// RUST // 10 · usar un valor despues de moverlo. Falla A PROPOSITO.
//   error[E0382]: borrow of moved value: `s1`   (value borrowed here after move)
// despedir(s1) se queda con la propiedad del String; main ya no es su dueno.
// Arreglos: pasar una referencia (&s1, ver ep11.rs) o clonar antes: despedir(s1.clone()).
