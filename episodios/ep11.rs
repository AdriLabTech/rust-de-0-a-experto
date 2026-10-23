// RUST // 11 · Prestamo (referencias)
// Que ensena: & (lectura), &mut (escritura exclusiva) y la regla: muchos lectores o un solo escritor, nunca los dos.
// Ejecutar:   rustc --edition 2024 episodios/ep11.rs -o /tmp/ep11 && /tmp/ep11

// &str: presta el texto solo para leerlo, no lo consume
fn contar_palabras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

// &mut String: presta el texto para modificarlo
fn añadir_exclamacion(texto: &mut String) {
    texto.push('!');
}

fn main() {
    let mut frase = String::from("beber agua cada dia");
    let n = contar_palabras(&frase); // prestamo de solo lectura
    println!("{n} palabras");

    añadir_exclamacion(&mut frase); // prestamo exclusivo
    println!("{frase}");

    let a = &frase;
    let b = &frase; // varias lecturas a la vez: bien
    println!("{a} {b}");
}
