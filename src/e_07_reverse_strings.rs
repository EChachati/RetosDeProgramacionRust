/*
Crea un programa que invierta el orden de una cadena de texto
sin usar funciones propias del lenguaje que lo hagan de forma automática.
- Si le pasamos "Hola mundo" nos retornaría "odnum aloH"
*/

pub fn reverse_string(string: &str) -> String {
    let mut reversed_string = String::new();
    for c in string.chars().rev() {
        reversed_string.push(c);
    }
    return reversed_string;
}
