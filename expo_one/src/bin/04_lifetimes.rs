/**
 * Explicación de lifetimes
 * El uso de los tics 'a, 'b, 'c, etc es la manera en la que se identifican los lifetimes
 * Se puede usar cualquier nombre (Siguiendo los mismos lineamientos que las variables) en lso tics
 * Son necesarios para evitar el uso de un garbage collector
 *
 */

fn main() {
    // 'a
    // Inicia scope principal, scope de variable_1
    let mut variable_1 = 10;
    {
        // ´b
        // Inicia scope de Variable_2
        //
        //
        let variable_2 = 2;
        let ref_variable_1 = &mut variable_1;
        //
        //

        {
            // 'c
            // Inicia Scope de Variable_3
            //
            //
            let variable_3 = 5;
            println!(
                "( 10 * 2) / 5 = {}",
                (*ref_variable_1 * variable_2) / variable_3
            );
            //
            //
            // Fin del scope 'c de por lo tanto variable_3 es eliminada de memoria
        }
        // TODO! crear referencias rodeando esta instrucción
        *ref_variable_1 += 1;
        //
        //
        println!("Valor de variable 1 desde scope 'b: {}", variable_1);
        //
        // Fin del scope 'b de por lo tanto variable_2 y ref_variable1 son eliminadas de memoria
    }

    println!("valor de variable 1 en scope 'a: {}", variable_1);
    //
    //
    // Termina la ejecución del programa, fin del scope principal, por lo tanto variable_1 es eliminada de memoria
}
