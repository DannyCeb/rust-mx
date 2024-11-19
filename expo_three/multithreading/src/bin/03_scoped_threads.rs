use std::thread;

///
/// La principal razon para usar scoped threads es indicarle al compilador que nuestros hilos no van a vivir más que los objetos fuera del scope de los threads
///

// Los spawns toman ownership por defecto
fn main() {
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        // se quiere de la palabra move para que el closure tome ownership del vector
        println!("longitud: {}", numbers.len());
    })
    .join()
    .unwrap();

    //println!("numbers: {:?}", numbers); // el valor ha sido movido
}

/*
fn main() {
    let numbers = vec![1, 2, 3];

    // pasamos referencias
    thread::scope(|scope| {
        scope.spawn(|| { // el closure inviere fn
            println!("longitud: {}", numbers.len());
        });
    }); // Todos los threads dentro del scope se joinean de manera automatica

    println!("numbers: {:?}", numbers);
}

*/
