use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_player_data, s_blf_chunk_service_record};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;


pub mod user;

#[wasm_bindgen]
pub struct HALO3 {}
#[wasm_bindgen]
impl HALO3 {
    pub fn build_user_file(
        &self,
        srid: Option<s_blf_chunk_service_record>,
        fupd: Option<s_blf_chunk_player_data>
    ) -> Uint8Array {
        user::build_user_file(srid, fupd)
    }
}