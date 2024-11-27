use std::{
    collections::VecDeque,  // Importa VecDeque, una cola doblemente terminada.
    sync::{Condvar, Mutex}, // Importa Condvar y Mutex para sincronización.
    thread,                 // Importa la librería de hilos.
    time::Duration,         // Importa la estructura de duración para pausas.
};

fn main() {
    let queue = Mutex::new(VecDeque::<i32>::new()); // Crea un Mutex que protege una cola VecDeque.

    let not_empty = Condvar::new(); // Crea una variable de condición para notificaciones de cola no vacía.

    thread::scope(|scope| {
        // Hilo consumidor
        scope.spawn(|| loop {
            let mut q = queue.lock().unwrap(); // Bloquea el Mutex y obtiene acceso a la cola.

            // Espera hasta que haya un elemento en la cola.
            let item = loop {
                match q.pop_front() {
                    Some(item) => {
                        break item; // Si hay un elemento, lo saca y lo retorna.
                    }
                    None => {
                        println!("waiting...");
                        q = not_empty.wait(q).unwrap(); // Si la cola está vacía, espera la señal de no vacío.
                    }
                }
            };
            drop(q); // Libera el lock del Mutex.
            dbg!(item); // Depura e imprime el ítem procesado.
        });

        // Hilo productor
        scope.spawn(|| {
            for l in 0.. {
                queue.lock().unwrap().push_back(l); // Bloquea el Mutex y añade un elemento a la cola.
                not_empty.notify_one(); // Notifica a un hilo esperando que la cola no está vacía.
                thread::sleep(Duration::from_millis(1000)); // Pausa para simular tiempo de procesamiento.
            }
        });
    });
}
