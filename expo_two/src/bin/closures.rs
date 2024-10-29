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

    let x = 1;

    // el compilador infiere los tipos: | val: i32| -> i32 { val + x };
    let closure = |val| val + x;

    println!("{}", closure(7));

    let mut v = vec![1, 2, 3];

    let mut losure_inner_mut = |nx| {
        v.push(nx);
        println!("{:?}", v)
    };

    losure_inner_mut(15);
    println!("{:?}", v);
}
