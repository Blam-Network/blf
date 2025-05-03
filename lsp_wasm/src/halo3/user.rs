use napi::bindgen_prelude::Uint8Array;
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_player_data, s_blf_chunk_service_record, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::types::byte_order_mark::byte_order_mark;

pub fn build_user_file(srid: s_blf_chunk_service_record) -> Uint8Array {
    Uint8Array::new(
        BlfFileBuilder::new()
            .add_chunk(s_blf_chunk_start_of_file::new("halo3 user", byte_order_mark::default()))
            .add_chunk(s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>())
            .add_chunk(s_blf_chunk_player_data::default())
            .add_chunk(srid.clone())
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write()
    )
}