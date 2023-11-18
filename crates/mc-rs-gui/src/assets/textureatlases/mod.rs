use bevy::{
    asset::RecursiveDependencyLoadState, prelude::*, render::render_resource::TextureFormat,
    utils::HashMap,
};
use mc_rs_core::ResourceLocation;
use mc_rs_resourcepack::ResourcePackAsset;
use strum::{Display, EnumIter, IntoEnumIterator};

pub mod atlases;
use atlases::*;

mod traits;
pub use traits::AtlasFromWorld;

use super::resourcepacks::ResourcePacks;

pub(super) fn setup(app: &mut App) { app.init_resource::<TextureAtlases>(); }

/// A collection of texture atlases
///
/// Created after loading all of the [`ResourcePack`](crate::assets::resourcepacks::ResourcePacks)s
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct TextureAtlases {
    pub atlases: HashMap<TextureAtlasType, Handle<TextureAtlas>>,
}

impl TextureAtlases {
    pub(crate) fn loaded(atlases: Res<TextureAtlases>, assets: Res<AssetServer>) -> bool {
        atlases.atlases.values().all(|handle| {
            let state = assets.get_recursive_dependency_load_state(handle);

            matches!(state, None | Some(RecursiveDependencyLoadState::Loaded))
        })
    }

    pub(crate) fn build(
        packs: Res<ResourcePacks>,
        pack_assets: Res<Assets<ResourcePackAsset>>,

        mut atlases: ResMut<TextureAtlases>,
        mut atlas_assets: ResMut<Assets<TextureAtlas>>,
        mut image_assets: ResMut<Assets<Image>>,
    ) {
        for kind in TextureAtlasType::iter() {
            let path: ResourceLocation = kind.into();
            let coords: Vec<Rect> = kind.into();

            // Get the image handle
            let Some(handle) = packs.get_texture(&path, &pack_assets) else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Missing texture for atlas: {kind}");
                continue;
            };

            // Get the image
            let Some(image) = image_assets.get(handle) else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Missing image for atlas: {kind}");
                continue;
            };

            // Get the image size and the coordinate size
            let (image_width, image_height) = image.size().into();
            let (coord_width, coord_height) = kind.into();

            // Build the atlas
            let mut builder = TextureAtlasBuilder::default().format(TextureFormat::Rgba8Unorm);
            builder.add_texture(handle.id(), image);

            let mut atlas = match builder.finish(&mut image_assets) {
                Err(err) => {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    error!("Failed to build atlas: {kind}: {err}");
                    continue;
                }
                Ok(atlas) => atlas,
            };

            // Add coordinates to the atlas
            for mut coord in coords {
                // Scale the coordinates to the image size
                coord.min.x *= image_width as f32 / coord_width as f32;
                coord.max.x *= image_width as f32 / coord_width as f32;
                coord.min.y *= image_height as f32 / coord_height as f32;
                coord.max.y *= image_height as f32 / coord_height as f32;

                atlas.add_texture(coord);
            }

            // Add the atlas to the list
            let handle = atlas_assets.add(atlas);
            atlases.insert(kind, handle);

            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Added atlas: {kind}");
        }
    }

    pub fn get(&self, atlas: impl Into<TextureAtlasType>) -> Option<&Handle<TextureAtlas>> {
        self.atlases.get(&atlas.into())
    }

    pub fn get_and_index(
        &self,
        atlas: impl Into<TextureAtlasType>,
        index: usize,
    ) -> Option<(&Handle<TextureAtlas>, UiTextureAtlasImage)> {
        self.get(atlas).map(|handle| {
            (
                handle,
                UiTextureAtlasImage {
                    index,
                    flip_x: false,
                    flip_y: false,
                },
            )
        })
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum TextureAtlasType {
    Icons,
}

impl From<TextureAtlasType> for (u32, u32) {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::size(),
        }
    }
}

impl From<TextureAtlasType> for ResourceLocation {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::path(),
        }
    }
}

impl From<TextureAtlasType> for Vec<Rect> {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => IconAtlas::coords(),
        }
    }
}