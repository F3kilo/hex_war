use crate::math::screen_coords::ScreenCoords;
use glam::Vec2;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::fmt;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, TryRecvError};

#[derive(Debug)]
pub struct VkTextureLoader {
    stab: VkTextureData,
}

impl VkTextureLoader {
    pub fn new() -> Self {
        todo!()
    }

    pub fn load(&mut self, path: PathBuf) -> VkTextureLoadingData {
        let (tx, rx) = mpsc::channel();
        let progress = VkTextureLoadingProgressProvider::new(rx);
        let stab = self.get_stab();
        VkTextureLoadingData { stab, progress }
    }

    fn get_stab(&mut self) -> VkTextureData {
        self.stab
    }
}

#[derive(Debug)]
enum VkTextureState {
    Ready(VkTextureData),
    Loading(VkTextureLoadingData),
}

#[derive(Debug, Copy, Clone)]
struct VkTextureLocation {
    uv_offset: Vec2,
    uv_size: Vec2,
}

#[derive(Debug, Copy, Clone)]
struct VkTextureData {
    pub location: VkTextureLocation,
    pub size: ScreenCoords,
}

#[derive(Debug)]
enum VkTextureLoadingProgress {
    Loading,
    Ready(VkTextureData),
}

#[derive(Debug)]
struct VkTextureLoadingProgressProvider {
    source: Receiver<VkTextureData>,
}

impl VkTextureLoadingProgressProvider {
    pub fn new(source: Receiver<VkTextureData>) -> Self {
        Self { source }
    }

    pub fn check_progress(&self) -> VkTextureLoadingProgress {
        let recv = self.source.try_recv();
        match recv {
            Ok(data) => VkTextureLoadingProgress::Ready(data),
            Err(e) => match e {
                TryRecvError::Empty => VkTextureLoadingProgress::Loading,
                TryRecvError::Disconnected => todo!("Error processing"),
            },
        }
    }
}

#[derive(Debug)]
pub struct VkTextureLoadingData {
    stab: VkTextureData,
    progress: VkTextureLoadingProgressProvider,
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
