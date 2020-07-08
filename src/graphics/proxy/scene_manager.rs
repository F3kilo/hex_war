use crate::graphics::error::LoadError;
use crate::graphics::low_level::GraphicsBackend;
use crate::graphics::manager::manage_scenes::{AdditionError, SceneId, TexturedGeometry};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SceneManager {
    backend: Rc<RefCell<Box<dyn GraphicsBackend>>>,
}

impl SceneManager {
    pub fn new(backend: Rc<RefCell<Box<dyn GraphicsBackend>>>) -> Self {
        Self { backend }
    }

    fn create_scene(&mut self) -> Result<SceneId, LoadError> {
        let mut back = RefCell::borrow_mut(&self.backend);
        back.get_mut_scene_manager().create_scene()
    }

    fn drop_scene(&mut self, id: SceneId) -> bool {
        let mut back = RefCell::borrow_mut(&self.backend);
        back.get_mut_scene_manager().drop_scene()
    }

    fn add_textured_geometry(
        &mut self,
        id: SceneId,
        instance: TexturedGeometry,
    ) -> Result<(), AdditionError> {
    }

    fn render(&mut self, id: SceneId, context: &RenderContext) -> TextureId;

    fn contains(&self, id: SceneId) -> bool;
    fn ids(&self) -> Vec<SceneId>;
}
