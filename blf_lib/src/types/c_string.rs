use std::ffi::c_char;
use std::fmt::Write;
use blf_lib::types::array::StaticArray;
use serde::{Deserializer, Serialize, Serializer};
use widestring::U16CString;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use binrw::{BinRead, BinWrite};
use serde::de::Error;

#[cfg(feature = "napi")]
use napi::sys::{napi_env, napi_env__, napi_value};
#[cfg(feature = "napi")]
use napi::bindgen_prelude::{FromNapiMutRef, FromNapiValue, ToNapiValue, TypeName, ValidateNapiValue};
#[cfg(feature = "napi")]
use napi::{Env, JsString, NapiRaw, ValueType};
#[cfg(feature = "napi")]
use napi_derive::napi;

pub fn to_string(chars: &[c_char]) -> String {
    let mut res = String::new();
    for char in chars {
        let copy: u8 = *char as u8;
        if copy == 0 {
            break;
        }
        res.write_char(char::from(copy)).unwrap();
    }
    res
}

pub fn from_string_with_length(string: String, length: usize) -> Vec<c_char> {
    let mut vec = from_string(string);

    vec.resize(length, 0);

    vec
}

pub fn from_string(string: String) -> Vec<c_char> {
    let mut vec = Vec::new();

    let bytes = string.as_bytes();

    if string.len() != bytes.len() {
        panic!("Invalid string.");
    }

    for i in 0..bytes.len() {
        vec.push(bytes[i] as c_char);
    }

    vec
}

#[derive(PartialEq, Debug, Clone, Default, BinRead, BinWrite)]
pub struct StaticWcharString<const N: usize> {
    buf: StaticArray<u16, N>,
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
            .map_err(|e| napi::Error::from_reason(e))
    }
}




impl<const N: usize> StaticWcharString<N> {
    pub fn from_string(value: &String) -> Result<Self, String> {
        let mut new = Self {
            buf: StaticArray::default()
        };

        let result = new.set_string(value);
        if result.is_ok() { Ok(new) }
        else { Err(result.unwrap_err()) }
    }

    pub fn set_string(&mut self, value: &String) -> Result<(), String> {
        let u16Str = U16CString::from_str(value).unwrap();
        let u16s = u16Str.as_slice();
        if u16s.len() > N {
            return Err(format!("String too long ({} > {}) bytes", N, u16s.len()));
        }
        let buf = self.buf.get_mut();
        buf.fill(0);
        buf[0..u16s.len()].copy_from_slice(u16s);
        Ok(())
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
        let res = Self::from_string(&s);
        if res.is_err() {
            Err(D::Error::custom(res.unwrap_err()))
        } else {
            Ok(res.unwrap())
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, BinRead, BinWrite)]
pub struct StaticString<const N: usize> {
    buf: [u8; N],
}



impl<const N: usize> StaticString<N> {
    pub fn from_string(value: impl Into<String>) -> Result<Self, String> {
        let mut new = Self {
            buf: [0; N],
        };

        let result = new.set_string(&value.into());
        if result.is_ok() { Ok(new) }
        else { Err(result.unwrap_err()) }
    }

    pub fn set_string(&mut self, value: &String) -> Result<(), String> {
        let mut bytes = value.as_bytes();
        // if a null termination was provided at the end, chop it off
        if !bytes.is_empty() && bytes[bytes.len() - 1] == 0 {
            bytes = &bytes[0..bytes.len() - 1];
        }
        if bytes.len() > N {
            return Err(format!("String \"{value}\" too long ({} > {}) bytes", N, bytes.len()));
        }
        self.buf.fill(0);
        self.buf[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn get_string(&self) -> String {
        let null_index = self.buf.iter().position(|c|c == &0u8).unwrap_or(N);
        String::from_utf8(self.buf.as_slice()[0..null_index].to_vec()).unwrap()
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
        serializer.serialize_str(&self.get_string().to_string())
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for StaticString<N> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let res = Self::from_string(&s);
        if res.is_err() {
            Err(D::Error::custom(res.unwrap_err()))
        } else {
            Ok(res.unwrap())
        }
    }
}

#[cfg(feature = "napi")]
impl<const N: usize> FromNapiValue for StaticString<N> {
    unsafe fn from_napi_value(env: *mut napi_env__, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let js_string = JsString::from_napi_value(env, napi_val)?;
        Ok(StaticString::from_string(js_string.into_utf8()?.as_str()?).unwrap())
    }
}

#[cfg(feature = "napi")]
impl<const N: usize> ToNapiValue for StaticString<N> {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi_value> {
        let s = val.get_string(); // Assuming this returns a `&str`
        Env::from_raw(env).create_string(&s).map(|js_str| js_str.raw()) // Convert string to JsString and return raw napi valu    }
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
            let c = (napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val));
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
        let s = val.get_string(); // Assuming this returns a `&str`
        Env::from_raw(env).create_string(&s).map(|js_str| js_str.raw()) // Convert string to JsString and return raw napi valu    }
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
            let c = (napi::bindgen_prelude::sys::napi_unwrap(env, napi_val, &mut wrapped_val));
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