use std::future::Future;
use std::sync::LazyLock;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        // number of threads
        .worker_threads(4)
        // Some tasks can be spawned using spawn blocking function and those ones are managed by the blocking threads
        .max_blocking_threads(1)
        // functions to be executed on some events
        .on_thread_start(|| println!("Thread strarting for runtime A"))
        .on_thread_stop(|| println!("Thread stopping for runtime A"))
        // timeout for blocking tasks
        .thread_keep_alive(Duration::from_secs(60))
        /*
           El valor está determinado en ticks, valores recomendados:
           - 61 para multihilo
           - 31 para un solo hilo
           Es el intervalo de tiempo en el que los workers revisarán si hay una tarea nueva a ejecutar, con esto en cuenta mientras más seguido revisen menos tiempo destinarán a ejecutar las tareas
           Generalmente se recomienda un analisis de la cantidad de awaits que se realiza, es una relación inversa, es decir a mas awaits, menor el numero de ticks recomendados
        */
        .global_queue_interval(61)
        // un worker thread se parkea si no hay nada que hacer
        .on_thread_park(|| println!("Thread parking for tuntime A"))
        .thread_name("our custom runtime A")
        .thread_stack_size(3 * 1024 * 1024)
        // Activa las funciones relacionadas al tiempo en nuestro runtime
        .enable_time()
        .build()
        .unwrap()
});

pub fn spawn_task<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    RUNTIME.spawn(future)
}

async fn sleep_example() -> u32 {
    println!("sleeping for 2 seconds");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("done sleeping");
    20
}

fn main() {
    let handle = spawn_task(sleep_example());
    println!("Spawned task");
    println!("task status: {}", handle.is_finished());
    std::thread::sleep(Duration::from_secs(3));
    println!("task status: {}", handle.is_finished());
    let result = RUNTIME.block_on(handle).unwrap();
    println!("task result: {}", result);
}
