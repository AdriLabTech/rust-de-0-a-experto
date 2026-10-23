// RUST // 14 · Option y Result
// Que ensena: errores como valores, un enum de error propio, Display, From y el operador ? ; Option para "puede no haber valor".
// Ejecutar:   rustc --edition 2024 episodios/ep14.rs -o /tmp/ep14 && /tmp/ep14
use std::fmt;
use std::num::ParseIntError;

// Los errores posibles de leer una linea, como un enum: el compilador te obliga a tratarlos
#[derive(Debug)]
enum ErrorEntrada {
    Vacia,
    CantidadNoNumerica(ParseIntError),
    CantidadCero,
}

// Display: como se muestra el error a una persona (lo usa el {e} del println!)
impl fmt::Display for ErrorEntrada {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorEntrada::Vacia => write!(f, "la linea esta vacia"),
            ErrorEntrada::CantidadNoNumerica(e) => write!(f, "cantidad no valida: {e}"),
            ErrorEntrada::CantidadCero => write!(f, "la cantidad debe ser al menos 1"),
        }
    }
}

// From: permite que el operador ? convierta un ParseIntError en nuestro ErrorEntrada
impl From<ParseIntError> for ErrorEntrada {
    fn from(e: ParseIntError) -> Self {
        ErrorEntrada::CantidadNoNumerica(e)
    }
}

// "agua 8" -> ("agua", 8)
// Result<T, E>: Ok(valor) si va bien, Err(error) si no
fn leer_linea(linea: &str) -> Result<(String, u32), ErrorEntrada> {
    let linea = linea.trim();
    if linea.is_empty() {
        return Err(ErrorEntrada::Vacia);
    }
    // rsplit_once separa por el ultimo espacio; si no hay espacio, la cantidad por defecto es "1"
    let (nombre, cantidad) = linea.rsplit_once(' ').unwrap_or((linea, "1"));
    let cantidad: u32 = cantidad.parse()?; // ? convierte el error y sale
    if cantidad == 0 {
        return Err(ErrorEntrada::CantidadCero);
    }
    Ok((nombre.to_string(), cantidad))
}

fn main() {
    let lineas = ["agua 8", "correr", "", "leer abc", "leer 0"];
    for l in lineas {
        match leer_linea(l) {
            Ok((n, c)) => println!("ok: {n} x{c}"),
            Err(e) => println!("error: {e}"),
        }
    }

    // Option: puede no haber valor (Some(x) o None); first y get no entran en panico
    let nombres = vec!["agua", "correr"];
    println!("{:?} {:?}", nombres.first(), nombres.get(5));
}
