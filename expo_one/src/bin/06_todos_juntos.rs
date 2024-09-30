/*
    Crear un hashmap que use como llave referencias a una estructura custom y como valor los Strings que provengan del trait Display

    estructura custom:
        nombre
        edad



*/

use std::{collections::HashMap, fmt::Display, ops::Deref};

#[derive(Debug)]
pub struct Persona<'a> {
    nombre: &'a str,
    edad: u8,
}

impl<'a> Persona<'a> {
    pub fn new(nombre: &'a str, edad: u8) -> Self {
        Self { nombre, edad }
    }
}

impl<'a> std::hash::Hash for Persona<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edad.hash(state)
    }
}

impl<'a> Display for Persona<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nombre: {}\nEdad: {}", self.nombre, self.edad)
    }
}

impl<'a> PartialEq for Persona<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl<'a> Eq for Persona<'a> {}

pub struct MyHashMap<'a, T> {
    hashmap: HashMap<&'a Persona<'a>, T>,
    sumatoria_edad: u128,
    promedio_edad: u8,
}

impl<'a, T> Deref for MyHashMap<'a, T> {
    type Target = HashMap<&'a Persona<'a>, T>;

    fn deref(&self) -> &Self::Target {
        &self.hashmap
    }
}

impl<'a, T> MyHashMap<'a, T> {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
            promedio_edad: 0,
            sumatoria_edad: 0,
        }
    }

    pub fn insert(&mut self, k: &'a Persona, v: T) {
        self.sumatoria_edad += k.edad as u128;
        self.hashmap.insert(k, v);
        self.promedio_edad = (self.sumatoria_edad / self.hashmap.keys().len() as u128) as u8;
    }
}

fn main() {
    let p1 = Persona::new("Daniel", 28);
    let p2 = Persona::new("Genaro", 50);

    let mut mi_hashmap: MyHashMap<'_, String> = MyHashMap::new();

    mi_hashmap.insert(&p1, p1.to_string());
    mi_hashmap.insert(&p2, p2.to_string());

    println!("llave1: {:?}\nvalor1:{}", &p1, mi_hashmap.get(&p1).unwrap());

    println!("llave2: {:?}\nvalor2:{}", &p2, mi_hashmap.get(&p2).unwrap());

    println!("Promedio de edades: {}", mi_hashmap.promedio_edad);
}
