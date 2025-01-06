use tokio::sync::mpsc;
use tokio::task;

/**
* Existen distintos canales en tokio
   * mpsc (multi-producer, single-consumer): Permite que múltiples productores envíen mensajes a un único consumidor. Ideal para escenarios donde varias tareas necesitan enviar datos a una única tarea receptora1.

   * oneshot (single-producer, single-consumer): Diseñado para enviar un único valor de un productor a un consumidor. Útil cuando se espera una respuesta inmediata a una solicitud1.

   * broadcast (multi-producer, multi-consumer): Permite que varios productores envíen mensajes y todos los consumidores reciban cada mensaje. Útil para eventos donde todos los receptores necesitan conocer la misma información1.

   * watch (multi-producer, multi-consumer): Permite enviar múltiples valores, pero solo retiene el último valor enviado. Los receptores solo ven el valor más reciente1.
*/

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32); // Canal con capacidad máxima de 32 mensajes

    // Task 1: Envía números del 1 al 5
    let tx1 = tx.clone();
    let producer1 = task::spawn(async move {
        for i in 1..=5 {
            tx1.send(i).await.unwrap();
            println!("Producer 1 sent: {}", i);
        }
    });

    // Task 2: Envía números del 6 al 10
    let tx2 = tx.clone();
    let producer2 = task::spawn(async move {
        for i in 6..=10 {
            tx2.send(i).await.unwrap();
            println!("Producer 2 sent: {}", i);
        }
    });

    // Task 3: Envía números del 11 al 15
    let producer3 = task::spawn(async move {
        for i in 11..=15 {
            tx.send(i).await.unwrap();
            println!("Producer 3 sent: {}", i);
        }
    });

    // Task que lee los mensajes
    let consumer = task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Consumer received: {}", msg);
        }
    });

    // Esperamos a que todas las tasks terminen
    producer1.await.unwrap();
    producer2.await.unwrap();
    producer3.await.unwrap();
    consumer.await.unwrap();
}
