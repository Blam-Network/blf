use std::fmt::{Display, Formatter, Result};
use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Default, BinRead, BinWrite, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct build_number_identifier {
    pub build_number_version: u32,
    pub build_number: u32,
}

impl build_number_identifier {
    pub fn new(build_number_version: u32, build_number: u32) -> Self {
        build_number_identifier {
            build_number_version,
            build_number
        }
    }
}

impl Display for build_number_identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}.{}", self.build_number_version, self.build_number)
    }
}