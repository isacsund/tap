mod app;
mod logger;

use crate::logger::Logger;
use tap::extract;

use std::error;
use std::fs;
use std::process;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let app = app::app();

    if let Err(err) = Logger::init() {
        return Err(format!("Failed to initialize logger: {}", err).into());
    }
    log::set_max_level(log::LevelFilter::Info);

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
            Ok(())
        }
        ("extract", Some(extract_matches)) => {
            extract(extract_matches);
            Ok(())
        }
        _ => Err(format!("No command was provided").into()),
    }
}

fn main() {
    let result = run();

    match result {
        Err(error) => {
            eprintln!("An error occured: {}", error);
            process::exit(1);
        }
        Ok(()) => {
            process::exit(0);
        }
    }
}
