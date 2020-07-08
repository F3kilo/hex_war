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

    pub fn add_item(&mut self, item: SceneItem) {
        self.used_resources.extend(item.get_resources());
        let result = self.scene_manager.add_item(self.id, item.into());
        if let Err(e) = result {
            panic!(
                "Unexpected error, when adding textured geometry to scene: {:?}",
                e
            ) // TODO: error processing
        }
    }

    pub fn clear(&mut self) {
        self.scene_manager
            .clear(self.id)
            .expect("Scene was unexpectedly not found."); // TODO: error processing
        self.used_resources.clear();
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
pub enum Resource {
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

#[derive(Debug, Clone)]
pub enum SceneItem {
    TexturedGeometry(TexturedGeometry),
}

impl SceneItem {
    pub fn get_resources(&self) -> impl Iterator<Item = Resource> + '_ {
        match self {
            SceneItem::TexturedGeometry(tg) => TexturedGeometryResourcesIterator::new(tg),
        }
    }
}

impl From<SceneItem> for manage_scenes::SceneItem {
    fn from(item: SceneItem) -> Self {
        match item {
            SceneItem::TexturedGeometry(tg) => {
                manage_scenes::SceneItem::TexturedGeometry(tg.into())
            }
        }
    }
}

struct TexturedGeometryResourcesIterator<'a> {
    index: usize,
    tg: &'a TexturedGeometry,
}

impl<'a> TexturedGeometryResourcesIterator<'a> {
    pub fn new(textured_geometry: &'a TexturedGeometry) -> Self {
        Self {
            index: 0,
            tg: textured_geometry,
        }
    }
}

impl<'a> Iterator for TexturedGeometryResourcesIterator<'a> {
    type Item = Resource;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.index {
            0 => Some(Resource::Geometry(self.tg.geometry.clone())),
            1 => Some(Resource::Texture(self.tg.texture.clone())),
            _ => None,
        }
    }
}
