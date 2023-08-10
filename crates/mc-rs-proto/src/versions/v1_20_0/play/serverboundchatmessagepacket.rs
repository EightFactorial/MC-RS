use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundChatMessagePacket {
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub data: UnsizedByteBuffer,
}