use std::{future::Future, sync::LazyLock};

// Declaración de una constante en el stack usando LazyLock con valor inicial 0
const STACKED_VALUE: LazyLock<i64> = LazyLock::new(|| 0);

// Función asincrónica ejemplo
async fn ejemplo() {
    let aux: i64 = 0; // Variable en el stack
    let aux_vec: Vec<i32> = vec![1, 2, 3]; // Vector en el heap

    // Imprimir la dirección de memoria de la variable en el stack
    println!("aux_dir     : {:p}", &aux);

    // Imprimir la dirección de memoria del segundo elemento del vector (ubicado en el heap)
    println!("heaped_dir  : {:p}", &aux_vec[1]);
}

// Definición de una estructura Foo con un campo foo de tipo i32
struct Foo {
    foo: i32,
}

// Implementación del trait Future para la estructura Foo
impl Future for Foo {
    type Output = ();

    // Función poll que se llama para verificar el estado del Future
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // Obtener la dirección de STACKED_VALUE y convertirla a usize
        let stacked_dir = &STACKED_VALUE as *const _ as usize;

        // Obtener la dirección de self.foo y convertirla a usize
        let foo_dir = &self.foo as *const _ as usize;

        // Comparar las direcciones para determinar si self.foo está en el heap o en el stack
        if stacked_dir > foo_dir {
            // Si foo_dir es menor, se asume que self.foo está en el heap
            println!("Foo : {}", self.foo);
            println!("Foo dir: {:p}", &self.foo);
        }

        // Indicar que el Future está listo
        std::task::Poll::Ready(())
    }
}

// Función main asincrónica usando Tokio
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Imprimir la dirección de memoria de STACKED_VALUE
    println!("stacked_dir : {:p}", &STACKED_VALUE);

    // Llamar a la función ejemplo y esperar a que termine
    ejemplo().await;

    // Crear una instancia de Foo y esperar a que se complete
    Foo { foo: 10000 }.await;

    // Vector para almacenar handles de tareas asincrónicas
    let mut aux_jh_vec: Vec<tokio::task::JoinHandle<()>> = vec![];

    // Crear y ejecutar 10 tareas asincrónicas, cada una con una instancia de Foo
    for l in 0..10 {
        aux_jh_vec.push(tokio::spawn(Foo { foo: l }));
    }

    // Esperar a que todas las tareas asincrónicas terminen
    for l in aux_jh_vec {
        l.await.unwrap();
    }
}
