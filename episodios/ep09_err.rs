fn main() {
    let v = vec![1, 2, 3];
    let i = v.len() + 2;
    println!("{}", v[i]);
}

// ----------------------------------------------------------------------------
// RUST // 09 · indice fuera de rango. COMPILA, pero entra en panico al ejecutar.
//   thread 'main' panicked at ...: index out of bounds: the len is 3 but the index is 5   (codigo de salida 101)
// El compilador no puede saber el valor de `i`; el fallo se detecta al ejecutar.
// Arreglo: v.get(i) devuelve Option y no entra en panico; o comprobar i < v.len() antes.
