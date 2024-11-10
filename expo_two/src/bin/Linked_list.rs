// Importamos tipos necesarios de la librería estándar de Rust
use std::{
    cell::RefCell, // Permite mutabilidad interna, útil para nodos mutables en una estructura inmutable
    fmt::Display,  // Trait para implementar la impresión formateada
    rc::{Rc, Weak}, // Rc y Weak: contadores de referencia para punteros fuertes y débiles
};

// Definimos tipos alias para mejorar la legibilidad en el código
// StrongPointer es un alias para un puntero fuerte opcional a un Rc que contiene un nodo en RefCell
type StrongPointer = Option<Rc<RefCell<Node>>>;
// WeakPointer es un alias para un puntero débil opcional que apunta a un nodo
type WeakPointer = Option<Weak<RefCell<Node>>>;

// Definimos la estructura Node que representará un nodo en una lista doblemente enlazada
#[derive(Debug)]
struct Node {
    item: i32,             // Dato almacenado en el nodo, en este caso un entero
    next: StrongPointer,   // Puntero fuerte opcional al siguiente nodo
    previous: WeakPointer, // Puntero débil opcional al nodo anterior
}

// Implementación de la estructura Node
impl Node {
    // Constructor de Node que crea un nodo con un valor dado y punteros opcionales a nodos previos y siguientes
    fn new(item: i32, next: StrongPointer, previous: WeakPointer) -> Self {
        Self {
            item,
            next,
            previous,
        }
    }
}

// Estructura de la lista doblemente enlazada, con punteros al primer y último nodo
#[derive(Debug)]
struct MyDoubleLinkedList {
    first: StrongPointer, // Puntero al primer nodo de la lista
    last: StrongPointer,  // Puntero al último nodo de la lista
}

// Implementación de la lista doblemente enlazada
impl MyDoubleLinkedList {
    // Constructor para crear una lista vacía
    fn new() -> Self {
        Self {
            first: None, // Inicialmente no hay primer nodo
            last: None,  // Inicialmente no hay último nodo
        }
    }

    // Función para verificar si la lista está vacía
    fn is_empty(&self) -> bool {
        // Devuelve true si el primer nodo es None, indicando que no hay nodos en la lista
        self.first.is_none()
    }

