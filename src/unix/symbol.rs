use crate::unix::error::ElfError;
use elf::endian::AnyEndian;
use elf::string_table::StringTable;
use elf::ElfBytes;

pub struct Symbol {
    pub name: String,
    pub shndx: u16,
    pub value: u64,
    pub size: u64,
}

impl Symbol {
    fn parse(value: &elf::symbol::Symbol, string_table: &StringTable) -> Result<Self, ElfError> {
        let name = match value.st_name {
            0 => String::new(),
            idx => string_table.get(idx as usize)?.to_string(),
        };
        Ok(Symbol {
            name,
            shndx: value.st_shndx,
            value: value.st_value,
            size: value.st_size,
        })
    }

    fn parse_table<'a>(
        table: &elf::symbol::SymbolTable<'a, AnyEndian>,
        string_table: &StringTable,
    ) -> Result<Vec<Self>, ElfError> {
        table
            .iter()
            .map(|s| Self::parse(&s, string_table))
            .collect()
    }

    pub(crate) fn parse_symbol_table(file: &ElfBytes<AnyEndian>) -> Result<Vec<Self>, ElfError> {
        match file.symbol_table()? {
            None => Ok(Vec::new()),
            Some((table, string_table)) => Self::parse_table(&table, &string_table),
        }
    }

    pub(crate) fn parse_dynamic_symbol_table(file: &ElfBytes<AnyEndian>) -> Result<Vec<Self>, ElfError> {
        match file.symbol_table()? {
            None => Ok(Vec::new()),
            Some((table, string_table)) => Self::parse_table(&table, &string_table),
        }
    }
}
