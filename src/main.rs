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
use ray::{Hittable, Ray};
use vec3::Vec3;

pub fn ray_color<T>(ray: &Ray, world: &T) -> Color
where
    T: Hittable,
{
    let interval = Interval::new(0.0, f32::INFINITY);

    if let Some(record) = world.hit(ray, interval) {
        return 0.5 * (record.normal + Vec3(1.0, 1.0, 1.0));
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
        .build();

    camera.render(&world);
}
