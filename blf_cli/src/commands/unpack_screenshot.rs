use std::fs;
use std::path::Path;
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::chunks::{find_chunk_in_file, search_for_chunk_in_file};
use blf_lib::blf::versions::halo3::k_title_halo3;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_content_header, s_blf_chunk_end_of_file, s_blf_chunk_screenshot_camera, s_blf_chunk_screenshot_data, s_blf_chunk_start_of_file};
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::k_build_string_halo3_ship_12070;

pub fn unpack_screenshot(
    screenshot_path: String,
    output_path: Option<String>,
) {
    let header: s_blf_chunk_content_header = search_for_chunk_in_file(&screenshot_path).expect("Could not find Screenshot header.");
    let camera: s_blf_chunk_screenshot_camera = search_for_chunk_in_file(&screenshot_path).expect("Could not find Screenshot camera.");
    let data: s_blf_chunk_screenshot_data = search_for_chunk_in_file(&screenshot_path).expect("Could not find Screenshot data.");

    println!("{} by {} ({:X})", header.metadata.name.get_string(), header.metadata.author.get_string(), header.metadata.author_id);
    println!("Taken on {}", header.metadata.date);
    println!("Description: \"{}\"", header.metadata.description.get_string());
    println!("--- File Data ---");
    println!("Halo Version: {}", header.build_number);
    println!("Map ID: {}", header.metadata.map_id);
    println!("Game ID: {}", header.metadata.game_id);
    println!("Unique ID: {}", header.metadata.unique_id);
    if header.metadata.hopper_id != -1 { println!("Hopper ID: {}", header.metadata.hopper_id); }
    println!("--- Image Data ---");
    println!("JPEG length: {}", camera.jpeg_data_length);
    println!("Camera Position: {}, {}, {}", camera.camera.camera.position.x, camera.camera.camera.position.y, camera.camera.camera.position.z);
    println!("Tick: game {}, film {}", camera.game_tick, camera.film_tick);
    println!();

    let output_path = output_path.unwrap_or(
        Path::new(&screenshot_path)
            .parent().unwrap()
            .join(format!("{}.jpg", header.metadata.unique_id))
            .to_str().unwrap()
            .to_string()
    );

    let unpacked = fs::write(&output_path, data.jpeg_data);

    if unpacked.is_err() {
        eprintln!("Could not unpack screenshot.");
        std::process::exit(1);
    }
    else {
        println!("Image saved to {}", output_path);
        std::process::exit(0);
    }
}