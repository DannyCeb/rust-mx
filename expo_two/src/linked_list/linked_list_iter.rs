use super::{linked_list_core::MyDoubleLinkedList, StrongPointer};

// Estructura de un iterador inmutable que permite recorrer `MyDoubleLinkedList` sin modificarla
pub struct Iter<'a> {
    next: StrongPointer, // Apunta al siguiente nodo en el recorrido
    _marker: std::marker::PhantomData<&'a MyDoubleLinkedList>, // Garantiza la duración del iterador
}

// Implementación del trait `Iterator` para el iterador inmutable
impl<'a> Iterator for Iter<'a> {
    type Item = i32;

    // Método `next` devuelve el siguiente valor sin modificar la lista
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let node_borrow = node.borrow(); // Accedemos a los datos del nodo de forma segura
            self.next = node_borrow.next.clone(); // Avanzamos al siguiente nodo
            node_borrow.item // Devolvemos el valor actual del nodo
        })
    }
}

// Implementación del método `iter` en `MyDoubleLinkedList`, que crea un iterador inmutable
impl MyDoubleLinkedList {
    pub fn iter(&self) -> Iter {
        Iter {
            next: self.first.clone(), // Inicia el iterador desde el primer nodo de la lista
            _marker: std::marker::PhantomData, // Usado para gestionar el tiempo de vida
        }
    }
}
