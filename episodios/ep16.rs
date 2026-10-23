// RUST // 16 · Ficheros y errores (ejemplo corto del episodio)
// Que ensena: escribir y leer un fichero de texto, descartar lineas mal formadas y propagar errores de E/S con ?.
// El proyecto completo (gestor de habitos con comandos) esta en ep16_habitos.rs.
// Ejecutar:   rustc --edition 2024 episodios/ep16.rs -o /tmp/ep16 && /tmp/ep16
// El fichero de prueba se crea y se borra en la carpeta temporal del sistema.
use std::fs;

#[derive(Debug, Clone)]
struct Registro {
    habito: String,
    cantidad: u32,
}

// Una linea por registro: "agua;8"
fn guardar(ruta: &str, lista: &[Registro]) -> std::io::Result<()> {
    let texto: String = lista
        .iter()
        .map(|r| format!("{};{}\n", r.habito, r.cantidad))
        .collect();
    fs::write(ruta, texto)
}

// Devuelve los registros validos y cuantas lineas se descartaron
fn cargar(ruta: &str) -> std::io::Result<(Vec<Registro>, usize)> {
    let texto = fs::read_to_string(ruta)?; // si no se puede leer, el error sale de la funcion
    let mut buenas = Vec::new();
    let mut descartadas = 0;
    for linea in texto.lines() {
        // split_once(';') puede fallar (None) y parse tambien: and_then + ? dentro del closure encadena las dos comprobaciones
        let parseada = linea.split_once(';').and_then(|(h, c)| {
            Some(Registro { habito: h.to_string(), cantidad: c.parse().ok()? })
        });
        match parseada {
            Some(r) => buenas.push(r),
            None => descartadas += 1,
        }
    }
    Ok((buenas, descartadas))
}

// main tambien puede devolver Result: si falla, el programa termina con el error
fn main() -> std::io::Result<()> {
    let lista = vec![
        Registro { habito: "agua".into(), cantidad: 8 },
        Registro { habito: "correr".into(), cantidad: 1 },
    ];
    let ruta = std::env::temp_dir().join("habitos_demo.txt");
    let ruta = ruta.to_str().unwrap();

    guardar(ruta, &lista)?;
    // Estropeamos a proposito una linea ("treinta" no es un numero) y anadimos otra sin separador
    let mut contenido = fs::read_to_string(ruta)?;
    contenido.push_str("leer;treinta\nsin separador\n");
    fs::write(ruta, contenido)?;

    let (leidas, descartadas) = cargar(ruta)?;
    println!("leidas {}, descartadas {descartadas}", leidas.len());
    for r in &leidas {
        println!("{} -> {}", r.habito, r.cantidad);
    }

    // Un fichero que no existe: el error de E/S llega como Err, sin panico
    match cargar("/ruta/que/no/existe.txt") {
        Ok(_) => println!("inesperado"),
        Err(e) => println!("error: {e}"),
    }
    fs::remove_file(ruta)?;
    Ok(())
}
