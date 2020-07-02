use crate::graphics::camera::Camera;
use crate::graphics::low_level::{RenderContext, SceneTransforms};
use crate::graphics::scene::Scene;
use crate::graphics::texture::UniqueTexture;
use crate::graphics::Graphics;

pub fn render(graphics: &mut Graphics, camera: &impl Camera, scene: &Scene) -> UniqueTexture {
    let render_data = scene.get_render_data();
    let render_context = RenderContext {
        scene_transforms: SceneTransforms {
            world: Default::default(),
            view: camera.get_view_transform(),
            proj: camera.get_proj_transform(),
        },
    };
    let result_image = graphics.render(render_context, render_data);
    UniqueTexture::from_raw(result_image, graphics.get_texture_manager().clone())
        .expect("Just rendered texture is not found")
}
