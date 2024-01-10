mod section;
mod segment;

pub use section::*;
pub use segment::*;

use crate::parse::{Arch, BinParseError, Endian, CPU};
use ::elf;
use ::elf::endian::*;
use ::elf::file::Class;
use std::path::Path;
use tokio::fs;

pub struct Elf {
    arch: Arch,
    segments: Vec<Segment>,
    sections: Vec<Section>,
}

pub type RawElf<'a> = elf::ElfBytes<'a, AnyEndian>;

impl Elf {
    pub async fn parse(path: impl AsRef<Path>) -> Result<Self, BinParseError> {
        let data = fs::read(path).await?;
        let meta = RawElf::minimal_parse(&data)?;

        let arch = parse_arch(&meta);
        let segments = Segment::parse_segments(&meta);
        let sections = Section::parse_sections(&meta)?;

        Ok(Elf {
            arch,
            segments,
            sections,
        })
    }
}

//
// fn parse_got(meta: &RawElf) -> Result<elf::segment::SegmentTable<AnyEndian>, ParseError>
// {
//
//     Ok(())
// }

fn parse_arch(meta: &RawElf) -> Arch {
    let endianess = match meta.ehdr.endianness {
        AnyEndian::Little => Endian::Little,
        AnyEndian::Big => Endian::Big,
    };

    let class = match meta.ehdr.class {
        Class::ELF32 => super::Class::P32,
        Class::ELF64 => super::Class::P64,
    };

    let cpu = match meta.ehdr.e_machine {
        0x00 => CPU::NoSpecific,
        0x02 => CPU::SPARC,
        0x03 => CPU::X86,
        0x08 => CPU::MIPS,
        0x14 => CPU::PowerPC,
        0x16 => CPU::S390,
        0x28 => CPU::ARM,
        0x2A => CPU::SuperH,
        0x32 => CPU::IA64,
        0x3E => CPU::X86_64,
        0xB7 => CPU::AArch64,
        0xF3 => CPU::RISC_V,
        _ => CPU::Unknown,
    };

    Arch::new(endianess, class, cpu)
}
