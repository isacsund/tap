mod app;

use tap::extract;

use std::fs;

pub fn run() {
    let app = app::app();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("file", Some(file_matches)) => {
            let fname = file_matches.value_of("archive").unwrap();
            let fname = std::path::Path::new(fname);
            let file = fs::File::open(&fname).unwrap();
            let archive = zip::ZipArchive::new(file).unwrap();

            let comment = archive.comment();
            if !comment.is_empty() {
                println!("Comment:\n{:?}", comment);
            }
        }
        ("extract", Some(extract_matches)) => {
            extract(extract_matches);
        }
        _ => eprintln!("No command was provided"),
    }
}

fn main() {
    run();
}
