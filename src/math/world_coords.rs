use glam::Vec3;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, PartialOrd, PartialEq, Copy, Clone)]
pub struct WorldCoords(Vec3);

impl WorldCoords {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub fn zero() -> Self {
        Self(Vec3::zero())
    }

    pub fn get_inner(&self) -> Vec3 {
        self.0
    }

    pub fn x(&self) -> f32 {
        self.0.x()
    }

    pub fn y(&self) -> f32 {
        self.0.y()
    }

    pub fn z(&self) -> f32 {
        self.0.z()
    }

    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        self.0.abs_diff_eq(other.0, max_abs_diff)
    }
}

impl From<(f32, f32, f32)> for WorldCoords {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self::new(x, y, z)
    }
}

impl Div<WorldCoords> for WorldCoords {
    type Output = Self;
    #[inline]
    fn div(self, other: WorldCoords) -> Self {
        Self(self.0 / other.0)
    }
}

impl DivAssign<WorldCoords> for WorldCoords {
    #[inline]
    fn div_assign(&mut self, other: WorldCoords) {
        self.0 /= other.0
    }
}

impl Div<f32> for WorldCoords {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self(self.0 / other)
    }
}

impl DivAssign<f32> for WorldCoords {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.0 /= other;
    }
}

impl Mul<WorldCoords> for WorldCoords {
    type Output = Self;
    #[inline]
    fn mul(self, other: WorldCoords) -> Self {
        Self(self.0 * other.0)
    }
}

impl MulAssign<WorldCoords> for WorldCoords {
    #[inline]
    fn mul_assign(&mut self, other: WorldCoords) {
        self.0 *= other.0;
    }
}

impl Mul<f32> for WorldCoords {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self(self.0 * other)
    }
}

impl MulAssign<f32> for WorldCoords {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.0 *= other;
    }
}

impl Mul<WorldCoords> for f32 {
    type Output = WorldCoords;
    #[inline]
    fn mul(self, other: WorldCoords) -> WorldCoords {
        WorldCoords(self * other.0)
    }
}

impl Add for WorldCoords {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for WorldCoords {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for WorldCoords {
    type Output = Self;
    #[inline]
    fn sub(self, other: WorldCoords) -> Self {
        Self(self.0 - other.0)
    }
}

impl SubAssign for WorldCoords {
    #[inline]
    fn sub_assign(&mut self, other: WorldCoords) {
        self.0 -= other.0;
    }
}

impl Neg for WorldCoords {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl From<Vec3> for WorldCoords {
    fn from(v: Vec3) -> Self {
        WorldCoords::new(v.x(), v.y(), v.z())
    }
}
