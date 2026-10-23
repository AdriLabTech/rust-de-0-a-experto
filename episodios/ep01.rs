// RUST // 01 · Instalar Rust y Cargo: tu primer programa
// Que ensena: la estructura minima de un programa (fn main) y la macro println!.
// Ejecutar:   rustc --edition 2024 episodios/ep01.rs -o /tmp/ep01 && /tmp/ep01
// Con Cargo (como en el video):  cargo new habitos && cd habitos && cargo run

// Todo programa Rust empieza ejecutando la funcion main
fn main() {
    // println! es una macro (lleva !): imprime una linea. Cada {} se rellena con el valor siguiente
    println!("Hola, mundo");
    println!("Soy {} y llevo {} dias aprendiendo Rust", "Adrian", 1);
    let lenguaje = "Rust";
    // {nombre} toma directamente el valor de la variable con ese nombre
    println!("{lenguaje} se compila antes de ejecutarse");
}
