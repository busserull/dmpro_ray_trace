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
use material::Material;
use random::Random;
use ray::{Hittable, Ray};
use std::cell::RefCell;
use std::rc::Rc;
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
    let material_ground = Rc::new(Material::Lambertian {
        albedo: Vec3(0.8, 0.8, 0.0),
    });

    let material_center = Rc::new(Material::Lambertian {
        albedo: Vec3(0.7, 0.3, 0.3),
    });

    let material_left = Rc::new(Material::Metal {
        albedo: Vec3(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });

    let material_right = Rc::new(Material::Metal {
        albedo: Vec3(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    let world = vec![
        Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, &material_ground),
        Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, &material_center),
        Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5, &material_left),
        Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, &material_right),
    ];

    let camera = CameraBuilder::new()
        .with_width(400)
        .with_aspect_ratio(16.0 / 9.0)
        .with_samples_per_pixel(50)
        .with_max_depth(50)
        .build();

    camera.render(&world);
}
