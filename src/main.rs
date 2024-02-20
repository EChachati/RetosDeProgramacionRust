mod e_01_fizzbuzz;
mod e_02_anagram;
mod e_03_fibonacci;
mod e_04_is_prime_number;
mod e_05_area_of_a_polygon;
mod e_06_aspect_ratio_of_a_image;
mod e_07_reverse_strings;
mod e_08_counting_words;
fn space_between_exercises() {
    println!("\n\n");
}

fn main() {
    println!("Hello, world!");
    space_between_exercises();
    e_01_fizzbuzz::fizzbuzz();

    space_between_exercises();
    println!(
        "Silent and Listen: {}",
        e_02_anagram::are_anagrams("silent", "listen")
    );

    space_between_exercises();
    println!(
        " Escribe un programa que imprima los 50 primeros números de la sucesión
        de Fibonacci empezando en 0.
       
        La serie Fibonacci se compone por una sucesión de números en
        la que el siguiente siempre es la suma de los dos anteriores.
        0, 1, 1, 2, 3, 5, 8, 13...
        "
    );
    println!("Fibonacci 50: {:?}", e_03_fibonacci::fibonacci(50));
    println!(
        "Fibonacci sequence 10: {:?}",
        e_03_fibonacci::fibonacci_sequence(10)
    );

    space_between_exercises();
    println!(
        "Escribe un programa que se encargue de comprobar si un número es o no primo.
        Hecho esto, imprime los números primos entre 1 y 100.
        "
    );
    for i in 1..101 {
        println!(
            "{} is prime: {}",
            i,
            e_04_is_prime_number::is_prime_number(i)
        );
    }

    space_between_exercises();
    println!(
        "Crea una única función (importante que sólo sea una) que sea capaz
        de calcular y retornar el área de un polígono.
        - La función recibirá por parámetro sólo UN polígono a la vez.
        - Los polígonos soportados serán Triángulo, Cuadrado, Circulo y Rectángulo.
        - Imprime el cálculo del área de un polígono de cada tipo.
        "
    );

    let triangle = e_05_area_of_a_polygon::Triangle {
        base: 10.0,
        height: 5.0,
    };

    let rectangle = e_05_area_of_a_polygon::Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let circle = e_05_area_of_a_polygon::Circle { radius: 5.0 };

    let square = e_05_area_of_a_polygon::Square { side: 5.0 };

    println!(
        "Triangle area: {}",
        e_05_area_of_a_polygon::get_area_of_a_polygon(triangle)
    );
    println!(
        "Rectangle area: {}",
        e_05_area_of_a_polygon::get_area_of_a_polygon(rectangle)
    );
    println!(
        "Circle area: {}",
        e_05_area_of_a_polygon::get_area_of_a_polygon(circle)
    );
    println!(
        "Square area: {}",
        e_05_area_of_a_polygon::get_area_of_a_polygon(square)
    );

    println!(
        "Crea un programa que se encargue de calcular el aspect ratio de una
        imagen a partir de una url.
        - Url de ejemplo:
        https://blog.rust-lang.org/images/rust-logo-blk.svg
        - Por ratio hacemos referencia por ejemplo a los \"16:9\" de una
        imagen de 1920*1080px.
        "
    );

    let image = e_06_aspect_ratio_of_a_image::image_from_url(
        "https://logowik.com/content/uploads/images/rust8244.jpg",
    );
    println!(
        "Aspect ratio of image: {}",
        e_06_aspect_ratio_of_a_image::aspect_ratio_of_image(image)
    );
    println!(
        "Aspect ratio of 1920x1080: {}",
        e_06_aspect_ratio_of_a_image::get_ratio(1920, 1080)
    );

    space_between_exercises();

    println!(
        "Crea un programa que invierta el orden de una cadena de texto
        sin usar funciones propias del lenguaje que lo hagan de forma automática.
        - Si le pasamos \"Hola mundo\" nos retornaría \"odnum aloH\"
        "
    );

    println!(
        "Reverse string: {}",
        e_07_reverse_strings::reverse_string("Hola mundo")
    );

    space_between_exercises();
    println!(
        "Crea un programa que cuente cuantas veces se repite cada palabra
        y que muestre el recuento final de todas ellas.
        - Los signos de puntuación no forman parte de la palabra.
        - Una palabra es la misma aunque aparezca en mayúsculas y minúsculas.
        - No se pueden utilizar funciones propias del lenguaje que
          lo resuelvan automáticamente.
        "
    );

    let text = "Hola mundo, hola mundo, hola mundo. que tal";
    println!("Count words: {:?}", e_08_counting_words::count_words(text));
}
