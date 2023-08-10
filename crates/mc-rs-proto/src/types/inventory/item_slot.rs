use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Default, Clone, Transcode)]
pub enum ItemSlot {
    #[default]
    Empty,
    Item(ItemSlotData),
}

impl ItemSlot {
    pub fn count(&self) -> i8 {
        match self {
            ItemSlot::Empty => 0,
            ItemSlot::Item(slot) => slot.count,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ItemSlot::Empty => true,
            ItemSlot::Item(slot) => slot.is_empty(),
        }
    }

    pub fn update_slot(&mut self) {
        if self.is_empty() {
            *self = ItemSlot::Empty;
        }
    }
}

#[derive(Debug, Clone, Transcode)]
pub struct ItemSlotData {
    #[var]
    pub kind: u32,
    pub count: i8,
    pub nbt: UnsizedByteBuffer,
}

impl ItemSlotData {
    pub fn new(kind: u32, count: i8, nbt: UnsizedByteBuffer) -> Self { Self { kind, count, nbt } }

    pub fn is_empty(&self) -> bool {
        self.count == 0 || self.kind.to_string().as_str() == "minecraft:air"
    }
}
