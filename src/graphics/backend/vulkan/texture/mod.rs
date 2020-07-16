pub mod loader;

use crate::graphics::backend::vulkan::texture::loader::{
    VkTextureData, VkTextureLoader, VkTextureLoadingData, VkTextureLoadingProgress,
};
use crate::math::screen_coords::ScreenCoords;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
enum VkTextureState {
    Ready(VkTextureData),
    Loading(VkTextureLoadingData),
}

pub struct VkTexture {
    state: RefCell<VkTextureState>,
    path: PathBuf,
}

impl VkTexture {
    pub fn load(path: PathBuf, loader: &mut VkTextureLoader) -> Self {
        let state = RefCell::new(VkTextureState::Loading(loader.load(path.clone())));
        Self { state, path }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_size(&self) -> ScreenCoords {
        self.get_texture_data().size
    }

    fn get_texture_data(&self) -> VkTextureData {
        self.try_update_state();
        let state_ref = RefCell::borrow(&self.state);
        let state: &VkTextureState = Ref::borrow(&state_ref);
        match state {
            VkTextureState::Ready(data) => *data,
            VkTextureState::Loading(data) => data.stab,
        }
    }

    fn try_update_state(&self) {
        let new_state = self.get_new_state();
        match new_state {
            None => {}
            Some(s) => {
                self.state.replace(s);
            }
        };
    }

    fn get_new_state(&self) -> Option<VkTextureState> {
        let state_ref = RefCell::borrow(&self.state);
        let state: &VkTextureState = Ref::borrow(&state_ref);
        match state {
            VkTextureState::Ready(_) => None,
            VkTextureState::Loading(load_data) => match load_data.progress.check_progress() {
                VkTextureLoadingProgress::Loading => None,
                VkTextureLoadingProgress::Ready(tex_data) => Some(VkTextureState::Ready(tex_data)),
            },
        }
    }
}

impl fmt::Debug for VkTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VkTexture: path: {:?}, state: {:?}",
            self.path,
            self.state.borrow(),
        )
    }
}
