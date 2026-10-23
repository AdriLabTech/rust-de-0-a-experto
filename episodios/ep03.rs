// RUST // 03 · Tipos de datos
// Que ensena: enteros, decimales, bool y char; limites; division entera; `as`; desbordamiento.
// Ejecutar:   rustc --edition 2024 episodios/ep03.rs -o /tmp/ep03 && /tmp/ep03

fn main() {
    let edad: u8 = 17;            // 0..=255
    let temperatura: i32 = -4;    // con signo
    let precio: f64 = 19.99;      // decimal
    let activo: bool = true;
    let inicial: char = 'R';
    println!("{edad} {temperatura} {precio} {activo} {inicial}");

    // Los tipos se deducen, y los literales aceptan sufijo
    let a = 10;        // i32 por defecto
    let b = 2.5;       // f64 por defecto
    let c = 7u64;
    println!("{a} {b} {c}");

    // Limites
    println!("u8: {}..={}", u8::MIN, u8::MAX);
    println!("i32: {}..={}", i32::MIN, i32::MAX);

    // Division entera frente a decimal: 7 / 2 da 3 (se descarta el resto); 7.0 / 2.0 da 3.5
    println!("{} {}", 7 / 2, 7.0 / 2.0);

    // Conversion explicita con 'as' (puede perder informacion: no es una comprobacion)
    let grande = 300i32;
    println!("300 as u8 = {}", grande as u8);       // 300 no cabe en u8: se queda con 44
    println!("3.99 as i32 = {}", 3.99_f64 as i32);  // trunca, no redondea: 3

    // Desbordamiento: tres formas de pedirlo
    // checked_*: devuelve Option (None si se desborda); wrapping_*: da la vuelta; saturating_*: se queda en el limite
    let m: u8 = 255;
    println!("{:?} {:?} {}", m.checked_add(1), m.checked_sub(1), m.wrapping_add(1));
    println!("{}", m.saturating_add(10));

    // Un char es un valor Unicode de 4 bytes
    println!("{} bytes", std::mem::size_of::<char>());
}
