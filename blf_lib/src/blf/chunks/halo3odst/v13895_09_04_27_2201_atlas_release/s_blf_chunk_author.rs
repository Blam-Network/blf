use binrw::{binrw};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derivable::blf::chunks::{BlfChunkHooks, TitleAndBuild};
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("athr", 3.1)]
#[Size(0x44)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_13895_09_04_27_2201_atlas_release"))]
pub struct s_blf_chunk_author {
    pub program_name: StaticString<16>, // eg GameData.Halo3, GameData.Reach
    pub build_number: u32,              // eg 0, 11860, 12065
    pub build_number_sequence: u32,     // eg 1, 2
    pub build_string: StaticString<28>, // eg 11860.10.07.24.0147.omaha_r, untracked
    pub author_name: StaticString<16>,  // eg davidav, dagasca
}

impl BlfChunkHooks for s_blf_chunk_author {}

impl s_blf_chunk_author {
    pub fn for_build<T: TitleAndBuild>() -> s_blf_chunk_author {
        let build_number = T::get_build_string()[..5].parse().unwrap_or(0xFFFFFFFF);

        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");

        let author_name = format!("{name} v{version}");
        let author_name = &author_name[..16.min(author_name.len())];

        Self {
            program_name: StaticString::from_string(author_name)
                .expect("s_blf_chunk_author::for_build has a bad program name! This should never happen"),
            build_number,
            build_number_sequence: 2,
            build_string: StaticString::from_string(T::get_build_string()[..28].to_string())
                .expect("s_blf_chunk_author::for_build has a bad build string! This should never happen"),
            author_name: Default::default(),
        }
    }
}
