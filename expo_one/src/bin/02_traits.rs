/*
    * Ejemplo de como los traits pueden hacernos la vida más fácil

*/

// region: PL

enum LenguajeDeProgramacion {
    Rust,
    Java,
    C,
    Python,
}

impl PartialEq for LenguajeDeProgramacion {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LenguajeDeProgramacion::Rust, LenguajeDeProgramacion::Rust) => true,
            (LenguajeDeProgramacion::Java, LenguajeDeProgramacion::Java) => true,
            (LenguajeDeProgramacion::C, LenguajeDeProgramacion::C) => true,
            (LenguajeDeProgramacion::Python, LenguajeDeProgramacion::Python) => true,
            _ => false,
        }
    }
}

impl PartialOrd for LenguajeDeProgramacion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (LenguajeDeProgramacion::Rust, _) => Some(std::cmp::Ordering::Greater),
            (LenguajeDeProgramacion::Python, _) => Some(std::cmp::Ordering::Less),
            (LenguajeDeProgramacion::C, LenguajeDeProgramacion::Java) => {
                Some(std::cmp::Ordering::Greater)
            }
            _ => Some(std::cmp::Ordering::Equal),
        }
    }
}

// endregion: PL

fn main() {
    use LenguajeDeProgramacion::*;

    println!("¿Rust es mejor que python? {}", Rust > Python);

    println!("¿Python es tan rifado como C? {}", Python >= C);

    println!("¿Java es Java? (xD) {}", Java == Java);
}
