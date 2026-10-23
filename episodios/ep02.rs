// RUST // 02 · Variables y constantes
// Que ensena: let, mut, const, sombreado (shadowing), ambito y declarar-despues-asignar.
// Ejecutar:   rustc --edition 2024 episodios/ep02.rs -o /tmp/ep02 && /tmp/ep02

// Constante: siempre lleva tipo, se escribe en MAYUSCULAS_CON_GUIONES y puede ir fuera de las funciones
const MAX_VASOS: u32 = 8; // constante: siempre con tipo, en MAYUSCULAS

fn main() {
    let nombre = "habitos";       // inmutable: por defecto las variables no cambian
    let mut racha = 0;            // mutable: con mut si puede cambiar
    racha += 1;
    racha += 1;
    println!("{nombre}: racha {racha} de maximo {MAX_VASOS}");

    // Sombreado: mismo nombre, valor nuevo (y puede cambiar el tipo)
    // La primera "espacios" es texto; la segunda es un numero. No es mut: es otra variable
    let espacios = "   ";
    let espacios = espacios.len();
    println!("espacios: {espacios}");

    // Sombreado en un bloque: el original vuelve al salir (ambito o "scope")
    let x = 5;
    {
        let x = x * 2;
        println!("dentro: {x}");
    }
    println!("fuera: {x}");

    // Declarar ahora, asignar despues (una sola vez y antes de usarla)
    let resultado;
    resultado = 42;
    println!("{resultado}");
}
