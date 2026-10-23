fn main() {
    for i in 1..=15 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{i}"),
        }
    }
}

// ---------------------------------------------------------------------------
// VS // 01 · FizzBuzz en Rust.   Ejecutar:  rustc --edition 2024 fizzbuzz.rs -o /tmp/fb && /tmp/fb
// match sobre la pareja de restos (i % 3, i % 5): (0, 0) FizzBuzz, (0, _) Fizz, (_, 0) Buzz.
// El compilador comprueba que se han cubierto todos los casos: si falta uno, no compila (vease fizzbuzz_incompleto.rs).
