// RUST // 07 · Bucles
// Que ensena: for sobre rangos, rev, while, loop con break que devuelve valor, continue y etiquetas.
// Ejecutar:   rustc --edition 2024 episodios/ep07.rs -o /tmp/ep07 && /tmp/ep07

fn main() {
    // for sobre un rango: 1..5 no incluye el 5, 1..=5 si
    for i in 1..5 {
        print!("{i} ");
    }
    println!();
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // for hacia atras
    for i in (1..=3).rev() {
        print!("{i} ");
    }
    println!("¡ya!");

    // while: la conjetura de Collatz. Si es par, mitad; si es impar, triple mas uno. 27 tarda 111 pasos
    let mut n = 27u32;
    let mut pasos = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        pasos += 1;
    }
    println!("Collatz(27): {pasos} pasos");

    // loop + break con valor: el bucle es una expresion y devuelve lo que sale con break
    let mut intento = 0;
    let primer_multiplo = loop {
        intento += 1;
        if intento % 7 == 0 && intento % 5 == 0 {
            break intento;
        }
    };
    println!("primer multiplo de 5 y 7: {primer_multiplo}");

    // continue: salta a la siguiente vuelta (aqui se ignoran los pares)
    let mut suma = 0;
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        suma += i;
    }
    println!("suma de impares: {suma}");

    // tabla de multiplicar
    for i in 1..=5 {
        println!("7 x {i} = {}", 7 * i);
    }

    // bucles anidados con etiqueta: break 'fuera sale de los dos bucles a la vez
    'fuera: for a in 1..10 {
        for b in 1..10 {
            if a * b == 42 {
                println!("{a} x {b} = 42");
                break 'fuera;
            }
        }
    }
}
