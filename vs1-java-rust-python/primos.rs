use std::time::Instant;

fn es_primo(numero: u32) -> bool {
    if numero < 2 {
        return false;
    }
    for i in 2..numero {
        if numero % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let limite: u32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let t = Instant::now();
    let cuenta = (0..limite).filter(|&n| es_primo(n)).count();
    println!("{cuenta} {} ms", t.elapsed().as_millis());
}

// ---------------------------------------------------------------------------
// VS // 01 · Contar primos por debajo de un limite.
// Ejecutar optimizado (como se midio):  rustc --edition 2024 -O primos.rs -o /tmp/primos && /tmp/primos 30000
// Sin -O (depuracion) tarda varias veces mas: Rust es rapido, pero solo si compilas en release.
// Misma cuenta que en Python y Java (3245 para 30000).
