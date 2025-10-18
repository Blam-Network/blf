use std::fs::{exists, read};
use std::path::Path;
use filesize::PathExt;
use blf_lib::blf::chunks::DynTitleAndBuild;
use blf_lib::result::{BLFLibError, BLFLibResult};
use crate::title_storage::ares::v_untracked_ares::v_untracked_ares;
use crate::title_storage::halo3::v08117_07_03_07_1702_delta::v08117_07_03_07_1702_delta;
use crate::title_storage::halo3::v08172_07_03_08_2240_delta::v08172_07_03_08_2240_delta;
use crate::title_storage::halo3::v10015_07_05_14_2217_delta::v10015_07_05_14_2217_delta;
use crate::title_storage::halo3::v11856_07_08_20_2332_release::v11856_07_08_20_2332_release;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::v12070_08_09_05_2031_halo3_ship;
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::v13895_09_04_27_2201_atlas_release;
use crate::title_storage::haloonline::v1_106708_cert_ms23___release::v1_106708_cert_ms23___release;
use crate::title_storage::haloreach::v09449_10_03_25_1545_omaha_beta::v09449_10_03_25_1545_omaha_beta;
use crate::title_storage::haloreach::v09730_10_04_09_1309_omaha_delta::v09730_10_04_09_1309_omaha_delta;
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::v12065_11_08_24_1738_tu1actual;
use crate::title_storage::haloreach::v08516_10_02_19_1607_omaha_alpha::v08516_10_02_19_1607_omaha_alpha;

pub mod halo3;
pub mod halo3odst;
pub mod haloreach;
pub mod ares;
pub mod haloonline;

pub trait TitleConverter: DynTitleAndBuild {
    fn build_blfs(&mut self, config_path: &String, blfs_path: &String);
    fn build_config(&mut self, blfs_path: &String, config_path: &String);
}

#[macro_export]
macro_rules! title_converter {
    ($i:item) => {
        #[derive(blf_lib::derive::TitleAndBuild, Default)]
        $i
    }
}

// These are the titles we support.
fn get_title_converters() -> Vec<Box<dyn TitleConverter>> {
    vec![
        // Releases
        Box::new(v12070_08_09_05_2031_halo3_ship::default()),       // Halo 3 (TU2)
        Box::new(v13895_09_04_27_2201_atlas_release::default()),    // Halo 3: ODST
        Box::new(v12065_11_08_24_1738_tu1actual::default()),        // Halo: Reach (TU1)

        // Pre-Releases
        Box::new(v11856_07_08_20_2332_release::default()),          // Halo 3 (Epsilon)
        Box::new(v10015_07_05_14_2217_delta::default()),            // Halo 3 (Beta TU1)
        Box::new(v08172_07_03_08_2240_delta::default()),            // Halo 3 (Pre-Release, March 8th 2007)
        Box::new(v08117_07_03_07_1702_delta::default()),            // Halo 3 (Pre-Release, March 7th 2007)
        Box::new(v09730_10_04_09_1309_omaha_delta::default()),      // Halo: Reach (Public Beta)
        Box::new(v09449_10_03_25_1545_omaha_beta::default()),       // Halo: Reach (Private Beta)
        Box::new(v08516_10_02_19_1607_omaha_alpha::default()),      // Halo: Reach (Private Alpha)

        // Mods & Others
        Box::new(v_untracked_ares::default()),                      //
        Box::new(v1_106708_cert_ms23___release::default()),         // Halo: Online (ms23)
    ]
}

pub fn get_title_converter (title: impl Into<String>, build: impl Into<String>) -> Option<Box<dyn TitleConverter>> {
    let title = title.into();
    let build = build.into();

    get_title_converters()
        .into_iter()
        .find(|title_converter| 
            title_converter.title() == title && title_converter.build_string() == build
        )
}

#[deprecated]
pub fn check_file_exists(path: &String) -> bool {
    exists(path).unwrap_or(false)
}

// Used for validating images used in Title Storage.
pub fn validate_jpeg(path: impl Into<String>, width: usize, height: usize, max_filesize: Option<usize>) -> BLFLibResult {
    let path = path.into();
    let path = Path::new(&path);

    if !exists(path)? {
        return Err(BLFLibError::from("Does not exist."))
    }

    if let Some(max_filesize) = max_filesize {
        let image_filesize = path.size_on_disk()?;
        if image_filesize > max_filesize as u64 {
            return Err(format!("Image file size is too large ({}B > {}B)", image_filesize, max_filesize).into());
        }
    }

    let jpeg_data = read(path)?;
    let mut decoder = jpeg_decoder::Decoder::new(jpeg_data.as_slice());
    decoder.read_info().map_err(|e|e.to_string())?;
    let header = decoder.info().unwrap();

    if header.width != width as u16 {
        return Err(BLFLibError::from(format!("Invalid image width ({}px != {}px)", header.width, width)));
    }
    if header.height != height as u16 {
        return Err(BLFLibError::from(format!("Invalid image height ({}px != {}px)", header.height, height)));
    }

    Ok(())
}