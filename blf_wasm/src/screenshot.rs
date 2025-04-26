use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use blf_lib::blam::common::math::real_math::real_point3d;
use blf_lib::blf::chunks::search_for_chunk;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_saved_camera;
use blf_lib::blf::versions::halo3odst::v13895_09_04_27_2201_atlas_release;

#[wasm_bindgen(getter_with_clone)]
pub struct ScreenshotData {
    pub build_number: u16,
    pub name: String,
    pub author: String,
    pub author_xuid: u64,
    pub description: String,
    pub unique_id: u64,
    pub game_id: u64,
    pub hopper_id: Option<i16>,
    pub camera_position: real_point3d,
    pub game_tick: u32,
    pub film_tick: u32,
    pub jpeg_data: Uint8Array,
}

fn get_h3_release_screenshot_data(data: Vec<u8>) -> Option<ScreenshotData> {
    let content_header  = search_for_chunk::<
        v12070_08_09_05_2031_halo3_ship::s_blf_chunk_content_header
    >(data.to_vec())?;
    let screenshot_camera  = search_for_chunk::<
        v12070_08_09_05_2031_halo3_ship::s_blf_chunk_screenshot_camera
    >(data.to_vec())?;
    let screenshot_data  = search_for_chunk::<
        v12070_08_09_05_2031_halo3_ship::s_blf_chunk_screenshot_data
    >(data.to_vec())?;

    Some(ScreenshotData {
        build_number: content_header.build_number,
        name: content_header.metadata.name.get_string(),
        author: content_header.metadata.author.get_string(),
        author_xuid: content_header.metadata.author_id,
        description: content_header.metadata.description.get_string(),
        unique_id: content_header.metadata.unique_id,
        game_id: content_header.metadata.game_id,
        hopper_id:
            if content_header.metadata.hopper_id != -1 {
                Some(content_header.metadata.hopper_id)
            } else {
                None
            },
        camera_position: screenshot_camera.camera.camera.position,
        game_tick: screenshot_camera.game_tick,
        film_tick: screenshot_camera.film_tick,
        jpeg_data: Uint8Array::from(screenshot_data.jpeg_data.as_slice()),
    })
}

fn get_h3odst_release_screenshot_data(data: Vec<u8>) -> Option<ScreenshotData> {
    let content_header  = search_for_chunk::<
        v13895_09_04_27_2201_atlas_release::s_blf_chunk_content_header
    >(data.to_vec())?;
    let screenshot_camera  = search_for_chunk::<
        v13895_09_04_27_2201_atlas_release::s_blf_chunk_screenshot_camera
    >(data.to_vec())?;
    let screenshot_data  = search_for_chunk::<
        v13895_09_04_27_2201_atlas_release::s_blf_chunk_screenshot_data
    >(data.to_vec())?;

    Some(ScreenshotData {
        build_number: content_header.build_number,
        name: content_header.metadata.name.get_string(),
        author: content_header.metadata.author.get_string(),
        author_xuid: content_header.metadata.author_id,
        description: content_header.metadata.description.get_string(),
        unique_id: content_header.metadata.unique_id,
        game_id: content_header.metadata.game_id,
        hopper_id: None,
        camera_position: screenshot_camera.camera.camera.position,
        game_tick: screenshot_camera.game_tick,
        film_tick: screenshot_camera.film_tick,
        jpeg_data: Uint8Array::from(screenshot_data.jpeg_data.as_slice()),
    })
}

pub fn get_screenshot_data(data: Vec<u8>) -> Option<ScreenshotData> {
    get_h3_release_screenshot_data(data.clone())
        .or_else(|| get_h3odst_release_screenshot_data(data))
}