use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_game_engine_timer_rate {
    #[default]
    zero = 0,
    minus_10x = 1,
    minus_25x = 2,
    minus_50x = 3,
    minus_75x = 4,
    minus_100x = 5,
    minus_125x = 6,
    minus_150x = 7,
    minus_175x = 8,
    minus_200x = 9,
    minus_300x = 10,
    minus_400x = 11,
    minus_500x = 12,
    minus_1000x = 13,
    _10x = 14,
    _25x = 15,
    _50x = 16,
    _75x = 17,
    _100x = 18,
    _125x = 19,
    _150x = 20,
    _175x = 21,
    _200x = 22,
    _300x = 23,
    _400x = 24,
    _500x = 25,
    _1000x = 26,
}
