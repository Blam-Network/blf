use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

/// HUD widget screen anchor (`m_hud_widgets` entries, 4 bits each).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize, crate::derive::c_enum)]
#[bits(4)]
pub enum e_megalo_widget_position {
    #[default]
    top_left = 0,
    top_center = 1,
    top_right = 2,
    high_left = 3,
    high_center = 4,
    high_right = 5,
    low_left = 6,
    low_center = 7,
    low_right = 8,
    bottom_left = 9,
    bottom_center = 10,
    bottom_right = 11,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_megalogamengine_hud_meter_input_type {
    #[default]
    none = 0,
    number = 1,
    timer = 2,
}
