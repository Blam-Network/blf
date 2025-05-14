use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use shrinkwraprs::Shrinkwrap;
use std::net::Ipv4Addr as WrappedIpv4Addr;

#[cfg(feature = "napi")]
use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
#[cfg(feature = "napi")]
use napi::sys::{napi_env, napi_value};

#[derive(Debug, Clone, PartialEq, Copy, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Ipv4Addr(pub WrappedIpv4Addr);

impl Default for Ipv4Addr {
    fn default() -> Self {
        Self(WrappedIpv4Addr::new(0, 0, 0, 0))
    }
}

#[cfg(feature = "napi")]
impl FromNapiValue for Ipv4Addr {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        u32::from_napi_value(env, napi_val).map(|val| Ipv4Addr(WrappedIpv4Addr::from(val)))
    }
}

#[cfg(feature = "napi")]
impl ToNapiValue for Ipv4Addr {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        u32::to_napi_value(env, val.0.to_bits())
    }
}


// Custom Serialize implementation
impl Serialize for Ipv4Addr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the value directly as a boolean, not as an object
        self.0.serialize(serializer)
    }
}

// Custom Deserialize implementation
impl<'de> Deserialize<'de> for Ipv4Addr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a boolean directly into the s_bool struct
        let value = WrappedIpv4Addr::deserialize(deserializer)?;
        Ok(Ipv4Addr(value))
    }
}

impl BinRead for Ipv4Addr {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, binrw::Error> {
        Ok(Ipv4Addr(WrappedIpv4Addr::from(reader.read_type::<u32>(Endian::NATIVE)?)))
    }

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        Ok(Ipv4Addr(WrappedIpv4Addr::from(reader.read_type::<u32>(endian)?)))
    }
}

impl BinWrite for Ipv4Addr {
    type Args<'a> = ();

    fn write<W: Write>(&self, writer: &mut W) -> Result<(), binrw::Error> {
        writer.write(&self.0.to_bits().to_ne_bytes())?;
        Ok(())
    }

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        match endian {
            Endian::Big => {
                writer.write(&self.0.to_bits().to_be_bytes())?;
            }
            Endian::Little => {
                writer.write(&self.0.to_bits().to_le_bytes())?;
            }
        }

        Ok(())
    }
}
