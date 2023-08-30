use std::{hash::Hash, ops::Range, sync::Arc};

use bevy::prelude::*;
use mc_rs_proto::types::ResourceLocation;
use nohash_hasher::IntMap;
use parking_lot::RwLock;

use self::properties::BlockProperties;

pub mod properties;

mod blocksmap;
pub use blocksmap::BlocksMapFn;

#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct Blocks(pub Arc<RwLock<BlocksMap>>);
pub(super) type BlocksMap = IntMap<u32, Block>;

#[derive(Debug, Clone)]
pub struct Block {
    pub block_id: u32,
    pub block_states: Range<u32>,
    pub name: String,
    pub key: ResourceLocation,
    pub properties: BlockProperties,
}

impl Eq for Block {}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool { self.block_id == other.block_id }
}
impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.block_id.hash(state); }
}

impl Blocks {
    pub(super) fn create(asset_server: Res<AssetServer>, mut commands: Commands) {
        let mut blocks = BlocksMap::default();

        commands.insert_resource(Blocks(Arc::new(RwLock::new(blocks))));
    }
}
