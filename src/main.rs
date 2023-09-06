mod camera;
mod color;
mod geometry;
mod interval;
mod material;
mod random;
mod ray;
mod vec3;

use camera::CameraBuilder;
use color::Color;
use geometry::Sphere;
use interval::Interval;
use random::Random;
use ray::{Hittable, Ray};
use std::cell::RefCell;
use vec3::Vec3;

pub fn ray_color<T>(ray: Ray, world: &T, depth: u32, random: &RefCell<Random>) -> Color
where
    T: Hittable,
{
    if depth == 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    let interval = Interval::new(0.001, f32::INFINITY);

    if let Some(record) = world.hit(ray, interval) {
        if let Some((scatter, attenuation)) = record.material.scatter(ray, &record, random) {
            return attenuation * ray_color(scatter, world, depth - 1, random);
        }
        return Vec3(0.0, 0.0, 0.0);
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
        .with_max_depth(50)
        .build();

    camera.render(&world);
}
