use crate::ray::{HitRecord, Hittable, Ray};
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center_to_ray_origin = ray.origin - self.center;

        let a = ray.direction.len_squared();
        let b_half = center_to_ray_origin.dot(ray.direction);
        let c = center_to_ray_origin.len_squared() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let d = discriminant.sqrt();
        let root = (-b_half - d) / a;

        if root <= t_min || root >= t_max {
            let root = (-b_half + d) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let t = root;
        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let front_face = ray.direction.dot(normal) < 0.0;

        Some(HitRecord {
            t,
            p,
            normal,
            front_face,
        })
    }
}
