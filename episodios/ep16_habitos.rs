// RUST // 16 · Proyecto final: gestor de habitos por terminal
// Une todo lo anterior: structs, enums, match, Result y ?, ficheros, iteradores y HashMap.
// Comandos:   add <habito> <cantidad> · list · totals · del <indice> · quit
// Los datos se guardan en un fichero de texto (una linea por registro: "agua;8"); por defecto habitos.txt,
// o la ruta que se pase como primer argumento.
// Ejecutar:   rustc --edition 2024 episodios/ep16_habitos.rs -o /tmp/hab && /tmp/hab /tmp/h.txt
// Sin teclado:  printf 'add agua 8\nlist\nquit\n' | /tmp/hab /tmp/h.txt
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone)]
struct Registro {
    habito: String,
    cantidad: u32,
}

// Un comando ya interpretado: la entrada de texto se convierte a este enum antes de ejecutarse
enum Comando {
    Anadir(Registro),
    Listar,
    Totales,
    Borrar(usize),
    Salir,
}

// Todo lo que puede ir mal al interpretar una linea
enum ErrorComando {
    Vacio,
    Desconocido(String),
    Falta(&'static str),
    NoNumerico,
}

impl fmt::Display for ErrorComando {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorComando::Vacio => write!(f, "escribe un comando"),
            ErrorComando::Desconocido(c) => write!(f, "comando desconocido: {c}"),
            ErrorComando::Falta(que) => write!(f, "falta {que}"),
            ErrorComando::NoNumerico => write!(f, "la cantidad o el indice no es un numero valido"),
        }
    }
}

// Interpreta una linea: la primera palabra es la orden y el resto, sus argumentos.
// Devuelve Err con un error concreto en vez de entrar en panico con entradas raras
fn leer_comando(linea: &str) -> Result<Comando, ErrorComando> {
    let mut partes = linea.split_whitespace();
    let orden = partes.next().ok_or(ErrorComando::Vacio)?;
    match orden {
        "add" => {
            let habito = partes.next().ok_or(ErrorComando::Falta("el habito"))?;
            let cantidad = partes.next().ok_or(ErrorComando::Falta("la cantidad"))?;
            let cantidad: u32 = cantidad.parse().map_err(|_| ErrorComando::NoNumerico)?;
            Ok(Comando::Anadir(Registro { habito: habito.to_string(), cantidad }))
        }
        "list" => Ok(Comando::Listar),
        "totals" => Ok(Comando::Totales),
        "del" => {
            let i = partes.next().ok_or(ErrorComando::Falta("el indice"))?;
            Ok(Comando::Borrar(i.parse().map_err(|_| ErrorComando::NoNumerico)?))
        }
        "quit" => Ok(Comando::Salir),
        otro => Err(ErrorComando::Desconocido(otro.to_string())),
    }
}

// Lee el fichero; las lineas mal formadas se ignoran en silencio (filter_map) y, si no existe, empieza vacio
fn cargar(ruta: &str) -> Vec<Registro> {
    let Ok(texto) = fs::read_to_string(ruta) else {
        return Vec::new(); // primera vez: no hay fichero
    };
    texto
        .lines()
        .filter_map(|l| {
            let (h, c) = l.split_once(';')?;
            Some(Registro { habito: h.to_string(), cantidad: c.parse().ok()? })
        })
        .collect()
}

// Reescribe el fichero entero con la lista actual
fn guardar(ruta: &str, lista: &[Registro]) -> io::Result<()> {
    let texto: String = lista.iter().map(|r| format!("{};{}\n", r.habito, r.cantidad)).collect();
    fs::write(ruta, texto)
}

// Total por habito, ordenado de mayor a menor (y por nombre si empatan)
fn totales(lista: &[Registro]) -> Vec<(&str, u32)> {
    let mut mapa: HashMap<&str, u32> = HashMap::new();
    for r in lista {
        *mapa.entry(r.habito.as_str()).or_insert(0) += r.cantidad;
    }
    let mut v: Vec<_> = mapa.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    v
}

fn main() -> io::Result<()> {
    let ruta = std::env::args().nth(1).unwrap_or_else(|| "habitos.txt".to_string());
    let mut lista = cargar(&ruta);
    println!("{} registros cargados de {ruta}", lista.len());

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?; // sin flush el prompt no se veria hasta el siguiente salto de linea
        let mut linea = String::new();
        if stdin.lock().read_line(&mut linea)? == 0 {
            break; // fin de la entrada
        }
        match leer_comando(&linea) {
            Ok(Comando::Anadir(r)) => {
                println!("anadido: {} x{}", r.habito, r.cantidad);
                lista.push(r);
                guardar(&ruta, &lista)?;
            }
            Ok(Comando::Listar) => {
                for (i, r) in lista.iter().enumerate() {
                    println!("{i}: {} x{}", r.habito, r.cantidad);
                }
            }
            Ok(Comando::Totales) => {
                for (h, t) in totales(&lista) {
                    println!("{h}: {t}");
                }
            }
            // Una "guarda" (if) en el patron: solo entra si el indice existe
            Ok(Comando::Borrar(i)) if i < lista.len() => {
                lista.remove(i);
                guardar(&ruta, &lista)?;
            }
            Ok(Comando::Borrar(_)) => println!("no existe ese indice"),
            Ok(Comando::Salir) => break,
            Err(e) => println!("error: {e}"),
        }
    }
    Ok(())
}
