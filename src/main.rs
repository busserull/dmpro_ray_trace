mod vec3;

fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3");
    println!("{} {}", image_height, image_width);
    println!("255");

    for row in 0..image_height {
        for col in 0..image_width {
            let r = (col as f32) / (image_width - 1) as f32;
            let g = (row as f32) / (image_height - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
