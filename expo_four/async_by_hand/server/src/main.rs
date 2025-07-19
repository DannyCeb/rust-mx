use std::{
    io::{self, Cursor, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, channel},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use async_runtime::{executor::Executor, sleep::Sleep};
use data_layer::data::Data;

// Flags atómicas para rastrear el estado de los workers
// Cada flag indica si el worker correspondiente está dormido
static FLAGS: [AtomicBool; 3] = [
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
];

/// Inicia un worker thread que procesa conexiones TCP
///
/// # Parámetros
/// - `name`: Identificador del worker
/// - `rx`: Canal receptor para nuevas conexiones
/// - `flag`: Flag atómica para comunicar estado de reposo
///
/// # Comportamiento
/// 1. Recibe conexiones del canal
/// 2. Ejecuta futuros asíncronos para manejar cada cliente
/// 3. Entra en reposo cuando no hay trabajo
fn spawn_worker(
    name: &'static str,
    rx: Receiver<TcpStream>,
    flag: &'static AtomicBool,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut executor = Executor::new();
        loop {
            // Intenta recibir nuevas conexiones
            if let Ok(stream) = rx.try_recv() {
                println!(
                    "{} Received connection {}",
                    name,
                    stream.peer_addr().unwrap()
                );
                // Crea una nueva tarea asíncrona para el cliente
                executor.spawn(handle_client(stream));
            } else {
                // Si no hay tareas, entra en reposo
                if executor.polling.is_empty() {
                    println!("{} is sleeping", name);
                    flag.store(true, Ordering::SeqCst);
                    thread::park(); // Suspende el thread
                }

                // Procesa tareas pendientes
                executor.poll();
            }
        }
    })
}

/// Maneja una conexión cliente de forma asíncrona
///
/// # Flujo de trabajo
/// 1. Lee datos hasta encontrar EOF o bloqueo
/// 2. Deserializa los datos en una estructura `Data`
/// 3. Envía una respuesta después de un retraso simulado
async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_nonblocking(true)?;
    let mut buffer = Vec::new();
    let mut local_buf = [0; 1024]; // Buffer de lectura temporal

    loop {
        match stream.read(&mut local_buf) {
            // Fin de conexión
            Ok(0) => break,

            // Datos recibidos
            Ok(len) => {
                buffer.extend_from_slice(&local_buf[..len]);
            }

            // Bloqueo temporal - procesa si tenemos datos
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                if !buffer.is_empty() {
                    break;
                }
                // Espera breve antes de reintentar
                Sleep::new(Duration::from_millis(10)).await;
                continue;
            }

            // Error fatal
            Err(e) => {
                println!("Failed to read from connection: {}", e);
                return Err(e);
            }
        }
    }

    // Deserializa los datos recibidos
    match Data::deserialize(&mut Cursor::new(buffer.as_slice())) {
        Ok(message) => {
            println!("Received message: {:?}", message);
        }
        Err(e) => {
            println!("Failed to decode message: {}", e);
        }
    }

    // Simula procesamiento y envía respuesta
    Sleep::new(Duration::from_secs(1)).await;
    stream.write_all(b"Hello, Client!")?;

    Ok(())
}

/// Punto de entrada principal del servidor TCP
///
/// # Arquitectura
/// - 3 workers threads con ejecutores asíncronos
/// - Balanceador round-robin para distribuir conexiones
/// - Sistema de reactivación para workers dormidos
fn main() -> io::Result<()> {
    // Canales de comunicación con los workers
    let (one_tx, one_rx) = channel::<TcpStream>();
    let (two_tx, two_rx) = channel::<TcpStream>();
    let (three_tx, three_rx) = channel::<TcpStream>();

    // Inicia los workers
    let worker_one = spawn_worker("One", one_rx, &FLAGS[0]);
    let worker_two = spawn_worker("Two", two_rx, &FLAGS[1]);
    let worker_three = spawn_worker("Three", three_rx, &FLAGS[2]);

    // Configuración de enrutamiento
    let router = [one_tx, two_tx, three_tx];
    let threads = [worker_one, worker_two, worker_three];
    let mut index: usize = 0; // Índice para round-robin

    // Escucha en el puerto 7878
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    println!("Server Listening on port 7878");

    // Bucle principal de aceptación de conexiones
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Envía conexión al worker actual
                let _ = router[index].send(stream);

                // Reactiva worker si estaba dormido
                if FLAGS[index].load(Ordering::SeqCst) {
                    FLAGS[index].store(false, Ordering::SeqCst);
                    threads[index].thread().unpark();
                }

                // Actualiza índice para round-robin
                index = (index + 1) % 3;
            }
            Err(e) => {
                println!("Connection failed: {}", e)
            }
        }
    }

    Ok(())
}
