mod color;
mod ray;
mod vec3;

use color::Color;
use vec3::Vec3;

fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3");
    println!("{} {}", image_height, image_width);
    println!("255");

    for row in 0..image_height {
        for col in 0..image_width {
            let pixel_color = Vec3(
                col as f32 / (image_width - 1) as f32,
                row as f32 / (image_height - 1) as f32,
                0.0,
            );

            println!("{}", pixel_color);
        }
    }
}
