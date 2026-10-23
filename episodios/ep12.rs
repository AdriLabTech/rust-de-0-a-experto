// RUST // 12 · Structs y metodos
// Que ensena: struct con campos, impl, funcion asociada, &self / &mut self / self, struct tupla y sintaxis de actualizacion.
// Ejecutar:   rustc --edition 2024 episodios/ep12.rs -o /tmp/ep12 && /tmp/ep12

// derive(Debug) permite imprimir el struct con {:?}
#[derive(Debug)]
struct Registro {
    habito: String,
    cantidad: u32,
}

impl Registro {
    // Funcion asociada: se llama con Registro::nuevo(...)
    fn nuevo(habito: &str, cantidad: u32) -> Registro {
        Registro { habito: habito.to_string(), cantidad }
    }

    // &self: solo lee
    fn describir(&self) -> String {
        format!("{} x{}", self.habito, self.cantidad)
    }

    // &mut self: modifica
    fn sumar(&mut self, mas: u32) {
        self.cantidad += mas;
    }

    // self: se lleva el registro (lo consume)
    fn cerrar(self) -> String {
        format!("cerrado: {}", self.describir())
    }
}

struct Punto(i32, i32); // struct tupla: campos sin nombre (.0 y .1)

fn main() {
    let mut r = Registro::nuevo("agua", 8);
    println!("{}", r.describir());
    r.sumar(2);
    println!("{r:?}");

    // Sintaxis de actualizacion: ..r toma de r los campos que no se escriben (aqui solo cantidad, que es Copy)
    let otro = Registro { habito: "correr".to_string(), ..r };
    println!("{}", otro.describir());

    let p = Punto(3, -4);
    println!("{} {}", p.0, p.1);

    println!("{}", otro.cerrar());
}
