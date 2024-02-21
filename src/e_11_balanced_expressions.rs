/*
 * Crea un programa que comprueba si los paréntesis, llaves y corchetes
 * de una expresión están equilibrados.
 * - Equilibrado significa que estos delimitadores se abren y cierran
 *   en orden y de forma correcta.
 * - Paréntesis, llaves y corchetes son igual de prioritarios.
 *   No hay uno más importante que otro.
 * - Expresión balanceada: { [ a * ( c + d ) ] - 5 }
 * - Expresión no balanceada: { a * ( c + d ) ] - 5 }
 */

use std::collections::HashMap;

fn hashmap_contains_value(hashmap: &HashMap<char, char>, value: char) -> bool {
    return hashmap.values().any(|&val| val == value);
}

pub fn balance_expression(expression: &str) -> bool {
    println!("Expression: {:?}", expression);
    let data: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    let mut delimiters: Vec<char> = Vec::new();
    for e in expression.chars() {
        if data.contains_key(&e) {
            if delimiters.last() == data.get(&e) {
                delimiters.pop();
            } else {
                return false;
            }
        } else if hashmap_contains_value(&data, e) {
            delimiters.push(e);
        }
    }

    return delimiters.is_empty();
}
