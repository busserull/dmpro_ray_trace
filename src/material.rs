use crate::color::Color;
use crate::random::Random;
use crate::ray::{HitRecord, Ray};
use std::cell::RefCell;

pub enum Material {
    Default,
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

impl Material {
    pub fn scatter(
        &self,
        ray: Ray,
        record: &HitRecord,
        random: &RefCell<Random>,
    ) -> Option<(Ray, Color)> {
        use Material::*;

        match self {
            Default => None,

            Lambertian { albedo } => {
                let mut direction = record.normal + random.borrow_mut().get_vec3_unit_vector();

                if direction.near_zero() {
                    direction = record.normal;
                }

                let scatter = Ray::new(record.p, direction);
                let attenuation = *albedo;

                Some((scatter, attenuation))
            }

            Metal { albedo } => {
                let reflection = ray.direction.reflect(record.normal).unit_vector();
                let scatter = Ray::new(record.p, reflection);

                let attenuation = *albedo;

                Some((scatter, attenuation))
            }
        }
    }
}
