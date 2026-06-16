use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use blf_lib::types::string::StaticString;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
#[cfg(feature = "napi")]
use napi_derive::napi;

/// MCC fileshare metadata (`_fsm` 1.1): digests, item id, RSA-SHA512 attestation.
#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "mcc_v2025_08_16_178512_1_release"))]
#[derive(BlfChunk, Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[Header("_fsm", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_fileshare_metadata {
    pub unknown0: u64,
    pub unknown8: StaticArray<u8, 32>,
    pub unknown28: StaticString<36>,
    pub unknown4c: u64,
    pub unknown54: StaticArray<u8, 32>,
    pub unknown74: StaticString<36>,
    pub unknown98: StaticString<40>,
    #[brw(pad_after = 4)]
    pub attestation_signature: StaticArray<u8, 256>,
}

impl BlfChunkHooks for s_blf_chunk_fileshare_metadata {}
