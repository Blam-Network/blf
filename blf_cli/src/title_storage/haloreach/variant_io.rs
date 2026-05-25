use std::path::Path;

pub fn variant_json_output_name(variant_path: &str, metadata_name: &str) -> String {
    if variant_path.ends_with(".bin") {
        Path::new(variant_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".bin", ".json")
    } else {
        format!("{}.json", metadata_name.to_lowercase())
    }
}
