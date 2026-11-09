use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize,Default,BinRead,BinWrite)]
#[brw(big)]
pub struct s_persistent_per_commendation_state {
    pub progress: i16,
}