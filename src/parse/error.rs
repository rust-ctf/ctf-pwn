use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinParseError {
    #[error("io error {0}")]
    IOError(#[from] std::io::Error),
    #[error("elf parse error {0}")]
    ElfParseError(#[from] elf::ParseError),
    #[error("elf section flags parse error: {0}")]
    ElfSectionFlagsParseError(u64),
    #[error("unknown error")]
    Unknown,
}
