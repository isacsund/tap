use clap::{crate_authors, crate_version, App, AppSettings, ArgMatches};
use std::error::Error;

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
}

pub fn parse_and_process<'a, 'b, F>(app: App<'a, 'b>, handler: F) -> !
where
    F: FnOnce(ArgMatches<'a>) -> Result<(), Box<dyn Error>>,
{
    let result = match app.get_matches_safe() {
        Ok(args) => handler(args),
        Err(error) => match error.kind {
            _ => Err(error.message.into()),
        },
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1)
        }
    }
}

pub fn parse_and_process_subcommand<'a, 'b, F>(app: App<'a, 'b>, handler: F) -> !
where
    F: FnOnce(&str, &ArgMatches<'a>) -> Result<(), Box<dyn Error>>,
{
    parse_and_process(app.setting(AppSettings::SubcommandRequired), |args| {
        if let (name, Some(args)) = args.subcommand() {
            handler(name, args)
        } else {
            Err("Unable to determine the sub-command".into())
        }
    })
}
