use std::io::{Read, Seek, Write};
use std::ops::Index;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "napi")]
use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
#[cfg(feature = "napi")]
use napi::sys;
#[cfg(feature = "napi")]
use napi::sys::{napi_env};
use blf_lib::result::BLFLibResult;
use crate::SERDE_DESERIALIZE_RESULT;

#[derive(PartialEq, Debug)]
pub struct StaticArray<E: 'static, const N: usize> {
    _data: Vec<E> // 1984
}

impl<'a, E: BinRead<Args<'a> = ()> + 'static, const N: usize> BinRead for StaticArray<E, N> {
    type Args<'b> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        let mut data = Vec::with_capacity(N);

        // Manually read each element in the array with options
        for _ in 0..N {
            let element = E::read_options(reader, endian, ())?; // Read each element using BinRead for E with options
            data.push(element);
        }

        Ok(StaticArray { _data: data })
    }
}

impl<'a, E: BinWrite<Args<'a> = ()> + 'static, const N: usize> BinWrite for StaticArray<E, N> {
    type Args<'b> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        // Manually write each element in the array with options
        for item in &self._data {
            item.write_options(writer, endian, ())?; // Write each element using BinWrite for E with options
        }
        Ok(())
    }
}

impl<E: Clone, const N: usize> Clone for StaticArray<E, N> {
    fn clone(&self) -> Self {
        StaticArray {
            _data: self._data.clone(), // Clone the vector
        }
    }
}

impl<E: Default + Clone, const N: usize> StaticArray<E, N> {
    pub fn get(&self) -> &Vec<E> {
         &self._data
    }

    pub fn get_mut(&mut self) -> &mut Vec<E> {
        &mut self._data
    }

    pub fn from_slice(slice: &[E]) -> Result<Self, String> {
        if slice.len() != N {
            return Err(format!("Expected {N} elements but got {}.", slice.len()));
        }
        let mut new = Self {
            _data: vec![E::default(); N],
        };
        new._data.clone_from_slice(slice);
        Ok(new)
    }

    pub fn from_vec(vec: &Vec<E>) -> BLFLibResult<Self> {
        if vec.len() > N {
            return Err(format!("Expected {N} elements but got {}.", vec.len()).into());
        }
        let mut new = Self::default();
        new._data.clone_from(vec);
        new._data.extend_from_slice(&vec![E::default(); N - vec.len()]);
        Ok(new)
    }
}

impl<E: Default + Clone, const N: usize> Default for StaticArray<E, N> {
    fn default() -> Self {
        Self {
            _data: vec![E::default(); N]
        }
    }
}

impl<E: Clone, const N: usize> Index<usize> for StaticArray<E, N> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self._data[index]
    }
}

impl<E: Serialize + Clone + Default + PartialEq, const N: usize> Serialize for StaticArray<E, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut populated_size = 0;
        let default = E::default();
        self._data.iter().enumerate().for_each(|(index, item)| if *item != default { populated_size = index + 1; });

        let filtered = &self._data
            .as_slice()[0..populated_size];

        filtered.serialize(serializer)
    }
}

impl<'de, E: Deserialize<'de>+ Clone + Default, const N: usize> serde::Deserialize<'de> for StaticArray<E, N> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let vec: Vec<E> = Deserialize::deserialize(d)?;
        SERDE_DESERIALIZE_RESULT!(Self::from_vec(&vec))
    }
}

#[cfg(feature = "napi")]
impl<E: ToNapiValue, const N: usize> ToNapiValue for StaticArray<E, N> {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi::Result<napi::sys::napi_value> {
        let vec: Vec<E> = val._data;
        <Vec<E>>::to_napi_value(env, vec)
    }
}

#[cfg(feature = "napi")]
impl<E: FromNapiValue + Default + Clone, const N: usize> FromNapiValue for StaticArray<E, N> {
    unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> napi::Result<Self> {
        Self::from_vec(&Vec::<E>::from_napi_value(env, napi_val)?).map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}