use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_player_data, s_blf_chunk_service_record, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::types::byte_order_mark::byte_order_mark;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

pub fn build_user_file(
    srid: Option<s_blf_chunk_service_record>,
    fupd: Option<s_blf_chunk_player_data>,
) -> Uint8Array {
    let mut builder = BlfFileBuilder::new();

    builder.add_chunk(s_blf_chunk_start_of_file::new("halo3 user", byte_order_mark::default()));
    builder.add_chunk(fupd.unwrap_or(s_blf_chunk_player_data::default()));

    if let Some(srid) = srid {
        builder.add_chunk(srid);
    }

    builder.add_chunk(s_blf_chunk_end_of_file::default());

    Uint8Array::from(builder.write().as_slice())
}