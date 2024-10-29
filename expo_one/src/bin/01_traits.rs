use std::fmt::format;

// region: traits
trait Describir {
    fn describirse(&self) -> String;
}

// endregion: traits

// region: Libro
struct Libro {
    titulo: String,
    autor: String,
    n_paginas: u64,
}

impl Libro {
    fn new(titulo: String, autor: String, n_paginas: u64) -> Self {
        Self {
            titulo,
            autor,
            n_paginas,
        }
    }
}

// Cumplimos con el trato/compromiso
impl Describir for Libro {
    fn describirse(&self) -> String {
        //  el compromiso se basa en la firma del metodo
        format!(
            "Descripción de libro:\nTitulo: {}\nAutor: {}\nNumero de paginas: {}\n",
            self.titulo, self.autor, self.n_paginas
        )
    }
}

// endregion: Libro

// region: MarcasComputadoras

pub enum Marcascomputadoras {
    Lenovo,
    HP,
    Apple,
    Huawei,
}

impl Describir for Marcascomputadoras {
    fn describirse(&self) -> String {
        match self {
            Marcascomputadoras::Lenovo => format!("Marca: Lenovo"),
            Marcascomputadoras::Apple => format!("Marca: Apple"),
            Marcascomputadoras::HP => format!("Marca: HP"),
            Marcascomputadoras::Huawei => format!("Marca: Huawei"),
        }
    }
}

// endregion: MarcasComputadoras

// region: Computadoras

struct Computadora {
    marca: Marcascomputadoras,
    cpu: String,
    ram: u64,
    almacenamiento: u128,
}

impl Computadora {
    fn new(marca: Marcascomputadoras, cpu: String, ram: u64, almacenamiento: u128) -> Self {
        Self {
            marca,
            cpu,
            ram,
            almacenamiento,
        }
    }
}

impl Describir for Computadora {
    fn describirse(&self) -> String {
        format!(
            "\n{}\nCPU: {}\nRAM: {} GB\n Almacenamiento: {}GB",
            self.marca.describirse(), // Se puede llamar a cualquier metodo/funcion dentro del cuerpo de una funcion de un trait
            self.cpu,
            self.ram,
            self.almacenamiento
        )
    }
}

// endregion: Computadoras

// TODO!
// Hacer que los tests funcionen

fn main() {
    let l1 = Libro::new(
        "Cracking the coding interview".to_string(), // ** Display
        String::from("Gayle LaakMann McDowell"),
        696,
    );

    let c1: Computadora = Computadora::new(
        Marcascomputadoras::Huawei,
        "Intel Core i7".to_string(),
        32,
        1024,
    );

    println!("{}", l1.describirse());

    println!("{}", c1.describirse());
}

impl Describir for Vec<i32> {
    fn describirse(&self) -> String {
        format!(
            "soy un vecor con las siguientes caracteristicas:\nLongitud: {}\nCapacidad: {}",
            self.len(),
            self.capacity(),
        )
    }
}

impl Describir for i32 {
    fn describirse(&self) -> String {
        format!("Mi valor es: {}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::Describir;

    #[test]
    fn describe_vector() {
        assert_eq!(
            "soy un vecor con las siguientes caracteristicas:\nLongitud: 0\nCapacidad: 0",
            Vec::<i32>::new().describirse()
        );
    }

    #[test]
    fn describir_i32() {
        assert_eq!("Mi valor es: 0", 0.describirse());
    }
}
