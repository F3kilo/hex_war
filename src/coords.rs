use glam::Vec2;

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
