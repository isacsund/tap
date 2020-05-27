use anyhow::{anyhow, Result};
use std::str;

pub enum ArchiveFormat {
    Zip,
}

impl ArchiveFormat {
    pub fn get_format(archive: &str) -> Result<ArchiveFormat> {
        let archive = archive.to_lowercase();
        if archive.ends_with(".zip") {
            Ok(ArchiveFormat::Zip)
        } else {
            Err(anyhow!("Unkown archive format"))
        }
    }
}
