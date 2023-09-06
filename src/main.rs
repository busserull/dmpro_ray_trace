mod camera;
mod color;
mod geometry;
mod interval;
mod random;
mod ray;
mod vec3;

use camera::CameraBuilder;
use color::Color;
use geometry::Sphere;
use interval::Interval;
use random::Random;
use ray::{Hittable, Ray};
use vec3::Vec3;

use once_cell::sync::Lazy;

pub fn ray_color<T>(ray: &Ray, world: &T) -> Color
where
    T: Hittable,
{
    static mut RAND: Lazy<Random> = Lazy::new(|| Random::new(1));

    let interval = Interval::new(0.0, f32::INFINITY);

    if let Some(record) = world.hit(ray, interval) {
        let direction = unsafe { RAND.get_vec3_in_hemisphere(record.normal) };
        return 0.5 * ray_color(&Ray::new(record.p, direction), world);
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let world = vec![
        Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0),
    ];

    let camera = CameraBuilder::new()
        .with_width(400)
        .with_aspect_ratio(16.0 / 9.0)
        .with_samples_per_pixel(20)
        .build();

    camera.render(&world);
}
