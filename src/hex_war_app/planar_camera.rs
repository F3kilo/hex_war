use crate::graphics::Camera;
use crate::math::screen_coords::ScreenCoords;
use crate::math::world_coords::WorldCoords;
use glam::Mat4;

pub struct OrthographicCamera {
    center: WorldCoords,
    size: WorldCoords,
}

impl OrthographicCamera {
    pub fn new(center: WorldCoords, size: WorldCoords) -> Self {
        Self { center, size }
    }

    pub fn screen_origin(&self) -> WorldCoords {}
}

impl Camera for OrthographicCamera {
    fn to_world(&self, point: ScreenCoords, depth: f32, screen_size: ScreenCoords) -> WorldCoords {
        let x = point.x() as f32 / screen_size.x() as f32;
        let y = point.y() as f32 / screen_size.y() as f32;
        let screen = (x, y, depth, 1f32).into();
        self.get_proj_transform().mul_vec4(screen).truncate().into()
    }

    fn to_screen(&self, world: WorldCoords) -> (ScreenCoords, f32) {
        unimplemented!()
    }

    fn size_to_world(&self, screen: ScreenCoords, depth: f32) -> WorldCoords {
        unimplemented!()
    }

    fn size_to_screen(&self, world: WorldCoords) -> (ScreenCoords, f32) {
        unimplemented!()
    }

    fn get_view_transform(&self) -> Mat4 {
        unimplemented!()
    }

    fn get_proj_transform(&self) -> Mat4 {
        unimplemented!()
    }
}
