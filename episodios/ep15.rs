// RUST // 15 · Iteradores, closures y HashMap
// Que ensena: map / filter / sum / count / max_by_key / collect, closures que capturan variables y totales con HashMap.
// Ejecutar:   rustc --edition 2024 episodios/ep15.rs -o /tmp/ep15 && /tmp/ep15
use std::collections::HashMap;

struct Registro {
    habito: String,
    cantidad: u32,
}

fn registros() -> Vec<Registro> {
    vec![
        Registro { habito: "agua".into(), cantidad: 8 },
        Registro { habito: "correr".into(), cantidad: 1 },
        Registro { habito: "agua".into(), cantidad: 6 },
        Registro { habito: "leer".into(), cantidad: 30 },
    ]
}

fn main() {
    let lista = registros();

    // Iteradores: map, filter, sum, count, max_by_key (son perezosos: no hacen nada hasta que se consumen)
    let total: u32 = lista.iter().map(|r| r.cantidad).sum();
    let grandes = lista.iter().filter(|r| r.cantidad >= 8).count();
    let mayor = lista.iter().max_by_key(|r| r.cantidad).unwrap();
    println!("total {total}, grandes {grandes}, mayor {} ({})", mayor.habito, mayor.cantidad);

    // collect recoge el resultado en una coleccion (aqui un Vec<String>)
    let nombres: Vec<String> = lista.iter().map(|r| r.habito.to_uppercase()).collect();
    println!("{nombres:?}");

    // Closure que captura una variable: |r| ... usa `minimo` del entorno
    let minimo = 5;
    let pasan: Vec<&str> = lista
        .iter()
        .filter(|r| r.cantidad > minimo)
        .map(|r| r.habito.as_str())
        .collect();
    println!("{pasan:?}");

    // HashMap: totales por habito. entry(...).or_insert(0) crea la entrada si no existe y devuelve una referencia para sumar
    let mut totales: HashMap<&str, u32> = HashMap::new();
    for r in &lista {
        *totales.entry(r.habito.as_str()).or_insert(0) += r.cantidad;
    }
    println!("agua: {:?}, nada: {:?}", totales.get("agua"), totales.get("nada"));

    // El orden de un HashMap no esta garantizado: se ordena a mano
    // (primero por total descendente; a igualdad, por nombre)
    let mut orden: Vec<_> = totales.into_iter().collect();
    orden.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (h, t) in orden {
        println!("{h}: {t}");
    }
}
