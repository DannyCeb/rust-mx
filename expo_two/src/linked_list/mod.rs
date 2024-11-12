pub mod linked_list_core;
pub mod linked_list_into_iter;
pub mod linked_list_iter;
pub mod linked_list_iter_mut;
pub mod linked_list_traits;

// Importamos tipos necesarios de la librería estándar de Rust
use std::{
    cell::RefCell, // Permite mutabilidad interna, útil para nodos mutables en una estructura inmutable// Trait para implementar la impresión formateada
    rc::{Rc, Weak}, // Rc y Weak: contadores de referencia para punteros fuertes y débiles
};

// Definimos tipos alias para mejorar la legibilidad en el código
// StrongPointer es un alias para un puntero fuerte opcional a un Rc que contiene un nodo en RefCell
pub type StrongPointer = Option<Rc<RefCell<Node>>>;
// WeakPointer es un alias para un puntero débil opcional que apunta a un nodo
pub type WeakPointer = Option<Weak<RefCell<Node>>>;

// Definimos la estructura Node que representará un nodo en una lista doblemente enlazada

#[derive(Debug)]
pub struct Node {
    item: i32,             // Dato almacenado en el nodo, en este caso un entero
    next: StrongPointer,   // Puntero fuerte opcional al siguiente nodo
    previous: WeakPointer, // Puntero débil opcional al nodo anterior
}

// Implementación de la estructura Node
impl Node {
    // Constructor de Node que crea un nodo con un valor dado y punteros opcionales a nodos previo y siguiente
    pub fn new(item: i32, next: StrongPointer, previous: WeakPointer) -> Self {
        Self {
            item,
            next,
            previous,
        }
    }
}
