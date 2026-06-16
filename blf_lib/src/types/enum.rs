//! `c_enum` — declaration-order index enum encoding for megalo bitstreams.
//!
//! Blam stores enum fields as a **declaration-order index**, not the Rust/C++
//! discriminant value.
//!
//! Mirrors Blam's `c_enum<enum T, storage, min, max, resolver>`.

use crate::result::{BLFLibError, BLFLibResult};

pub trait c_enum: Copy + PartialEq + std::fmt::Debug + 'static {
    /// Enum discriminant for each index, in declaration order.
    const MEMBERS: &'static [i32];

    fn from_index(index: u32) -> BLFLibResult<Self>;

    fn to_index(&self) -> BLFLibResult<u32>;

    fn from_index_named(name: &str, index: u32) -> BLFLibResult<Self> {
        Self::from_index(index).map_err(|_| {
            BLFLibError::from(format!("Unexpected enum value for {name}: {index}"))
        })
    }

    fn member_count() -> usize {
        Self::MEMBERS.len()
    }

    /// Minimum bits required to store any declaration-order index (`0..member_count`).
    fn size_in_bits() -> usize {
        let count = Self::member_count();
        if count <= 1 {
            1
        } else {
            ((count - 1) as u32).ilog2() as usize + 1
        }
    }

    fn assert_fits_in_bits(name: &str, index: u32, size_in_bits: usize) -> BLFLibResult<()> {
        let max = (1u32 << size_in_bits).saturating_sub(1);
        if index > max {
            return Err(BLFLibError::from(format!(
                "Value {index} for {name} does not fit in {size_in_bits} bits (max {max})"
            )));
        }
        Ok(())
    }
}

/// Read an index that fits in `size_in_bits` without requiring a declared member.
pub trait c_enum_within_bits: c_enum {
    fn from_bits_named(name: &str, index: u32, size_in_bits: usize) -> BLFLibResult<Self> {
        Self::assert_fits_in_bits(name, index, size_in_bits)?;
        if (index as usize) < Self::member_count() {
            Self::from_index(index)
        } else {
            Err(BLFLibError::from(format!(
                "Reserved enum index for {name}: {index}"
            )))
        }
    }
}

impl<T: c_enum> c_enum_within_bits for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, crate::derive::c_enum)]
    #[bits(2)]
    enum e_test_icon {
        none = -1,
        speaker = 0,
        num = 11,
    }

    #[test]
    fn index_maps_declaration_order_not_discriminant() {
        assert_eq!(e_test_icon::from_index(0).unwrap(), e_test_icon::none);
        assert_eq!(e_test_icon::from_index(1).unwrap(), e_test_icon::speaker);
        assert_eq!(e_test_icon::from_index(2).unwrap(), e_test_icon::num);

        assert_eq!(e_test_icon::none.to_index().unwrap(), 0);
        assert_eq!(e_test_icon::speaker.to_index().unwrap(), 1);
        assert_eq!(e_test_icon::num.to_index().unwrap(), 2);
    }

    #[test]
    fn size_in_bits_from_bits_attribute() {
        assert_eq!(<e_test_icon as c_enum>::size_in_bits(), 2);
    }
}
