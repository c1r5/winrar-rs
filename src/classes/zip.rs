use std::fs::File;
use std::path::Path;

use crate::interfaces::archive::{ArchiveFileInfo, ArchiveInfo, ArchiveReader};
use zip::ZipArchive;

#[derive(Debug)]
pub struct ArchiveFileZip {
    archive: ZipArchive<File>,
}

impl ArchiveFileZip {
    pub fn new(file: File) -> Self {
        let archive = ZipArchive::new(file).unwrap();
        Self { archive }
    }
}

impl ArchiveReader for ArchiveFileZip {
    fn info(&mut self) -> std::io::Result<ArchiveInfo> {
        let length = self.archive.len();

        let comment = self.archive.comment().to_vec();

        let has_password = self.archive.by_index_decrypt(0, b"~notexist~")?;

        Ok(ArchiveInfo {
            length,
            comment,
            has_password: has_password.is_err(),
        })
    }

    fn enumeratefiles(&mut self) -> std::io::Result<Vec<ArchiveFileInfo>> {
        let listing = (0..self.archive.len())
            .map(|i| {
                let file = self.archive.by_index(i).unwrap();
                let archive_info = if file.is_file() {
                    let new_path = Path::new(file.name());

                    let Some(base_dir) = new_path.parent() else {
                        return None;
                    };

                    let Some(filename) = new_path.file_name() else {
                        return None;
                    };

                    let info = ArchiveFileInfo {
                        basedir: base_dir.display().to_string(),
                        filename: filename.to_string_lossy().to_string(),
                        index: i,
                    };

                    Some(info)
                } else {
                    None
                };
                archive_info
            })
            .filter_map(|c| if c.is_some() { c } else { None })
            .collect::<Vec<ArchiveFileInfo>>();

        Ok(listing)
    }

    fn read_by_index(&mut self, index: usize) -> std::io::Result<Vec<u8>> {
        todo!()
    }
}
