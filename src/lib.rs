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
        let filename = "teste.zip";
        let mut reader = Triage::reader(filename)?;

        let info = reader.info()?;

        let password: Option<&[u8]> = if info.has_password {
            Some(b"PASSOWRD_HERE")
        } else {
            None
        };

        let resolved_pass: &[u8] = password.unwrap_or(b"nopass");

        let _files = reader.enumeratefiles(resolved_pass)?;

        let buffer = reader.read_by_index(10, resolved_pass)?;
        let readable = String::from_utf8_lossy(&buffer);

        println!("{}", readable);

        Ok(())
    }
}
