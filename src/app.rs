use clap::{crate_authors, crate_version, App, Arg};

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
        .arg(
            Arg::with_name("extract")
                .short("e")
                .takes_value(true)
                .help("Extracts the zip archive"),
        );

    app
}
