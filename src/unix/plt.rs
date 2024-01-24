use crate::unix::error::ElfError;
use crate::unix::symbol::Symbol;
use elf::endian::AnyEndian;
use elf::relocation::{RelIterator, RelaIterator};
use elf::{abi, ElfBytes};
use std::collections::HashMap;
use elf::section::SectionHeader;


    fn find_plt_rel_data<'a>(
        file: &'a ElfBytes<AnyEndian>,
    ) -> Result<Option<(&'a [u8], bool)>, ElfError> {
        let (dynamic_table, segment_table) = match (file.dynamic()?, file.segments()) {
            (Some(dynamic_table), Some(segment_table)) => (dynamic_table, segment_table),
            _ => return Ok(None),
        };

        let jmp_rel = match dynamic_table.iter().find(|t| t.d_tag == abi::DT_JMPREL) {
            Some(jmp_rel) => jmp_rel,
            None => return Ok(None),
        };

        let plt_rel = match dynamic_table.iter().find(|t| t.d_tag == abi::DT_PLTREL) {
            Some(plt_rel) => plt_rel,
            None => return Ok(None),
        };

        let plt_rel_sz = match dynamic_table.iter().find(|t| t.d_tag == abi::DT_PLTRELSZ) {
            Some(plt_rel_sz) => plt_rel_sz,
            None => return Ok(None),
        };

        let jmp_rel_addr = jmp_rel.d_val();

        let jmp_rel_header = segment_table
            .iter()
            .filter(|s| s.p_type == abi::PT_LOAD)
            .find(|s| jmp_rel_addr >= s.p_vaddr && (jmp_rel_addr - s.p_vaddr) < s.p_memsz);

        let jmp_rel_header = match jmp_rel_header {
            Some(jmp_rel_header) => jmp_rel_header,
            None => return Ok(None),
        };

        let data = file.segment_data(&jmp_rel_header)?;
        let data = &data[(jmp_rel_addr - jmp_rel_header.p_vaddr) as usize..];
        let data = &data[..plt_rel_sz.d_val() as usize];

        let is_rela = (plt_rel.d_val() as usize) == (abi::DT_RELA as usize);

        Ok(Some((data, is_rela)))
    }

    pub fn parse_plt_from_elf(
        file: &ElfBytes<AnyEndian>,
        dynamic_symbols: &[Symbol],
    ) -> Result<HashMap<String, u64>, ElfError> {
        let mut result = HashMap::new();
        let (rel_data, is_rela) = match find_plt_rel_data(file)? {
            Some((rela_data, is_rela)) => (rela_data, is_rela),
            None => return Ok(result),
        };

        let plt_section = match file.section_header_by_name(".plt")?
        {
            Some(plt_section) => plt_section,
            None => return Ok(result),
        };

        match is_rela {
            false => parse_rel_iterator(
                &plt_section,
                RelIterator::new(file.ehdr.endianness, file.ehdr.class, rel_data),
                &dynamic_symbols,
                &mut result,
            ),
            true => parse_rela_iterator(
                &plt_section,
                RelaIterator::new(file.ehdr.endianness, file.ehdr.class, rel_data),
                &dynamic_symbols,
                &mut result,
            ),
        }

        Ok(result)
    }

    fn parse_rela_iterator(
        plt_section: &SectionHeader,
        iter: RelaIterator<AnyEndian>,
        dynamic_symbols: &[Symbol],
        buf: &mut HashMap<String, u64>,
    ) {
        parse_plt(plt_section, iter.map(|r| r.r_sym) , dynamic_symbols, buf)
    }

    fn parse_rel_iterator(
        plt_section: &SectionHeader,
        iter: RelIterator<AnyEndian>,
        dynamic_symbols: &[Symbol],
        buf: &mut HashMap<String, u64>,
    ) {
        parse_plt(plt_section, iter.map(|r| r.r_sym), dynamic_symbols, buf)
    }

    fn parse_plt(
        plt_section: &SectionHeader,
        iter: impl Iterator<Item = u32>,
        dynamic_symbols: &[Symbol],
        buf: &mut HashMap<String, u64>,
    ) {
        for (i, r_sym) in iter.enumerate() {
            match dynamic_symbols.get(r_sym as usize) {
                Some(sym) =>{
                    buf.insert(sym.name.to_string(), plt_section.sh_addr + (plt_section.sh_entsize * (i + 1) as u64))
                },
                _ => continue,
            };
        }
    }

