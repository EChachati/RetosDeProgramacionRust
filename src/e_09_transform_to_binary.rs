/*
 Crea un programa se encargue de transformar un número
 decimal a binario sin utilizar funciones propias del lenguaje que lo hagan directamente.
*/

pub fn decimal_to_binary(decimal: u32) -> String {
    let mut binary = String::new();
    let mut decimal = decimal;
    while decimal > 0 {
        binary.push_str(&format!("{}", decimal % 2));
        decimal /= 2;
    }
    binary.chars().rev().collect()
}
