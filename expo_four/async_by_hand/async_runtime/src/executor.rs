use crate::waker::create_raw_waker;
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, mpsc},
    task::{Context, Poll, Waker},
};

/// Representa una tarea asíncrona en el ejecutor
///
/// Contiene:
/// - `future`: El futuro a ejecutar, fijado en memoria (pinned)
/// - `waker`: El mecanismo de notificación para reactivar la tarea
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    waker: Arc<Waker>,
}

/// Manejador para recuperar el resultado de una tarea asíncrona
///
/// Proporciona una interfaz para bloquear la ejecución hasta que
/// la tarea completa su procesamiento y devuelve un resultado.
pub struct JoinHandle<T> {
    receiver: mpsc::Receiver<T>,
}

impl<T> JoinHandle<T> {
    /// Bloquea el hilo actual hasta obtener el resultado de la tarea
    ///
    /// # Retorno
    /// - `Ok(T)` si se recibe el resultado exitosamente
    /// - `Err(mpsc::RecvError)` si el canal se cierra antes de recibir el dato
    pub fn block_on(self) -> Result<T, mpsc::RecvError> {
        self.receiver.recv()
    }
}

/// Ejecutor simple para tareas asíncronas
///
/// Utiliza una cola de doble extremo (VecDeque) para gestionar
/// las tareas pendientes de procesamiento.
pub struct Executor {
    pub polling: VecDeque<Task>,
}

impl Executor {
    /// Crea una nueva instancia del ejecutor
    pub fn new() -> Self {
        Executor {
            polling: VecDeque::new(),
        }
    }

    /// Añade una nueva tarea al ejecutor
    ///
    /// # Parámetros
    /// - `future`: Futuro a ejecutar que produce un resultado de tipo `T`
    ///
    /// # Retorno
    /// `JoinHandle<T>`: Manejador para recuperar el resultado asíncrono
    ///
    /// # Comportamiento
    /// 1. Crea un canal para comunicación con la tarea
    /// 2. Envuelve el futuro original para que:
    ///     a. Ejecute el futuro interno
    ///     b. Envíe el resultado a través del canal
    /// 3. Crea un waker para notificaciones
    /// 4. Almacena la tarea en la cola de procesamiento
    pub fn spawn<F, T>(&mut self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + 'static + Send,
        T: Send + 'static,
    {
        // Canal para comunicación con la tarea (Single Producer)
        let (tx, rx) = mpsc::channel();

        // Adaptador que envía el resultado al completarse
        let wrapped_future = Box::pin(async move {
            let result = future.await;
            let _ = tx.send(result); // Ignora errores de envío
        });

        // Construye la tarea con su mecanismo de notificación
        let task = Task {
            future: wrapped_future,
            waker: self.create_waker(),
        };

        // Añade la tarea a la cola de procesamiento
        self.polling.push_back(task);

        JoinHandle { receiver: rx }
    }

    /// Procesa una tarea de la cola de ejecución
    ///
    /// # Comportamiento
    /// 1. Extrae la primera tarea de la cola
    /// 2. Crea un contexto de ejecución con el waker
    /// 3. Intenta progresar la ejecución del futuro
    /// 4. Vuelve a encolar si la tarea está pendiente
    pub fn poll(&mut self) {
        // Extrae la siguiente tarea (si existe)
        let mut task = match self.polling.pop_front() {
            Some(task) => task,
            None => return, // Finaliza si no hay tareas
        };

        // Prepara el contexto de ejecución
        let waker_ref = task.waker.clone();
        let context = &mut Context::from_waker(&waker_ref);

        // Ejecuta el futuro hasta su próximo punto de espera
        match task.future.as_mut().poll(context) {
            Poll::Ready(()) => {} // Tarea completada (no se vuelve a encolar)
            Poll::Pending => {
                // Vuelve a encolar para procesamiento posterior
                self.polling.push_back(task);
            }
        }
    }

    /// Crea un nuevo Waker para notificar al ejecutor
    ///
    /// # Seguridad
    /// Utiliza `create_raw_waker` que debe implementar correctamente:
    /// - Clonación segura
    /// - Notificación eficiente
    /// - Manejo adecuado de memoria
    fn create_waker(&self) -> Arc<Waker> {
        Arc::new(unsafe {
            // Conversión segura solo si create_raw_waker es correcta
            Waker::from_raw(create_raw_waker())
        })
    }
}
