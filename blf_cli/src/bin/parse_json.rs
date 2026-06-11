use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::c_game_variant;
use blf_lib::io::read_json_file;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| r"C:\Users\codie\Downloads\tmp\1nvasion.json".to_string());
    match read_json_file::<c_game_variant>(&path) {
        Ok(_) => println!("OK"),
        Err(e) => eprintln!("{}", e),
    }
}
