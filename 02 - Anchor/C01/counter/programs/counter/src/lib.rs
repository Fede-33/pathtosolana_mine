use anchor_lang::prelude::*;

declare_id!("CKJdBT3qj1iNkYe8x4Lve3BDg2qEMHVnmkCVfybQfy41");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
