use std::{
    future::Future,
    io::{self, Write},
    net::TcpStream,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

/// Futuro asíncrono para enviar datos a través de un TcpStream
///
/// Características:
/// - Envía datos de forma no bloqueante
/// - Maneja automáticamente situaciones de bloqueo
/// - Implementa la interfaz Future para integración con ejecutores asíncronos
///
/// Nota: Utiliza un `Arc<Mutex<TcpStream>>` para compartir el socket de forma segura entre tareas.
pub struct TcpSender {
    /// Socket TCP protegido por mutex para acceso concurrente
    pub stream: Arc<Mutex<TcpStream>>,

    /// Datos a enviar (buffer completo)
    pub buffer: Vec<u8>,
}

impl Future for TcpSender {
    /// Tipo de salida: Resultado vacío o error de IO
    type Output = io::Result<()>;

    /// Avanza el estado del futuro en cada poll
    ///
    /// Comportamiento:
    /// 1. Intenta adquirir el lock del socket
    /// 2. Configura modo no bloqueante
    /// 3. Intenta enviar todos los datos
    /// 4. Maneja diferentes casos de escritura
    ///
    /// Estrategias de reactivación:
    /// - Reactiva el waker en casos de bloqueo temporal
    /// - Finaliza inmediatamente en éxito o error fatal
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Intenta adquirir el lock del socket (operación no bloqueante)
        let mut stream = match self.stream.try_lock() {
            Ok(stream) => stream,
            Err(_) => {
                // Si el lock está ocupado, programa nueva reactivación
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        // Configura el socket en modo no bloqueante
        // Nota: Esto podría optimizarse haciéndolo una sola vez
        if let Err(e) = stream.set_nonblocking(true) {
            return Poll::Ready(Err(e));
        }

        // Intenta enviar TODO el buffer en una operación
        match stream.write_all(&self.buffer) {
            // Éxito: todos los datos enviados
            Ok(_) => Poll::Ready(Ok(())),

            // Bloqueo temporal: programa nueva reactivación
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }

            // Error fatal: termina el futuro
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
