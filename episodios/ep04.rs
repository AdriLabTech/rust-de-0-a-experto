// RUST // 04 · Operadores
// Que ensena: aritmeticos, comparacion, logicos, precedencia, asignacion compuesta y el problema de los decimales.
// Ejecutar:   rustc --edition 2024 episodios/ep04.rs -o /tmp/ep04 && /tmp/ep04

fn main() {
    let a = 17;
    let b = 5;
    // suma, resta, producto, division entera y resto (modulo)
    println!("{} {} {} {} {}", a + b, a - b, a * b, a / b, a % b);

    // Comparacion: da bool
    println!("{} {} {}", a > b, a == b, a != b);

    // Logicos: && (y) || (o) ! (no). && y || se evaluan en cortocircuito
    let edad = 20;
    let tiene_carnet = true;
    println!("{}", edad >= 18 && tiene_carnet);
    println!("{}", edad < 18 || !tiene_carnet);

    // Precedencia: * antes que +; los parentesis mandan
    println!("{} {}", 2 + 3 * 4, (2 + 3) * 4);

    // Asignacion compuesta: ((10 + 5) * 2 - 1) / 3 = 9
    let mut total = 10;
    total += 5;
    total *= 2;
    total -= 1;
    total /= 3;
    println!("{total}");

    // Entre decimales: 0.1 + 0.2 no es exactamente 0.3 (representacion binaria), asi que no se compara con ==
    let x = 0.1 + 0.2;
    println!("{x} {}", x == 0.3);
    println!("{}", (x - 0.3_f64).abs() < 1e-9); // se compara con una tolerancia

    // Potencias y raiz
    println!("{} {} {:.3}", 2_i32.pow(10), 2.0_f64.powi(3), 2.0_f64.sqrt());
}
