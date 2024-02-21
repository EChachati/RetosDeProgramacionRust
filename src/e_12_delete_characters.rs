/*
 * Crea una función que reciba dos cadenas como parámetro (str1, str2)
 * e imprima otras dos cadenas como salida (out1, out2).
 * - out1 contendrá todos los caracteres presentes en la str1 pero NO
 *   estén presentes en str2.
 * - out2 contendrá todos los caracteres presentes en la str2 pero NO
 *   estén presentes en str1.
 */

pub fn delete_characters(str1: &str, str2: &str) -> (String, String) {
    let mut out1 = String::new();
    let mut out2 = String::new();
    for c in str1.chars() {
        if !str2.contains(c) {
            out1.push(c);
        }
    }
    for c in str2.chars() {
        if !str1.contains(c) {
            out2.push(c);
        }
    }
    return (out1, out2);
}
