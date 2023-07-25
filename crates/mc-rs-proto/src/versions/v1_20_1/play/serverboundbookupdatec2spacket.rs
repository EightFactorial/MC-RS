use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundBookUpdateC2SPacket {
    pub a: u32,
    pub b: Vec,
    pub c: Option,
}