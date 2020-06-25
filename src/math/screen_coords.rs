use glam::{Vec2, Vec2Mask};
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub type Int = i64;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ScreenCoords(Int, Int);

impl ScreenCoords {
    /// Returns a new `Vec4` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive
    /// - `-1.0` if the number is negative
    #[inline]
    pub fn sign(self) -> Self {
        let x = if self.0 > 0 { 1 } else { -1 };
        let y = if self.1 > 0 { 1 } else { -1 };
        Self(x, y)
    }

    /// Creates a new `IVec2`.
    #[inline]
    pub fn new(x: Int, y: Int) -> ScreenCoords {
        ScreenCoords(x, y)
    }

    /// Creates a new `IVec2` with all elements set to `0`.
    #[inline]
    pub fn zero() -> ScreenCoords {
        ScreenCoords(0, 0)
    }

    /// Creates a new `IVec2` with all elements set to `1`.
    #[inline]
    pub fn one() -> ScreenCoords {
        ScreenCoords(1, 1)
    }

    /// Creates a new `IVec2` with values `[x: 1, y: 0]`.
    #[inline]
    pub fn unit_x() -> ScreenCoords {
        ScreenCoords(1, 0)
    }

    /// Creates a new `IVec2` with values `[x: 0, y: 1]`.
    #[inline]
    pub fn unit_y() -> ScreenCoords {
        ScreenCoords(0, 1)
    }

    /// Creates a new `IVec2` with all elements set to `v`.
    #[inline]
    pub fn splat(v: Int) -> ScreenCoords {
        ScreenCoords(v, v)
    }

    /// Returns element `x`.
    #[inline]
    pub fn x(self) -> Int {
        self.0
    }

    /// Returns element `y`.
    #[inline]
    pub fn y(self) -> Int {
        self.1
    }

    /// Returns a mutable reference to element `x`.
    #[inline]
    pub fn x_mut(&mut self) -> &mut Int {
        &mut self.0
    }

    /// Returns a mutable reference to element `y`.
    #[inline]
    pub fn y_mut(&mut self) -> &mut Int {
        &mut self.1
    }

    /// Sets element `x`.
    #[inline]
    pub fn set_x(&mut self, x: Int) {
        self.0 = x;
    }

    /// Sets element `y`.
    #[inline]
    pub fn set_y(&mut self, y: Int) {
        self.1 = y;
    }

