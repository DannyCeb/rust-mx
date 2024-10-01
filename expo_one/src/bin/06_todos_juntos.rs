/*

    Completar el código para que la función test pasé todas las validaciones


*/

// Hint: Estos imports son importantes
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

// region: -- Persona
#[derive(Debug)]
pub struct Persona<'a> {
    pub nombre: &'a str,
    pub edad: u8,
}

// endregion: -- Persona

// region: -- MyHashMap

pub struct MyHashMap<'a, T> {
    hashmap: HashMap<&'a Persona<'a>, T>,
    sumatoria_edad: u128,
    promedio_edad: u8,
}

// endregion: -- MyHashMap

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{MyHashMap, Persona};

    #[test]
    fn test_solution() {
        // Crea dos personas
        let p1 = Persona::new("Daniel", 28);
        let p2 = Persona::new("Genaro", 50);

        // crea un hashmap
        let mut mi_hashmap = MyHashMap::<String>::new();

        // inserta valores en el hashmap
        mi_hashmap.insert(&p1, p1.to_string());
        mi_hashmap.insert(&p2, p2.to_string());

        // valida que mi_hashmap pueda insertar
        // valida que mi_hashmap pueda verificar la existencia de una llave
        assert_eq!(true, mi_hashmap.contains_key(&p1));
        assert_eq!(true, mi_hashmap.contains_key(&p2));

        // valida que mi_hashmap esté guardando los datos como se espera
        assert_eq!(*mi_hashmap.get(&p1).unwrap(), p1.to_string());
        assert_eq!(*mi_hashmap.get(&p2).unwrap(), p2.to_string());

        // valida que mi_hashmap calcule correctamente el promedio de edades
        assert_eq!(39, mi_hashmap.promedio_edad);

        // valida que mi hashmap pueda eliminar datos
        mi_hashmap.remove(&p1);
        mi_hashmap.remove(&p2);

        // valida que mi_hashmap verifique si está vacio o no
        assert_eq!(true, mi_hashmap.is_empty());
    }
}
