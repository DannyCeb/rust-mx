use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Estructura contador que implementa el trait Future
#[derive(Debug)]
struct MyCounter {
    count: u32,
}

// Implementación de MyCounter
impl MyCounter {
    fn new() -> MyCounter {
        MyCounter { count: 0 }
    }
}

// Implementación del trait Future para MyCounter
impl Future for MyCounter {
    type Output = u32;

    // Función poll que se llama cada vez que se intenta avanzar en la ejecución de la tarea
    fn poll(
        // self es un puntero a la estructura MyCounter, debe estar en un Pin para garantizar que no se mueva en memoria
        mut self: Pin<&mut Self>,
        // cx es un contexto administrado por el runtime de tokio que se utiliza para despertar la tarea
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        self.count += 1;
        // Si el contador es menor a 10, se despierta el waker y se retorna Pending
        if self.count < 10 {
            // Al momento de llamar al waker y devolver pending se le conoce como "yield"
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            // Si el contador es mayor o igual a 10, se retorna Ready
            Poll::Ready(self.count)
        }
    }
}

#[tokio::main]
async fn main() {
    // Se crea una instancia de MyCounter
    let mut couter = MyCounter::new();

    // Se imprime el valor de contador antes de llamar await
    println!("Valor de contador antes de llamar await: {:?}", couter);

    // Se crea una referencia mutable a la instancia de MyCounter
    let ref_count = &mut couter;
    // Se llama await sobre la referencia mutable
    // usar el metodo await consume la referencia mutable
    ref_count.await;

    // Se imprime el valor de contador despues de llamar await
    println!("Valor de contador despues de llamar await: {:?}", couter);
}