    /// Computes the dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: ScreenCoords) -> Int {
        (self.0 * other.0) + (self.1 * other.1)
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        (self.dot(self) as f32).sqrt()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `IVec2::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> Int {
        self.dot(self)
    }

    /// Computes `1.0 / IVec2::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_reciprocal(self) -> f32 {
        1.0 / self.length()
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: ScreenCoords) -> ScreenCoords {
        ScreenCoords(self.0.min(other.0), self.1.min(other.1))
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: ScreenCoords) -> ScreenCoords {
        ScreenCoords(self.0.max(other.0), self.1.max(other.1))
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y)`.
    #[inline]
    pub fn min_element(self) -> Int {
        self.0.min(self.1)
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y)`.
    #[inline]
    pub fn max_element(self) -> Int {
        self.0.max(self.1)
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `IVec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
    #[inline]
    pub fn cmpeq(self, other: ScreenCoords) -> Vec2Mask {
        Vec2Mask::new(self.0.eq(&other.0), self.1.eq(&other.1))
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `IVec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
    #[inline]
    pub fn cmpne(self, other: ScreenCoords) -> Vec2Mask {
        Vec2Mask::new(self.0.ne(&other.0), self.1.ne(&other.1))
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `IVec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
    #[inline]
    pub fn cmpge(self, other: ScreenCoords) -> Vec2Mask {
        Vec2Mask::new(self.0.ge(&other.0), self.1.ge(&other.1))
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `IVec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
    #[inline]
    pub fn cmpgt(self, other: ScreenCoords) -> Vec2Mask {
        Vec2Mask::new(self.0.gt(&other.0), self.1.gt(&other.1))
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `IVec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
    #[inline]
    pub fn cmple(self, other: ScreenCoords) -> Vec2Mask {
        Vec2Mask::new(self.0.le(&other.0), self.1.le(&other.1))
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `IVec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
    #[inline]
    pub fn cmplt(self, other: ScreenCoords) -> Vec2Mask {
        Vec2Mask::new(self.0.lt(&other.0), self.1.lt(&other.1))
    }

    /// Returns a new `IVec2` containing the absolute value of each element of the original
    /// `IVec2`.
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(self, other: ScreenCoords) -> Int {
        (self.0 * other.1) - (self.1 * other.0)
    }

    /// Returns the angle between two vectors, in radians.
    ///
    /// The vectors do not need to be unit length, but this function does
    /// perform a `sqrt`.
    #[inline]
    pub fn angle_between(self, other: Self) -> f32 {
        let s = Vec2::new(self.0 as f32, self.1 as f32);
        let o = Vec2::new(other.0 as f32, other.1 as f32);
        s.angle_between(o)
    }
}

impl fmt::Display for ScreenCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

impl Div<ScreenCoords> for ScreenCoords {
    type Output = Self;
    #[inline]
    fn div(self, other: ScreenCoords) -> Self {
        Self(self.0 / other.0, self.1 / other.1)
    }
}

impl DivAssign<ScreenCoords> for ScreenCoords {
    #[inline]
    fn div_assign(&mut self, other: ScreenCoords) {
        self.0 /= other.0;
        self.1 /= other.1;
    }
}

impl Div<Int> for ScreenCoords {
    type Output = Self;
    #[inline]
    fn div(self, other: Int) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}

impl DivAssign<Int> for ScreenCoords {
    #[inline]
    fn div_assign(&mut self, other: Int) {
        self.0 /= other;
        self.1 /= other;
    }
}

impl Mul<ScreenCoords> for ScreenCoords {
    type Output = Self;
    #[inline]
    fn mul(self, other: ScreenCoords) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl MulAssign<ScreenCoords> for ScreenCoords {
    #[inline]
    fn mul_assign(&mut self, other: ScreenCoords) {
        self.0 *= other.0;
        self.1 *= other.1;
    }
}

impl Mul<Int> for ScreenCoords {
    type Output = Self;
    #[inline]
    fn mul(self, other: Int) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl MulAssign<Int> for ScreenCoords {
    #[inline]
    fn mul_assign(&mut self, other: Int) {
        self.0 *= other;
        self.1 *= other;
    }
}

impl Mul<ScreenCoords> for Int {
    type Output = ScreenCoords;
    #[inline]
    fn mul(self, other: ScreenCoords) -> ScreenCoords {
        ScreenCoords(self * other.0, self * other.1)
    }
}

impl Add for ScreenCoords {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for ScreenCoords {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Sub for ScreenCoords {
    type Output = Self;
    #[inline]
    fn sub(self, other: ScreenCoords) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for ScreenCoords {
    #[inline]
    fn sub_assign(&mut self, other: ScreenCoords) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl Neg for ScreenCoords {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl AsRef<[Int; 2]> for ScreenCoords {
    #[inline]
    fn as_ref(&self) -> &[Int; 2] {
        unsafe { &*(self as *const ScreenCoords as *const [Int; 2]) }
    }
}

impl AsMut<[Int; 2]> for ScreenCoords {
    #[inline]
    fn as_mut(&mut self) -> &mut [Int; 2] {
        unsafe { &mut *(self as *mut ScreenCoords as *mut [Int; 2]) }
    }
}

impl Index<usize> for ScreenCoords {
    type Output = Int;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for ScreenCoords {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl From<(Int, Int)> for ScreenCoords {
    #[inline]
    fn from(t: (Int, Int)) -> Self {
        Self(t.0, t.1)
    }
}

impl From<ScreenCoords> for (Int, Int) {
    #[inline]
    fn from(v: ScreenCoords) -> Self {
        (v.0, v.1)
    }
}

impl From<[Int; 2]> for ScreenCoords {
    #[inline]
    fn from(a: [Int; 2]) -> Self {
        Self(a[0], a[1])
    }
}

impl From<ScreenCoords> for [Int; 2] {
    #[inline]
    fn from(v: ScreenCoords) -> Self {
        [v.0, v.1]
    }
}
