use std::ffi::c_char;
use std::fmt::{Display, Formatter, Write};
use blf_lib::types::array::StaticArray;
use serde::{Deserializer, Serialize, Serializer};
use widestring::U16CString;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::cmp::min;
use std::error::Error;
use binrw::{BinRead, BinWrite};

#[cfg(feature = "napi")]
use napi::sys::{napi_env, napi_env__, napi_value};
#[cfg(feature = "napi")]
use napi::bindgen_prelude::{FromNapiMutRef, FromNapiValue, ToNapiValue, TypeName, ValidateNapiValue};
#[cfg(feature = "napi")]
use napi::{Env, JsString, ValueType};
use napi::JsValue;
#[cfg(feature = "napi")]
use napi_derive::napi;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::describe::WasmDescribe;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib::{SERDE_DESERIALIZE_RESULT, SERDE_SERIALIZE_RESULT};

pub fn to_string(chars: &[c_char]) -> BLFLibResult<String> {
    let mut res = String::new();
    for char in chars {
        let copy: u8 = *char as u8;
        if copy == 0 {
            break;
        }
        res.write_char(char::from(copy))?;
    }
    Ok(res)
}

pub fn from_string_with_length(string: String, length: usize) -> Result<Vec<c_char>, Box<dyn Error>> {
    let mut vec = from_string(string)?;

    vec.resize(length, 0);

    Ok(vec)
}

pub fn from_string(string: String) -> Result<Vec<c_char>, Box<dyn Error>> {
    let mut vec = Vec::new();

    let bytes = string.as_bytes();

    if string.len() != bytes.len() {
        return Err("Invalid string.".into());
    }

    for i in 0..bytes.len() {
        vec.push(bytes[i] as c_char);
    }

    Ok(vec)
}

#[derive(PartialEq, Debug, Clone, Default, BinRead, BinWrite)]
pub struct StaticWcharString<const N: usize> {
    buf: StaticArray<u16, N>,
}

impl<const N: usize> Display for StaticWcharString<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.get_string().fmt(f)
    }
}

#[cfg(feature = "napi")]
impl<const N: usize> ToNapiValue for StaticWcharString<N> {
    unsafe fn to_napi_value(env: *mut napi_env__, val: Self) -> napi::Result<napi::sys::napi_value> {
        let s = val.get_string();
        Ok(Env::from_raw(env).create_string(&s)?.raw())
    }
}

#[cfg(feature = "napi")]
impl<const N: usize> FromNapiValue for StaticWcharString<N> {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let js_string = JsString::from_napi_value(env, napi_val)?;
        let rust_string = js_string.into_utf8()?.into_owned()?;
        StaticWcharString::from_string(&rust_string)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}

impl<const N: usize> StaticWcharString<N> {
    pub fn from_string(value: &String) -> BLFLibResult<Self> {
        let mut new = Self {
            buf: StaticArray::default()
        };
        new.set_string(value)?;

        Ok(new)
    }

    pub fn set_string(&mut self, value: &String) -> BLFLibResult {
        let u16Str = U16CString::from_str(value).map_err(|e|e.to_string())?;
        let u16s = u16Str.as_slice();
        if u16s.len() > N {
            return Err(format!("String too long ({} > {}) bytes", N, u16s.len()).into());
        }
        let buf = self.buf.get_mut();
        buf.fill(0);
        buf[0..u16s.len()].copy_from_slice(u16s);
        Ok(())
    }

    pub fn set_string_trimmed(&mut self, value: &String) -> BLFLibResult {
        let u16Str = U16CString::from_str(value).map_err(|e|e.to_string())?;
        let u16s = u16Str.as_slice();
        let buf = self.buf.get_mut();
        buf.fill(0);
        buf[0..min(u16s.len(), N - 1)].copy_from_slice(u16s);

        Ok(())
    }

    pub unsafe fn set_string_trimmed_unchecked(&mut self, value: &String) {
        let u16Str = U16CString::from_str_unchecked(value);
        let u16s = u16Str.as_slice();
        let buf = self.buf.get_mut();
        buf.fill(0);
        buf[0..min(u16s.len(), N - 1)].copy_from_slice(u16s);
    }

    pub fn get_string(&self) -> String {
         decode_utf16(self.buf.get().iter().cloned())
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .take_while(|&c| c != '\u{0000}')
            .collect::<String>()
    }
}

impl<const N: usize> Serialize for StaticWcharString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.get_string().to_string())
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for StaticWcharString<N> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        SERDE_DESERIALIZE_RESULT!(Self::from_string(&s))
    }
}

#[derive(PartialEq, Debug, Clone, Copy, BinRead, BinWrite)]
pub struct StaticString<const N: usize> {
    buf: [u8; N],
}

impl<const N: usize> StaticString<N> {
    pub fn is_empty(&self) -> bool {
        self.buf[0] == 0x00
    }

    pub fn from_string(value: impl Into<String>) -> BLFLibResult<Self> {
        let mut new = Self {
            buf: [0; N],
        };

        new.set_string(&value.into()).map(|_| new)
    }

    pub fn from_string_trimmed(value: impl Into<String>) -> Self {
        let mut new = Self {
            buf: [0; N],
        };

        new.set_string_trimmed(&value.into());

        new
    }

