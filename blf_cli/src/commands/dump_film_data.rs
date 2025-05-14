use blf_lib::blf::chunks::search_for_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_content_header, s_blf_chunk_saved_film_header};
use blf_lib::io::write_json_file;
use crate::build_path;
use crate::console::console_task;
use crate::io::get_files_in_folder;

pub fn dump_film_data(
    films_folder: String,
) {
    let mut task = console_task::start("Reading film info");

    let film_file_names = get_files_in_folder(&films_folder).unwrap_or_else(|err|{
        task.fail_with_error(err);
        panic!()
    });

    film_file_names.iter().for_each(|file_name| {
        let file_path = build_path!(&films_folder, file_name);
        let chdr = search_for_chunk_in_file::<s_blf_chunk_content_header>(
            &file_path,
        );
        let flmh = search_for_chunk_in_file::<s_blf_chunk_saved_film_header>(
            &file_path,
        );

        if flmh.is_some() && chdr.is_some() {
            let flmh = flmh.unwrap();
            let chdr = chdr.unwrap();
            write_json_file(&flmh, format!("{file_path}.json")).unwrap();
            task.add_message(format!("{} - {}", file_name, chdr.metadata.name.get_string()));
        }
    });

    task.complete();
}