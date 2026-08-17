pub mod player_commendations;

use blf_lib::bitfield;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

// I'm not sure where this belongs, probably not here though.
bitfield! {
    #[derive(Serialize,Deserialize)]
    #[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
    pub struct e_purchase_state: u8 {
        purchased,
        forced_visible_and_purchasable,
        bypassed,
        granted_by_lsp,
        banned,
    }
}