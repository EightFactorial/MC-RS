use mc_rs_macros::Transcode;

use crate::types::position::Vec3;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerPositionLookPacket {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub relative_flags: u8,
    #[var]
    pub id: u32,
}
