/*
   Crea un programa que cuente cuantas veces se repite cada palabra
   y que muestre el recuento final de todas ellas.
   - Los signos de puntuación no forman parte de la palabra.
   - Una palabra es la misma aunque aparezca en mayúsculas y minúsculas.
   - No se pueden utilizar funciones propias del lenguaje que
     lo resuelvan automáticamente.
*/

use std::collections::HashMap;

pub fn count_words(text: &str) -> HashMap<String, u32> {
    let mut words: HashMap<String, u32> = HashMap::new();
    let text: String = text.to_lowercase();
    let text: String = text
        .replace(".", "")
        .replace(",", "")
        .replace(";", "")
        .replace(":", "")
        .replace("!", "")
        .replace("?", "")
        .replace("(", "")
        .replace(")", "")
        .replace("[", "")
        .replace("]", "")
        .replace("{", "")
        .replace("}", "")
        .replace("\"", "")
        .replace("'", "")
        .replace("\n", " ")
        .replace("\t", " ")
        .replace("  ", " ");

    for word in text.split(" ") {
        match words.get_mut(word) {
            Some(count) => *count += 1,
            None => {
                words.insert(word.to_string(), 1);
            }
        }
    }

    return words;
}
