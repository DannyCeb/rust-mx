use super::linked_list_core::MyDoubleLinkedList;

// Implementación de `Default` para `MyDoubleLinkedList`, permitiendo su inicialización por defecto
impl Default for MyDoubleLinkedList {
    fn default() -> Self {
        Self::new() // Devuelve una lista nueva vacía
    }
}

// Implementación del trait `Display` para `MyDoubleLinkedList`, permitiendo formateo personalizado
impl std::fmt::Display for MyDoubleLinkedList {
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
