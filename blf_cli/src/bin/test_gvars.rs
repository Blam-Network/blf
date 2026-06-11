use blf_lib::blf::chunks::search_for_chunk_in_file;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_matchmaking_game_variant;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let root = env::args().nth(1).expect("usage: test_gvars <hoppers_blf_path>");
    let hopper_re = regex::Regex::new(r"^\d{5}$").unwrap();
    let file_re = regex::Regex::new(r"^.+\.bin$").unwrap();

    for hopper_entry in fs::read_dir(&root).expect("read hoppers dir") {
        let hopper_entry = hopper_entry.unwrap();
        if !hopper_entry.file_type().unwrap().is_dir() {
            continue;
        }
        let hopper_name = hopper_entry.file_name().to_string_lossy().to_string();
        if !hopper_re.is_match(&hopper_name) {
            continue;
        }

        for file_entry in fs::read_dir(hopper_entry.path()).expect("read hopper dir") {
            let file_entry = file_entry.unwrap();
            let file_name = file_entry.file_name().to_string_lossy().to_string();
            if !file_re.is_match(&file_name) {
                continue;
            }

            let path = file_entry.path().to_string_lossy().to_string();
            match search_for_chunk_in_file::<s_blf_chunk_matchmaking_game_variant>(&path) {
                Ok(Some(_)) => {}
                Ok(None) => eprintln!("no chunk: {path}"),
                Err(e) => {
                    eprintln!("FAIL {path}");
                    eprintln!("  {e}");
                    return;
                }
            }
        }
    }

    println!("all game variants parsed OK");
}
