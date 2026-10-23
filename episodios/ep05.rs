// RUST // 05 · Texto y entrada de usuario
// Que ensena: &str frente a String, bytes frente a caracteres, metodos de texto, formato y leer de teclado.
// Ejecutar:   rustc --edition 2024 episodios/ep05.rs -o /tmp/ep05 && /tmp/ep05
// Sin teclado:  echo 5 | /tmp/ep05     (la ultima parte pregunta cuantos vasos has bebido)
use std::io;

fn main() {
    // &str: texto fijo prestado. String: texto propio que puede crecer
    let saludo: &str = "hola";
    let mut frase = String::from(saludo);
    frase.push_str(" mundo");
    frase.push('!');
    println!("{frase}");

    // len cuenta BYTES, chars().count() cuenta caracteres ("canción": la ó ocupa 2 bytes en UTF-8)
    let palabra = "canción";
    println!("{} bytes, {} caracteres", palabra.len(), palabra.chars().count());

    // Metodos habituales
    println!("{}", frase.to_uppercase());
    println!("{}", "  con espacios  ".trim());
    println!("{}", frase.contains("mundo"));
    println!("{}", frase.replace("mundo", "Rust"));

    // Formato: {:.1} un decimal; {:>8} alinea a la derecha, {:<8} a la izquierda, {:^8} centra; {:05} rellena con ceros
    let nombre = "agua";
    let veces = 8;
    let ratio = 0.8333;
    println!("{nombre}: {veces} veces ({ratio:.1})");
    println!("[{:>8}] [{:<8}] [{:^8}]", nombre, nombre, nombre);
    println!("{:05}", 42);

    // Entrada del usuario: read_line devuelve un Result; parse tambien (puede fallar)
    println!("¿Cuántos vasos has bebido?");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("no se pudo leer");
    match entrada.trim().parse::<u32>() {
        Ok(n) => println!("Has bebido {n} vasos"),
        Err(_) => println!("Eso no es un número"),
    }
}
