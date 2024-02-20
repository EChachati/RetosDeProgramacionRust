/*
Crea un programa que se encargue de calcular el aspect ratio de una
imagen a partir de una url.
- Url de ejemplo:
  https://blog.rust-lang.org/images/rust-logo-blk.svg
- Por ratio hacemos referencia por ejemplo a los "16:9" de una
  imagen de 1920*1080px.
*/

use image::{load_from_memory, DynamicImage};
use reqwest::blocking::get;

pub fn image_from_url(url: &str) -> DynamicImage {
    let image_bytes = get(url).unwrap().bytes().unwrap();
    let image = load_from_memory(&image_bytes).unwrap();
    return image;
}

fn get_gcf(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return get_gcf(b, a % b);
}

pub fn aspect_ratio_of_image(image: DynamicImage) -> String {
    let width = image.width();
    let height = image.height();

    let gcf = get_gcf(width, height);
    let ratio = format!("{}:{}", width / gcf, height / gcf);
    return ratio;
}
