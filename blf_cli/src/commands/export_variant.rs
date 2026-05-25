use blf_lib::blf::versions::halo3::k_title_halo3;
use blf_lib::blf::versions::haloreach::k_title_haloreach;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::k_build_string_halo3_ship_12070;
use crate::title_storage::haloreach::v08516_10_02_19_1607_omaha_alpha;
use crate::title_storage::haloreach::v08516_10_02_19_1607_omaha_alpha::k_build_string_omaha_alpha_08516;
use crate::title_storage::haloreach::v09730_10_04_09_1309_omaha_delta;
use crate::title_storage::haloreach::v09730_10_04_09_1309_omaha_delta::k_build_string_omaha_delta_09730;
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual;
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::k_build_string_tu1actual_12065;

pub fn export_variant(
    hoppers_config_path: String,
    variant_path: String,
    title: String,
    version: String,
) {
    match (title.as_str(), version.as_str()) {
        (k_title_halo3, k_build_string_halo3_ship_12070) => {
            v12070_08_09_05_2031_halo3_ship::variant_exporter::export_variant(&hoppers_config_path, &variant_path);
        }
        (k_title_haloreach, k_build_string_tu1actual_12065) => {
            v12065_11_08_24_1738_tu1actual::variant_exporter::export_variant(&hoppers_config_path, &variant_path);
        }
        (k_title_haloreach, k_build_string_omaha_delta_09730)
        | (k_title_haloreach, "09449.10.03.25.1545.omaha_beta")
        | (k_title_haloreach, "09664.10.04.06.2121.omaha_beta") => {
            v09730_10_04_09_1309_omaha_delta::variant_exporter::export_variant(&hoppers_config_path, &variant_path);
        }
        (k_title_haloreach, k_build_string_omaha_alpha_08516) => {
            v08516_10_02_19_1607_omaha_alpha::variant_exporter::export_variant(&hoppers_config_path, &variant_path);
        }
        _ => {
            println!("Unsupported title or version.");
        }
    }
}