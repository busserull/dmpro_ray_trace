use crate::vec3::Vec3;

/// This is an XorShift based PRNG taken from the
/// 3rd edition of Numerical Recipes

pub struct Random(u64);

impl Random {
    pub fn new(seed: u64) -> Self {
        let state = std::cmp::max(seed, 1);
        Self(state)
    }

    pub fn get(&mut self) -> u64 {
        self.0 = self.0 ^ self.0.wrapping_shr(21);
        self.0 = self.0 ^ self.0.wrapping_shl(35);
        self.0 = self.0 ^ self.0.wrapping_shr(4);

        self.0
    }

    pub fn get_f32(&mut self) -> f32 {
        (self.get() as f32) / (u64::MAX as f32 + 1.0)
    }

    pub fn get_f32_in_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.get_f32()
    }

    pub fn get_vec3(&mut self) -> Vec3 {
        Vec3(self.get_f32(), self.get_f32(), self.get_f32())
    }

    pub fn get_vec3_in_range(&mut self, min: f32, max: f32) -> Vec3 {
        Vec3(
            self.get_f32_in_range(min, max),
            self.get_f32_in_range(min, max),
            self.get_f32_in_range(min, max),
        )
    }

    pub fn get_vec3_in_unit_sphere(&mut self) -> Vec3 {
        loop {
            let v = self.get_vec3_in_range(-1.0, 1.0);
            if v.len_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn get_vec3_unit_vector(&mut self) -> Vec3 {
        self.get_vec3_in_unit_sphere().unit_vector()
    }

    pub fn get_vec3_in_hemisphere(&mut self, normal: Vec3) -> Vec3 {
        let on_unit_sphere = self.get_vec3_unit_vector();

        if on_unit_sphere.dot(normal) < 0.0 {
            -on_unit_sphere
        } else {
            on_unit_sphere
        }
    }
}
