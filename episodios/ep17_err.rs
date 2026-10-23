fn mayor<T>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}
fn main() { println!("{}", mayor(1, 2)); }

// ----------------------------------------------------------------------------
// RUST // 17 · generico sin trait bound. Falla A PROPOSITO.
//   error[E0369]: binary operation `>=` cannot be applied to type `T`
//   help: consider restricting type parameter `T` with trait `PartialOrd`
// T puede ser cualquier tipo, y no todos se pueden comparar.
// Arreglo: fn mayor<T: PartialOrd>(a: T, b: T) -> T { ... }
