use std::{
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

/// Futuro asíncrono que simula una espera temporal
///
/// Este futuro se completa después de un intervalo de tiempo especificado.
/// Útil para implementar timeouts, retrasos programados o limitar frecuencia de operaciones.
pub struct Sleep {
    /// El momento exacto en el que el futuro debe completarse
    when: Instant,
}

impl Sleep {
    /// Crea una nueva instancia de Sleep con la duración especificada
    ///
    /// # Argumentos
    /// * `duration` - El intervalo de tiempo a esperar
    ///
    /// # Ejemplo
    /// ```
    /// use std::time::Duration;
    /// use async_timer::Sleep;
    ///
    /// let sleep_future = Sleep::new(Duration::from_secs(2));
    /// ```
    pub fn new(duration: Duration) -> Self {
        Sleep {
            when: Instant::now() + duration,
        }
    }
}

impl Future for Sleep {
    type Output = ();

    /// Verifica si ha transcurrido el tiempo de espera
    ///
    /// Comportamiento:
    /// - Devuelve `Poll::Ready(())` si ha pasado el tiempo especificado
    /// - Devuelve `Poll::Pending` y reactiva la tarea si aún no ha finalizado
    ///
    /// # Estrategia de reactivación
    /// Siempre reactiva la tarea inmediatamente cuando el tiempo no ha expirado.
    /// Esto puede ser ineficiente en producción ya que causa ejecución continua.
    /// En entornos reales, se recomienda integrar con un sistema de tiempo del sistema operativo.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = Instant::now();

        // Comprueba si ha alcanzado el tiempo objetivo
        if now >= self.when {
            // Tiempo completado
            Poll::Ready(())
        } else {
            // Reactiva inmediatamente para volver a comprobar
            // NOTA: En implementaciones reales usar un timer del sistema
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
