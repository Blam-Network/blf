pub use blf_lib_derivable::types::*;

pub mod string;

pub mod byte_order_mark;

pub mod array;

pub mod time;

pub mod bool;

pub mod numbers;

pub mod net;

pub mod u64;

pub mod bitfield;

pub mod r#enum;

pub use r#enum::{c_enum, c_enum_within_bits};

#[macro_export]
macro_rules! c_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($tokens:tt)*
        }
    ) => {
        $(#[$meta])*
        #[derive($crate::derive::c_enum)]
        $vis enum $name {
            $($tokens)*
        }
    };
}
