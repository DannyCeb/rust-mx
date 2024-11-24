use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

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

/*  Soluciones al problema */

/*
/// 1:        ---- Buffer static ----
/// - El buffer vivirá durante todo el programa
///     - Nace antes que main
///     - muere despues de main
/// BUFFER vive en el stack
// Declaración de un buffer estático y mutable con valores iniciales
static mut BUFFER: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    // Bloque unsafe para acceder a la variable estática mutable
    unsafe {
        // Imprime el valor actual del buffer
        println!("Valor dentro de buffer: {:?}", BUFFER);

        // Modifica el primer elemento del buffer
        BUFFER[0] = 500;

        // Imprime el valor del buffer después de la modificación
        println!(
            "Valor dentro de buffer despues de modificarlo: {:?}",
            BUFFER
        );
    }

    // Crea el primer hilo que modifica el segundo elemento del buffer
    let t1 = thread::spawn(|| unsafe {
        BUFFER[1] = 200;
    });

    // Crea el segundo hilo que modifica el tercer elemento del buffer
    let t2 = thread::spawn(|| unsafe {
        BUFFER[2] = 300;
    });

    // Espera a que ambos hilos terminen su ejecución
    t1.join().unwrap();
    t2.join().unwrap();

    // Bloque unsafe para acceder a la variable estática mutable
    unsafe {
        // Imprime el valor final del buffer después de las modificaciones por los hilos
        println!("Valor dentro de buffer: {:?}", BUFFER);
    }
}

*/

// 2: Reference counted
// Arc es como Rc pero seguro de usar en hilos
// Mutex es como Cell pero seguro de usar en hilos
/*
fn main() {
    // Crea un Arc (puntero atómico de referencia) que contiene un Mutex protegiendo un vector
    let shared_value = Arc::new(Mutex::new(vec![6, 10, 3, 2, 8, 1]));

    // Crea un ámbito de hilos
    thread::scope(|scope| {
        // Primer hilo
        scope.spawn(|| {
            // Clona el Arc para incrementar el conteo de referencias
            let c_shared_value = shared_value.clone();

            // Bloquea el Mutex para acceder al vector de manera segura
            let mut guardia = c_shared_value.lock().unwrap(); // Si un hilo paniquea con el mutex bloqueado (Osea con el guardia vivo) este se contaminará y el metodo lock devolverá un error

            //c_shared_value.lock().unwrap().push(90); // El mutes puede morir en la misma linea si usamos la referencia que toma sin guardarla en un valor
            // Simula una espera de 10 segundos
            sleep(Duration::from_secs(10));

            // Añade el valor 90 al vector
            guardia.push(90);
        });

        // Segundo hilo
        scope.spawn(|| {
            sleep(Duration::from_millis(50)); // sleep para asegurarnos que el spawn anterior va antes
            println!("Impresion antes de mutar");
            // Clona el Arc para incrementar el conteo de referencias
            let c_shared_value = shared_value.clone();

            // Bloquea el Mutex para acceder al vector de manera segura
            let mut guardia = c_shared_value.lock().unwrap();

            // Añade el valor 1000 al vector
            guardia.push(1000);
            println!("Impresion despues de mutar");
        });

        // Imprime el valor actual del vector desde el ámbito del hilo
        println!(
            "Valor desde el threadscope: {:?}",
            shared_value.lock().unwrap()
        );
    });

    // Imprime el valor actual del vector desde la función principal
    println!("Valor desde el main: {:?}", shared_value.lock().unwrap());
}
*/
