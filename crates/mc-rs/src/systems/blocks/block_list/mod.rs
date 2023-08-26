use std::sync::{Arc, RwLock};

use bevy::{asset::LoadState, prelude::*};
use mc_rs_proto::types::ResourceLocation;
use nohash_hasher::IntMap;

use super::block::{voxel_texture::VoxelTexture, voxel_type::VoxelType, Block, BlockType};

mod list;

/// Adds all block systems to the app
pub(super) fn add_systems(app: &mut App) { app.add_systems(Startup, Blocks::init_blocks); }

#[derive(Clone, Default, Resource, Deref, DerefMut)]
pub struct Blocks(Arc<RwLock<IntMap<u32, Block>>>);

impl Blocks {
    /// Initialize the block list and load the block textures
    fn init_blocks(mut commands: Commands, assets: Res<AssetServer>) {
        let blocks = Self::default();

        // Insert the error block
        blocks.write().unwrap().insert(
            u32::MAX,
            Block {
                id: u32::MAX,
                name: "Error".to_string(),
                key: ResourceLocation::new("mc-rs:error"),
                block_type: BlockType::new_voxel(
                    VoxelType::Opaque(u32::MAX),
                    VoxelTexture::from_paths(&["light_blue_wool.png"], &assets).unwrap(),
                ),
            },
        );

        // Initialize the block list
        {
            let mut write = blocks.write().unwrap();
            list::initialize(&mut write, &assets);
        }

        commands.insert_resource(blocks);
    }

    /// Returns true if all the block textures are loaded
    pub fn is_loaded(&self, assets: &AssetServer) -> bool {
        self.blocks_loaded(assets) == self.blocks_with_textures()
    }

    // Get the number of blocks with all textures loaded
    pub fn blocks_loaded(&self, assets: &AssetServer) -> f32 {
        self.read().unwrap().values().fold(0u32, |acc, block| {
            let Some(textures) = block.textures() else {
                return acc;
            };

            let ids = textures.iter().map(|t| t.id());
            acc + matches!(
                assets.get_group_load_state(ids),
                LoadState::Loaded | LoadState::Failed
            ) as u32
        }) as f32
    }

    // Get the number of blocks with textures
    pub fn blocks_with_textures(&self) -> f32 {
        self.read()
            .unwrap()
            .values()
            .filter(|&b| b.textures().is_some())
            .count() as f32
    }

    /// Return the progress of loading the block textures
    pub fn progress(&self, assets: &AssetServer) -> f32 {
        self.blocks_loaded(assets) / self.blocks_with_textures()
    }

    /// Replaces all failed block textures with the error block texture
    ///
    /// Returns the number of blocks that were fixed
    pub fn replace_errors(&mut self, assets: &AssetServer) -> u32 {
        let fallback = self.read().unwrap()[&u32::MAX].clone();
        let mut acc = 0;

        for block in self.write().unwrap().values_mut() {
            if let Some(textures) = block.textures() {
                // Check if any of the textures failed to load
                let ids = textures.iter().map(|t| t.id());
                if assets.get_group_load_state(ids) == LoadState::Failed {
                    // Replace the block with the error block
                    if let BlockType::Voxel {
                        voxel_type,
                        texture,
                    } = &fallback.block_type
                    {
                        let voxel_type = match voxel_type {
                            VoxelType::Opaque(_) => VoxelType::Opaque(block.id),
                            VoxelType::Translucent(_) => VoxelType::Translucent(block.id),
                            VoxelType::NoMesh(_) => VoxelType::NoMesh(rand::random()),
                            VoxelType::Empty => VoxelType::Empty,
                        };

                        block.block_type = BlockType::new_voxel(voxel_type, texture.clone());
                    } else {
                        block.block_type = fallback.block_type.clone();
                    }

                    acc += 1;
                }
            }
        }

        acc
    }
}