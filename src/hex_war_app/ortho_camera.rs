use crate::graphics::camera::{Camera, Depth};
use crate::math::screen_coords::{Int, ScreenCoords};
use crate::math::world_coords::WorldCoords;
use glam::{Mat4, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct OrthographicCamera {
    center: WorldCoords,
    size: WorldCoords,
}

impl OrthographicCamera {
    pub fn new(center: WorldCoords, size: WorldCoords) -> Self {
        Self { center, size }
    }
}

impl Camera for OrthographicCamera {
    fn to_world(
        &self,
        point: ScreenCoords,
        depth: Depth,
        screen_size: ScreenCoords,
    ) -> WorldCoords {
        let x = 2.0 * point.x() as f32 / screen_size.x() as f32 - 1.0;
        let y = 2.0 * point.y() as f32 / screen_size.y() as f32 - 1.0;
        let screen: Vec4 = (x, y, depth, 1f32).into();
        let inverse_proj = self.get_proj_transform().inverse();
        inverse_proj.mul_vec4(screen).truncate().into()
    }

    fn to_screen(&self, world: WorldCoords, screen_size: ScreenCoords) -> (ScreenCoords, Depth) {
        let screen_space = self
            .get_proj_transform()
            .mul_vec4(world.get_inner().extend(1.0));

        println!("Screen space: {:?}", screen_space);

        let x = (screen_space.x() + 1.0) / 2.0 * screen_size.x() as f32;
        let y = (screen_space.y() + 1.0) / 2.0 * screen_size.y() as f32;

        (
            (x.round() as Int, y.round() as Int).into(),
            screen_space.z(),
        )
    }

    fn get_view_transform(&self) -> Mat4 {
        Mat4::default()
    }

    fn get_proj_transform(&self) -> Mat4 {
        let top_left_near = self.center - (self.size / 2.0);
        let bot_right_far = self.center + (self.size / 2.0);
        Mat4::orthographic_lh(
            top_left_near.x(),
            bot_right_far.x(),
            bot_right_far.y(),
            top_left_near.y(),
            top_left_near.z(),
            bot_right_far.z(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::camera::Camera;
    use crate::hex_war_app::ortho_camera::OrthographicCamera;

    #[test]
    fn to_world() {
        let cam_center = (0f32, 0f32, 0f32).into();
        let cam_size = (100f32, 100f32, 100f32).into();
        let camera = OrthographicCamera::new(cam_center, cam_size);
        let screen_size = (800, 600).into();
        let screen_point = (0, 0).into();
        let depth = 0.0;
        let world = camera.to_world(screen_point, depth, screen_size);
        let eps = 0.001;
        assert!(world.abs_diff_eq((-50.0, 50.0, -50.0).into(), eps))
    }

    #[test]
    fn to_screen() {
        let cam_center = (0f32, 0f32, 0f32).into();
        let cam_size = (100f32, 100f32, 100f32).into();
        let camera = OrthographicCamera::new(cam_center, cam_size);
        let screen_size = (800, 600).into();
        let (screen, depth) = camera.to_screen((0.0, 0.0, 50.0).into(), screen_size);
        println!("Screen: {:?}", screen);
        let screen_point = (400, 300).into();
        assert_eq!(screen, screen_point);
        let eps = 0.001;
        assert!(depth - 1.0 < eps);
    }
}
