use std::process;

use blf_lib::blf::versions::haloreach::k_title_haloreach;

use crate::title_storage::haloreach::v08516_10_02_19_1607_omaha_alpha;
use crate::title_storage::haloreach::v08516_10_02_19_1607_omaha_alpha::k_build_string_omaha_alpha_08516;
use crate::title_storage::haloreach::v09730_10_04_09_1309_omaha_delta;
use crate::title_storage::haloreach::v09730_10_04_09_1309_omaha_delta::k_build_string_omaha_delta_09730;
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual;
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::k_build_string_tu1actual_12065;

pub fn build_megalo(
    json_input_folder: String,
    mglo_output_folder: String,
    title: String,
    version: String,
) {
    let result = match (title.as_str(), version.as_str()) {
        (k_title_haloreach, k_build_string_omaha_alpha_08516) => {
            v08516_10_02_19_1607_omaha_alpha::megalo_builder::build_megalo(
                &json_input_folder,
                &mglo_output_folder,
            )
        }
        (k_title_haloreach, k_build_string_omaha_delta_09730)
        | (k_title_haloreach, "09449.10.03.25.1545.omaha_beta")
        | (k_title_haloreach, "09664.10.04.06.2121.omaha_beta") => {
            v09730_10_04_09_1309_omaha_delta::megalo_builder::build_megalo(
                &json_input_folder,
                &mglo_output_folder,
            )
        }
        (k_title_haloreach, k_build_string_tu1actual_12065)
        | (k_title_haloreach, "11860.10.07.24.0147.omaha_release") => {
            v12065_11_08_24_1738_tu1actual::megalo_builder::build_megalo(
                &json_input_folder,
                &mglo_output_folder,
            )
        }
        _ => {
            eprintln!("Unsupported title or version for build-megalo: {title}, {version}");
            process::exit(1);
        }
    };

    if let Err(err) = result {
        eprintln!("build-megalo failed: {err}");
        process::exit(1);
    }
}
