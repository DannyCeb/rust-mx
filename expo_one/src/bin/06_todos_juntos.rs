/*

    Completar el código para que la función test pasé todas las validaciones


*/

// Hint: Estos imports son importantes
use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::{Deref, DerefMut},
};

// region: -- Persona
#[derive(Debug)]
pub struct Persona<'a> {
    pub nombre: &'a str,
    pub edad: u8,
}

// endregion: -- Persona

impl<'a> Persona<'a> {
    pub fn new(nombre: &'a str, edad: u8) -> Self {
        Self { nombre, edad }
    }
}

impl<'a> Display for Persona<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nombre: {}\nEdad: {}", self.nombre, self.edad)
    }
}

impl<'a> Hash for Persona<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edad.hash(state)
    }
}

impl<'a> PartialEq for Persona<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl<'a> Eq for Persona<'a> {}

// region: -- MyHashMap

pub struct MyHashMap<'a, T> {
    hashmap: HashMap<&'a Persona<'a>, T>,
    sumatoria_edad: u128,
    promedio_edad: u8,
}

impl<'a, T> MyHashMap<'a, T> {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
            sumatoria_edad: 0,
            promedio_edad: 0,
        }
    }
}

impl<'a, T> Deref for MyHashMap<'a, T> {
    type Target = HashMap<&'a Persona<'a>, T>;

    fn deref(&self) -> &Self::Target {
        &self.hashmap
    }
}

impl<'a, T> DerefMut for MyHashMap<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hashmap
    }
}

impl<'a, T> MyHashMap<'a, T> {
    pub fn insert(&mut self, k: &'a Persona, v: T) {
        self.sumatoria_edad += k.edad as u128;
        self.hashmap.insert(k, v);
        self.promedio_edad = (self.sumatoria_edad / self.hashmap.keys().len() as u128) as u8;
    }
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

        for l in [1, 2, 3, 4] {
            let aux_p = Persona::new("Luis", 32);
            mi_hashmap.insert(&aux_p, aux_p.to_string());
        }
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
