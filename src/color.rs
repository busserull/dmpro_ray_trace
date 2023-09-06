use crate::interval::Interval;
use crate::vec3::Vec3;
use std::fmt;

pub type Color = Vec3;

fn linear_to_gamma(lin: f32) -> f32 {
    lin.sqrt()
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = linear_to_gamma(self.x());
        let g = linear_to_gamma(self.y());
        let b = linear_to_gamma(self.z());

        let intensity = Interval::new(0.0, 0.999);

        write!(
            f,
            "{} {} {}",
            (256.0 * intensity.clamp(r)) as u32,
            (256.0 * intensity.clamp(g)) as u32,
            (256.0 * intensity.clamp(b)) as u32
        )
    }
}
