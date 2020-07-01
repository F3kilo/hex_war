use crate::graphics::error::LoadError;
use crate::graphics::low_level::ProvideTextureManager;
use crate::graphics::manager::texture_manager::TextureId;
use crate::math::screen_coords::ScreenCoords;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

pub struct UniqueTexture<M: ProvideTextureManager> {
    id: TextureId,
    manager_provider: M,
}

impl<M: ProvideTextureManager> UniqueTexture<M> {
    pub fn new(path: PathBuf, mut manager_provider: M) -> Result<Self, LoadError> {
        let id = manager_provider
            .get_mut_texture_manager()
            .create_texture(path)?;
        Ok(Self {
            manager_provider,
            id,
        })
    }

    pub fn from_raw(id: TextureId, manager_provider: M) -> Self {
        if !manager_provider.get_texture_manager().contains(id) {
            panic!("Can't create Texture. Manager doesn't contains specified id.")
        }
        Self {
            id,
            manager_provider,
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.manager_provider
            .get_texture_manager()
            .get_path(self.id)
            .expect("Texture id is not found, unexpectedly.")
    }

    pub fn get_size(&self) -> ScreenCoords {
        self.manager_provider
            .get_texture_manager()
            .get_size(self.id)
            .expect("Texture id is not found, unexpectedly.")
    }
}

impl<M: ProvideTextureManager> Drop for UniqueTexture<M> {
    fn drop(&mut self) {
        self.manager_provider
            .get_mut_texture_manager()
            .drop_texture(self.id);
    }
}

impl<M: ProvideTextureManager> fmt::Debug for UniqueTexture<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture #{:?}", self.id)
    }
}

pub type Texture<M> = Rc<UniqueTexture<M>>;
