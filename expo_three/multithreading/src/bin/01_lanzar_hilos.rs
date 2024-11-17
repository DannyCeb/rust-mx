use std::thread;

fn main() {
    let t1 = thread::spawn(saludar);
    let t2 = thread::spawn(saludar);

    // println! usa std::io::Stdout::lock() para asegurarse de que el output no se interrumple
    println!("Hola desde el hilo principal");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn saludar() {
    println!("Hola desde otro hilo");
    let id = thread::current().id();
    println!("El id de este hilo es: {:?}", id);
}
