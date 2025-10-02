use binrw::{BinRead, BinWrite};
use serde::{Deserializer, Serialize, Serializer};
use blf_lib::SERDE_DESERIALIZE_RESULT;
use blf_lib_derivable::result::BLFLibError;

#[cfg(feature = "napi")]
use napi_derive::napi;
use blf_lib::types::array::StaticArray;

const HTTP_REQUEST_HASH_LENGTH: usize = 20;

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object))]
pub struct s_network_http_request_hash {
    pub data: StaticArray<u8, HTTP_REQUEST_HASH_LENGTH>
}

impl Serialize for s_network_http_request_hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&hex::encode_upper(self.data.get()).to_string())
    }
}

impl<'de> serde::Deserialize<'de> for s_network_http_request_hash {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(SERDE_DESERIALIZE_RESULT!(
            SERDE_DESERIALIZE_RESULT!(hex::decode(String::deserialize(d)?))?.try_into()
        )?)
    }
}

impl TryFrom<Vec<u8>> for s_network_http_request_hash {
    type Error = BLFLibError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let data: [u8; HTTP_REQUEST_HASH_LENGTH] = value.try_into()
            .map_err(|v: Vec<u8>| -> Self::Error { format!("Expected {HTTP_REQUEST_HASH_LENGTH} bytes, got {} bytes", v.len()).into() })?;

        Ok(s_network_http_request_hash {
            data: StaticArray::from_slice(&data)?,
        })
    }
}