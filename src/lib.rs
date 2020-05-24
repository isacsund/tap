use clap::ArgMatches;
use log;
use std::{fs, io, str};

pub fn extract(matches: &ArgMatches) {
    let fname = matches.value_of("archive").unwrap();
    let format = ArchiveFormat::get_format_from_archive(fname).unwrap();

    match format {
        ArchiveFormat::Rar => {
            let archive = rar::Archive::extract_all(fname, ".", "").unwrap();
            for i in 0..archive.files.len() {
                log::info!("File {} extracted.", archive.files[i].name);
            }
        }
        ArchiveFormat::Zip => {
            let fname = std::path::Path::new(fname);
            let file = fs::File::open(&fname).unwrap();
            let mut archive = zip::ZipArchive::new(file).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let outpath = file.sanitized_name();

                {
                    let comment = file.comment();
                    if !comment.is_empty() {
                        println!("File {} comment: {}", i, comment);
                    }
                }

                log::info!(
                    "File {} extracted to \"{}\"",
                    i,
                    outpath.as_path().display()
                );

                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }
}
enum ArchiveFormat {
    Zip,
    Rar,
}

impl ArchiveFormat {
    fn get_format_from_archive(archive_path: &str) -> Result<ArchiveFormat, &'static str> {
        let archive_path = archive_path.to_lowercase();
        if archive_path.ends_with(".zip") {
            Ok(ArchiveFormat::Zip)
        } else if archive_path.ends_with(".rar") {
            Ok(ArchiveFormat::Rar)
        } else {
            Err("Unknown format!")
        }
    }
}
