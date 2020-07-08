use crate::graphics::error::NotFoundError;
use crate::graphics::manager::manage_scenes::{
    AdditionError, ManageScenes, RenderContext, SceneId, TexturedGeometry,
};
use crate::graphics::manager::manage_textures::TextureId;

pub struct VkSceneManager {}

impl VkSceneManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl ManageScenes for VkSceneManager {
    fn create_scene(&mut self) -> SceneId {
        todo!()
    }

    fn drop_scene(&mut self, id: SceneId) -> bool {
        todo!()
    }

    fn add_textured_geometry(
        &mut self,
        id: SceneId,
        instance: TexturedGeometry,
    ) -> Result<(), AdditionError> {
        todo!()
    }

    fn clear(&mut self, id: SceneId) -> Result<(), NotFoundError> {
        todo!()
    }

    fn render(&mut self, id: SceneId, context: &RenderContext) -> TextureId {
        todo!()
    }

    fn contains(&self, id: SceneId) -> bool {
        todo!()
    }

    fn ids(&self) -> Vec<SceneId> {
        todo!()
    }
}
