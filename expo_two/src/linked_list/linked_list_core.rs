use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::{Node, StrongPointer};

// Estructura de la lista doblemente enlazada, con punteros al primer y último nodo
#[derive(Debug)]
pub struct MyDoubleLinkedList {
    pub first: StrongPointer, // Puntero al primer nodo de la lista
    pub last: StrongPointer,  // Puntero al último nodo de la lista
}

// Implementación de la lista doblemente enlazada
impl MyDoubleLinkedList {
    // Constructor para crear una lista vacía
    pub fn new() -> Self {
        Self {
            first: None, // Inicialmente no hay primer nodo
            last: None,  // Inicialmente no hay último nodo
        }
    }

    // Función para verificar si la lista está vacía
    pub fn is_empty(&self) -> bool {
        // Devuelve true si el primer nodo es None, indicando que no hay nodos en la lista
        self.first.is_none()
    }

    // Función para verificar si la lista tiene un solo elemento
    pub fn has_one_element(&self) -> bool {
        // Primero, si la lista está vacía, entonces no tiene un solo elemento
        if self.is_empty() {
            false
        } else {
            // Compara la dirección del primer y último nodo usando punteros; si son iguales, hay un único nodo
            std::ptr::eq(
                self.first.clone().unwrap().as_ptr(),
                self.last.clone().unwrap().as_ptr(),
            )
        }
    }

    // Añade un elemento al final de la lista
    pub fn push_back(&mut self, item: i32) {
        // Si la lista está vacía, creamos un nodo que se convierte en el primer y último nodo
        if self.is_empty() {
            let new_node = Some(Rc::new(RefCell::new(Node::new(item, None, None))));
            self.first = new_node.clone(); // El nuevo nodo es ahora el primer nodo
            self.last = new_node; // El nuevo nodo es también el último nodo
        } else {
            // Si la lista no está vacía, creamos un nuevo nodo y lo enlazamos al final
            let new_node = Some(Rc::new(RefCell::new(Node::new(
                item,
                None, // No hay nodo siguiente, ya que será el último nodo
                Some(Rc::downgrade(&self.last.clone().unwrap())), // Enlaza al nodo actual como nodo previo
            ))));

            // Establece el nuevo nodo como el siguiente del nodo que hasta ahora era el último
            self.last.clone().unwrap().as_ref().borrow_mut().next = new_node.clone();

            // Finalmente, actualizamos el puntero al último nodo con el nuevo nodo creado
            self.last = new_node;
        }
    }

    // Añade un elemento al inicio de la lista
    pub fn push_front(&mut self, item: i32) {
        // Si la lista está vacía, el nuevo nodo será tanto el primer como el último nodo
        if self.is_empty() {
            let new_node = Some(Rc::new(RefCell::new(Node::new(item, None, None))));
            self.first = new_node.clone(); // El nuevo nodo es ahora el primer nodo
            self.last = new_node; // El nuevo nodo es también el último nodo
        } else {
            // Si la lista no está vacía, creamos un nuevo nodo que apuntará al nodo actual como siguiente
            let new_node = Some(Rc::new(RefCell::new(Node::new(
                item,
                self.first.clone(), // Apunta al nodo actual como el siguiente
                None,               // No hay nodo previo, ya que será el primer nodo
            ))));

            // El nodo actual (primero) actualiza su puntero anterior al nuevo nodo
            self.first.clone().unwrap().as_ref().borrow_mut().previous =
                Some(Rc::downgrade(&new_node.clone().unwrap()));

            // Actualizamos el puntero al primer nodo con el nuevo nodo creado
            self.first = new_node;
        }
    }

    // Elimina el último elemento de la lista y devuelve su valor
    pub fn remove_last(&mut self) -> Option<i32> {
        if self.is_empty() {
            // Si la lista está vacía, no hay elementos que eliminar
            None
        } else if self.has_one_element() {
            // Si la lista tiene un solo elemento, guardamos su valor y vaciamos la lista
            let element = self.first.clone().unwrap().as_ref().borrow().item;
            self.first = None; // Eliminamos el único nodo de la lista
            self.last = None; // Eliminamos el único nodo de la lista
            Some(element) // Devolvemos el valor del elemento eliminado
        } else {
            // Si hay más de un elemento, eliminamos el último nodo
            let element = self.last.clone().unwrap().as_ref().borrow().item;
            let aux_weak_ptr = self
                .last
                .clone()
                .unwrap()
                .as_ref()
                .borrow()
                .previous
                .clone();
            self.last = Weak::upgrade(&aux_weak_ptr.unwrap()); // Actualizamos el último nodo al nodo previo
            self.last.clone().unwrap().as_ref().borrow_mut().next = None; // Removemos el enlace al nodo eliminado
            Some(element) // Devolvemos el valor del elemento eliminado
        }
    }

    // Elimina el primer elemento de la lista y devuelve su valor
    pub fn remove_first(&mut self) -> Option<i32> {
        if self.is_empty() {
            // Si la lista está vacía, no hay elementos que eliminar
            None
        } else if self.has_one_element() {
            // Si la lista tiene un solo elemento, guardamos su valor y vaciamos la lista
            let element = self.first.clone().unwrap().as_ref().borrow().item;
            self.first = None; // Eliminamos el único nodo de la lista
            self.last = None; // Eliminamos el único nodo de la lista
            Some(element) // Devolvemos el valor del elemento eliminado
        } else {
            // Si hay más de un elemento, eliminamos el primer nodo
            let element = self.first.clone().unwrap().as_ref().borrow().item;
            self.first = self.first.clone().unwrap().as_ref().borrow().next.clone(); // Actualizamos el primer nodo
            self.first.clone().unwrap().as_ref().borrow_mut().previous = None; // Removemos el enlace al nodo eliminado
            Some(element) // Devolvemos el valor del elemento eliminado
        }
    }
}
