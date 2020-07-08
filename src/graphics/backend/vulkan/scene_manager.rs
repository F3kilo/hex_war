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
        unimplemented!()
    }

    fn drop_scene(&mut self, id: SceneId) -> bool {
        unimplemented!()
    }

    fn add_textured_geometry(
        &mut self,
        id: SceneId,
        instance: TexturedGeometry,
    ) -> Result<(), AdditionError> {
        unimplemented!()
    }

    fn render(&mut self, id: SceneId, context: &RenderContext) -> TextureId {
        unimplemented!()
    }

    fn contains(&self, id: SceneId) -> bool {
        unimplemented!()
    }

    fn ids(&self) -> Vec<SceneId> {
        unimplemented!()
    }
}
