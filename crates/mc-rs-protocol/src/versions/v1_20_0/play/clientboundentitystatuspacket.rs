use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [28, 0, 0, 1, 231])]
pub struct ClientboundEntityStatusPacket {
    pub entity_id: u32,
    pub event_id: u8,
}
