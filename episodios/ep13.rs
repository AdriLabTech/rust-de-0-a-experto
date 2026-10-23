// RUST // 13 · Enums y match
// Que ensena: enum con datos en cada variante, match exhaustivo y un programa que ejecuta una lista de comandos.
// Ejecutar:   rustc --edition 2024 episodios/ep13.rs -o /tmp/ep13 && /tmp/ep13

#[derive(Debug)]
struct Registro {
    habito: String,
    cantidad: u32,
}

// Cada variante puede llevar datos distintos: ninguno (Listar), un struct (Anadir) o campos con nombre (Borrar)
#[derive(Debug)]
enum Comando {
    Anadir(Registro),
    Listar,
    Borrar { indice: usize },
    Salir,
}

impl Registro {
    fn nuevo(habito: &str, cantidad: u32) -> Registro {
        Registro { habito: habito.to_string(), cantidad }
    }

    fn describir(&self) -> String {
        format!("{} x{}", self.habito, self.cantidad)
    }
}

// Devuelve false cuando hay que parar (Salir). match obliga a tratar TODAS las variantes
fn ejecutar(cmd: Comando, lista: &mut Vec<Registro>) -> bool {
    match cmd {
        Comando::Anadir(r) => lista.push(r),
        Comando::Listar => {
            for (i, r) in lista.iter().enumerate() {
                println!("{i}: {}", r.describir());
            }
        }
        Comando::Borrar { indice } => {
            if indice < lista.len() {
                lista.remove(indice);
            }
        }
        Comando::Salir => return false,
    }
    true
}

fn main() {
    let mut lista = Vec::new();
    // En vez de leer del teclado, un "guion" fijo de comandos hace la salida siempre igual
    let guion = vec![
        Comando::Anadir(Registro::nuevo("agua", 8)),
        Comando::Anadir(Registro::nuevo("correr", 1)),
        Comando::Listar,
        Comando::Borrar { indice: 0 },
        Comando::Listar,
        Comando::Salir,
    ];
    for cmd in guion {
        if !ejecutar(cmd, &mut lista) {
            break;
        }
    }
}
