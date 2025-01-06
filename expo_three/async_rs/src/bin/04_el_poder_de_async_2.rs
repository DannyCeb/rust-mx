use std::thread; // Se importa el módulo de hilos
use std::time::Duration; // Se importa la estructura Duration para manejar tiempos de espera
use std::time::Instant; // Se importa la estructura Instant para medir el tiempo transcurrido
use tokio::time::sleep; // Se importa la función sleep del módulo tokio para dormir asíncronamente

// Las esperas con tokio no son bloqueantes representando esperas en las que nuestra atención no es requerida+
// LAs esperas con thread son bloqueantes representando tareas realizadas manualmente y que nos impiden realizar otra cosa
// Las esperas bloqueantes representan tareas ejecutandose por el future mientras que las no bloqueantes representan tareas ejecutandose fuera del future

// Función asíncrona para preparar una taza de café
async fn prep_coffee_mug() {
    sleep(Duration::from_millis(100)).await; // Espera asíncrona de 100 milisegundos
    println!("Vertiendo leche..."); // Imprime mensaje
    thread::sleep(Duration::from_secs(3)); // Espera bloqueante de 3 segundos
    println!("Leche vertida"); // Imprime mensaje
    println!("Poniendo café instantáneo..."); // Imprime mensaje
    thread::sleep(Duration::from_secs(3)); // Espera bloqueante de 3 segundos
    println!("Café instantáneo puesto."); // Imprime mensaje
}

// Función asíncrona para hacer café
async fn make_coffee() {
    println!("Hirviendo la tetera..."); // Imprime mensaje
    sleep(Duration::from_secs(10)).await; // Espera asíncrona de 10 segundos
    println!("Tetera hervida"); // Imprime mensaje
    println!("Vertiendo agua hervida"); // Imprime mensaje
    thread::sleep(Duration::from_secs(3)); // Espera bloqueante de 3 segundos
    println!("Agua hervida vertida"); // Imprime mensaje
}

// Función asíncrona para hacer tostadas
async fn make_toast() {
    println!("Poniendo pan en la tostadora..."); // Imprime mensaje
    sleep(Duration::from_secs(10)).await; // Espera asíncrona de 10 segundos
    println!("Pan tostado."); // Imprime mensaje
    println!("Untando mantequilla en el pan tostado..."); // Imprime mensaje
    thread::sleep(Duration::from_secs(5)); // Espera bloqueante de 5 segundos
    println!("Pan tostado con mantequilla untada."); // Imprime mensaje
}

// Función principal asíncrona utilizando tokio con múltiples hilos de trabajo
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let start_time = Instant::now(); // Guarda el instante de inicio

    // Define la tarea para la primera persona
    let person_one = tokio::task::spawn(async {
        let coffee_mug_step = prep_coffee_mug(); // Paso de preparar la taza de café
        let coffee_step = make_coffee(); // Paso de hacer el café
        let toast_step = make_toast(); // Paso de hacer tostadas

        tokio::join!(coffee_mug_step, coffee_step, toast_step); // Ejecuta los pasos concurrentemente
    });

    // Define la tarea para la segunda persona
    //let person_two = tokio::task::spawn(async {
    //    let coffee_mug_step = prep_coffee_mug(); // Paso de preparar la taza de café
    //    let coffee_step = make_coffee(); // Paso de hacer el café
    //    let toast_step = make_toast(); // Paso de hacer tostadas
    //
    //    tokio::join!(coffee_mug_step, coffee_step, toast_step); // Ejecuta los pasos concurrentemente
    //});

    //let _ = tokio::join!(person_one, person_two); // Ejecuta las tareas de ambas personas

    let _ = person_one.await;
    // -----------------------
    let elapsed_time = start_time.elapsed(); // Calcula el tiempo transcurrido
    println!("Tomó: {} segundos", elapsed_time.as_secs()); // Imprime el tiempo total de ejecución
}
