use std::cell::Cell;

fn main() {
    // creamos un valor
    let valor = "Un string importante".to_string();

    println!("String antes de mutar: {}\n", &valor);

    // creamos un cell y una referencia al cell para ver su comportamiento
    let cell_valor = Cell::new(valor);
    let ref_ayuda = &cell_valor;

    println!("Creación:");

    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        ref_ayuda,
        cell_valor.as_ptr()
    );

    // sacamos el valor del cell
    let mut valor_cell_valor = cell_valor.take();

    // mutamos el valor que nos devolvió el metodo take
    valor_cell_valor.push_str(" un pusheo");

    println!("\nExtracción:");
    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        ref_ayuda,
        cell_valor.as_ptr()
    );

    // intercambiamos el valor del cell por uno nuevo (En este caso el valor mutado)
    cell_valor.replace(valor_cell_valor);

    println!("\nRemplazo:");
    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        ref_ayuda,
        cell_valor.as_ptr()
    );
    println!("\nResultado: {:?}", cell_valor.take());
}
