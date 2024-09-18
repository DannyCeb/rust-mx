struct Generica<T: Clone> {
    cualquier_cosa: T,
}

impl<T: Clone> Generica<T> {
    fn generica(&self) -> T {
        self.cualquier_cosa.clone()
    }
}

impl Generica<String> {
    fn generica_s(&self) -> String {
        format!("Mi valor es: {}", self.cualquier_cosa)
    }
}

fn main() {
    let g1 = Generica { cualquier_cosa: 7 };
    let g2 = Generica {
        cualquier_cosa: String::from("Rust"),
    };

    println!("{}", g1.generica());
    println!("{}", g2.generica());
    println!("{}", g2.generica_s());
}
