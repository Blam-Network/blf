use std::fmt::{Display, Formatter, UpperHex};
use std::io::{Read, Seek, Write};
use std::ops::{Add, Div, Mul, Sub};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, Endian};
#[cfg(feature = "napi")]
use napi::bindgen_prelude::{BigInt, FromNapiValue, ToNapiValue};
#[cfg(feature = "napi")]
use napi::sys::{napi_env, napi_value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_hex::{HexConf, SerHex};
use shrinkwraprs::Shrinkwrap;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::describe::WasmDescribe;

#[derive(Debug, Clone, PartialEq, Copy, Default, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Unsigned64(pub u64);

impl Display for Unsigned64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <u64 as Display>::fmt(&self.0, f)
    }
}

impl From<u64> for Unsigned64 {
    fn from(x: u64) -> Unsigned64 {
        Unsigned64(x)
    }
}

impl Into<u64> for Unsigned64 {
    fn into(self) -> u64 {
        self.0
    }
}

impl Add<u64> for Unsigned64 {
    type Output = Unsigned64;
    fn add(self, rhs: u64) -> Self::Output {
        Unsigned64(self.0 + rhs)
    }
}

impl Mul<u64> for Unsigned64 {
    type Output = Unsigned64;
    fn mul(self, rhs: u64) -> Self::Output {
        Unsigned64(self.0 * rhs)
    }
}

impl Sub<u64> for Unsigned64 {
    type Output = Unsigned64;
    fn sub(self, rhs: u64) -> Self::Output {
        Unsigned64(self.0 - rhs)
    }
}

impl Div<u64> for Unsigned64 {
    type Output = Unsigned64;
    fn div(self, rhs: u64) -> Self::Output {
        Unsigned64(self.0 / rhs)
    }
}

impl<C> SerHex<C> for Unsigned64 where C: HexConf,
{
    type Error = serde_hex::Error;

    fn into_hex_raw<D>(&self, dst: D) -> Result<(), Self::Error>
    where
        D: Write
    {
        <u64 as SerHex<C>>::into_hex_raw(&self.0, dst)
    }

    fn from_hex_raw<S>(src: S) -> Result<Self, Self::Error>
    where
        S: AsRef<[u8]>
    {
        Ok(Unsigned64(<u64 as SerHex<C>>::from_hex_raw(src)?))
    }
}

impl UpperHex for Unsigned64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <u64 as UpperHex>::fmt(&self.0, f)
    }
}

#[cfg(feature = "napi")]
impl ToNapiValue for Unsigned64 {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        u64::to_napi_value(env, val.0)
    }
}

#[cfg(feature = "napi")]
impl FromNapiValue for Unsigned64 {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        BigInt::from_napi_value(env, napi_val).map(|val| Unsigned64(val.get_u64().1))
    }
}


// Custom Serialize implementation
impl Serialize for Unsigned64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the value directly as a boolean, not as an object
        Serialize::serialize(&self.0, serializer)
    }
}

// Custom Deserialize implementation
impl<'de> Deserialize<'de> for Unsigned64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = <u64 as Deserialize>::deserialize(deserializer)?;
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

impl WasmDescribe for Unsigned64 {
    fn describe() {
        u64::describe()
    }
}

impl IntoWasmAbi for Unsigned64 {
    type Abi = <u64 as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        u64::into_abi(self.0)
    }
}

impl FromWasmAbi for Unsigned64 {
    type Abi = <u64 as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        Unsigned64(u64::from_abi(js))
    }
}