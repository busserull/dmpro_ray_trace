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
}
