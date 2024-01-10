use bitflags::bitflags;

bitflags! {
    pub struct SectionFlags: u64 {
        const WRITE = 0x1;
        const ALLOC = 0x2;
        const EXECINSTR = 0x4;
        const MERGE = 0x10;
        const STRINGS = 0x20;
        const INFO_LINK = 0x40;
        const LINK_ORDER = 0x80;
        const OS_NONCONFORMING = 0x100;
        const GROUP = 0x200;
        const TLS = 0x400;
        const COMPRESSED = 0x800;
        const MASKOS = 0x0ff00000;
        const MASKPROC = 0xf0000000;
        // Additional processor-specific flags can be added here
    }
}
