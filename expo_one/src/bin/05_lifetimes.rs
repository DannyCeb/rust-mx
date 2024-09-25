fn str_mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ParteImportante<'a> {
    parte: &'a str,
}

fn main() {
    let texto = String::from("Rust es un lenguaje, de programación claro....");
    let primera_parte = texto.split(',').next().expect("Could not find a ','");
    let i = ParteImportante {
        parte: primera_parte,
    };
}
