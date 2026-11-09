use binrw::{BinRead, BinWrite};
use blf_lib::TestSize;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(4)]
pub struct s_network_session_privacy_mode {
    pub network_session_privacy: u8,
    pub network_session_closed_status: u8,
    pub unknown2: u8,
    pub unknown3: u8,
}
