fn str_mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ParteImportante<'a> {
    parte: &'a str,
}

fn main() {
    let texto = String::from("Rust es un lenguaje, de programación claro....");
    let primera_parte = texto.split(',').next().expect("No pude encontrar una ','");
    let i: ParteImportante<'_> = ParteImportante {
        parte: primera_parte,
    };

    println!("i: {:?}", i.parte);

    {
        // 'a

        let un_str = "un texto";

        {
            // 'b
            let otro_str = "otro texto";

            println!(
                "El str más largo entre \"{}\" y \"{}\" es: \"{}\"",
                un_str,
                otro_str,
                str_mas_largo(un_str, otro_str) 
            );
        }
    }
}
