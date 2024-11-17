//! Ejemplificar la relación de la base binaria y hexadecimal desde un punto de vista enfocado al hardware

use std::mem::transmute;

/// # Funcion principal del modulo que ejemplifica codigo binario y hexadecimal
/// Relación entre los numeros Decimales, Hexadecimales y Binarios
/// Todo se basa en los bits
///
/// | Decimal | Hexadecimal | Binario  |
/// |---------|-------------|----------|
/// | 0       | 0           | 0000     |
/// | 1       | 1           | 0001     |
/// | 2       | 2           | 0010     |
/// | 3       | 3           | 0011     |
/// | 4       | 4           | 0100     |
/// | 5       | 5           | 0101     |
/// | 6       | 6           | 0110     |
/// | 7       | 7           | 0111     |
/// | 8       | 8           | 1000     |
/// | 9       | 9           | 1001     |
/// | 10      | A           | 1010     |
/// | 11      | B           | 1011     |
/// | 12      | C           | 1100     |
/// | 13      | D           | 1101     |
/// | 14      | E           | 1110     |
/// | 15      | F           | 1111     |
///

fn main() {
    /*
        let binario_1: u8 = 0b11111111;
        let hexadecimal_1: u8 = 0xff;

        println!("El valor de {:b} en decimal es: {}", binario_1, binario_1);
        println!(
            "El valor de {:x} en decimal es: {}",
            hexadecimal_1, hexadecimal_1
        );

        // todo viene de los nibbles, es decir 1/2 byte, 4 bits
        println!("{}", 0b1111 == 0xf);


        println!("Decimal:     {}", u128::MAX);
        println!("Binario:     {:0128b}", u128::MAX);
        println!("Hexadecimal: {:016x}", u128::MAX);

    */

    let num_f = 12.5;
    let num_bits: u64 = unsafe { transmute(num_f) };

    println!("{:064b}", num_bits);
    println!("{}", num_bits);

    let num_int = 233;
    let apuntador_int = &num_int as *const i32;
    let apuntador_int_bits: usize = unsafe { transmute(apuntador_int) };

    println!("{:032b}", num_int);
    println!("{:?}", apuntador_int);
    println!("{:064b}", apuntador_int_bits)
}
