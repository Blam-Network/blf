use std::cmp::min;
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
#[Header("athr", 2.1)]
#[Size(60)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_08117_07_03_07_1702_delta"))]
pub struct s_blf_chunk_author {
    pub program_name: StaticString<16>, // eg GameData.Halo3
    pub build_number_sequence: u32,     // eg 1
    pub build_number: u32,              // eg 12070
    pub build_string: StaticString<16>, // eg 08117.07.03.07.
    pub author_name: StaticString<20>,  // eg sameling
}

impl BlfChunkHooks for s_blf_chunk_author {}

impl s_blf_chunk_author {
    pub fn for_build<T: TitleAndBuild>() -> s_blf_chunk_author {
        let build_number = T::get_build_string()[..5].parse().unwrap_or(0xFFFFFFFF);

        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");

        let author_name = format!("{name} v{version}");
        let author_name = &author_name[..20.min(author_name.len())];

        Self {
            program_name: StaticString::from_string(author_name)
                .expect("s_blf_chunk_author::for_build has a bad program name! This should never happen"),
            build_number,
            build_number_sequence: 1,
            build_string: StaticString::from_string(T::get_build_string()[..min(T::get_build_string().len(), 16)].to_string())
                .expect("s_blf_chunk_author::for_build has a bad build string! This should never happen"),
            author_name: Default::default(),
        }
    }
}
