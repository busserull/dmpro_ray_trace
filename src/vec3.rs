use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn origin() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3(
            -self.2 * rhs.1 + self.1 * rhs.2,
            self.2 * rhs.0 - self.0 * rhs.2,
            -self.1 * rhs.0 + self.0 * rhs.1,
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Self, refraction_ratio: f32) -> Self {
        let v_in = *self;
        let cos_theta = (-v_in).dot(normal).min(1.0);

        let out_perpendicular = refraction_ratio * (v_in + cos_theta * normal);
        let out_parallel = -(1.0 - out_perpendicular.len_squared()).abs().sqrt() * normal;

        out_perpendicular + out_parallel
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}
