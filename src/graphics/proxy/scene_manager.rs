use crate::graphics::error::NotFoundError;
use crate::graphics::low_level::GraphicsBackend;
use crate::graphics::manager::manage_scenes::{AdditionError, RenderContext, SceneId, SceneItem};
use crate::graphics::manager::manage_textures::TextureId;
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

    pub fn create_scene(&mut self) -> SceneId {
        let mut back = RefCell::borrow_mut(&self.backend);
        back.get_mut_scene_manager().create_scene()
    }

    pub fn drop_scene(&mut self, id: SceneId) -> bool {
        let mut back = RefCell::borrow_mut(&self.backend);
        back.get_mut_scene_manager().drop_scene(id)
    }

    pub fn add_item(&mut self, id: SceneId, item: SceneItem) -> Result<(), AdditionError> {
        let mut back = RefCell::borrow_mut(&self.backend);
        back.get_mut_scene_manager().add_item(id, item)
    }

    pub fn clear(&mut self, id: SceneId) -> Result<(), NotFoundError> {
        let mut back = RefCell::borrow_mut(&self.backend);
        back.get_mut_scene_manager().clear(id)
    }

    pub fn render(&mut self, id: SceneId, context: &RenderContext) -> TextureId {
        let mut back = RefCell::borrow_mut(&self.backend);
        back.get_mut_scene_manager().render(id, context)
    }

    pub fn contains(&self, id: SceneId) -> bool {
        let back = RefCell::borrow(&self.backend);
        back.get_scene_manager().contains(id)
    }

    pub fn ids(&self) -> Vec<SceneId> {
        let back = RefCell::borrow(&self.backend);
        back.get_scene_manager().ids()
    }
}
