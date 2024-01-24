use thiserror::Error;

#[derive(Error, Debug)]
pub enum ElfError {
    #[error("io error {0}")]
    IOError(#[from] std::io::Error),
    #[error("elf parse error {0}")]
    ParseError(#[from] elf::ParseError),
    #[error("unknown error")]
    Unknown,
}
