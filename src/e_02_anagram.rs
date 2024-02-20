/**

Escribe una función que reciba dos palabras (String) y retorne
verdadero o falso (Bool) según sean o no anagramas.
- Un Anagrama consiste en formar una palabra reordenando TODAS
  las letras de otra palabra inicial.
- NO hace falta comprobar que ambas palabras existan.
- Dos palabras exactamente iguales no son anagrama.

*/

pub fn are_anagrams(word1: &str, word2: &str) -> bool {
    println!(
        "
  Escribe una función que reciba dos palabras (String) y retorne
  verdadero o falso (Bool) según sean o no anagramas.
  - Un Anagrama consiste en formar una palabra reordenando TODAS
    las letras de otra palabra inicial.
  - NO hace falta comprobar que ambas palabras existan.
  - Dos palabras exactamente iguales no son anagrama.
  "
    );

    let mut word1 = word1.to_lowercase();
    let mut word2 = word2.to_lowercase();
    let mut char_vector1: Vec<char> = word1.chars().collect();
    let mut char_vector2: Vec<char> = word2.chars().collect();

    char_vector1.sort();
    char_vector2.sort();

    word1 = char_vector1.iter().collect::<String>();
    word2 = char_vector2.iter().collect::<String>();

    return word1 == word2;
}
