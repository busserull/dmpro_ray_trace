use crate::color::Color;
use crate::ray::{HitRecord, Ray};

pub enum Material {
    Default,
}

impl Material {
    pub fn scatter(ray: Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}
