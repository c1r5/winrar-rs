use std::io;

pub struct ArchiveError;

impl ArchiveError {
    pub fn unsupported(message: &str) -> io::Error {
        io::Error::new(io::ErrorKind::Unsupported, message)
    }

    pub fn unexpected() -> io::Error {
        io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected Errro")
    }
}