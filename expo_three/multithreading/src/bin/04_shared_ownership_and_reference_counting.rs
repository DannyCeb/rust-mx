use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};
/*
fn main() {
    let mut numbers = vec![1, 2, 3];

    thread::scope(|scope| {
        scope.spawn(|| {
            numbers.push(4);
        });

        // no pueden existir dos referencias mutables al mismo tiempo
        //scope.spawn(|| {
        //    numbers.push(5);
        //});
    });

    println!("resultado: {:?}", numbers);
}
    */

/*  Soluciones al problema */

/*
/// 1:        ---- Buffer static ----
/// - El buffer vivirá durante todo el programa
///     - Nace antes que main
///     - muere despues de main
/// BUFFER vive en el stack
static mut BUFFER: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
fn main() {
    unsafe {
        println!("Valor dentro de buffer: {:?}", BUFFER);

        BUFFER[0] = 500;

        println!(
            "Valor dentro de buffer despues de modificarlo: {:?}",
            BUFFER
        );
    }

    let t1 = thread::spawn(|| unsafe {
        BUFFER[1] = 200;
    });
    let t2 = thread::spawn(|| unsafe {
        BUFFER[2] = 300;
    });

    t1.join().unwrap();
    t2.join().unwrap();

    unsafe {
        println!("Valor dentro de buffer: {:?}", BUFFER);
    }
}
*/

// 2: Reference counted
// Arc es como Rc pero seguro de usar en hilos
//

fn main() {
    let shared_value = Arc::new(Mutex::new(vec![6, 10, 3, 2, 8, 1]));
}
