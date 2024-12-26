use std::future::Future; // Se importa el rasgo Future de la biblioteca estándar

// Define una función asíncrona que retorna un futuro
fn my_async_function() -> impl Future<Output = ()> {
    async {
        // Bloque de código asíncrono que imprime un mensaje
        println!("Hola desde un bloque async!");
    }
}

// Define una estructura llamada MyStruct
struct MyStruct;

// Implementa el rasgo Future para la estructura MyStruct
impl Future for MyStruct {
    type Output = (); // El tipo de valor de salida del futuro es unit (vacío)

    // Implementa la función poll para MyStruct
    fn poll(
        self: std::pin::Pin<&mut Self>, // Se recibe un puntero pin a la estructura
        _cx: &mut std::task::Context<'_>, // Contexto de la tarea
    ) -> std::task::Poll<Self::Output> {
        // Imprime un mensaje desde la función poll
        println!("Hola desde la función Poll!");
        // Indica que el futuro está listo y retorna el valor unit
        std::task::Poll::Ready(())
    }
}

// Define una segunda función asíncrona que retorna un futuro con MyStruct
fn my_second_async_function() -> impl Future<Output = ()> {
    MyStruct {} // Retorna una instancia de MyStruct
}

// Define una tercera función asíncrona que imprime un mensaje
async fn my_third_async_function() {
    println!("Hola desde la función async!");
}

// Define la función principal asíncrona utilizando el macro #[tokio::main]
#[tokio::main]
async fn main() {
    // Espera a que se completen las funciones asíncronas
    my_async_function().await;
    my_second_async_function().await;
    my_third_async_function().await;
}
