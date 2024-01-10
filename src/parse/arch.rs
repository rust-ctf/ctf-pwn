pub struct Arch {
    endian: Endian,
    class: Class,
    cpu: CPU,
}

pub enum Endian {
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
    pub fn new(endian: Endian, class: Class, cpu: CPU) -> Self {
        Arch {
            endian,
            class,
            cpu,
        }
    }
}
