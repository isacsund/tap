use crate::archive::ArchiveFormat;

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::{fs, io, str};

pub const COMMAND_NAME: &str = "extract";

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name(COMMAND_NAME)
        .about("Extract files contained in an archive.")
        .arg(Arg::with_name("archive").required(true))
}

pub fn handler(args: &ArgMatches) -> Result<()> {
    // This is safe to unwrap since it's required
    let archive = args.value_of("archive").unwrap();
    let format = ArchiveFormat::get_format(archive)?;

    match format {
        ArchiveFormat::Zip => {
            let path = std::path::Path::new(archive);
            let file = fs::File::open(&path).unwrap();
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

                if (&*file.name()).ends_with('/') {
                    fs::create_dir_all(&outpath).unwrap();
                } else {
                    log::debug!(
                        "File {} extracted to \"{}\"",
                        i,
                        outpath.as_path().display()
                    );

                    let mut outfile = fs::File::create(&outpath).unwrap();
                    io::copy(&mut file, &mut outfile).unwrap();
                }
            }
            Ok(())
        }
    }
}
