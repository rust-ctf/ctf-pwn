use std::str::FromStr;

use ctf_pwn_macro::asm_intel;


fn main()
{
    let result = asm_intel!(32,
        nop
        nop
        nop
    );

    println!("{:?}", result);
}