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
                        $field: ((value >> i) & 1) != 0,
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
