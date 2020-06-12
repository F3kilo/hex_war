use glam::Vec2;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, PartialOrd, PartialEq, Copy, Clone)]
pub struct ScreenCoords {
    x: u32,
    y: u32,
}

impl ScreenCoords {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

impl From<(u32, u32)> for ScreenCoords {
    fn from((x, y): (u32, u32)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Debug, Default, PartialOrd, PartialEq, Copy, Clone)]
pub struct WorldCoords(Vec2);

impl WorldCoords {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn zero() -> Self {
        Self(Vec2::zero())
    }
}

impl From<(f32, f32)> for WorldCoords {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
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
