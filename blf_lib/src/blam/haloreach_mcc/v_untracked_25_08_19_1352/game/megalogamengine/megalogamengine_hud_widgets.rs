use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_megalogamengine_hud_meter_input_type {
    #[default]
    none = 0,
    number = 1,
    timer = 2,
}
