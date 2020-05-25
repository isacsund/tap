extern crate clap;

extern crate tap;

mod extract;

use tap::app;

fn main() {
    let app = app::new().subcommand(extract::command());

    app::parse_and_process_subcommand(app, |name, args| match name {
        extract::COMMAND_NAME => extract::handler(args),
        _ => Err("Unknown subcommand".into()),
    })
}
