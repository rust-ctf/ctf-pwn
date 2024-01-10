use std::sync::Arc;

pub struct Arch {
    endianess: Endianess,
    class: Class,
    cpu: CPU,
}

pub enum Endianess {
    Little,
    Big,
}

pub enum Class {
    P32,
    P64,
}

pub enum CPU {
    NoSpecific,
    SPARC,
    X86,
    MIPS,
    PowerPC,
    S390,
    ARM,
    SuperH,
    IA64,
    X86_64,
    AArch64,
    RISC_V,
    Unknown,
}

impl Arch {
    pub fn new(endianess: Endianess, class: Class, cpu: CPU) -> Self {
        Arch {
            endianess,
            class,
            cpu,
        }
    }
}
