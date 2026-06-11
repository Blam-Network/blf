use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use blf_lib::io::bitstream::{c_bitstream_writer, e_bitstream_byte_order};
use blf_lib::result::BLFLibResult;

const INITIAL_MGLO_BUFFER_SIZE: usize = 4 * 1024 * 1024;

pub fn encode_megalo_bitstream(encode: impl FnOnce(&mut c_bitstream_writer) -> BLFLibResult) -> BLFLibResult<Vec<u8>> {
    let mut bitstream = c_bitstream_writer::new(
        INITIAL_MGLO_BUFFER_SIZE,
        e_bitstream_byte_order::_bitstream_byte_order_big_endian,
    );
    bitstream.begin_writing();
    encode(&mut bitstream)?;
    bitstream.finish_writing();
    bitstream.get_data()
}

pub fn build_megalo_from_folder(
    json_input_folder: &str,
    mglo_output_folder: &str,
    encode_json_file: impl Fn(&Path) -> Result<Vec<u8>, Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(mglo_output_folder)?;

    let mut converted = 0usize;
    let mut failures = Vec::new();

    for entry in fs::read_dir(json_input_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("invalid json filename: {}", path.display()))?;

        let output_path = PathBuf::from(mglo_output_folder).join(format!("{stem}.mglo"));

        match encode_json_file(&path) {
            Ok(data) => {
                fs::write(&output_path, data)?;
                converted += 1;
                println!("built {stem}.mglo");
            }
            Err(err) => {
                eprintln!("FAILED {stem}.json: {err}");
                failures.push((stem.to_string(), err));
            }
        }
    }

    println!("built {converted} mglo files");
    if !failures.is_empty() {
        let failure_count = failures.len();
        eprintln!("{failure_count} failures:");
        for (name, err) in failures {
            eprintln!("  {name}: {err}");
        }
        return Err(format!("{failure_count} megalo build failures").into());
    }

    Ok(())
}
