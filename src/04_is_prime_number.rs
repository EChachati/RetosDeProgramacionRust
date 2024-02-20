/*
Escribe un programa que se encargue de comprobar si un número es o no primo.
Hecho esto, imprime los números primos entre 1 y 100.
*/

fn is_prime_number(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    for i in 0..100 {
        println!("{} is prime: {}", i, is_prime_number(i));
    }
}
