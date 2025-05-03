use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;

pub mod user;

#[napi(namespace = "BLF")]
struct Halo3 {}

#[napi(namespace = "BLF")]
impl Halo3 {
    #[napi]
    pub fn build_user_file(xuid: String) -> Uint8Array {
        user::build_user_file(xuid)
    }
}