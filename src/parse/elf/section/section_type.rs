pub enum SectionType {
    Null,
    ProgBits,
    SymTab,
    StrTab,
    Rela,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Rel,
    ShLib,
    DynSym,
    InitArray,
    FiniArray,
    PreInitArray,
    Group,
    SymTabShndx,
    LoOs,   // Start of OS-specific
    HiOs,   // End of OS-specific
    LoProc, // Start of processor-specific
    HiProc, // End of processor-specific
    GnuAttributes,
    GnuHash,
    GnuLibList,
    Checksum,
    Other(u32), // For unrecognized types
}

impl SectionType {
    pub fn from(sh_type: u32) -> Self {
        match sh_type {
            0 => SectionType::Null,
            1 => SectionType::ProgBits,
            2 => SectionType::SymTab,
            3 => SectionType::StrTab,
            4 => SectionType::Rela,
            5 => SectionType::Hash,
            6 => SectionType::Dynamic,
            7 => SectionType::Note,
            8 => SectionType::NoBits,
            9 => SectionType::Rel,
            10 => SectionType::ShLib,
            11 => SectionType::DynSym,
            14 => SectionType::InitArray,
            15 => SectionType::FiniArray,
            16 => SectionType::PreInitArray,
            17 => SectionType::Group,
            18 => SectionType::SymTabShndx,
            0x60000000 => SectionType::LoOs,
            0x6fffffff => SectionType::HiOs,
            0x70000000 => SectionType::LoProc,
            0x7fffffff => SectionType::HiProc,
            0x6ffffff5 => SectionType::GnuAttributes,
            0x6ffffff6 => SectionType::GnuHash,
            0x6ffffff7 => SectionType::GnuLibList,
            0x6ffffff8 => SectionType::Checksum,
            _ => SectionType::Other(sh_type),
        }
    }
}
