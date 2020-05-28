extern crate tap;

use tap::{app, cmd};

use anyhow::anyhow;

fn main() {
    let app = app::new()
        .subcommand(cmd::extract::command())
        .subcommand(cmd::info::command());

    app::parse_and_process_subcommand(app, |name, args| match name {
        cmd::extract::COMMAND_NAME => cmd::extract::handler(args),
        cmd::info::COMMAND_NAME => cmd::info::handler(args),
        _ => Err(anyhow!("Unknown subcommand")),
    })
}
