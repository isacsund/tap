use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};

use anyhow::{anyhow, Result};

use crate::logger::Logger;

const ABOUT: &str = "
tap description

Use -h for short descriptions and --help for more details.

Project home page: https://github.com/isacsund/tap
";

const USAGE: &str = "
    tap [OPTIONS] FILE
";

const TEMPLATE: &str = "\
{bin} {version}
{author}
{about}

USAGE:{usage}

ARGS:
{positionals}

OPTIONS:
{unified}
";

pub fn new() -> App<'static, 'static> {
    App::new("tap")
        .author(crate_authors!())
        .version(crate_version!())
        .about(ABOUT)
        .usage(USAGE)
        .template(TEMPLATE)
        .help_message("Prints help information. Use --help for more details.")
        .arg(Arg::with_name("debug").short("d"))
}

pub fn parse_and_process<'a, 'b, F>(app: App<'a, 'b>, handler: F) -> !
where
    F: FnOnce(ArgMatches<'a>) -> Result<()>,
{
    let result = match app.get_matches_safe() {
        Ok(args) => handler(args),
        Err(error) => Err(anyhow::Error::from(error)),
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(error) => {
            eprintln!("{:?}", error);
            std::process::exit(1)
        }
    }
}

pub fn parse_and_process_subcommand<'a, 'b, F>(app: App<'a, 'b>, handler: F) -> !
where
    F: FnOnce(&str, &ArgMatches<'a>) -> Result<()>,
{
    parse_and_process(app.setting(AppSettings::SubcommandRequired), |args| {
        if let Err(error) = Logger::init() {
            return Err(anyhow!("Failed to initialize logger: {}", error));
        }
        if args.is_present("debug") {
            log::set_max_level(log::LevelFilter::Debug);
        } else {
            log::set_max_level(log::LevelFilter::Warn);
        }
        if let (name, Some(args)) = args.subcommand() {
            handler(name, args)
        } else {
            // This should not happen since subcommands are required
            Err(anyhow!("Unable to determine the subcommand"))
        }
    })
}
