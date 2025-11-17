use std::path::Path;
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::{s_blf_chunk_map_variant, s_blf_chunk_start_of_file, s_blf_chunk_end_of_file};
use blf_lib::io::write_json_file;

fn main() {
    // Use this as a space to test blf_lib-private while in development :)
    println!("Running test_harness");
    do_your_thing();
}

// FINDINGS:
// - Issue is consistent with blamnet prod, there's a fault in decode, encode or both.
// - Re-reading a blf_lib encoded file provides different values. Inconsistent encode/decode.
// - Position and rotation affected, but only position Y
fn do_your_thing() {
    read_reach_map("/Users/codiestella/Desktop/bnet/reach_map_debug/bungie_backup.bin");
    read_reach_map("/Users/codiestella/Desktop/bnet/reach_map_debug/prod.bin");

    // Re-encode bungies
    let mvar = find_chunk_in_file::<s_blf_chunk_map_variant>("/Users/codiestella/Desktop/bnet/reach_map_debug/bungie_backup.bin").unwrap();
    let mut blffile = BlfFileBuilder::new();
    blffile.add_chunk(s_blf_chunk_start_of_file::default())
        .add_chunk(s_blf_chunk_map_variant::default())
        .add_chunk(s_blf_chunk_end_of_file::default());

    let read = blffile.read_file("/Users/codiestella/Desktop/bnet/reach_map_debug/prod.bin").unwrap();
    read.write_file("/Users/codiestella/Desktop/bnet/reach_map_debug/prod_rebuild.bin");

    read_reach_map("/Users/codiestella/Desktop/bnet/reach_map_debug/prod_rebuild.bin");

}

fn read_reach_map(path: &str) {
    let mvar = find_chunk_in_file::<s_blf_chunk_map_variant>(path).unwrap();
    write_json_file(&mvar, path.replace(".bin", "_fixed.json")).unwrap();
}