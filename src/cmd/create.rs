use crate::archive::ArchiveFormat;

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};

pub const COMMAND_NAME: &str = "create";

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name(COMMAND_NAME)
        .about("Create an archive with the specified files and format")
        .arg(
            Arg::with_name("archive")
                .help("Archive to create")
                .required(true),
        )
        .arg(
            Arg::with_name("files")
                .help("Files to put in the archvie")
                .multiple(true)
                .required(true),
        )
}

pub fn handler(args: &ArgMatches) -> Result<()> {
    // This is safe to unwrap since it's required
    let archive = args.value_of("archive").unwrap();
    let format = ArchiveFormat::get_format(archive)?;

    match format {
        ArchiveFormat::Zip => {
            // TODO
            unimplemented!()
        }
        ArchiveFormat::Tar => {
            // TODO
            unimplemented!()
        }
    }
}
