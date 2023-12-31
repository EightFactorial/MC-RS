use azalea_nbt::Nbt;
use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
pub enum ItemSlot {
    #[default]
    Empty,
    Item(ItemSlotData),
}

impl PartialEq for ItemSlot {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => true,
            (Self::Item(l0), Self::Item(r0)) => l0 == r0,
            (Self::Item(item), Self::Empty) | (Self::Empty, Self::Item(item)) => item.is_empty(),
        }
    }
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

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ItemSlotData {
    #[var]
    pub kind: u32,
    pub count: i8,
    pub nbt: Nbt,
}

impl ItemSlotData {
    pub fn new(kind: u32, count: i8, nbt: Nbt) -> Self { Self { kind, count, nbt } }

    pub fn is_empty(&self) -> bool { self.count == 0 || self.kind == 0 }
}
