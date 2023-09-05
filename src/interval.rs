#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: f32) -> bool {
        self.min <= value && self.max >= value
    }

    pub fn surrounds(&self, value: f32) -> bool {
        self.min < value && self.max > value
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }
}
