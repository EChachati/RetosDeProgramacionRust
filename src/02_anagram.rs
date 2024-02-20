/**

Escribe una función que reciba dos palabras (String) y retorne
verdadero o falso (Bool) según sean o no anagramas.
- Un Anagrama consiste en formar una palabra reordenando TODAS
  las letras de otra palabra inicial.
- NO hace falta comprobar que ambas palabras existan.
- Dos palabras exactamente iguales no son anagrama.

*/

fn are_anagrams(word1: &str, word2: &str) -> bool {
    let mut word1 = word1.to_lowercase();
    let mut word2 = word2.to_lowercase();
    word1.chars().sort();
    word2.chars().sort();
    word1 == word2
}
