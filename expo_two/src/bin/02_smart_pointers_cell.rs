use std::cell::Cell;

fn main() {
    let valor = "Un string importante".to_string();

    println!("String antes de mutar: {}\n", &valor);

    let cell_valor = Cell::new(valor);

    println!("Creación:");

    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        &cell_valor.as_ptr(),
        cell_valor.as_ptr()
    );

    let mut valor_cell_valor = cell_valor.take();

    valor_cell_valor.push_str(" un pusheo");

    println!("\nExtracción:");
    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        &cell_valor.as_ptr(),
        cell_valor.as_ptr()
    );

    cell_valor.replace(valor_cell_valor);

    println!("\nRemplazo:");
    println!(
        "Direccion del apuntador: {:p}\nDireccion a la que apunta: {:p}",
        &cell_valor.as_ptr(),
        cell_valor.as_ptr()
    );
    println!("\nResultado: {:?}", cell_valor.take());
}
