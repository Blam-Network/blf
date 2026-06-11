use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::result::BLFLibResult;

#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, ToPrimitive, FromPrimitive, Serialize, Deserialize)]
pub enum e_game_variant_icon {
    #[default]
    none = 0,
    random = 1,
    rotate = 2,
    description = 3,
    category = 4,
    ctf = 5,
    slayer = 6,
    oddball = 7,
    koth = 8,
    juggernaut = 9,
    territories = 10,
    assault = 11,
    infection = 12,
    vip = 13,
    invasion = 14,
    stockpile = 15,
    action_sack = 16,
    race = 17,
    headhunter = 18,
    wip = 19,
    dogfight = 20,
    insane = 21,
    bungie = 22,
    ms343 = 23,
    heroic = 24,
    legendary = 25,
    mythic = 26,
    mantis = 27,
    shishka = 28,
    huevos = 29,
    jonnyo = 30,
    dangerboy = 31,
    holiday = 32,
    community = 33,
    matchmaking = 34,
    pre_game_warm_up = 35,
}

impl e_game_variant_icon {
    pub fn from_i8(value: i8) -> BLFLibResult<Self> {
        FromPrimitive::from_i8(value)
            .ok_or_else(|| format!("Unknown game variant icon index: {value}").into())
    }
}

impl From<e_game_variant_icon> for i8 {
    fn from(value: e_game_variant_icon) -> i8 {
        value as i8
    }
}
