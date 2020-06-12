use crate::coords::WorldCoords;
use glam::f32::Vec4;

/// Four-component color. Last element represent transparency.
pub type ColorRgba = Vec4;

trait Sprite2D {
    /// Draws sprite on screen.
    fn draw(&self);

    /// Returns top left world position of sprite.
    fn get_position(&self) -> WorldCoords;
    /// Moves spite top left corner world coordinates to  `pos`.
    fn move_to(&mut self, pos: WorldCoords);
    /// Moves spite top left corner world coordinates by `shift`.
    fn move_by(&mut self, shift: WorldCoords);

    /// Returns current rotation of sprite in range: ```[0, 2*Pi]```.
    fn get_rotation(&self) -> f32;
    /// Rotates sprite to `angle`.
    fn rotate_to(&mut self, angle: f32);
    /// Rotates sprite by `angle`.
    fn rotate_by(&mut self, angle_shift: f32);

    /// Returns current size of sprite in world coordinates.
    fn get_size(&self) -> WorldCoords;
    /// Sets size of sprite in world coordinates.
    fn set_size(&mut self, size: WorldCoords);

    /// Returns color of sprite. Sprite texture color multiplies by it.
    fn get_color(&self) -> ColorRgba;
    /// Sets color of sprite. Sprite texture color multiplies by it.
    fn set_color(&mut self, rgba: ColorRgba);
}
