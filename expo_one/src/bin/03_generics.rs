use std::fmt::Display;

fn generic_func<T: Display>(una_cosa: T) -> String {
    // Cualquier variable que implemente Display, se generará el metodo to_string
    una_cosa.to_string()
}

// Struct sencilla que almacena cualquier dato que pueda ser clonado
struct MyStruct<T: Clone> {
    cualquier_cosa: T,
}

// los traits se van acumulando
// T Debe implementar Display para que mi struct lo haga
// T debe implementar clone porque así lo requiere Generica
impl<T: Display + Clone> Display for MyStruct<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Implementación del trait Display para mi estructura generica que muestra cualquier cosa: {}", self.cualquier_cosa)
    }
}

// Implementación sencilla para la estructura generica
// el metodo regresa una nueva instancia de su valor contenido
impl<T: Clone> MyStruct<T> {
    fn new(cualquier_cosa: T) -> Self {
        Self { cualquier_cosa }
    }

    fn metodo1(&self) -> T {
        self.cualquier_cosa.clone()
    }
}

// una implementación exclusiva
// cuando el tipo contenido sea de tipo &str la estructura tendrá funciones adicionales
impl MyStruct<&str> {
    fn metodo_str(&self) -> String {
        format!("Mi valor es: {}", self.cualquier_cosa)
    }
}

//  TODO!  Generar implementación para f64

fn main() {
    let g1 = MyStruct::new(7);
    let g2 = MyStruct::new("Rust");

    println!("Cualquier cosa dentro de g1: {}", g1.metodo1());
    println!("Cualquier cosa dentro de g2: {}", g2.metodo1());
    println!("{}", g2.metodo_str());

    // una funcion generica puede aceptar referencias y variable a la vez
    // CUIDADO con el ownership
    println!("{}", generic_func(g1));
    println!("{}", generic_func(&g2));
}
