// Box es la manera en la que alojamos estructuras en el heap
// Es de tamaño conocido en tiempo de compilación a pesar de poder apuntar a estructuras complejas
// Nos ayuda principalmente a resolver temas de lifetimes y provee polimorfismo

fn main() {
    let numero = 100;

    let ref_numero = &numero;

    let box_numero = Box::new(numero);

    println!("Sumando con ref +200: {}", ref_numero + 200);
    println!("Sumando con box +200: {}", *box_numero + 200);

    let im_ref: *const i32;
    {
        let num = 0;
        im_ref = &num as *const i32;
    }
    unsafe { println!("num: {}", *im_ref) };
}

pub trait Golpear {
    fn hit(&self) -> i32;
}

pub struct Programador {
    fuerza: i32,
}

impl Golpear for Programador {
    fn hit(&self) -> i32 {
        self.fuerza
    }
}

pub struct Boxeador {
    fuerza: i32,
}

impl Golpear for Boxeador {
    fn hit(&self) -> i32 {
        self.fuerza
    }
}

pub fn generar_golpeador(f: i32) -> Box<dyn Golpear> {
    if f > 50 {
        Box::new(Boxeador { fuerza: f })
    } else {
        Box::new(Programador { fuerza: f })
    }
}

#[cfg(test)]
mod tests {
    use crate::{generar_golpeador, Golpear};

    #[test]
    fn test_box_1() {
        let fuerzas: [i32; 5] = [200, 10, 3, 143, 49];

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
