use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); //La PubKey que identifica el programa

#[program]
pub mod init { //modulo que define funciones que pueden ser invocadas por transacciones (desde afuera)
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result <()> {
        msg!("Greeting from {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

fn main() {
    println!("{}",initialize(Context<Initialize>{}));
}
