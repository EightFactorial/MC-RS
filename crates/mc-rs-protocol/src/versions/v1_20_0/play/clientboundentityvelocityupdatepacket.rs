use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundEntityVelocityUpdatePacket {
    pub entity_id: EntityId,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}
