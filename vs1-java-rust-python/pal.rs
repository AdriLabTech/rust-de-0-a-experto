fn es_palindromo(palabra: &str) -> bool {
    let limpia: String = palabra.to_lowercase().replace(' ', "");
    limpia.chars().eq(limpia.chars().rev())
}

fn main() {
    for i in 1..16 {
        match (i % 3, i % 5) {
            (0, 0) => println!("{i}: FizzBuzz"),
            (0, _) => println!("{i}: Fizz"),
            (_, 0) => println!("{i}: Buzz"),
            _ => {}
        }
    }
    println!("{} {}", es_palindromo("Somos o no somos"), es_palindromo("hola"));
}

// ---------------------------------------------------------------------------
// VS // 01 · Palindromos en Rust.   Ejecutar:  rustc --edition 2024 pal.rs -o /tmp/pal && /tmp/pal   -> true false
// Compara el iterador de caracteres con el mismo iterador al reves (chars().eq(chars().rev())):
// recorre CARACTERES, no bytes, que es lo correcto en texto UTF-8 (vease utf8.rs).
// Nota: esta version imprime ademas las lineas "3: Fizz", "5: Buzz"... antes del resultado (pal.py no lo hace).
