use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0])]
pub struct ServerboundTeleportConfirmPacket(#[var] u32);
