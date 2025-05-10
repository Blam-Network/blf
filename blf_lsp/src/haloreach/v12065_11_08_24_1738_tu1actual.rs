use napi::bindgen_prelude::{Uint8Array, Undefined};
use napi::Either;
use napi_derive::napi;
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_player_data, s_blf_chunk_service_record, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12065_11_08_24_1738_tu1actual;
use blf_lib::types::byte_order_mark::byte_order_mark;

#[napi(namespace = "haloreach_12065_11_08_24_1738_tu1actual")]
pub fn build_user_file(
    fupd: Either<s_blf_chunk_player_data, Undefined>,
    srid: Either<s_blf_chunk_service_record, Undefined>,
) -> Uint8Array {
    let mut builder = BlfFileBuilder::new();
    builder.add_chunk(s_blf_chunk_start_of_file::new("omaha user", byte_order_mark::default()));
    builder.add_chunk(s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>());

    match fupd {
        Either::A(fupd) => {
            builder.add_chunk(fupd);
        }
        _ => {
            builder.add_chunk(s_blf_chunk_player_data::default());
        }
    }

    match srid {
        Either::A(srid) => {
            builder.add_chunk(srid);
        }
        _ => {}
    }

    builder.add_chunk(s_blf_chunk_end_of_file::default());

    Uint8Array::new(builder.write())
}
