use iced_x86::code_asm::CodeAssembler;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
pub enum Instruction
{
    NoOperand(Ident),
}

impl Parse for Instruction {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        Ok(Instruction::NoOperand(name))
    }
}


impl Instruction
{
    pub fn assemble(&self, assembler: &mut CodeAssembler)
    {
        match self {
            Instruction::NoOperand(ident) => assemble_nooperand(assembler, &format!("{ident}").to_lowercase()),
        }
    }


}

fn assemble_nooperand(assembler: &mut CodeAssembler, instr: &str)
{
    match instr
    {
        "nop" => assembler.nop(),
        _ => todo!()
    }.unwrap()
}