use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use napi::bindgen_prelude::{BigInt, FromNapiValue, ToNapiValue};
use napi::sys::{napi_env, napi_value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use shrinkwraprs::Shrinkwrap;

#[derive(Debug, Clone, PartialEq, Copy, Default, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Unsigned64(pub u64);

#[cfg(feature = "napi")]
impl FromNapiValue for Unsigned64 {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        BigInt::from_napi_value(env, napi_val).map(|val| Unsigned64(val.get_u64().1))
    }
}

#[cfg(feature = "napi")]
impl ToNapiValue for Unsigned64 {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        u64::to_napi_value(env, val.0)
    }
}


// Custom Serialize implementation
impl Serialize for Unsigned64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the value directly as a boolean, not as an object
        self.0.serialize(serializer)
    }
}

// Custom Deserialize implementation
impl<'de> Deserialize<'de> for Unsigned64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a boolean directly into the s_bool struct
        let value = u64::deserialize(deserializer)?;
        Ok(Unsigned64(value))
    }
}

impl BinRead for Unsigned64 {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, binrw::Error> {
        Ok(Unsigned64(reader.read_type(Endian::NATIVE)?))
    }

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        Ok(Unsigned64(reader.read_type(endian)?))
    }
}

impl BinWrite for Unsigned64 {
    type Args<'a> = ();

    fn write<W: Write>(&self, writer: &mut W) -> Result<(), binrw::Error> {
        writer.write(&self.0.to_ne_bytes())?;
        Ok(())
    }

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        match endian {
            Endian::Big => {
                writer.write(&self.0.to_be_bytes())?;
            }
            Endian::Little => {
                writer.write(&self.0.to_le_bytes())?;
            }
        }

        Ok(())
    }
}
