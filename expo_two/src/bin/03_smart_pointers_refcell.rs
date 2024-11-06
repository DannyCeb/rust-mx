use std::cell::RefCell;

fn main() {
    let valor = String::from("Otro string importante");
    println!("String antes de mutar: {}\n", &valor);

    let ref_valor = RefCell::new(valor);
    let ref_aux = &ref_valor;

    println!("Creación:");

    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        ref_aux,
        ref_valor.as_ptr()
    );

    ref_valor.borrow_mut().push_str(" otro pusheo");

    println!("\nMutación:");

    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        ref_aux,
        ref_valor.as_ptr()
    );

    println!("\nResultado: {}", ref_valor.borrow());
}
