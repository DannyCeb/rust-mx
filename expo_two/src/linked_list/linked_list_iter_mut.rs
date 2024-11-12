use super::{linked_list_core::MyDoubleLinkedList, StrongPointer};

// Estructura de un iterador mutable que permite modificar los elementos de `MyDoubleLinkedList` durante el recorrido
pub struct IterMut<'a> {
    next: StrongPointer, // Apunta al siguiente nodo en el recorrido
    _marker: std::marker::PhantomData<&'a mut MyDoubleLinkedList>, // Para asegurar el tiempo de vida
}

// Implementación de `Iterator` para `IterMut`, permitiendo modificación de los elementos
impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    // Método `next` devuelve una referencia mutable al siguiente valor en el iterador
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let mut node_borrow = node.borrow_mut(); // Tomamos una referencia mutable al nodo actual
            self.next = node_borrow.next.clone(); // Avanzamos al siguiente nodo
                                                  // Usamos un puntero crudo para extender el tiempo de vida mutable
            let ptr = &mut node_borrow.item as *mut _;
            unsafe { &mut *ptr } // Convertimos el puntero a una referencia mutable segura
        })
    }
}

// Implementación del método `iter_mut` en `MyDoubleLinkedList` que crea un iterador mutable
impl MyDoubleLinkedList {
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            next: self.first.clone(), // Comienza el iterador desde el primer nodo
            _marker: std::marker::PhantomData, // Usado para gestionar el tiempo de vida
        }
    }
}
