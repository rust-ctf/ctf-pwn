use std::fmt;
use std::fmt::Write;
use crossterm::{Command, csi};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Home;
impl Command for Home {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("1~"))
    }
    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Insert;
impl Command for Insert {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("2~"))
    }
    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delete;
impl Command for Delete {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("3~"))
    }
    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct End;
impl Command for End {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("4~"))
    }
    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgUp;
impl Command for PgUp {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("5~"))
    }
    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgDn;
impl Command for PgDn {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("6~"))
    }
    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F(u8);
impl Command for F {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let index = match self.0 {
            0 => "10", // F0
            1 => "11", // F1
            2 => "12", // F2
            3 => "13", // F3
            4 => "14", // F4
            5 => "15", // F5
            6 => "17", // F6
            7 => "18", // F7
            8 => "19", // F8
            9 => "20", // F9
            10 => "21", // F10
            11 => "23", // F11
            12 => "24", // F12
            13 => "25", // F13
            14 => "26", // F14
            15 => "28", // F15
            16 => "29", // F16
            17 => "31", // F17
            18 => "32", // F18
            19 => "33", // F19
            20 => "34", // F20
            _ => panic!("Unsupported F key")
        };
        write!(f, csi!("{}~"), index)
    }
    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        Ok(())
    }
}