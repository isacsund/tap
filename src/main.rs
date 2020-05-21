use std::{fs, io};
mod app;

fn run() {
    let matches = app::app().get_matches();

    if matches.is_present("extract") {
        let fname = matches.value_of("extract").unwrap();
        let fname = std::path::Path::new(fname);
        let file = fs::File::open(&fname).unwrap();

        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = file.sanitized_name();

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {} comment: {}", i, comment);
                }
            }

            println!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}
fn main() {
    run();
}
