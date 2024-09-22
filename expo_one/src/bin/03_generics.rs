use std::fmt::Display;

fn generic_func<T: Display>(una_cosa: T) {
    println!("Parametro: {}", una_cosa)
}

// Struct sencilla que almacena cualquier dato que pueda ser clonado
struct Generica<T: Clone> {
    cualquier_cosa: T,
}

// los traits se van acumulando
// T Debe implementar Display para que mi struct lo haga
// T debe implementar clone porque así lo requiere Generica
impl<T: Display + Clone> Display for Generica<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Implementación del trait Display para mi estructura generica que muestra cualquier cosa: {}", self.cualquier_cosa)
    }
}

// Implementación sencilla para la estructura generica
// el metodo regresa una nueva instancia de su valor contenido
impl<T: Clone> Generica<T> {
    fn new(cualquier_cosa: T) -> Self {
        Self { cualquier_cosa }
    }

    fn generica(&self) -> T {
        self.cualquier_cosa.clone()
    }
}

// una implementación exclusiva
// cuando el tipo contenido sea de tipo &str la estructura tendrá funciones adicionales
impl Generica<&str> {
    fn generica_s(&self) -> String {
        format!("Mi valor es: {}", self.cualquier_cosa)
    }
}

fn main() {
    let g1 = Generica::new(7);
    let g2 = Generica::new("Rust");

    println!("Cualquier cosa dentro de g1: {}", g1.generica());
    println!("Cualquier cosa dentro de g2: {}", g2.generica());
    println!("{}", g2.generica_s());

    // una funcion generica puede aceptar referencias y variable a la vez
    // CUIDADO con el ownership
    generic_func(g1);
    generic_func(&g2);

    //g1.cualquier_cosa;
    g2.cualquier_cosa;
}
