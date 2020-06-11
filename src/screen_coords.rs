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
