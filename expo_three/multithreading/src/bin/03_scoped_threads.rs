use std::thread;
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

fn main() {
    let numbers = vec![1, 2, 3];

    thread::spawn(|| {
        println!("longitud: {}", numbers.len());
    });

    println!("numbers: {:?}", numbers);
}
