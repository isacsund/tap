use std::str;

pub enum ArchiveFormat {
    Zip,
}

impl ArchiveFormat {
    pub fn get_format(archive: &str) -> Result<ArchiveFormat, &'static str> {
        let archive = archive.to_lowercase();
        if archive.ends_with(".zip") {
            Ok(ArchiveFormat::Zip)
        } else {
            Err("Unkown format!")
        }
    }
}
