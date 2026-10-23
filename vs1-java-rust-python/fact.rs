fn factorial(n: u32) -> u64 {
    let mut resultado: u64 = 1;
    for i in 1..=n as u64 {
        resultado *= i;
    }
    resultado
}

fn factorial_checked(n: u32) -> Option<u64> {
    let mut resultado: u64 = 1;
    for i in 1..=n as u64 {
        resultado = resultado.checked_mul(i)?;
    }
    Some(resultado)
}

fn factorial_u128(n: u32) -> u128 {
    (1..=n as u128).product()
}

fn main() {
    println!("{}", factorial(5));
    println!("{:?}", factorial_checked(27));
    println!("{}", factorial_u128(27));
    println!("{}", factorial(27));
}

// ---------------------------------------------------------------------------
// VS // 01 · Factorial en Rust.   Ejecutar:  rustc --edition 2024 fact.rs -o /tmp/fact && /tmp/fact
// Es el unico caso donde el resultado depende de COMO compiles (comportamiento documentado de Rust):
//   - Depuracion (por defecto): factorial(27) entra en panico en la linea 4: "attempt to multiply with overflow" (codigo 101).
//   - Con optimizacion (rustc -O, cargo build --release): no hay comprobacion y devuelve 12963097176472289280, sin aviso.
// Las formas correctas de pedirlo explicitamente: checked_mul devuelve Option (None si se desborda),
// y u128 si cabe (da 10888869450418352160768000000).
// Por eso el ultimo println! de main falla en depuracion: es la demostracion. Comprobado con rustc 1.99.0.
