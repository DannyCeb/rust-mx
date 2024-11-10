fn main() {
    // 1. map: Aplica una función a cada elemento del iterador
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("map: {:?}", doubled); // [2, 4, 6, 8, 10]

    // 2. filter: Mantiene solo los elementos que cumplen con un predicado
    let numbers = vec![1, 2, 3, 4, 5];
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("filter: {:?}", evens); // [2, 4]

    // 3. filter_map: Combina filter y map en un solo paso
    let numbers = vec![Some(1), None, Some(3), None, Some(5)];
    let present_values: Vec<i32> = numbers.into_iter().filter_map(|x| x).collect();
    println!("filter_map: {:?}", present_values); // [1, 3, 5]

    // 4. cloned: Convierte un iterador de referencias en un iterador de valores clonados
    let words = vec![&0, &1];
    let cloned_words: Vec<_> = words.iter().cloned().collect();
    println!("cloned: {:?}", cloned_words); // ["hello", "world"]

    // 5. enumerate: Retorna un nuevo iterador que produce pares (índice, valor)
    let letters = vec!['a', 'b', 'c'];
    let enumerated: Vec<(usize, &char)> = letters.iter().enumerate().collect();
    println!("enumerate: {:?}", enumerated); // [(0, 'a'), (1, 'b'), (2, 'c')]

    // 6. skip: Omite los primeros n elementos del iterador
    let numbers = vec![1, 2, 3, 4, 5];
    let skipped: Vec<i32> = numbers.iter().skip(2).cloned().collect();
    println!("skip: {:?}", skipped); // [3, 4, 5]

    // 7. take: Detiene el iterador después de n elementos
    let numbers = vec![1, 2, 3, 4, 5];
    let taken: Vec<i32> = numbers.iter().take(3).cloned().collect();
    println!("take: {:?}", taken); // [1, 2, 3]

    // 8. chain: Combina dos iteradores en uno solo
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<i32> = a.into_iter().chain(b.into_iter()).collect();
    println!("chain: {:?}", chained); // [1, 2, 3, 4, 5, 6]
}
