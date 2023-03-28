use std::marker::PhantomData;

use iced_x86::code_asm::CodeAssembler;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};

use crate::{INTEL, ATNT};



#[derive(Debug)]
pub enum Instruction<T>
{
    NoOperand(Ident),

    _Phantom(PhantomData<T>)
}


impl Parse for Instruction<INTEL> {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        Ok(Instruction::NoOperand(name))
    }
}

impl Parse for Instruction<ATNT> {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        Ok(Instruction::NoOperand(name))
    }
}




impl<T> Instruction<T>
{
    pub fn assemble(&self, assembler: &mut CodeAssembler)
    {
        match self {
            Instruction::NoOperand(ident) => assemble_nooperand(assembler, &format!("{ident}").to_lowercase()),
            Instruction::_Phantom(_) => unreachable!()
        }
    }
}


fn assemble_nooperand(assembler: &mut CodeAssembler, instr: &str)
{
    match instr
    {
        "nop" => assembler.nop(),
        "ret" => assembler.ret(),
        _ => todo!()
    }.unwrap()
}