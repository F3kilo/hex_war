use crate::graphics::backend::vulkan::texture::loader::VkTextureLoader;
use crate::graphics::backend::vulkan::texture::VkTexture;
use crate::graphics::error::{LoadError, UnavailableError};
use crate::graphics::manager::manage_textures::{ManageTextures, TextureId};
use crate::math::screen_coords::ScreenCoords;
use slog::Logger;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static TEXTURE_IDS: AtomicU64 = AtomicU64::new(0);

struct TextureIdGenerator;

impl TextureIdGenerator {
    pub fn next() -> TextureId {
        TEXTURE_IDS.fetch_add(1, Ordering::Relaxed).into()
    }
}

#[derive(Debug)]
pub struct VkTextureManager {
    logger: Logger,
    textures: HashMap<TextureId, VkTexture>,
    loader: VkTextureLoader,
}

impl VkTextureManager {
    pub fn new(logger: Logger, loader: VkTextureLoader) -> Self {
        let textures = HashMap::new();
        Self {
            logger,
            textures,
            loader,
        }
    }
}

impl ManageTextures for VkTextureManager {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        let id = TextureIdGenerator::next();
        let texture = VkTexture::load(path, &mut self.loader);
        self.textures.insert(id, texture);
        Ok(id)
    }

    fn drop_texture(&mut self, id: TextureId) -> bool {
        self.textures.remove(&id).is_some()
    }

    fn get_path(&self, id: TextureId) -> Result<PathBuf, UnavailableError> {
        match self.textures.get(&id) {
            None => Err(UnavailableError::NotFound),
            Some(t) => Ok(t.get_path()),
        }
    }

    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, UnavailableError> {
        match self.textures.get(&id) {
            None => Err(UnavailableError::NotFound),
            Some(t) => Ok(t.get_size()?),
        }
    }

    fn contains(&self, id: TextureId) -> bool {
        self.textures.contains_key(&id)
    }

    fn ids(&self) -> Vec<TextureId> {
        self.textures.keys().cloned().collect()
    }
}
