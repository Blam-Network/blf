use std::fmt;
use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use shrinkwraprs::Shrinkwrap;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "napi")]
use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
#[cfg(feature = "napi")]
use napi::sys::{napi_env, napi_value};

#[derive(Debug, Clone, PartialEq, Copy, Default, Shrinkwrap)]
#[shrinkwrap(mutable)]
#[wasm_bindgen(getter_with_clone)]
pub struct Bool(pub bool);

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.0 { "true" } else { "false" })
    }
}

impl From<Bool> for bool {
    fn from(val: Bool) -> Self {
        val.0
    }
}

impl From<bool> for Bool {
    fn from(val: bool) -> Bool {
        Bool(val)
    }
}

impl PartialEq<bool> for Bool {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}


// Custom Serialize implementation
impl Serialize for Bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the value directly as a boolean, not as an object
        self.0.serialize(serializer)
    }
}

// Custom Deserialize implementation
impl<'de> Deserialize<'de> for Bool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a boolean directly into the s_bool struct
        let value = bool::deserialize(deserializer)?;
        Ok(Bool(value))
    }
}

impl BinRead for Bool {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, binrw::Error> {
        // Standard read function for reading a single byte (boolean)
        let byte: u8 = reader.read_type(Endian::NATIVE)?;
        let value = byte != 0; // Interpreting 0 as false, any non-zero value as true
        Ok(Bool(value))
    }

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        // Since we are only reading a single byte, endian doesn't affect this for a bool,
        // but this prepares it for future types where endian would matter.
        let byte: u8 = reader.read_type(endian)?;
        let value = byte != 0;

        Ok(Bool(value))
    }
}

impl BinWrite for Bool {
    type Args<'a> = ();

    fn write<W: Write>(&self, writer: &mut W) -> Result<(), binrw::Error> {
        let byte = if self.0 { 1u8 } else { 0u8 };
        writer.write(&byte.to_ne_bytes())?;
        Ok(())
    }

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        // Since we are only writing a single byte, endian doesn't affect this for a bool,
        // but this prepares it for future types where endian would matter.
        let byte = if self.0 { 1u8 } else { 0u8 };

        match endian {
            Endian::Big => {
                writer.write(&byte.to_be_bytes())?;
            }
            Endian::Little => {
                writer.write(&byte.to_le_bytes())?;
            }
        }

        Ok(())
    }
}

#[cfg(feature = "napi")]
impl ToNapiValue for Bool {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        bool::to_napi_value(env, val.0)
    }
}

#[cfg(feature = "napi")]
impl FromNapiValue for Bool {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        Ok(Self {
            0: bool::from_napi_value(env, napi_val)?,
        })
    }
}