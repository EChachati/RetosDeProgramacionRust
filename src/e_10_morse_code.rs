/*
Crea un programa que sea capaz de transformar texto natural a código
morse y viceversa.
- Debe detectar automáticamente de qué tipo se trata y realizar
  la conversión.
- En morse se soporta raya "—", punto ".", un espacio " " entre letras
  o símbolos y dos espacios entre palabras "  ".
- El alfabeto morse soportado será el mostrado en
  https://es.wikipedia.org/wiki/Código_morse.
*/

use std::collections::HashMap;

pub fn translate_to_morse(text: &str) -> String {
    let morse_code: HashMap<&str, &str> = HashMap::from([
        ("a", ".-"),
        ("b", "-..."),
        ("c", "-.-."),
        ("d", "-.."),
        ("e", "."),
        ("f", "..-."),
        ("g", "--."),
        ("h", "...."),
        ("i", ".."),
        ("j", ".---"),
        ("k", "-.-"),
        ("l", ".-.."),
        ("m", "--"),
        ("n", "-."),
        ("o", "---"),
        ("p", ".--."),
        ("q", "--.-"),
        ("r", ".-."),
        ("s", "..."),
        ("t", "-"),
        ("u", "..-"),
        ("v", "...-"),
        ("w", ".--"),
        ("x", "-..-"),
        ("y", "-.--"),
        ("z", "--.."),
        ("1", ".----"),
        ("2", "..---"),
        ("3", "...--"),
        ("4", "....-"),
        ("5", "....."),
        ("6", "-...."),
        ("7", "--..."),
        ("8", "---.."),
        ("9", "----."),
        ("0", "-----"),
        (" ", "  "),
    ]);

    let mut morse = String::new();

    for letter in text.split("") {
        if letter != "" {
            let lowercase_letter = letter.to_lowercase();
            let coded_letter = morse_code.get(lowercase_letter.as_str()).unwrap();
            println!("{} -> {:?}", lowercase_letter, coded_letter);
            morse.push_str(coded_letter)
        }
    }

    return morse;
}
