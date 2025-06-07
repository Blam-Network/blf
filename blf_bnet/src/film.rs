use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::chunks::search_for_chunk;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_content_header, s_blf_chunk_end_of_file, s_blf_chunk_game_variant, s_blf_chunk_map_variant, s_blf_chunk_start_of_file};
use num_traits::cast::ToPrimitive;

#[wasm_bindgen(getter_with_clone)]
pub struct FilmVariants {
    pub film_name: String,
    pub map_name: String,
    pub map_author: String,
    pub map_variant: Uint8Array,
    pub game_name: String,
    pub game_author: String,
    pub game_engine: u32,
    pub game_variant: Uint8Array
}

pub fn get_film_variants(data: Vec<u8>) -> Option<FilmVariants> {
    let content_header  = search_for_chunk::<
        v12070_08_09_05_2031_halo3_ship::s_blf_chunk_content_header
    >(data.to_vec()).unwrap_or(None)?;
    let film_header  = search_for_chunk::<
        v12070_08_09_05_2031_halo3_ship::s_blf_chunk_saved_film_header
    >(data.to_vec()).unwrap_or(None)?;

    let map_chunk = s_blf_chunk_map_variant::create(film_header.options.map_variant);
    let game_chunk = s_blf_chunk_game_variant::create(film_header.options.multiplayer_variant);

    let mut map_variant_blf_file = BlfFileBuilder::new();
    map_variant_blf_file.add_chunk(s_blf_chunk_start_of_file::default());
    map_variant_blf_file.add_chunk(s_blf_chunk_content_header::create_for_map_variant(&map_chunk.map_variant));
    map_variant_blf_file.add_chunk(map_chunk.clone());
    map_variant_blf_file.add_chunk(s_blf_chunk_end_of_file::default());

    let mut game_variant_blf_file = BlfFileBuilder::new();
    game_variant_blf_file.add_chunk(s_blf_chunk_start_of_file::default());
    game_variant_blf_file.add_chunk(s_blf_chunk_content_header::create_for_game_variant(&game_chunk.game_variant));
    game_variant_blf_file.add_chunk(game_chunk.clone());
    game_variant_blf_file.add_chunk(s_blf_chunk_end_of_file::default());

    Some(FilmVariants {
        film_name: content_header.metadata.name.get_string(),
        map_name: map_chunk.map_variant.m_metadata.name.get_string(),
        map_author: map_chunk.map_variant.m_metadata.author.get_string(),
        map_variant: Uint8Array::from(map_variant_blf_file.write().ok()?.as_slice()),
        game_name: game_chunk.game_variant.m_base_variant.m_metadata.name.get_string(),
        game_author: game_chunk.game_variant.m_base_variant.m_metadata.author.get_string(),
        game_engine: game_chunk.game_variant.m_game_engine.to_u32()?,
        game_variant: Uint8Array::from(game_variant_blf_file.write().ok()?.as_slice())
    })
}