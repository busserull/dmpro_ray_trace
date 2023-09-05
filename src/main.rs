mod color;
mod geometry;
mod ray;
mod vec3;

use color::Color;
use geometry::Sphere;
use ray::{Hittable, Ray};
use vec3::Vec3;

fn ray_color<T>(ray: &Ray, world: &T) -> Color
where
    T: Hittable,
{
    if let Some(record) = world.hit(ray, 0.0, f32::INFINITY) {
        return 0.5 * (record.normal + Vec3(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height = std::cmp::max(1, (image_width as f32 / aspect_ratio) as i32);

    // World

    let world = vec![
        Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0),
    ];

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::origin();

    let viewport_u = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            println!("{}", pixel_color);
        }
    }
}
