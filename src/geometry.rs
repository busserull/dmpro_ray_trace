use crate::interval::Interval;
use crate::material::Material;
use crate::ray::{HitRecord, Hittable, Ray};
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: &Rc<Material>) -> Self {
        Self {
            center,
            radius,
            material: Rc::clone(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        let center_to_ray_origin = ray.origin - self.center;

        let a = ray.direction.len_squared();
        let b_half = center_to_ray_origin.dot(ray.direction);
        let c = center_to_ray_origin.len_squared() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let d = discriminant.sqrt();
        let mut root = (-b_half - d) / a;

        if !interval.surrounds(root) {
            root = (-b_half + d) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let front_face = ray.direction.dot(normal) < 0.0;
        let material = Rc::clone(&self.material);

        Some(HitRecord {
            t,
            p,
            normal,
            front_face,
            material,
        })
    }
}
