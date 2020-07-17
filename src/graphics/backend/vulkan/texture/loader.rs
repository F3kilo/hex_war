use crate::graphics::backend::vulkan::async_tasker::{AsyncTask, TaskSender};
use crate::math::screen_coords::ScreenCoords;
use glam::Vec2;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

#[derive(Debug, Copy, Clone)]
pub struct VkTextureLocation {
    uv_offset: Vec2,
    uv_size: Vec2,
}

#[derive(Debug, Copy, Clone)]
pub struct VkTextureData {
    pub location: VkTextureLocation,
    pub size: ScreenCoords,
}

#[derive(Debug)]
pub enum VkTextureLoadingProgress {
    Loading,
    Ready(VkTextureData),
}

#[derive(Debug)]
pub struct VkTextureLoadingProgressProvider {
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
    pub stab: VkTextureData,
    pub progress: VkTextureLoadingProgressProvider,
}

#[derive(Debug)]
pub struct VkTextureLoader {
    stab: VkTextureData,
    task_sender: TaskSender,
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
struct LoadTextureTask {
    loaded_sender: Sender<VkTextureData>,
}

impl LoadTextureTask {
    pub fn new(loaded_sender: Sender<VkTextureData>) -> Self {
        Self { loaded_sender }
    }

    pub fn load_texture(&mut self) -> VkTextureData {
        todo!()
    }
}

impl AsyncTask for LoadTextureTask {
    fn perform(&mut self) {
        let data = self.load_texture();
        self.loaded_sender.send(data).unwrap();
    }
}
