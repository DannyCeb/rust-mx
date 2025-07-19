use std::{
    future::Future,
    io::{self, Read},
    net::TcpStream,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

/// Futuro asíncrono para recibir datos de un TcpStream
///
/// Características principales:
/// - Lee datos de forma no bloqueante
/// - Acumula datos en un buffer interno
/// - Implementa la interfaz Future para integración con ejecutores asíncronos
///
/// Nota: Utiliza un `Arc<Mutex<TcpStream>>` para compartir el socket de forma segura entre tareas.
pub struct TcpReceiver {
    /// Socket TCP protegido por mutex para acceso concurrente
    pub stream: Arc<Mutex<TcpStream>>,

    /// Buffer acumulador para datos recibidos
    pub buffer: Vec<u8>,
}

impl Future for TcpReceiver {
    /// Tipo de salida: Resultado con los datos recibidos o error de IO
    type Output = io::Result<Vec<u8>>;

    /// Avanza el estado del futuro en cada poll
    ///
    /// Comportamiento:
    /// 1. Intenta adquirir el lock del socket
    /// 2. Configura modo no bloqueante
    /// 3. Lee datos al buffer local
    /// 4. Maneja diferentes casos de lectura
    ///
    /// Estrategias de reactivación:
    /// - Siempre reactiva el waker en casos de bloqueo
    /// - Propaga errores inmediatamente
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Intenta adquirir el lock del socket (operación no bloqueante)
        let mut stream = match self.stream.try_lock() {
            Ok(stream) => stream,
            Err(_) => {
                // Si no se puede adquirir el lock, programa nueva reactivación
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        // Configura el socket en modo no bloqueante
        // Nota: Esto podría moverse a la creación para evitar llamadas repetidas
        if let Err(e) = stream.set_nonblocking(true) {
            return Poll::Ready(Err(e));
        }

        let mut local_buf = [0; 1024]; // Buffer de lectura temporal

        match stream.read(&mut local_buf) {
            // Conexión cerrada: Devuelve datos acumulados
            Ok(0) => Poll::Ready(Ok(self.buffer.to_vec())),

            // Datos recibidos: Acumula y programa nueva reactivación
            Ok(n) => {
                // Libera explícitamente el lock antes de modificar el estado
                std::mem::drop(stream);

                // Acumula datos en el buffer principal
                self.buffer.extend_from_slice(&local_buf[..n]);

                // Programa nueva ejecución para más datos
                cx.waker().wake_by_ref();
                Poll::Pending
            }

            // Bloqueo temporal: Programa nueva reactivación
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }

            // Error fatal: Termina el futuro
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
