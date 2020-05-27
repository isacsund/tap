extern crate tap;

use tap::{app, cmd};

use anyhow::anyhow;

fn main() {
    let app = app::new().subcommand(cmd::extract::command());

    app::parse_and_process_subcommand(app, |name, args| match name {
        cmd::extract::COMMAND_NAME => cmd::extract::handler(args),
        _ => Err(anyhow!("Unknown subcommand")),
    })
}
