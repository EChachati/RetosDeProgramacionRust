/*
 Escribe un programa que imprima los 50 primeros números de la sucesión
 de Fibonacci empezando en 0.

 La serie Fibonacci se compone por una sucesión de números en
 la que el siguiente siempre es la suma de los dos anteriores.
 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...
*/

pub fn fibonacci(n: u64) -> u64 {
    let mut fib: u64 = 1;
    let mut prev: u64 = 0;

    for _ in 1..n {
        let temp = fib;
        fib += prev;
        prev = temp;
    }
    return prev;
}

pub fn fibonacci_sequence(n: u64) -> Vec<u64> {
    let mut sequence = Vec::new();
    for i in 1..n + 1 {
        sequence.push(fibonacci(i));
    }
    return sequence;
}
