#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{command, Parser};
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::chunks::{find_chunk_in_file, search_for_chunk_in_file};
use blf_lib::blf::versions::haloreach::v09730_10_04_09_1309_omaha_delta::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_hopper_configuration_table, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::{s_blf_chunk_content_header, s_blf_chunk_game_set, s_blf_chunk_megalo_categories, s_blf_chunk_nag_message, s_blf_chunk_predefined_queries};
use blf_lib::blf::versions::v12065_11_08_24_1738_tu1actual;
use blf_lib::io::bitstream::e_bitstream_byte_order;
use blf_lib::io::write_json_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::commands::Commands;
use crate::commands::Commands::{ConvertH3MCCMapVariants, TitleStorage};
use crate::commands::convert_halo3mcc_map_variants::convert_halo3mcc_map_variants;
use crate::commands::dump_film_data::dump_film_data;
use crate::commands::import_rsa_signatures::import_rsa_signatures;
use crate::commands::import_variant::import_variant;
use crate::commands::export_variant::export_variant;
use crate::commands::title_storage::TitleStorageSubcommands;
use crate::commands::unpack_screenshot::unpack_screenshot;

mod title_storage;
mod io;
mod console;
mod commands;
mod result;

#[derive(Debug, Parser)]
#[command(name = "blf_cli")]
#[command(about = "blam! engine file editor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let fpre = search_for_chunk_in_file::<s_blf_chunk_predefined_queries>(
        "/Users/codiestella/Downloads/RawGames-Halo/Halo Reach/11860.10.07.24.0147.omaha_relea/title storage/tu1_hoppers/en/file_predefined_queries.bin"
    ).unwrap().unwrap();

    BlfFileBuilder::new()
        .add_chunk(s_blf_chunk_start_of_file::new("file_predefined_queries", byte_order_mark::big_endian))
        .add_chunk(s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>())
        .add_chunk(fpre.clone())
        .add_chunk(s_blf_chunk_end_of_file::default())
        .write_file("/Users/codiestella/Desktop/file_predefined_queries.bin").unwrap();

    write_json_file(&fpre, "/Users/codiestella/Desktop/file_predefined_queries.json").unwrap();

    let args = Cli::parse();

    match args.command {
        TitleStorage(title_storage_command) => match title_storage_command.command {
            TitleStorageSubcommands::Build { config_input_path, blf_output_path, title, version } => {
                let mut title_converter =
                    title_storage::get_title_converter(&title, &version)
                        .expect(&format!("No title converter was found for the provided title and version: {title}, {version}"));

                title_converter.build_blfs(
                    &config_input_path,
                    &blf_output_path
                );
            },
            TitleStorageSubcommands::BuildConfig { blf_input_path, config_output_path, title, version } => {
                let mut title_converter =
                    title_storage::get_title_converter(title, version)
                        .expect("No title converter was found for the provided title and version.");

                title_converter.build_config(
                    &blf_input_path,
                    &config_output_path
                );
            },
            TitleStorageSubcommands::ImportRsaSignatures { hoppers_config_path, halo_maps_folder, title, version } => {
                import_rsa_signatures(hoppers_config_path, halo_maps_folder, title, version);
            },
            TitleStorageSubcommands::ImportVariant { hoppers_config_path, variant_path, title, version } => {
                import_variant(hoppers_config_path, variant_path, title, version);
            },
            TitleStorageSubcommands::ExportVariant { variant_json_path, destination_path, title, version } => {
                export_variant(variant_json_path, destination_path, title, version);
            }
        },
        ConvertH3MCCMapVariants { mcc_maps_folder, converted_maps_folder} => {
            convert_halo3mcc_map_variants(mcc_maps_folder, converted_maps_folder);
        }
        Commands::UnpackScreenshot { screenshot_path, output_path } => {
            unpack_screenshot(screenshot_path, output_path);
        }
        Commands::DumpFilmData { film_folder} => {
            dump_film_data(film_folder);
        }
    }
}