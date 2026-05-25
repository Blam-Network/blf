use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_weapon_pickup_priority {
    #[default]
    normal = 0,
    high = 1,
    automatic = 2,
}
