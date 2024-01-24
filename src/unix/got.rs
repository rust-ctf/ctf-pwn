use crate::unix::error::ElfError;
use crate::unix::symbol::Symbol;
use elf::endian::AnyEndian;
use elf::relocation::{RelIterator, RelaIterator};
use elf::{abi, ElfBytes};
use std::collections::HashMap;


pub fn parse_got_from_elf(
    file: &ElfBytes<AnyEndian>,
    dynamic_symbols: &[Symbol],
) -> Result<HashMap<String, u64>, ElfError> {
    let mut result = HashMap::new();
    let section_table = match file.section_headers_with_strtab()? {
        (Some(section_table), _) => section_table,
        _ => {
            return Ok(result);
        }
    };

    for section in section_table {
        match section.sh_type {
            abi::SHT_REL => {
                let iter = file.section_data_as_rels(&section)?;
                parse_rel_iterator(iter, &dynamic_symbols, &mut result);
            }
            abi::SHT_RELA => {
                let iter = file.section_data_as_relas(&section)?;
                parse_rela_iterator(iter, &dynamic_symbols, &mut result);
            }
            _ => {}
        }
    }

    Ok(result)
}

fn parse_rela_iterator(
    iter: RelaIterator<AnyEndian>,
    dynamic_symbols: &[Symbol],
    buf: &mut HashMap<String, u64>,
) {
    parse_got(iter.map(|r| (r.r_sym, r.r_offset)), dynamic_symbols, buf)
}

fn parse_rel_iterator(
    iter: RelIterator<AnyEndian>,
    dynamic_symbols: &[Symbol],
    buf: &mut HashMap<String, u64>,
) {
    parse_got(iter.map(|r| (r.r_sym, r.r_offset)), dynamic_symbols, buf)
}

fn parse_got(
    iter: impl Iterator<Item = (u32, u64)>,
    dynamic_symbols: &[Symbol],
    buf: &mut HashMap<String, u64>,
) {
    for rel in iter.filter(|r| r.0 != 0 && r.1 != 0) {
        match dynamic_symbols.get(rel.0 as usize) {
            Some(sym) if !sym.name.is_empty() => buf.insert(sym.name.clone(), rel.1),
            _ => continue,
        };
    }
}
