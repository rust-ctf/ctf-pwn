extern crate proc_macro;

mod instruction;
mod register;

use std::marker::PhantomData;

use iced_x86::{code_asm::CodeAssembler};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal};
use proc_macro_error::proc_macro_error;
use syn::{parse::{Parse, ParseStream, Result}, parse_macro_input, Token, token::In};
use instruction::Instruction;

struct ATNT;
struct INTEL;


#[derive(Debug)]
struct Asm<T> {
    _phantom: PhantomData<T>,
    bitness: u32,
    instructions: Vec<Instruction<T>>,
}


#[proc_macro_error]
#[proc_macro]
pub fn asm_intel(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Asm<INTEL>);
    let data = parsed.serialize();
    //TODO: Figure out nicer way to convert Vec to const byte array
    let mut result = String::new();

    if parsed.instructions.len() == 0
    {
        unimplemented!()
    }

    result.push('[');
    for by in data
    {
        result.push_str(format!("{}u8,", by).as_str())
    }
    result.push(']');
    result.parse().unwrap()
}

impl<T> Asm<T>
{
    fn serialize(&self) -> Vec<u8>
    {
        let mut a = CodeAssembler::new(64).unwrap();
        for instr in self.instructions.iter()
        {
            instr.assemble(&mut a)
        }
        a.assemble(0).unwrap()
    }
}




impl<T> Parse for Asm<T>
where instruction::Instruction<T>:Parse {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut instructions = Vec::new();
        let bitness: Literal = input.parse()?;
        let _ = input.parse::<Token![,]>()?;
        let bitness = format!("{bitness}").parse().unwrap();
        while !input.is_empty() {
            let tag: Instruction<T> = input.parse()?;
            instructions.push(tag);
        }
        Ok(Asm{bitness, instructions, _phantom: PhantomData})
    }
}