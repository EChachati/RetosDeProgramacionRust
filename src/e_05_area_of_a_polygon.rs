/*
Crea una única función (importante que sólo sea una) que sea capaz
de calcular y retornar el área de un polígono.
- La función recibirá por parámetro sólo UN polígono a la vez.
- Los polígonos soportados serán Triángulo, Cuadrado, Circulo y Rectángulo.
- Imprime el cálculo del área de un polígono de cada tipo.
*/

use std::f64::consts::PI;

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

pub struct Square {
    pub side: f64,
}

pub trait Polygon {
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

impl Polygon for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// The function `get_area_of_a_polygon` calculates the area of a polygon using the `area` method from
/// the `Polygon` trait.
///
/// Arguments:
///
/// * `polygon`: The `polygon` parameter in the `get_area_of_a_polygon` function is expected to be an
/// object that implements the `Polygon` trait. This trait likely contains a method `area()` that
/// calculates and returns the area of the polygon.
///
/// Returns:
///
/// The function `get_area_of_a_polygon` is returning the area of a polygon, which is a `f64`
/// (floating-point number).
pub fn get_area_of_a_polygon(polygon: impl Polygon) -> f64 {
    return polygon.area();
}
