use std::rc::Rc;

use super::linked_list_core::MyDoubleLinkedList;
// Estructura de un iterador que consume la lista

pub struct IntoIter(MyDoubleLinkedList);

impl Iterator for IntoIter {
    type Item = i32;

    // Implementación del método `next`, que devuelve el siguiente elemento al iterar
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.first.clone() {
            None => None, // Si el primer nodo es `None`, no hay elementos por iterar, se devuelve `None`
            Some(pointer) => {
                // Si la lista tiene más de un elemento, avanzamos el puntero del primer nodo
                if !std::ptr::eq(
                    self.0.first.clone().unwrap().as_ptr(),
                    self.0.last.clone().unwrap().as_ptr(),
                ) {
                    // Se mueve el puntero al siguiente nodo en la lista
                    self.0.first = self.0.first.clone().unwrap().as_ref().borrow().next.clone();

                    // Intentamos deshacernos de la referencia al nodo actual para obtener su valor
                    let item = Rc::try_unwrap(pointer);

                    match item {
                        Err(_) => None, // Si el nodo no puede ser desenlazado, devolvemos `None`
                        Ok(item) => Some(item.into_inner().item), // Extraemos el valor y lo devolvemos
                    }
                } else {
                    // Si hay un único nodo, devolvemos su valor y vaciamos los punteros
                    let res = pointer.clone().as_ref().borrow().item;
                    self.0.first = None;
                    self.0.last = None;
                    Some(res)
                }
            }
        }
    }
}

// Implementación de `IntoIterator` para `MyDoubleLinkedList`, permitiendo la conversión en un iterador que consume la lista
impl IntoIterator for MyDoubleLinkedList {
    type Item = i32;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self) // Crea un iterador que consume `MyDoubleLinkedList`
    }
}
