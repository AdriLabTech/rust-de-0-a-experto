// RUST // 06 · Condicionales
// Que ensena: if / else if / else como expresion, match con rangos, alternativas y comodin, y el bisiesto.
// Ejecutar:   rustc --edition 2024 episodios/ep06.rs -o /tmp/ep06 && /tmp/ep06

// Una funcion cuyo cuerpo es un if: cada rama devuelve un valor (sin ; y del mismo tipo)
fn clasificar(edad: u32) -> &'static str {
    if edad < 12 {
        "niño"
    } else if edad < 18 {
        "adolescente"
    } else if edad < 65 {
        "adulto"
    } else {
        "mayor"
    }
}

fn main() {
    for edad in [5, 15, 30, 70] {
        println!("{edad}: {}", clasificar(edad));
    }

    // if es una expresion: puede asignarse a una variable
    let n = 7;
    let paridad = if n % 2 == 0 { "par" } else { "impar" };
    println!("{n} es {paridad}");

    // match con valores, rangos y guardas
    // `5 | 6` son alternativas; `0..=4` es un rango inclusivo; `_` recoge todo lo demas.
    // match debe cubrir TODOS los casos: el compilador lo comprueba
    for nota in [3, 5, 7, 9, 10] {
        let texto = match nota {
            0..=4 => "suspenso",
            5 | 6 => "aprobado",
            7..=8 => "notable",
            9..=10 => "sobresaliente",
            _ => "nota no valida",
        };
        println!("{nota}: {texto}");
    }

    // bisiesto: divisible por 4 y no por 100, o divisible por 400 (2100 NO lo es)
    let anio = 2100;
    let bisiesto = (anio % 4 == 0 && anio % 100 != 0) || anio % 400 == 0;
    println!("{anio} bisiesto: {bisiesto}");
}
