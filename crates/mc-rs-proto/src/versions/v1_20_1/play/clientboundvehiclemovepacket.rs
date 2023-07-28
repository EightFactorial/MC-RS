use mc_rs_macros::Transcode;

use crate::types::position::Vec3;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundVehicleMovePacket {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}
