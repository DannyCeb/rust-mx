// Box es la manera en la que alojamos estructuras en el heap
// Es de tamaño conocido en tiempo de compilación a pesar de poder apuntar a estructuras complejas
// Nos ayuda principalmente a resolver temas de lifetimes y provee polimorfismo

fn main() {
    let numero = 100;

    let ref_numero = &numero;

    let box_numero = Box::new(numero);

    println!("Sumando con ref +200: {}", ref_numero + 200);
    println!("Sumando con box +200: {}", *box_numero + 200);

    /*
    let im_ref: &i32;
    {
        let num = 0;
        im_ref = &num;
    }
    println!("num: {}", im_ref);
    */
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn test_box_1() {
        let fuerzas: [u8; 5] = [200, 10, 3, 143, 49];

        let mut golpeadores = Vec::<Box<dyn Golpear>>::new();

        for fuerza in fuerzas {
            golpeadores.push(generar_golpeador(fuerza));
        }

        for (index, golpeador) in golpeadores.iter().enumerate() {
            println!(
                "Golpeador #{} te golpeó con fuerza de: {}",
                index + 1,
                golpeador.hit()
            )
        }
    }
}
    */
