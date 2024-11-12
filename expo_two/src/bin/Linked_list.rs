use expo_two::linked_list::linked_list_core::MyDoubleLinkedList;

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
