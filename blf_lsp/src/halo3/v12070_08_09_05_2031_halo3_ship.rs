use napi::bindgen_prelude::{Uint8Array, Undefined};
use napi::Either;
use napi_derive::napi;
use blf_lib::blf::{BlfFile, BlfFileBuilder};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_player_data, s_blf_chunk_service_record, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::types::byte_order_mark::byte_order_mark;
use bytes::{Buf, Bytes};

#[cfg(feature = "blf_private")]
use blf_lib_private::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::multi;

#[napi(namespace = "halo3_12070_08_09_05_2031_halo3_ship")]
pub fn build_user_file(
    fupd: Either<s_blf_chunk_player_data, Undefined>,
    srid: Either<s_blf_chunk_service_record, Undefined>,
) -> Uint8Array {
    let mut builder = BlfFileBuilder::new();
    builder.add_chunk(s_blf_chunk_start_of_file::new("halo3 user", byte_order_mark::default()));
    builder.add_chunk(s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>());

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

#[cfg(feature = "blf_private")]
#[napi(namespace = "halo3_12070_08_09_05_2031_halo3_ship")]
pub fn read_webstats(
    buffer: Uint8Array,
) -> Option<multi> {
    // let mut data = zlib_decompress(&Bytes::from(buffer.to_vec()));
    // if data.is_err() {return None};
    // let webstats = webstats::read(&mut data.unwrap().reader());
    let webstats = multi::read(&mut Bytes::from(buffer.to_vec()).reader());
    if webstats.is_err() {return None};

    Some(webstats.unwrap())
}
