use std::sync::RwLock;
use std::thread;
use std::time::Duration;

fn main() {
    // Crea un RwLock protegiendo un vector de 5 elementos inicializados en 0.
    let data = RwLock::new(vec![0; 5]);

    // Crea un scope para la ejecución de múltiples hilos.
    thread::scope(|scope| {
        // Primer hilo para escritura.
        scope.spawn(|| {
            for i in 0..5 {
                // Obtiene un lock de escritura y modifica el vector.
                let mut vec = data.write().unwrap();
                vec[i] = i as i64; // Escribe el valor de i en la posición i.
                println!("Hilo de escritura: {:?}", *vec);
                drop(vec); // Libera el lock de escritura.
                thread::sleep(Duration::from_millis(2)); // Pausa para simular procesamiento.
            }
        });

        // Segundo hilo para lectura.
        scope.spawn(|| {
            for _ in 5..10 {
                // Obtiene un lock de lectura y lee el vector.
                let vec = data.read().unwrap();
                println!("Hilo de lectura: {:?}", *vec);
                drop(vec); // Libera el lock de lectura.
                thread::sleep(Duration::from_millis(2)); // Pausa para simular procesamiento.
            }
        });
    });
}
