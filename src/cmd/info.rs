use crate::archive::ArchiveFormat;

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};

pub const COMMAND_NAME: &str = "info";

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name(COMMAND_NAME)
        .about("Display info about the archive")
        .arg(Arg::with_name("archive").required(true))
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
