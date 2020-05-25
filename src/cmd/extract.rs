use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;

pub const COMMAND_NAME: &str = "extract";

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name(COMMAND_NAME)
        .about("Extract files contained in an archive.")
        .arg(Arg::with_name("archive").required(true))
}

pub fn handler(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // TODO
    println!("Args: {:?}", args);

    unimplemented!();
}
