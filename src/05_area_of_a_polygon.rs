/*
Crea una única función (importante que sólo sea una) que sea capaz
de calcular y retornar el área de un polígono.
- La función recibirá por parámetro sólo UN polígono a la vez.
- Los polígonos soportados serán Triángulo, Cuadrado, Circulo y Rectángulo.
- Imprime el cálculo del área de un polígono de cada tipo.
*/

use std::f64::consts::PI;

struct Triangle {
    base: f64,
    height: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

trait Polygon {
    fn area(&self) -> f64;
}

impl Polygon for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

impl Polygon for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Polygon for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

fn get_area_of_a_polygon(polygon: Polygon) -> f64 {
    return polygon.area();
}
