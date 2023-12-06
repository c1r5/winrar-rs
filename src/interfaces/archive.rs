use std::io::Result;

#[derive(Debug)]
pub struct ArchiveInfo {
    pub length: usize,
    pub comment: Vec<u8>,
    pub has_password: bool,
}

#[derive(Debug)]
pub struct ArchiveFileInfo {
    pub basedir: String,
    pub filename: String,
    pub index: usize,
}

pub trait ArchiveReader {
    fn info(&mut self) -> Result<ArchiveInfo>;
    fn enumeratefiles(&mut self) -> Result<Vec<ArchiveFileInfo>>;
    fn read_by_index(&mut self, index: usize) -> Result<Vec<u8>>;
}
