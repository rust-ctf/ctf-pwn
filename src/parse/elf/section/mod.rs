mod section_flags;
mod section_type;

use crate::parse::*;
use ::elf::section::SectionHeader;
use ::elf::string_table::StringTable;
use section_flags::SectionFlags;
pub use section_type::*;

pub struct Section {
    pub name: String,
    pub sh_type: SectionType,
    pub sh_flags: SectionFlags,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl Section {
    pub fn parse_sections(meta: &crate::parse::RawElf) -> Result<Vec<Self>, BinParseError> {
        let (shdrs, strtab) = match meta.section_headers_with_strtab()? {
            (Some(shdrs), Some(strtab)) => (shdrs, strtab),
            _ => return Ok(Vec::new()),
        };

        let mut results = Vec::new();
        for shdr in shdrs {
            results.push(Self::from(&shdr, &strtab)?)
        }

        Ok(results)
    }

    fn from(hdr: &SectionHeader, strtab: &StringTable) -> Result<Self, BinParseError> {
        let name = strtab.get(hdr.sh_name as usize)?.to_string();
        let sh_type = SectionType::from(hdr.sh_type);
        let sh_flags = SectionFlags::from_bits(hdr.sh_flags)
            .ok_or(BinParseError::ElfSectionFlagsParseError(hdr.sh_flags))?;

        Ok(Self {
            name,
            sh_type,
            sh_flags,
            sh_addr: hdr.sh_addr,
            sh_offset: hdr.sh_offset,
            sh_size: hdr.sh_size,
            sh_link: hdr.sh_link,
            sh_info: hdr.sh_info,
            sh_addralign: hdr.sh_addralign,
            sh_entsize: hdr.sh_entsize,
        })
    }
}
