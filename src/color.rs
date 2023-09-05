use crate::interval::Interval;
use crate::vec3::Vec3;
use std::fmt;

pub type Color = Vec3;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.x();
        let g = self.y();
        let b = self.z();

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
