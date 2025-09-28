use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer}; // Para CPI de transferencia
use anchor_lang::solana_program::sysvar::rent::Rent; // Para calcular rent exento

declare_id!("H5bMAdeTNTfgbptbkr6ZkkaaQxYPJfXfVRbZXLGKmQGR");

#[program]
pub mod vault {
    use super::*;

    // Inicializa el vault: crea la cuenta de estado y transfiere lamports para rent
    pub fn initialize(ctx: Context<Initialize>, vault_bump: u8, state_bump: u8) -> Result<()> {
        ctx.accounts.initialize(vault_bump, state_bump)
    }

    // Deposita SOL desde el usuario hacia el vault
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    // Retira SOL desde el vault hacia el usuario
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    // Cierra el vault y la cuenta de estado, devolviendo lamports al usuario
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}

// ========================== CUENTAS ==========================

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // Vault: cuenta PDA que solo guarda lamports
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    // Cuenta de estado: almacena bumps
    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

// Deposit: usuario deposita lamports al vault
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

// Withdraw: usuario retira lamports desde el vault
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

// Close: cierra la cuenta de estado y devuelve lamports al usuario
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, close = user)]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

// ========================== STRUCTS ==========================

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

// Espacio de la cuenta: 8 bytes discriminator + 1 + 1
impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

// ========================== MÉTODOS ==========================

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, vault_bump: u8, state_bump: u8) -> Result<()> {
        // Guardamos los bumps en la cuenta de estado
        self.vault_state.vault_bump = vault_bump;
        self.vault_state.state_bump = state_bump;

        // Calculamos balance mínimo para rent exento
        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        // CPI: transferimos lamports desde el usuario al vault
        let cpi_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.user.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );

        system_program::transfer(cpi_ctx, rent_exempt)?;

        Ok(())
    }
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.user.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );

        system_program::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            },
        );

        system_program::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        // El atributo `close = user` en vault_state hace que los lamports de esta cuenta
        // se envíen automáticamente al user cuando se cierra
        // No necesitamos transferir lamports manualmente aquí

        Ok(())
    }
}
