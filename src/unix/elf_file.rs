use crate::unix::error::ElfError;
use crate::unix::symbol::Symbol;
use elf::endian::AnyEndian;
use elf::file::FileHeader;
use elf::ElfBytes;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs;
use crate::unix::{parse_got_from_elf, parse_plt_from_elf};

pub struct Elf {
    data: Vec<u8>,
    header: FileHeader<AnyEndian>,
    symbols: Vec<Symbol>,
    dynamic_symbols: Vec<Symbol>,
    got: HashMap<String, u64>,
    plt: HashMap<String, u64>,
}

impl Elf {
    pub async fn parse<T: ?Sized + AsRef<OsStr>>(path: &T) -> Result<Elf, ElfError> {
        let path = PathBuf::from(path);
        let file_data = fs::read(path).await?;
        let file = ElfBytes::<AnyEndian>::minimal_parse(file_data.as_slice())?;
        let symbols = Symbol::parse_symbol_table(&file)?;
        let dynamic_symbols = Symbol::parse_dynamic_symbol_table(&file)?;
        let got = parse_got_from_elf(&file, &dynamic_symbols)?;
        let plt = parse_plt_from_elf(&file, &dynamic_symbols)?;
        let header = file.ehdr;
        Ok(Elf {
            data: file_data,
            header,
            symbols,
            dynamic_symbols,
            got,
            plt,
        })
    }

    pub fn plt(&self) -> &HashMap<String, u64>
    {
        &self.plt
    }

    pub fn got(&self) -> &HashMap<String, u64>
    {
        &self.got
    }

    pub fn symbols(&self) -> &[Symbol]
    {
        &self.symbols
    }

    pub fn dynamic_symbols(&self) -> &[Symbol]
    {
        &self.dynamic_symbols
    }

    pub fn header(&self) -> &FileHeader<AnyEndian>
    {
        &self.header
    }

    pub fn data(&self) -> &[u8]
    {
        &self.data
    }
}
