#[macro_export]
macro_rules! bitfield {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident : $ty:ty {
            $($field:ident),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        $vis struct $name {
            $(pub $field: bool),+
        }

        impl $name {
            #[inline]
            pub fn to_raw(&self) -> $ty {
                let mut bits: $ty = 0;
                let mut i = 0;
                $(
                    if self.$field {
                        bits |= (1 as $ty) << i;
                    }
                    i += 1;
                )+
                bits
            }

            #[inline]
            pub fn from_raw(value: $ty) -> Self {
                let mut i = 0;
                Self {
                    $(
                        $field: {
                            let bit = ((value >> i) & 1) != 0;
                            i += 1;
                            bit
                        },
                    )+
                }
            }

            /// The number of bits used (for diagnostics)
            pub const fn bit_count() -> usize {
                let mut count = 0;
                $(
                    let _ = stringify!($field);
                    count += 1;
                )+
                count
            }
        }

        impl binrw::BinRead for $name {
            type Args<'a> = ();

            fn read_options<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                endian: binrw::Endian,
                _args: Self::Args<'_>,
            ) -> binrw::BinResult<Self> {
                let raw: $ty = <$ty>::read_options(reader, endian, ())?;
                Ok(Self::from_raw(raw))
            }
        }

        impl binrw::BinWrite for $name {
            type Args<'a> = ();

            fn write_options<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
                endian: binrw::Endian,
                _args: Self::Args<'_>,
            ) -> binrw::BinResult<()> {
                let raw = self.to_raw();
                raw.write_options(writer, endian, ())
            }
        }
    };
}

/// Like [`bitfield!`], but reads/writes `u32` words on the bitstream (32 bits at a time).
#[macro_export]
macro_rules! big_bitfield {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field:ident),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        $vis struct $name {
            $(pub $field: bool),+
        }

        impl $name {
            pub const fn bit_count() -> usize {
                let mut count = 0;
                $(
                    let _ = stringify!($field);
                    count += 1;
                )+
                count
            }

            pub const fn word_count() -> usize {
                (Self::bit_count() + 31) / 32
            }

            pub const fn padded_bit_count() -> usize {
                Self::word_count() * 32
            }

            fn last_word_mask() -> u32 {
                let remainder = Self::bit_count() % 32;
                if remainder == 0 {
                    u32::MAX
                } else {
                    (1u32 << remainder) - 1
                }
            }

            fn pack_words(&self) -> Vec<u32> {
                let mut words = vec![0u32; Self::word_count()];
                let mut index = 0;
                $(
                    if self.$field {
                        words[index / 32] |= 1 << (index % 32);
                    }
                    index += 1;
                )+
                if let Some(last) = words.last_mut() {
                    *last &= Self::last_word_mask();
                }
                words
            }

            fn unpack_words(words: &[u32]) -> Self {
                let mut value = Self::default();
                let mut index = 0;
                $(
                    value.$field = words.get(index / 32).map_or(false, |word| {
                        (word >> (index % 32)) & 1 != 0
                    });
                    index += 1;
                )+
                value
            }

            pub fn encode(
                &self,
                bitstream: &mut $crate::io::bitstream::c_bitstream_writer,
                name: &str,
            ) -> blf_lib_derivable::result::BLFLibResult {
                for word in self.pack_words() {
                    bitstream.write_integer(word, 32)?;
                    let _ = name;
                }
                Ok(())
            }

            pub fn decode(
                &mut self,
                bitstream: &mut $crate::io::bitstream::c_bitstream_reader,
                name: &str,
            ) -> blf_lib_derivable::result::BLFLibResult {
                let mut words = vec![0u32; Self::word_count()];
                for word in &mut words {
                    *word = bitstream.read_integer(name, 32)?;
                }
                if let Some(last) = words.last_mut() {
                    *last &= Self::last_word_mask();
                }
                *self = Self::unpack_words(&words);
                Ok(())
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(None)?;
                $(
                    map.serialize_entry(stringify!($field), &self.$field)?;
                )+
                map.end()
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                use serde::de::{MapAccess, Visitor};

                struct MapVisitor;

                impl<'de> Visitor<'de> for MapVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a map of bitfield flag names to booleans")
                    }

                    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                        let mut value = $name::default();
                        while let Some(key) = map.next_key::<String>()? {
                            match key.as_str() {
                                $(stringify!($field) => value.$field = map.next_value()?,)+
                                _ => {
                                    let _: serde::de::IgnoredAny = map.next_value()?;
                                }
                            }
                        }
                        Ok(value)
                    }
                }

                deserializer.deserialize_map(MapVisitor)
            }
        }
    };
}
