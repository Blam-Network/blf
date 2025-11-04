use std::fmt::Display;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[cfg(feature = "napi")]
use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
#[cfg(feature = "napi")]
use napi::sys::{napi_env, napi_value};
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::describe::WasmDescribe;
use blf_lib::types::u64::Unsigned64;

#[derive(Default, Clone, Debug, PartialEq, BinRead, BinWrite, Copy)]
pub struct time64_t(pub u64);

impl Display for time64_t {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single();
        if datetime.is_none() {
            return write!(f, "<Invalid Timestamp>");
        }
        let datetime = datetime.unwrap();
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(f, "{formatted}")
    }
}

impl Serialize for time64_t {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single()
            .ok_or_else(|| serde::ser::Error::custom("Invalid timestamp".to_string()))?;
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for time64_t {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(time64_t(datetime.and_utc().timestamp() as u64))
    }
}

impl From<time64_t> for u64 {
    fn from(val: time64_t) -> Self {
        val.0
    }
}

impl From<u64> for time64_t {
    fn from(t: u64) -> Self {
        Self(t)
    }
}

impl From<Unsigned64> for time64_t {
    fn from(t: Unsigned64) -> Self {
        Self(t.into())
    }
}

impl From<time64_t> for DateTime<Utc> {
    fn from(val: time64_t) -> Self {
        Utc.timestamp(val.0 as i64, 0)
    }
}

#[derive(Default, Clone, Debug, PartialEq, BinRead, BinWrite, Copy)]
pub struct time32_t(pub u32);

impl Display for time32_t {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single();
        if datetime.is_none() {
            return write!(f, "<Invalid Timestamp>");
        }
        let datetime = datetime.unwrap();
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(f, "{formatted}")
    }
}

impl Serialize for time32_t {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single()
            .ok_or_else(|| serde::ser::Error::custom("Invalid timestamp"))?;
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for time32_t {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(time32_t(datetime.and_utc().timestamp() as u32))
    }
}

impl From<time32_t> for u32 {
    fn from(val: time32_t) -> Self {
        val.0
    }
}

impl From<time32_t> for DateTime<Utc> {
    fn from(val: time32_t) -> Self {
        Utc.timestamp(val.0 as i64, 0)
    }
}

impl From<time32_t> for NaiveDateTime {
    fn from(val: time32_t) -> Self {
        NaiveDateTime::from_timestamp(val.0 as i64, 0)
    }
}

impl From<time64_t> for NaiveDateTime {
    fn from(val: time64_t) -> Self {
        NaiveDateTime::from_timestamp(val.0 as i64, 0)
    }
}

impl From<u32> for time32_t {
    fn from(t: u32) -> Self {
        Self(t)
    }
}

#[derive(Default, Clone, Debug, PartialEq, BinRead, BinWrite)]
pub struct filetime(u64);

impl filetime {
    // FILETIME to UNIX epoch conversion constant: difference in seconds between 1601 and 1970
    const FILETIME_EPOCH_OFFSET: u64 = 11644473600;

    // Converts FILETIME to time_t (in seconds since 1970)
    pub fn to_time_t(&self) -> u64 {
        if self.0 == 0 { return 0 }
        (self.0 / 10_000_000).checked_sub(Self::FILETIME_EPOCH_OFFSET).unwrap_or(0)
    }

    // Converts time_t (in seconds since 1970) to FILETIME (in 100-nanosecond intervals since 1601)
    pub fn from_time_t(t: u64) -> Self {
        Self((t + Self::FILETIME_EPOCH_OFFSET) * 10_000_000)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

}

impl Serialize for filetime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0 == 0 {
            return serializer.serialize_str("None")
        }

        let seconds_since_unix_epoch = self.to_time_t();
        let datetime = Utc.timestamp_opt(seconds_since_unix_epoch as i64, 0)
            .single()
            .ok_or_else(|| serde::ser::Error::custom("Invalid timestamp"))?;
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for filetime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        if s == "None" {
            return Ok(Self(0))
        }

        let datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(Self::from(datetime.and_utc().timestamp() as u64))
    }
}

impl From<u64> for filetime {
    fn from(t: u64) -> Self {
        Self(t.into())
    }
}

#[cfg(feature = "napi")]
impl ToNapiValue for time32_t {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        NaiveDateTime::to_napi_value(env, val.into())
    }
}

#[cfg(feature = "napi")]
impl FromNapiValue for time32_t {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        Ok(Self(NaiveDateTime::from_napi_value(env, napi_val)?.and_utc().timestamp() as u32))
    }
}

#[cfg(feature = "napi")]
impl ToNapiValue for time64_t {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        NaiveDateTime::to_napi_value(env, val.into())
    }
}

#[cfg(feature = "napi")]
impl FromNapiValue for time64_t {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        Ok(Self(NaiveDateTime::from_napi_value(env, napi_val)?.and_utc().timestamp() as u64))
    }
}

impl WasmDescribe for time32_t {
    fn describe() {
        u32::describe()
    }
}

impl IntoWasmAbi for time32_t {
    type Abi = <u32 as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        // TODO: Date obj
       u32::into_abi(self.0)
    }
}

impl WasmDescribe for time64_t {
    fn describe() {
        u64::describe()
    }
}

impl IntoWasmAbi for time64_t {
    type Abi = <u64 as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        // TODO: Date obj
        u64::into_abi(self.0)
    }
}

impl FromWasmAbi for time64_t {
    type Abi = <u64 as FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        time64_t::from(u64::from_abi(js))
    }
}