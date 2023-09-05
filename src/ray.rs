use crate::interval::Interval;
use crate::vec3::Vec3;

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

impl<T> Hittable for Vec<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut closest = interval.max;
        let mut record = None;

        for object in self.iter() {
            let interval = Interval::new(interval.min, closest);

            if let Some(hit) = object.hit(ray, interval) {
                closest = hit.t;
                record = Some(hit);
            }
        }

        record
    }
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
