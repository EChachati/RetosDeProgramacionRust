/*
Escribe una función que calcule y retorne el factorial de un número dado
de forma recursiva.
*/

pub fn recursive_factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    return n * recursive_factorial(n - 1);
}
