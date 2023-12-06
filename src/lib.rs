use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

pub mod classes;
pub mod interfaces;

pub struct Triage;

impl Triage {
    pub fn reader<F: Into<PathBuf>>(
        filename: F,
    ) -> Result<impl interfaces::archive::ArchiveReader> {
        let filepath: PathBuf = filename.into();
        let extension = filepath.extension().unwrap();
        let file = File::open(&filepath)?;
        match extension.to_str() {
            Some("zip") => Ok(classes::zip::ArchiveFileZip::new(file)),
            None => Err(classes::error::ArchiveError::unsupported(
                "invalid extension",
            )),
            _ => Err(classes::error::ArchiveError::unexpected()),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::interfaces::archive::ArchiveReader;

    use super::*;
    use std::io::Result;
    #[test]
    fn okay() -> Result<()> {
        let filename = "example.zip";
        let mut reader = Triage::reader(filename)?;
        let files = reader.enumerate_files()?;

        for file in files {
            println!("{:?}", file)
        }

        Ok(())
    }
}
