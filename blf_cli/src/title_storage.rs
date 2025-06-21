use std::fs::exists;
use blf_lib::blf::chunks::DynTitleAndBuild;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::v12070_08_09_05_2031_halo3_ship;
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::v13895_09_04_27_2201_atlas_release;

pub mod halo3;
pub mod halo3odst;

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

fn get_title_converters() -> Vec<Box<dyn TitleConverter>> {
    vec![
        Box::new(v12070_08_09_05_2031_halo3_ship::default()),
        Box::new(v13895_09_04_27_2201_atlas_release::default())
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

pub fn check_file_exists(path: &String) -> bool {
    exists(path).is_ok_and(|res| res)
}