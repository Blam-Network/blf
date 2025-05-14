use std::cmp::Ordering;
use std::fmt::Display;
use std::io::{Read, Seek, Write};
use std::ops::{Add, Div, Mul, MulAssign, Sub};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use shrinkwraprs::Shrinkwrap;

#[cfg(feature = "napi")]
use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
#[cfg(feature = "napi")]
use napi::sys::{napi_env, napi_value};
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::describe::WasmDescribe;

#[derive(Debug, Clone, PartialEq, Copy, Default, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Float32(pub f32);

impl Display for Float32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

impl Into<f32> for Float32 {
    fn into(self) -> f32 {
        self.0
    }
}

impl From<f32> for Float32 {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Add<f32> for Float32 {
    type Output = f32;
    fn add(self, rhs: f32) -> Self::Output {
        self.0 + rhs
    }
}

impl Sub<f32> for Float32 {
    type Output = f32;
    fn sub(self, rhs: f32) -> Self::Output {
        self.0 - rhs
    }
}

impl Mul<f32> for Float32 {
    type Output = f32;
    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl Div<f32> for Float32 {
    type Output = f32;
    fn div(self, rhs: f32) -> Self::Output {
        self.0 / rhs
    }
}

impl Mul<Float32> for f32 {
    type Output = Float32;

    fn mul(self, rhs: Float32) -> Self::Output {
        Float32::from(self * rhs.0)
    }
}

impl Div<Float32> for f32 {
    type Output = Float32;
    fn div(self, rhs: Float32) -> Self::Output {
        Float32::from(self / rhs.0)
    }
}

impl Add<Float32> for f32 {
    type Output = Float32;
    fn add(self, rhs: Float32) -> Self::Output {
        Float32::from(self + rhs.0)
    }
}

impl Sub<Float32> for f32 {
    type Output = Float32;
    fn sub(self, rhs: Float32) -> Self::Output {
        Float32::from(self - rhs.0)
    }
}

impl Add<Float32> for Float32 {
    type Output = Float32;
    fn add(self, rhs: Float32) -> Self::Output {
        Float32(self.0 + rhs.0)
    }
}

impl Sub<Float32> for Float32 {
    type Output = Float32;
    fn sub(self, rhs: Float32) -> Self::Output {
        Float32(self.0 - rhs.0)
    }
}

impl Mul<Float32> for Float32 {
    type Output = Float32;
    fn mul(self, rhs: Float32) -> Self::Output {
        Float32(self.0 * rhs.0)
    }
}

impl Div<Float32> for Float32 {
    type Output = Float32;
    fn div(self, rhs: Float32) -> Self::Output {
        Float32(self.0 / rhs.0)
    }
}

impl PartialOrd for Float32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq<f64> for Float32 {
    fn eq(&self, other: &f64) -> bool {
        (self.0 as f64).eq(other)
    }
}

impl PartialOrd<f64> for Float32 {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        (self.0 as f64).partial_cmp(other)
    }
}

impl PartialEq<f32> for Float32 {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<f32> for Float32 {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl MulAssign<f32> for Float32 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}


#[cfg(feature = "napi")]
impl FromNapiValue for Float32 {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        f64::from_napi_value(env, napi_val).map(|val| Float32(val as f32))
    }
}

#[cfg(feature = "napi")]
impl ToNapiValue for Float32 {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        f32::to_napi_value(env, val.0)
    }
}

impl WasmDescribe for Float32 {
    fn describe() {
        f32::describe()
    }
}

impl IntoWasmAbi for Float32 {
    type Abi = f32;

    fn into_abi(self) -> Self::Abi {
        f32::into_abi(self.0)
    }
}

impl FromWasmAbi for Float32 {
    type Abi = <f32 as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        Self(f32::from_abi(js))
    }
}


// Custom Serialize implementation
impl Serialize for Float32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the value directly as a boolean, not as an object
        self.0.serialize(serializer)
    }
}

// Custom Deserialize implementation
impl<'de> Deserialize<'de> for Float32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a boolean directly into the s_bool struct
        let value = f32::deserialize(deserializer)?;
        Ok(Float32(value))
    }
}

impl BinRead for Float32 {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, binrw::Error> {
        Ok(Float32(reader.read_type(Endian::NATIVE)?))
    }

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        Ok(Float32(reader.read_type(endian)?))
    }
}

impl BinWrite for Float32 {
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
