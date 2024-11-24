use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

fn main() {
    // Crea un Mutex protegiendo una cola doblemente terminada (VecDeque).
    let queue = Mutex::new(VecDeque::<i32>::new());

    // Crea un scope para la ejecución de múltiples hilos.
    thread::scope(|scope| {
        // Hilo consumidor
        let consumer_thread = scope.spawn(|| loop {
            // Obtiene un lock del Mutex y saca el primer elemento de la cola.
            let item = queue.lock().unwrap().pop_front();

            // Verifica si hay un valor en la cola.
            if let Some(value) = item {
                println!("El hilo consumidor sacó de la cola: {}", value);
            } else {
                // Si la cola está vacía, el hilo se estaciona esperando ser despertado.
                thread::park();
            }
        });

        // Agrega elementos a la cola y despierta al hilo consumidor.
        for item in 0..10 {
            // Obtiene un lock del Mutex y agrega un elemento al final de la cola.
            queue.lock().unwrap().push_back(item);
            // Despierta al hilo consumidor.
            consumer_thread.thread().unpark();
            // Pausa para simular tiempo de procesamiento.
            thread::sleep(Duration::from_millis(300));
        }
    });
}
