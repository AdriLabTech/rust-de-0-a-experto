// RUST // 09 · Arrays, tuplas y Vec
// Que ensena: array (tamano fijo), tupla (tipos mezclados), Vec (crece), recorrer, acceso seguro y ordenar.
// Ejecutar:   rustc --edition 2024 episodios/ep09.rs -o /tmp/ep09 && /tmp/ep09

fn main() {
    // Array: tamaño fijo, un solo tipo
    let dias = [31, 28, 31, 30];
    println!("{} elementos, primero {}, ultimo {}", dias.len(), dias[0], dias[dias.len() - 1]);
    let ceros = [0; 5]; // [valor; cuantos]
    println!("{ceros:?}");

    // Tupla: tipos mezclados, tamaño fijo. Se accede con .0 .1 o se desestructura
    let registro = ("agua", 8, true);
    println!("{} {} {}", registro.0, registro.1, registro.2);
    let (nombre, cantidad, _) = registro; // _ ignora ese campo
    println!("{nombre} x{cantidad}");

    // Vec: crece
    let mut lista = Vec::new();
    lista.push(10);
    lista.push(20);
    lista.push(30);
    lista.insert(1, 15);
    println!("{lista:?} len {}", lista.len());
    println!("{:?}", lista.pop()); // pop devuelve Option: Some(ultimo) o None si esta vacio
    lista.remove(0);
    println!("{lista:?}");

    // Recorrer
    for x in &lista {
        print!("{x} ");
    }
    println!();
    for (i, x) in lista.iter().enumerate() {
        println!("{i}: {x}");
    }

    // Acceso seguro: get devuelve Option en vez de entrar en panico si el indice no existe
    println!("{:?} {:?}", lista.get(0), lista.get(9));

    // Ordenar, maximo, suma
    let mut notas = vec![7, 3, 9, 5];
    notas.sort();
    println!("{notas:?} max {:?} suma {}", notas.iter().max(), notas.iter().sum::<i32>());
}
