use crate::vec3::Vec3;
use std::fmt;

pub type Color = Vec3;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = (255.999 * self.0) as i32;
        let g = (255.999 * self.1) as i32;
        let b = (255.999 * self.2) as i32;

        write!(f, "{} {} {}", r, g, b)
    }
}
