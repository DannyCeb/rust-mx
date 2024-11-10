use std::cell::RefCell;

fn main() {
    // creamos un valor
    let valor = String::from("Otro string importante");
    println!("String antes de mutar: {}\n", &valor);

    // creamos un refcell y un apuntador auxiliar para ver su comportamiento
    let ref_valor = RefCell::new(valor);
    let ref_aux = &ref_valor;

    println!("Creación:");

    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        ref_aux,
        ref_valor.as_ptr()
    );

    // mutamos por referencia mutable
    ref_valor.borrow_mut().push_str(" otro pusheo");

    println!("\nMutación:");

    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        ref_aux,
        ref_valor.as_ptr()
    );

    println!("\nResultado: {}", ref_valor.borrow());
}