    pub fn set_string(&mut self, value: &String) -> BLFLibResult {
        let mut bytes = value.as_bytes();
        // if a null termination was provided at the end, chop it off
        if !bytes.is_empty() && bytes[bytes.len() - 1] == 0 {
            bytes = &bytes[0..bytes.len() - 1];
        }
        if bytes.len() > N {
            return Err(format!("String \"{value}\" too long ({} > {}) bytes", bytes.len(), N).into());
        }
        self.buf.fill(0);
        self.buf[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn set_string_trimmed(&mut self, value: &String) {
        let mut bytes = value.as_bytes();
        // if a null termination was provided at the end, chop it off
        if !bytes.is_empty() && bytes[bytes.len() - 1] == 0 {
            bytes = &bytes[0..bytes.len() - 1];
        }
        self.buf.fill(0);
        self.buf[..min(bytes.len(), N - 1)].copy_from_slice(bytes);
    }

    pub fn get_string(&self) -> BLFLibResult<String> {
        let null_index = self.buf.iter().position(|c|c == &0u8).unwrap_or(N);
        Ok(String::from_utf8(self.buf.as_slice()[0..null_index].to_vec())?)
    }

    pub unsafe fn get_string_unchecked(&self) -> String {
        let null_index = self.buf.iter().position(|c|c == &0u8).unwrap_or(N);
        String::from_utf8_unchecked(self.buf.as_slice()[0..null_index].to_vec())
    }
}

impl<const N: usize> Default for StaticString<N>  {
    fn default() -> Self {
        Self{
            buf: [0; N],
        }
    }
}

impl<const N: usize> Serialize for StaticString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&SERDE_SERIALIZE_RESULT!(self.get_string())?.to_string())
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for StaticString<N> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        SERDE_DESERIALIZE_RESULT!(Self::from_string(&s))
    }
}

#[cfg(feature = "napi")]
impl<const N: usize> FromNapiValue for StaticString<N> {
    unsafe fn from_napi_value(env: *mut napi_env__, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let js_string = JsString::from_napi_value(env, napi_val)?;
        Ok(
            StaticString::from_string(
            js_string
                .into_utf8()?
                .as_str()?)
                .map_err(|e|napi::Error::from_reason(e.to_string()))?
        )
    }
}

#[cfg(feature = "napi")]
impl<const N: usize> ToNapiValue for StaticString<N> {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        let s = val.get_string()
            .map_err(|e|napi::Error::from_reason(e.to_string()))?;
        Env::from_raw(env).create_string(&s)
            .map(|js_str| js_str.raw())
    }
}

#[cfg(feature = "napi")]
impl<const N: usize> napi::bindgen_prelude::FromNapiMutRef for StaticString<N> {
    unsafe fn from_napi_mut_ref(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<&'static mut Self> {
        let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val);
            match c {
                napi::sys::Status::napi_ok => Ok(()),
                _ => Err(napi::Error::new(
                    napi::Status::from(c),
                    format!(
                            "Failed to recover `{}` type from napi value",
                            "StaticString",
                   ),
                )),
            }
        }?;
        Ok(&mut *(wrapped_val as *mut StaticString<N>))
    }
}


#[cfg(feature = "napi")]
impl<const N: usize> ToNapiValue for &mut StaticString<N> {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        let s = val.get_string()
            .map_err(|e|napi::Error::from_reason(e.to_string()))?;
        Env::from_raw(env).create_string(&s).map(|js_str| js_str.raw())
    }
}


// TODO: Refactor
#[cfg(feature = "napi")]
impl<const N: usize> napi::bindgen_prelude::FromNapiMutRef for StaticWcharString<N> {
    unsafe fn from_napi_mut_ref(
        env: napi::bindgen_prelude::sys::napi_env,
        napi_val: napi::bindgen_prelude::sys::napi_value,
    ) -> napi::bindgen_prelude::Result<&'static mut Self> {
        let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val);
            match c {
                napi::sys::Status::napi_ok => Ok(()),
                _ => Err(napi::Error::new(
                    napi::Status::from(c),
                    format!(
                        "Failed to recover `{}` type from napi value",
                        "StaticString",
                    ),
                )),
            }
        }?;
        Ok(&mut *(wrapped_val as *mut StaticWcharString<N>))
    }
}


#[cfg(feature = "napi")]
impl<const N: usize> ToNapiValue for &mut StaticWcharString<N> {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        String::to_napi_value(env, val.get_string())
    }
}

// TODO: Remove
#[cfg(feature = "napi")]
impl<const N: usize> TypeName for StaticString<N> {
    fn type_name() -> &'static str {
        todo!()
    }

    fn value_type() -> ValueType {
        todo!()
    }
}

// TODO: Remove?
#[cfg(feature = "napi")]
impl<const N: usize> ValidateNapiValue for StaticString<N> {

}

impl<const N: usize> WasmDescribe for StaticString<N> {
    fn describe() {
        String::describe()
    }
}

impl<const N: usize> IntoWasmAbi for StaticString<N> {
    type Abi = <String as IntoWasmAbi>::Abi;
    fn into_abi(self) -> Self::Abi {
        unsafe { self.get_string_unchecked().into_abi() }
    }
}

impl<const N: usize> FromWasmAbi for StaticString<N> {
    type Abi = <String as IntoWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        let mut res = Self {buf: [0;N]};
        res.set_string_trimmed(&String::from_abi(js));
        res
    }
}

impl<const N: usize> WasmDescribe for StaticWcharString<N> {
    fn describe() {
        String::describe()
    }
}

impl<const N: usize> IntoWasmAbi for StaticWcharString<N> {
    type Abi = <String as IntoWasmAbi>::Abi;
    fn into_abi(self) -> Self::Abi {
        self.get_string().into_abi()
    }
}

impl<const N: usize> FromWasmAbi for StaticWcharString<N> {
    type Abi = <String as IntoWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        let mut res = Self {buf: StaticArray::default()};
        res.set_string_trimmed_unchecked(&String::from_abi(js));
        res
    }
}