/*
 * Escribe una función que reciba un texto y retorne verdadero o
 * falso (Boolean) según sean o no palíndromos.
 * Un Palíndromo es una palabra o expresión que es igual si se lee
 * de izquierda a derecha que de derecha a izquierda.
 * NO se tienen en cuenta los espacios, signos de puntuación y tildes.
 * Ejemplo: Ana lleva al oso la avellana.
 */

pub fn is_palindrome(text: &str) -> bool {
    let text2: String = text.replace(" ", "").to_string().chars().rev().collect();

    let a = text.replace(" ", "").to_lowercase();
    let b = text2.as_str().to_lowercase();

    return a == b;
}
