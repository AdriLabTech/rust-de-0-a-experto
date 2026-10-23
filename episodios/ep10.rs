// RUST // 10 · Propiedad (ownership)
// Que ensena: Copy frente a move, que cada valor tiene un unico dueno y que la memoria se libera sola al salir de ambito.
// Ejecutar:   rustc --edition 2024 episodios/ep10.rs -o /tmp/ep10 && /tmp/ep10

fn despedir(nombre: String) {
    println!("adios, {nombre}");
} // aqui nombre sale de alcance y su memoria se libera

fn main() {
    // Copy: enteros viven en la pila y se copian
    let a = 5;
    let b = a;
    println!("{a} {b}");

    // Move: String es dueño de memoria en el monton
    // Pasar s1 a despedir() MUEVE la propiedad: despues s1 ya no se puede usar (vease ep10_err.rs).
    // Para seguir usando el texto se clona antes (s2), a costa de copiar el contenido
    let s1 = String::from("correr");
    let s2 = s1.clone(); // copia explicita y cara
    despedir(s1);
    println!("{s2} sigue vivo");
}
