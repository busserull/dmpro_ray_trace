use crate::color::Color;
use crate::random::Random;
use crate::ray::{HitRecord, Ray};
use std::cell::RefCell;

pub enum Material {
    Default,
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
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

            Metal { albedo, fuzz } => {
                let reflection = ray.direction.reflect(record.normal).unit_vector();
                let fuzziness = *fuzz * random.borrow_mut().get_vec3_unit_vector();
                let scatter = Ray::new(record.p, reflection + fuzziness);

                let attenuation = *albedo;

                (scatter.direction.dot(record.normal) > 0.0).then_some((scatter, attenuation))
            }
        }
    }
}
