use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_service_record;

pub mod user;

#[napi(namespace = "BLF")]
struct Halo3 {}

#[napi(namespace = "BLF")]
impl Halo3 {
    #[napi]
    pub fn build_user_file(
        #[napi(ts_arg_type = "halo3.blf_chunk_service_record")]
        srid: s_blf_chunk_service_record
    ) -> Uint8Array {
        user::build_user_file(srid)
    }
}