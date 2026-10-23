// RUST // 17 · Traits y genericos
// Que ensena: trait (comportamiento compartido), genericos con trait bound, trait objects (dyn) y la diferencia entre ambos.
// Ejecutar:   rustc --edition 2024 episodios/ep17.rs -o /tmp/ep17 && /tmp/ep17
use std::fmt::Display;

// Un trait define un comportamiento que varios tipos pueden implementar
trait Resumen {
    fn resumen(&self) -> String;
}

struct Habito {
    nombre: String,
    veces: u32,
}

struct Meta {
    texto: String,
    cumplida: bool,
}

impl Resumen for Habito {
    fn resumen(&self) -> String {
        format!("{}: {} veces", self.nombre, self.veces)
    }
}

impl Resumen for Meta {
    fn resumen(&self) -> String {
        let marca = if self.cumplida { "x" } else { " " };
        format!("[{marca}] {}", self.texto)
    }
}

// Generico con trait bound: se monomorfiza, sin coste en ejecucion
// (el compilador genera una copia para cada tipo concreto; T debe implementar Resumen)
fn imprimir<T: Resumen>(x: &T) {
    println!("{}", x.resumen());
}

// Dinamico: una lista con tipos distintos
// Box<dyn Resumen> guarda cualquier tipo que implemente el trait; la llamada se resuelve en ejecucion
fn imprimir_todo(items: &[Box<dyn Resumen>]) {
    for i in items {
        println!("- {}", i.resumen());
    }
}

// Generico sobre cualquier cosa ordenable y mostrable
// (sin PartialOrd no se puede comparar con >=: vease ep17_err.rs)
fn mayor<T: PartialOrd + Display>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

fn main() {
    let h = Habito { nombre: "agua".into(), veces: 14 };
    let m = Meta { texto: "correr 5 km".into(), cumplida: false };
    imprimir(&h);
    imprimir(&m);

    // Aqui h y m se mueven dentro de las cajas
    let todo: Vec<Box<dyn Resumen>> = vec![Box::new(h), Box::new(m)];
    imprimir_todo(&todo);

    println!("{} {} {}", mayor(3, 9), mayor(2.5, 1.0), mayor("a", "b"));
}
