use anchor_lang::prelude::*;

// Define la estructura de la cuenta del contador.
#[account]
#[derive(Default)]
pub struct GreetingAccount {
    pub counter: u32,
}

// Declara el ID del programa.
// Reemplaza esto con el ID real después de desplegar.
declare_id!("Hasw11c3aWnyQNxVwYeLszA1ipGV8ZzrbuewmWcSZUPL");

// El módulo del programa que contendrá la lógica.
#[program]
pub mod contador_solana {
    use super::*;

    // La función que inicializa la cuenta del contador.
    // Esta función se ejecuta una sola vez para crear la cuenta.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.counter = 0;
        Ok(())
    }

    // La función que incrementa el contador.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.counter += 1;
        Ok(())
    }
}

// Struct de contexto para la función initialize.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8, // 8 bytes para el discriminator de Anchor + 8 para u32
        seeds = [b"greeting-account", user.key().as_ref()],
        bump
    )]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Struct de contexto para la función increment.
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub greeting_account: Account<'info, GreetingAccount>,
}
