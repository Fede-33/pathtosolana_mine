//Importa la librería Anchor
use anchor_lang::prelude::*;

//Macro delare_id! con el ID del programa
declare_id!("Foj39PNzARZMXefHffK2iPFHdyEntaSVdmi5UgW4ub9M");

//Módulo del programa
#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World from Anchor", ctx.program_id);
        Ok(())
    }
}

//Validación Struct de Cuentas (struct Initialize) 
#[derive(Accounts)]
pub struct Initialize {}
