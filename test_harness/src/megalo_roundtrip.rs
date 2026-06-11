use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use binrw::{BinWrite, Endian};
use blf_lib::blf::chunks::{search_for_chunk, BlfChunk};
use blf_lib::blf::s_blf_header;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_matchmaking_game_variant as release_gvar_chunk;
use blf_lib::blf::versions::haloreach_mcc::v_untracked_25_08_16_1352::s_blf_chunk_matchmaking_game_variant as mcc_gvar_chunk;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::game_variant::c_game_engine_custom_variant as alpha_custom_variant;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::game_variant::c_game_engine_custom_variant as delta_custom_variant;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib::result::BLFLibResult;

const DEFAULT_GAMETYPES_ROOT: &str = r"C:\Users\codie\Desktop\gametypes";

fn gametypes_root() -> PathBuf {
    std::env::var("MEGALO_GAMETYPES_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_GAMETYPES_ROOT))
}

fn output_root() -> PathBuf {
    gametypes_root().join("_roundtrip_out")
}

const MGLO_BUFFER_SIZE: usize = 4 * 1024 * 1024;
#[derive(Debug)]
pub struct RoundtripFailure {
    pub path: PathBuf,
    pub message: String,
}

#[derive(Debug)]
pub struct RoundtripReport {
    pub passed: usize,
    pub failures: Vec<RoundtripFailure>,
}

impl RoundtripReport {
    pub fn success(&self) -> bool {
        self.failures.is_empty()
    }
}

pub fn run_all() -> RoundtripReport {
    let mut report = RoundtripReport {
        passed: 0,
        failures: Vec::new(),
    };

    let gametypes_root = gametypes_root();
    let output_root = output_root();

    // On-disk .mglo map gametypes store the custom-variant bitstream directly (no 4-bit game mode prefix).
    run_mglo_suite(
        "alpha",
        gametypes_root.join("alpha/maps/megalo"),
        output_root.join("alpha/maps/megalo"),
        roundtrip_alpha_mglo,
        &mut report,
    );
    run_mglo_suite(
        "beta",
        gametypes_root.join("beta/maps/megalo"),
        output_root.join("beta/maps/megalo"),
        roundtrip_delta_mglo,
        &mut report,
    );
    run_mglo_suite(
        "delta",
        gametypes_root.join("delta/maps/megalo"),
        output_root.join("delta/maps/megalo"),
        roundtrip_delta_mglo,
        &mut report,
    );
    run_blf_suite(
        "release-tu1",
        gametypes_root.join("release"),
        output_root.join("release"),
        roundtrip_release_blf_tu1,
        &mut report,
    );
    run_blf_suite(
        "release-mcc",
        gametypes_root.join("release"),
        output_root.join("release-mcc"),
        roundtrip_release_blf_mcc,
        &mut report,
    );

    report
}

fn run_mglo_suite(
    label: &str,
    input_dir: PathBuf,
    output_dir: PathBuf,
    roundtrip: fn(&[u8]) -> Result<Vec<u8>, String>,
    report: &mut RoundtripReport,
) {
    println!("=== megalo roundtrip: {label} (.mglo) ===");

    let files = match collect_files_with_extension(&input_dir, "mglo") {
        Ok(files) => files,
        Err(err) => {
            report.failures.push(RoundtripFailure {
                path: input_dir.clone(),
                message: format!("failed to enumerate input files: {err}"),
            });
            return;
        }
    };

    if files.is_empty() {
        report.failures.push(RoundtripFailure {
            path: input_dir,
            message: "no .mglo files found".to_string(),
        });
        return;
    }

    for input_path in files {
        match roundtrip_file(&input_path, &output_path_for(&input_path, &input_dir, &output_dir), roundtrip) {
            Ok(()) => {
                report.passed += 1;
                println!("OK {}", input_path.display());
            }
            Err(message) => {
                eprintln!("FAIL {}: {message}", input_path.display());
                report.failures.push(RoundtripFailure {
                    path: input_path,
                    message,
                });
            }
        }
    }
}

fn run_blf_suite(
    label: &str,
    input_dir: PathBuf,
    output_dir: PathBuf,
    roundtrip: fn(&[u8]) -> Result<Vec<u8>, String>,
    report: &mut RoundtripReport,
) {
    println!("=== megalo roundtrip: {label} (.bin) ===");

    let files = match collect_files_with_extension(&input_dir, "bin") {
        Ok(files) => files,
        Err(err) => {
            report.failures.push(RoundtripFailure {
                path: input_dir.clone(),
                message: format!("failed to enumerate input files: {err}"),
            });
            return;
        }
    };

    if files.is_empty() {
        report.failures.push(RoundtripFailure {
            path: input_dir,
            message: "no .bin files found".to_string(),
        });
        return;
    }

    for input_path in files {
        match roundtrip_file(&input_path, &output_path_for(&input_path, &input_dir, &output_dir), roundtrip) {
            Ok(()) => {
                report.passed += 1;
                println!("OK {}", input_path.display());
            }
            Err(message) => {
                eprintln!("FAIL {}: {message}", input_path.display());
                report.failures.push(RoundtripFailure {
                    path: input_path,
                    message,
                });
            }
        }
    }
}

fn roundtrip_file(
    input_path: &Path,
    output_path: &Path,
    roundtrip: fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<(), String> {
    let original = fs::read(input_path).map_err(|err| format!("read failed: {err}"))?;
    let rewritten = roundtrip(&original)?;

    if rewritten != original {
        return Err(format!(
            "bit mismatch (original {} bytes, rewritten {} bytes)",
            original.len(),
            rewritten.len()
        ));
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("create output dir failed: {err}"))?;
    }
    fs::write(output_path, &rewritten).map_err(|err| format!("write failed: {err}"))?;
    Ok(())
}

fn output_path_for(input_path: &Path, input_root: &Path, output_root: &Path) -> PathBuf {
    let relative = input_path
        .strip_prefix(input_root)
        .unwrap_or_else(|_| input_path.file_name().map(Path::new).unwrap_or(input_path));
    output_root.join(relative)
}

fn collect_files_with_extension(dir: &Path, extension: &str) -> Result<Vec<PathBuf>, String> {
    if !dir.is_dir() {
        return Err(format!("input directory does not exist: {}", dir.display()));
    }

    let mut files = Vec::new();
    collect_files_with_extension_recursive(dir, extension, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_with_extension_recursive(
    dir: &Path,
    extension: &str,
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|err| format!("read_dir failed: {err}"))? {
        let entry = entry.map_err(|err| format!("read_dir entry failed: {err}"))?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_with_extension_recursive(&path, extension, files)?;
            continue;
        }

        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
        {
            files.push(path);
        }
    }

    Ok(())
}

fn roundtrip_alpha_mglo(data: &[u8]) -> Result<Vec<u8>, String> {
    roundtrip_mglo(
        data,
        alpha_custom_variant::default,
        |variant, bitstream| variant.decode(bitstream),
        |variant, bitstream| variant.encode(bitstream),
    )
}

fn roundtrip_delta_mglo(data: &[u8]) -> Result<Vec<u8>, String> {
    roundtrip_mglo(
        data,
        delta_custom_variant::default,
        |variant, bitstream| variant.decode(bitstream),
        |variant, bitstream| variant.encode(bitstream),
    )
}

fn roundtrip_mglo<T, F, D, E>(data: &[u8], default: F, decode: D, encode: E) -> Result<Vec<u8>, String>
where
    F: FnOnce() -> T,
    D: Fn(&mut T, &mut c_bitstream_reader) -> BLFLibResult,
    E: Fn(&T, &mut c_bitstream_writer) -> BLFLibResult,
{
    let mut bitstream = c_bitstream_reader::new(data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
    bitstream.begin_reading();

    let mut game_variant = default();
    decode(&mut game_variant, &mut bitstream).map_err(|err| format!("decode failed: {err}"))?;

    let mut writer = c_bitstream_writer::new(
        MGLO_BUFFER_SIZE,
        e_bitstream_byte_order::_bitstream_byte_order_big_endian,
    );
    writer.begin_writing();
    encode(&game_variant, &mut writer).map_err(|err| format!("encode failed: {err}"))?;
    writer.finish_writing();
    writer
        .get_data()
        .map_err(|err| format!("get_data failed: {err}"))
}

fn roundtrip_release_blf_tu1(data: &[u8]) -> Result<Vec<u8>, String> {
    let chunk = search_for_chunk::<release_gvar_chunk>(data.to_vec())
        .map_err(|err| format!("search_for_chunk failed: {err}"))?
        .ok_or_else(|| "gvar chunk not found".to_string())?;
    patch_gvar_chunk_body(data, &chunk, release_gvar_chunk::get_signature(), release_gvar_chunk::get_version())
}

fn roundtrip_release_blf_mcc(data: &[u8]) -> Result<Vec<u8>, String> {
    let chunk = search_for_chunk::<mcc_gvar_chunk>(data.to_vec())
        .map_err(|err| format!("search_for_chunk failed: {err}"))?
        .ok_or_else(|| "gvar chunk not found".to_string())?;
    patch_gvar_chunk_body(data, &chunk, mcc_gvar_chunk::get_signature(), mcc_gvar_chunk::get_version())
}

fn patch_gvar_chunk_body<T>(
    data: &[u8],
    chunk: &T,
    signature: blf_lib_derivable::types::chunk_signature::chunk_signature,
    version: blf_lib_derivable::types::chunk_version::chunk_version,
) -> Result<Vec<u8>, String>
where
    T: BinWrite<Args<'static> = ()>,
{
    let (body_start, body_end) = find_chunk_body_range(data, signature, version)
        .ok_or_else(|| "gvar chunk offset not found".to_string())?;

    let mut new_body = Vec::new();
    chunk
        .write_options(&mut Cursor::new(&mut new_body), Endian::Big, ())
        .map_err(|err| format!("chunk write failed: {err}"))?;

    let original_body_len = body_end - body_start;
    if new_body.len() != original_body_len {
        return Err(format!(
            "gvar body size changed (original {original_body_len}, rewritten {})",
            new_body.len()
        ));
    }

    let mut output = data.to_vec();
    output[body_start..body_end].copy_from_slice(&new_body);
    Ok(output)
}

fn find_chunk_body_range(
    buffer: &[u8],
    signature: blf_lib_derivable::types::chunk_signature::chunk_signature,
    version: blf_lib_derivable::types::chunk_version::chunk_version,
) -> Option<(usize, usize)> {
    if buffer.len() < s_blf_header::size() {
        return None;
    }

    for offset in 0..=(buffer.len() - s_blf_header::size()) {
        let header = s_blf_header::decode(&buffer[offset..offset + s_blf_header::size()]).ok()?;
        if header.signature != signature || header.version != version {
            continue;
        }

        let chunk_end = offset
            .checked_add(header.chunk_size as usize)
            .filter(|end| *end <= buffer.len())?;
        let body_start = offset + s_blf_header::size();
        if body_start > chunk_end {
            return None;
        }

        return Some((body_start, chunk_end));
    }

    None
}
