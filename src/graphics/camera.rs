use crate::math::screen_coords::ScreenCoords;
use crate::math::world_coords::WorldCoords;
use glam::Mat4;

pub type Depth = f32;

pub trait Camera {
    fn to_world(&self, point: ScreenCoords, depth: Depth, screen_size: ScreenCoords)
        -> WorldCoords;
    fn to_screen(&self, world: WorldCoords, screen_size: ScreenCoords) -> (ScreenCoords, Depth);

    fn get_view_transform(&self) -> Mat4;
    fn get_proj_transform(&self) -> Mat4;
}
