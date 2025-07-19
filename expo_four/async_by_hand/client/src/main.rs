use async_runtime::{executor::Executor, reciever::TcpReceiver, sender::TcpSender};
use data_layer::data::Data;
use std::{
    io,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Instant,
};

/// Envía datos estructurados al servidor y recibe respuesta
///
/// # Flujo de operación:
/// 1. Establece conexión TCP con el servidor
/// 2. Serializa la estructura Data
/// 3. Envía los datos serializados usando TcpSender
/// 4. Recibe la respuesta usando TcpReceiver
/// 5. Convierte la respuesta a String UTF-8
///
/// # Parámetros
/// - `field1`, `field2`, `field3`: Datos a enviar
///
/// # Retorno
/// Respuesta del servidor como String o error de IO
async fn send_data(field1: u32, field2: u16, field3: String) -> io::Result<String> {
    // Conexión compartida con Arc<Mutex> para uso seguro en futuros
    let stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:7878")?));

    // Construye y serializa los datos
    let message = Data {
        field1,
        field2,
        field3,
    };
    let serialized = message.serialize()?;

    // Envía los datos (operación asíncrona)
    TcpSender {
        stream: stream.clone(),
        buffer: serialized,
    }
    .await?; // Espera hasta completar el envío

    // Prepara para recibir respuesta
    let receiver = TcpReceiver {
        stream: stream.clone(),
        buffer: Vec::new(),
    };

    // Recibe datos y convierte a String
    let response_bytes = receiver.await?;
    String::from_utf8(response_bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Respuesta no UTF-8"))
}

/// Punto de entrada del cliente de carga
///
/// Realiza una prueba de carga enviando 4000 peticiones concurrentes
/// al servidor y mide el tiempo total de ejecución.
fn main() -> io::Result<()> {
    // Inicializa ejecutor y contenedor de manejadores
    let mut executor = Executor::new();
    let mut handles = Vec::with_capacity(4000);

    // Registra tiempo inicial
    let start = Instant::now();

    // Genera 4000 tareas concurrentes
    for i in 0..4000 {
        // Crea tarea asíncrona para enviar datos
        let handle = executor.spawn(send_data(i, i as u16, format!("Mensaje {}", i)));
        handles.push(handle);
    }

    // Hilo dedicado para procesar tareas
    std::thread::spawn(move || {
        loop {
            executor.poll(); // Procesa todas las tareas pendientes
        }
    });

    println!("Esperando resultados...");

    // Recopila resultados de todas las tareas
    for handle in handles {
        match handle.block_on().unwrap() {
            Ok(result) => println!("Respuesta: {}", result),
            Err(e) => println!("Error: {}", e),
        };
    }

    // Calcula y muestra tiempo total
    let duration = start.elapsed();
    println!("Tiempo total: {:?}", duration);

    Ok(())
}
