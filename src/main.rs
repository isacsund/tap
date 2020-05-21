use clap::{App, Arg};

fn main() {
    App::new("tap")
        .version("0.1.0")
        .about("tap")
        .arg(Arg::with_name("extract").help("Extracts a zip file"))
        .get_matches();
}
