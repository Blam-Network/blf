pub mod player_commendations;

use blf_lib::bitfield;
use serde::{Deserialize, Serialize};

// I'm not sure where this belongs, probably not here though.
bitfield! {
    #[derive(Serialize,Deserialize)]
    pub struct e_purchase_state: u8 {
        purchased,
        forced_visible_and_purchasable,
        bypassed,
        granted_by_lsp,
        banned,
    }
}