use image::{GenericImageView, ImageBuffer, Rgb, Rgba};
use std::env;
use std::path::Path;

fn limpiar_lsb(color: u8) -> u8 {
    // 7
    color & !1
}

fn limpiar_imagen(input_path: &str, output_path: &str) {
    // 3
    let img = image::open(&Path::new(input_path)).expect("Failed to open input image");
    let (width, height) = img.dimensions();

    // 4
    let mut img_limpiada = ImageBuffer::new(width, height);

    // 5
    for (x, y, pixel) in img.pixels() {
        let Rgba(data) = pixel;

        // 6
        let r = limpiar_lsb(data[0]);
        let g = limpiar_lsb(data[1]);
        let b = limpiar_lsb(data[2]);

        // 8
        img_limpiada.put_pixel(x, y, Rgb([r, g, b]));
    }

    // 9
    img_limpiada
        .save(&Path::new(output_path))
        .expect("Failed to save output image");
    println!("Imagen limpiada guardada como {}", output_path);
}

fn main() {
    // 1
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Uso: {} <imagen_entrada> <imagen_salida>", args[0]);
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // 2
    limpiar_imagen(input_path, output_path);
}
