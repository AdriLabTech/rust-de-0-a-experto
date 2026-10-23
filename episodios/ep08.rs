// RUST // 08 · Funciones
// Que ensena: parametros, valor devuelto (sin ;), return anticipado, tuplas, slices y recursion.
// Ejecutar:   rustc --edition 2024 episodios/ep08.rs -o /tmp/ep08 && /tmp/ep08

fn saludar(nombre: &str) {
    println!("Hola, {nombre}");
}

fn cuadrado(x: i32) -> i32 {
    x * x // sin punto y coma: es el valor devuelto
}

fn dividir(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return f64::NAN; // return explicito para salir antes
    }
    a / b
}

// Varios valores: una tupla. &[i32] es un "slice": una vista de un array o Vec, sin copiarlo
fn min_y_max(v: &[i32]) -> (i32, i32) {
    let mut min = v[0];
    let mut max = v[0];
    for &x in v {
        if x < min { min = x; }
        if x > max { max = x; }
    }
    (min, max)
}

// Recursion: la funcion se llama a si misma. u64 llega hasta el factorial de 20
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

fn main() {
    saludar("Adrián");
    println!("{}", cuadrado(7));
    println!("{} {}", dividir(10.0, 4.0), dividir(1.0, 0.0));
    let (lo, hi) = min_y_max(&[4, -2, 9, 0]);
    println!("min {lo}, max {hi}");
    println!("{}", factorial(10));
}
