use clap::{crate_authors, crate_version, App, Arg, SubCommand};

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

pub fn app() -> App<'static, 'static> {
    let app = App::new("tap")
        .author(crate_authors!())
        .version(crate_version!())
        .about(ABOUT)
        .usage(USAGE)
        .template(TEMPLATE)
        .help_message("Prints help information. Use --help for more details.")
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extracts a zip file")
                .arg(Arg::with_name("archive").required(true)),
        );
    app
}
