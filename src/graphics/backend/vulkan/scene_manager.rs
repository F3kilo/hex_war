use crate::graphics::error::UnavailableError;
use crate::graphics::manager::manage_scenes::{
    AdditionError, ManageScenes, RenderContext, SceneId, SceneItem,
};
use crate::graphics::manager::manage_textures::TextureId;
use slog::Logger;

pub struct VkSceneManager {
    logger: Logger,
}

impl VkSceneManager {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }
}

impl ManageScenes for VkSceneManager {
    fn create_scene(&mut self) -> SceneId {
        todo!()
    }

    fn drop_scene(&mut self, id: SceneId) -> bool {
        todo!()
    }

    fn add_item(&mut self, id: SceneId, item: SceneItem) -> Result<(), AdditionError> {
        todo!()
    }

    fn clear(&mut self, id: SceneId) -> Result<(), UnavailableError> {
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
