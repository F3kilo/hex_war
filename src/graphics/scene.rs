use crate::graphics::geometry::Geometry;
use crate::graphics::manager::manage_scenes;
use crate::graphics::manager::manage_scenes::{Instance, RenderContext, SceneId};
use crate::graphics::proxy::scene_manager::SceneManager;
use crate::graphics::proxy::texture_manager::TextureManager;
use crate::graphics::texture::Texture;
use std::collections::HashSet;

pub struct Scene {
    id: SceneId,
    scene_manager: SceneManager,
    texture_manager: TextureManager,
    used_resources: HashSet<Resource>,
}

impl Scene {
    pub fn new(mut scene_manager: SceneManager, texture_manager: TextureManager) -> Self {
        let id = scene_manager.create_scene();
        let used_resources = HashSet::new();
        Self {
            id,
            scene_manager,
            texture_manager,
            used_resources,
        }
    }

    pub fn add_textured_geometry(&mut self, instance: TexturedGeometry) {
        self.used_resources.insert(instance.texture.clone().into());
        self.used_resources.insert(instance.geometry.clone().into());
        let result = self
            .scene_manager
            .add_textured_geometry(self.id, instance.into());
        if let Err(e) = result {
            panic!(
                "Unexpected error, when adding textured geometry to scene: {:?}",
                e
            )
        }
    }

    pub fn render(&mut self, context: &RenderContext) -> Texture {
        let tex_id = self.scene_manager.render(self.id, context);
        Texture::from_raw(tex_id, self.texture_manager.clone())
            .expect("Render returns invalid texture id.") // TODO: error processing
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        self.scene_manager.drop_scene(self.id);
    }
}

#[derive(Debug, Clone)]
pub struct TexturedGeometry {
    pub geometry: Geometry,
    pub texture: Texture,
    pub instance: Instance,
}

impl From<TexturedGeometry> for manage_scenes::TexturedGeometry {
    fn from(instance: TexturedGeometry) -> Self {
        manage_scenes::TexturedGeometry {
            texture: instance.texture.get_id(),
            geometry: instance.geometry.get_id(),
            instance: instance.instance,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum Resource {
    Texture(Texture),
    Geometry(Geometry),
}

impl From<Texture> for Resource {
    fn from(tex: Texture) -> Self {
        Resource::Texture(tex)
    }
}

impl From<Geometry> for Resource {
    fn from(geom: Geometry) -> Self {
        Resource::Geometry(geom)
    }
}
