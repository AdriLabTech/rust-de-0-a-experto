fn main() {
    for i in 1..=15 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
        }
    }
}

// ---------------------------------------------------------------------------
// VS // 01 · FizzBuzz al que le falta la rama `_`. Falla A PROPOSITO.
//   error[E0004]: non-exhaustive patterns: ... not covered
// El match de fizzbuzz.rs es exhaustivo; aqui no hay rama para "ni multiplo de 3 ni de 5".
// Arreglo: anadir   _ => println!("{i}"),