    // Función para verificar si la lista tiene un solo elemento
    fn has_one_element(&self) -> bool {
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
    fn push_back(&mut self, item: i32) {
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
    fn push_front(&mut self, item: i32) {
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
    fn remove_last(&mut self) -> Option<i32> {
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
    fn remove_first(&mut self) -> Option<i32> {
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

// Estructura de un iterador que consume la lista
struct IntoIter(MyDoubleLinkedList);

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

// Estructura de un iterador inmutable que permite recorrer `MyDoubleLinkedList` sin modificarla
struct Iter<'a> {
    next: Option<Rc<RefCell<Node>>>, // Apunta al siguiente nodo en el recorrido
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
    fn iter(&self) -> Iter {
        Iter {
            next: self.first.clone(), // Inicia el iterador desde el primer nodo de la lista
            _marker: std::marker::PhantomData, // Usado para gestionar el tiempo de vida
        }
    }
}

// Estructura de un iterador mutable que permite modificar los elementos de `MyDoubleLinkedList` durante el recorrido
struct IterMut<'a> {
    next: Option<Rc<RefCell<Node>>>, // Apunta al siguiente nodo en el recorrido
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
    fn iter_mut(&mut self) -> IterMut {
        IterMut {
            next: self.first.clone(), // Comienza el iterador desde el primer nodo
            _marker: std::marker::PhantomData, // Usado para gestionar el tiempo de vida
        }
    }
}

// region: traits

// Implementación de `Default` para `MyDoubleLinkedList`, permitiendo su inicialización por defecto
impl Default for MyDoubleLinkedList {
    fn default() -> Self {
        Self::new() // Devuelve una lista nueva vacía
    }
}

// Implementación del trait `Display` para `MyDoubleLinkedList`, permitiendo formateo personalizado
impl Display for MyDoubleLinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Buscamos un resultado en el formato: [1, 2, 3, 4]
        if self.is_empty() {
            write!(f, "[]") // Caso especial: si la lista está vacía, devolvemos `[]`
        } else {
            let mut res = String::new(); // Creamos un string vacío para almacenar el resultado
            res += "["; // Añadimos el primer corchete

            // Usamos un nodo auxiliar para recorrer los elementos de la lista
            let mut aux_node = self.first.clone();

            // Recorremos la lista mientras haya nodos
            while let Some(node) = aux_node {
                let item = node.borrow().item; // Extraemos el valor del nodo
                res += &item.to_string(); // Añadimos el valor al string `res`
                res += ", "; // Añadimos coma y espacio entre los elementos
                aux_node = node.borrow().next.clone(); // Avanzamos al siguiente nodo
            }

            // Quitamos la última coma y el espacio extra, y añadimos el corchete de cierre
            res.pop();
            res.pop();
            res += "]";

            write!(f, "{}", res) // Imprimimos el resultado formateado
        }
    }
}

// endregion: traits

// Implementación del trait `FromIterator` para `MyDoubleLinkedList`, permitiendo construir una lista a partir de un iterador
impl FromIterator<i32> for MyDoubleLinkedList {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut res_list = MyDoubleLinkedList::default(); // Creamos una lista vacía
        for item in iter {
            res_list.push_back(item); // Añadimos cada elemento al final de la lista
        }
        res_list // Devolvemos la lista construida
    }
}

fn main() {
    // Crear una lista vacía
    let mut list = MyDoubleLinkedList::new();

    // Insertar elementos en la lista al final
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!(
        "Lista después de push_back(10), push_back(20), push_back(30): {}",
        list
    );

    // Insertar elementos al principio de la lista
    list.push_front(5);
    list.push_front(1);

    println!("Lista después de push_front(5) y push_front(1): {}", list);

    // Eliminar el primer y último elemento
    let first_removed = list.remove_first();
    println!(
        "Primer elemento eliminado: {:?}, Lista después de remove_first(): {}",
        first_removed, list
    );

    let last_removed = list.remove_last();
    println!(
        "Último elemento eliminado: {:?}, Lista después de remove_last(): {}",
        last_removed, list
    );

    // Verificar si la lista está vacía o tiene un solo elemento
    println!("¿La lista está vacía?: {}", list.is_empty());
    println!(
        "¿La lista tiene un solo elemento?: {}",
        list.has_one_element()
    );

    // Iterar sobre la lista de manera inmutable y mostrar los valores
    println!("Iteración inmutable sobre la lista:");
    for item in list.iter() {
        print!("{} ", item);
    }
    println!();

    // Iterar sobre la lista de manera mutable y modificar los elementos
    println!("Modificando elementos de la lista en iteración mutable (+10):");
    for item in list.iter_mut() {
        *item += 10;
    }
    println!("Lista después de iter_mut(): {}", list);

    // Crear una lista a partir de un iterador utilizando FromIterator
    let from_iter_list: MyDoubleLinkedList = vec![100, 200, 300].into_iter().collect();
    println!(
        "Lista creada a partir de un iterador (vec![100, 200, 300]): {}",
        from_iter_list
    );

    // Uso de combinators con iteradores
    let even_elements: Vec<i32> = list.iter().filter(|&x| x % 2 == 0).collect();
    println!(
        "Elementos pares en la lista (usando filter): {:?}",
        even_elements
    );

    let mapped_elements: Vec<i32> = list.iter().map(|x| x * 2).collect();
    println!(
        "Elementos de la lista multiplicados por 2 (usando map): {:?}",
        mapped_elements
    );

    let first_two_elements: Vec<i32> = list.iter().take(2).collect();
    println!(
        "Primeros dos elementos de la lista (usando take): {:?}",
        first_two_elements
    );

    let skip_first_two_elements: Vec<i32> = list.iter().skip(2).collect();
    println!(
        "Lista sin los primeros dos elementos (usando skip): {:?}",
        skip_first_two_elements
    );

    let enumerated_elements: Vec<(usize, i32)> = list.iter().enumerate().collect();
    println!(
        "Elementos enumerados de la lista (usando enumerate): {:?}",
        enumerated_elements
    );

    // Demostración de `chain` combinando dos listas
    let mut list_a: MyDoubleLinkedList = vec![1, 2, 3].into_iter().collect();
    let list_b: MyDoubleLinkedList = vec![4, 5, 6].into_iter().collect();
    list_a = list_a.into_iter().chain(list_b.into_iter()).collect();
    println!(
        "Lista combinada (usando chain) de [1, 2, 3] y [4, 5, 6]: {}",
        list_a
    );
}
