use crate::color::Color;
use crate::random::Random;
use crate::ray::{HitRecord, Ray};
use crate::vec3::Vec3;
use std::cell::RefCell;

pub enum Material {
    Default,
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { refraction_index: f32 },
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

            Dielectric { refraction_index } => {
                let attenuation = Vec3(1.0, 1.0, 1.0);

                let refraction_ratio = if record.front_face {
                    1.0 / *refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = ray.direction.unit_vector();

                let cos_theta = record.normal.dot(-unit_direction).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let must_reflect = refraction_ratio * sin_theta > 1.0;

                let direction = if must_reflect {
                    unit_direction.reflect(record.normal)
                } else {
                    unit_direction.refract(record.normal, refraction_ratio)
                };

                let scatter = Ray::new(record.p, direction);

                Some((scatter, attenuation))
            }
        }
    }
}
