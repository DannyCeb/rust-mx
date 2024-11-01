fn main() {
    /*
     * Los closures son funciones que implementan uno de los siguientes traits
     *      - FnOnce
     *          Toma ownership de los valores, sólo se puede llamar una sola vez
     *      - FnMut
     *          Muta valores por referencia, puede ser llamada más de una vez
     *      - Fn
     *          No muta nada en su scope por lo que puede ser llamada más de una vez
     *
     * El compilador siempre va a inferir el tomar los datos de la manera menos restrictiva posible, es decir va a inferir los traits en el orden Fn, FnMut y FnOnce
     *
     */

    // Ejemplo de FnOnce

    let my_string = String::from("Hello, FnOnce!");

    // Closure FnOnce que toma ownership de `my_string`
    let consume_string = || my_string.max("7".to_string());

    // Llamar al closure, moviendo `my_string` dentro del closure
    println!("Fn Once: {}", consume_string());
    //consume_string();

    // Ejemplo de FnMut
    let mut value = 0;
    let mut add_to_value = |x| value += x;
    add_to_value(1);
    add_to_value(2);
    println!("FnMut: {}", value); // FnMut: 3

    // Ejemplo de Fn
    let calculate_sum = |a: i32, b: i32| a + b;
    let result = calculate_sum(5, 6);
    println!("Fn: {}", result); // Fn: 11
    println!("Fn: {}", calculate_sum(1, 2));

    // Ejemplo de Fn con ownership
    let consume_string = |s: String| println!("FnOnce: {}", s);
    let my_string = String::from("Hello, FnOnce!");
    consume_string(my_string); // Ok, toma el ownership
                               // consume_string(my_string); // Error, my_string ya ha sido movido
}
