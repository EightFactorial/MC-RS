use bevy::prelude::{Assets, Handle, Image, World};
use mc_rs_core::ResourceLocation;

use crate::resourcepacks::{ResourcePackAsset, ResourcePacks};

pub trait GetAssetFromWorld {
    fn get_texture(&self, resource: &ResourceLocation) -> &Handle<Image>;
    fn try_get_texture(&self, resource: &ResourceLocation) -> Option<&Handle<Image>>;
}

impl GetAssetFromWorld for World {
    fn get_texture(&self, resource: &ResourceLocation) -> &Handle<Image> {
        let packs = self.resource::<ResourcePacks>();
        let assets = self.resource::<Assets<ResourcePackAsset>>();

        packs.get_texture(resource, assets)
    }

    fn try_get_texture(&self, resource: &ResourceLocation) -> Option<&Handle<Image>> {
        let packs = self.resource::<ResourcePacks>();
        let assets = self.resource::<Assets<ResourcePackAsset>>();

        packs.try_get_texture(resource, assets)
    }
}
