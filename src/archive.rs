use anyhow::{anyhow, Result};
use std::str;

pub enum ArchiveFormat {
    Zip,
    Tar,
}

impl ArchiveFormat {
    pub fn get_format(archive: &str) -> Result<ArchiveFormat> {
        let archive = archive.to_lowercase();
        if archive.ends_with(".zip") {
            Ok(ArchiveFormat::Zip)
        } else if archive.ends_with(".tar") {
            Ok(ArchiveFormat::Tar)
        } else {
            Err(anyhow!("Unkown archive format"))
        }
    }
}
