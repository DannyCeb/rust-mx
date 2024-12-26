use reqwest::Error; // Se importa el tipo de error de la biblioteca reqwest
use serde::Deserialize; // Se importa la funcionalidad para deserializar JSON de la biblioteca serde
use serde_json; // Se importa la biblioteca serde_json
use std::time::Instant; // Se importa la estructura Instant para medir el tiempo

#[allow(dead_code)] // Se indica al compilador que ignore si esta estructura no se utiliza
#[derive(Debug, Deserialize)] // Se indica que esta estructura se puede deserializar desde JSON y se puede imprimir para depuración
struct MyCustomResponse {
    url: String,             // Campo que almacena una URL en formato de cadena de texto
    args: serde_json::Value, // Campo que almacena argumentos en formato JSON
}

// Función asíncrona que obtiene datos desde una URL con un retraso especificado en segundos
async fn fetch_data(seconds: u64) -> Result<MyCustomResponse, Error> {
    // Formatea la URL de la solicitud con el retraso en segundos
    let request_url = format!("https://httpbin.org/delay/{}", seconds);
    // Realiza la solicitud HTTP GET de manera asíncrona
    let response = reqwest::get(&request_url).await?;
    // Deserializa la respuesta JSON en una estructura MyCustomResponse
    let delayed_response = response.json::<MyCustomResponse>().await?;
    // Retorna la respuesta deserializada
    Ok(delayed_response)
}

// Función asíncrona que mide el tiempo de ejecución de llamadas bloqueantes
async fn test_bloqueante() -> Result<(), Error> {
    // Guarda el instante de inicio
    let start_time = Instant::now();

    // Realiza cuatro llamadas bloqueantes a fetch_data con diferentes retrasos
    fetch_data(2).await?;
    fetch_data(3).await?;
    fetch_data(1).await?;
    fetch_data(5).await?;

    // Calcula la duración total de las llamadas bloqueantes
    let duration = start_time.elapsed();
    // Imprime el tiempo total de ejecución de las llamadas bloqueantes
    println!("Tiempo que tomó el código bloqueante: {:?}", duration);
    Ok(())
}

// Función asíncrona que mide el tiempo de ejecución de llamadas no bloqueantes
async fn test_no_bloqueante() {
    // Guarda el instante de inicio
    let start_time = Instant::now();
    // Realiza las llamadas no bloqueantes a fetch_data de manera concurrente utilizando tokio::join!
    let _ = tokio::join!(fetch_data(2), fetch_data(3), fetch_data(1), fetch_data(5));
    // Calcula la duración total de las llamadas no bloqueantes
    let duration = start_time.elapsed();
    // Imprime el tiempo total de ejecución de las llamadas no bloqueantes
    println!("Tiempo que tomó el código no bloqueante: {:?}", duration);
}

// Función principal asíncrona que se ejecuta cuando el programa inicia
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Ejecuta el test bloqueante y espera su resultado
    test_bloqueante().await?;
    // Ejecuta el test no bloqueante
    test_no_bloqueante().await;
    Ok(())
}
