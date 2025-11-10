# VAULT
Una **bóveda** es una opción de almacenamiento de alta seguridad para activos digitales, que ofrece una protección superior a una billetera estándar. Las Wallets están diseñadas para la conveniencia y transacciones frecuentes, mientras que una vault prioriza la seguridad a largo plazo y la protección sobre la accesibilidad inmediata.

## CASO PRÁCTICO:
Para el siguiente ejemplo, se diseñará una bóveda que tenga cuatro funciones, **inicializar**, **depositar**, **retirar** y **cerrar**. El código incluye los siguientes bloques:

### Imports:
Se utlizarán las siguientes partes de la librería de Anchor:

    use anchor_lang::prelude::*;
    use anchor_lang::system_program::{self, Transfer}; // Para CPI de transferencia
    use anchor_lang::solana_program::sysvar::rent::Rent; // Para calcular rent exento

* **use anchor_lang::prelude::\*:** Importa todos los elementos esenciales de Anchor para el desarrollo de programas Solana (macros, tipos, etc.). 
* **use anchor_lang::system_program::{self, Transfer};** Importa el programa del sistema y la estructura Transfer para realizar llamadas entre programas (CPI) para transferencias de lamports (SOL).
* **use anchor_lang::solana_program::sysvar::rent::Rent;** Importa la Sysvar de Rent de Solana para poder consultar el cálculo del balance mínimo de lamports para estar exento de alquiler (rent-exempt).


### Program_ID:
Macro de Anchor que declara la Program ID única del programa en la red.

    declare_id!("H5bMAdeTNTfgbptbkr6ZkkaaQxYPJfXfVRbZXLGKmQGR");

### Entry point:
Define las instrucciones (funciones) públicas del programa, estableciendo las cuatro operaciones principales que los usuarios pueden realizar, dentro del módulo **vault**, que implementa la lógica central. 

    #[program]
    pub mod vault {
        use super::*;

        pub fn initialize(ctx: Context<Initialize>, vault_bump: u8, state_bump: u8) -> Result<()> {
            ctx.accounts.initialize(vault_bump, state_bump)
        }

        pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
            ctx.accounts.deposit(amount)
        }
        
        pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
            ctx.accounts.withdraw(amount)
        }
        
        pub fn close(ctx: Context<Close>) -> Result<()> {
            ctx.accounts.close()
        }
    }

* **#[program]** Atributo de Rust que marca el siguiente bloque de código como el cuerpo principal de la lógica del programa Anchor.
* **pub mod vault {** Define el módulo público llamado **vault** donde residirán las instrucciones, definidas como funciones (fn).
* **use super::*;** Importa todos los elementos del ámbito superior (prelude, estructuras de cuentas, VaultState, etc.) dentro del módulo del programa. Funciona como herencia.
* **pub fn initialize(ctx: Context<Initialize>, vault_bump: u8, state_bump: u8) -> Result<()> {** Define la instrucción initialize. Recibe el contexto (ctx), el bump para la cuenta vault y el bump para la cuenta vault_state (que se declaran posteriormente). Retorna un Result<()>.
    * **ctx.accounts.initialize(vault_bump, state_bump)** Llama al método initialize implementado posteriormente en la estructura de cuentas **Initialize**, pasando los bumps como parámetros (Generados por Anchor).
* **pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {** Define la instrucción deposit. Recibe el contexto (ctx) y la cantidad de lamports (amount) a depositar.
    * **ctx.accounts.deposit(amount)** Llama al método deposit implementado en la estructura de cuentas **Deposit**, pasando la cantidad de lamports que defina el usuario.
* **pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {** Define la instrucción withdraw. Recibe el contexto (ctx) y la cantidad de lamports (amount) a retirar.
    * **ctx.accounts.withdraw(amount)** Llama al método withdraw implementado en la estructura de cuentas **Withdraw**, pasando la cantidad de lamports que defina el usuario.
* **pub fn close(ctx: Context<Close>) -> Result<()> {** Define la instrucción close. Recibe solo el contexto (ctx).
    * **ctx.accounts.close()** Llama al método close implementado en la estructura de cuentas **Close**.

### Accounts: 
Este bloque define las reglas y validaciones necesarias para cada una de las cuatro operaciones principales del programa. Actúa como un mecanismo de seguridad y tipado para el smart contract, asegurando que:
1. Se proporcionen las cuentas correctas para cada instrucción (por ejemplo, no puedes depositar sin proporcionar la cuenta del vault).
2. Se cumplan las restricciones (por ejemplo, el usuario debe firmar, y la cuenta del vault debe ser una PDA con semillas específicas).
3. Se definan los roles de mutabilidad (mut) para que el programa pueda leer y modificar los balances y datos cuando sea necesario.

**Initialize:** inicializa la cuenta PDA que solo guarda lamports (Vault) y la Cuenta de estado (Vault_state) que almacena bumps. También inicia una instancia del System Program para que administre la creación de cuentas y transferencias.

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(
            mut,
            seeds = [b"vault", vault_state.key().as_ref()],
            bump
        )]
        pub vault: SystemAccount<'info>,

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

* **#[derive(Accounts)]** Macro de Anchor para derivar la lógica de validación y deserialización de las cuentas.
* **pub struct Initialize<'info> {** Define la estructura de cuentas Initialize.
    * **#[account(mut)]** Atributo que indica que la cuenta siguiente es mutable (su estado o balance cambiará).
    * **pub user: Signer<'info>,** Asigna al usuario que inicia la transacción. Debe ser un firmante (Signer) y es mutable, indicado por el trait anterior.
    * **#[account(** Inicio de la macro de validación para la cuenta vault.
        * **mut,** La cuenta es mutable (su balance cambiará al recibir SOL).
        * **seeds = [b"vault", vault_state.key().as_ref()],** Define las semillas (seeds) para generar la PDA del vault: la cadena literal "vault" y la clave de la cuenta **vault_state**.
        * **bump** Indica a Anchor que calcule automáticamente el bump (un byte necesario para la PDA).
    * **pub vault: SystemAccount<'info>,** Define la cuenta vault es una PDA que solo necesita ser inicializada como una SystemAccount (no tiene datos de programa, solo guarda lamports).
    * **#[account(** Inicio de la macro de validación para vault_state.
        * **init,** Indica a Anchor que inicialice esta cuenta.
        * **payer = user,** Indica que el coste de creación de la cuenta (rent) será pagado por la cuenta user.
        * **seeds = [b"state", user.key().as_ref()],** Define las semillas para la PDA de estado: la cadena "state" y la clave del user.
        * **bump,** Calcula el bump automáticamente para esta PDA.
        * **space = VaultState::INIT_SPACE** Define el espacio en bytes que debe tener esta cuenta al inicializarse, tomado de la constante definida posteriormente en la estructura **VaultState**.
    * **pub vault_state: Account<'info, VaultState>,** La cuenta vault_state es una cuenta de Anchor que contiene la estructura VaultState.
    * **pub system_program: Program<'info, System>,** Referencia al Programa del Sistema de Solana, necesario para la creación de cuentas y transferencias.

**Deposit:** define las cuentas que deben ser provistas y validadas para ejecutar la instrucción de depósito.

    #[derive(Accounts)]
    pub struct Deposit<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(mut)]
        pub vault: SystemAccount<'info>,

        pub system_program: Program<'info, System>,
    }

* **#[derive(Accounts)]** Deriva la lógica de validación de cuentas.
* **pub struct Deposit<'info> {** Define la estructura de cuentas Deposit.
    * **#[account(mut)]** La cuenta es mutable (su balance cambiará).
    * **pub user: Signer<'info>,** El usuario que envía el SOL, debe firmar y es mutable.
    * **#[account(mut)]** La cuenta es mutable (su balance cambiará al recibir SOL).
    * **pub vault: SystemAccount<'info>,** La cuenta vault PDA que recibe el SOL, es mutable.
    * **pub system_program: Program<'info, System>,** Referencia al Programa del Sistema, necesario para la transferencia.

**Withdraw:** define las cuentas que deben ser provistas y validadas para ejecutar la instrucción de retiro.

    #[derive(Accounts)]
    pub struct Withdraw<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(mut)]
        pub vault: SystemAccount<'info>,

        pub system_program: Program<'info, System>,
    }

* **#[derive(Accounts)]**	Deriva la lógica de validación de cuentas.
* **pub struct Withdraw<'info> {** Define la estructura de cuentas Withdraw.
75	#[account(mut)]	La cuenta es mutable (su balance cambiará al recibir SOL).
    * **pub user: Signer<'info>,** El usuario que recibe el SOL, debe firmar y es mutable.
    * **#[account(mut)]** La cuenta es mutable (su balance cambiará al enviar SOL).
    * **pub vault: SystemAccount<'info>,** La cuenta vault PDA que envía el SOL, es mutable.
    * **pub system_program: Program<'info, System>,** Referencia al Programa del Sistema, necesario para la transferencia.

**Close:** especifica las cuentas necesarias para cerrar la cuenta de estado del programa y devolver los fondos remanentes al usuario. El aspecto más importante se encuentra en el atributo de la cuenta vault_state, donde el argumento **close = user** le indica a Anchor que, al finalizar la instrucción close, debe cerrar la cuenta **vault_state** (marcar su espacio en el ledger como disponible), y transferir el balance completo de lamports que contenga vault_state al **user**.

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

* **#[derive(Accounts)]** Deriva la lógica de validación de cuentas.
* **pub struct Close<'info> {** Define la estructura de cuentas Close.
    * **#[account(mut)]** El usuario es mutable (recibirá los lamports de vault_state).
    * **pub user: Signer<'info>,** El usuario que inicia el cierre, debe firmar y es mutable.
    * **#[account(mut, close = user)]** Atributo: la cuenta es mutable, y el argumento close = user indica a Anchor que, al finalizar la instrucción, cierre la cuenta y transfiera su balance de lamports al user.
    * **pub vault_state: Account<'info, VaultState>,** La cuenta de estado que se cerrará.
    * **#[account(mut)]** La cuenta es mutable.
    * **pub vault: SystemAccount<'info>,** La cuenta vault PDA. Aunque esta cuenta no se cierra aquí automáticamente, se necesita para que la instrucción esté completa (los lamports del vault se retirarán en una instrucción previa de withdraw o se quedarán para otra instrucción, aunque el comentario sugiere el cierre completo del vault).
    * **pub system_program: Program<'info, System>,** Referencia al Programa del Sistema.

### Structs: 
Este bloque define la estructura de datos que se almacena dentro de la cuenta **vault_state**, para que el programa pueda interactuar correctamente con sus PDAs. Cumple dos funciones principales:

1. Almacena los bumps de las PDA. El último byte necesario para "derivar" la dirección de una PDA y para firmar transacciones en nombre de la PDA.
2. Implementa el trait **Space** de Anchor, para definir el tamaño total de la cuenta en bytes. 8 bytes (identificador único que Anchor añade a cada cuenta para verificar su tipo) + 1 byte (campo vault_bump u8) + 1 byte (campo state_bump u8).

    #[account]
    pub struct VaultState {
        pub vault_bump: u8,
        pub state_bump: u8,
    }

    impl Space for VaultState {
        const INIT_SPACE: usize = 8 + 1 + 1;
    }

* **#[account]** Atributo de Anchor que marca la estructura para ser usada como datos de cuenta, incluyendo la lógica de serialización/deserialización y el discriminador de 8 bytes.
* **pub struct VaultState {** Define la estructura de datos que se guarda en la cuenta vault_state.
    * **pub vault_bump: u8,** Almacena el bump de la cuenta vault (necesario para la firma de la PDA).
    * **pub state_bump: u8,** Almacena el bump de la propia cuenta vault_state.
* **impl Space for VaultState {** Implementa el trait Space de Anchor para definir el tamaño de la cuenta.
    * **const INIT_SPACE: usize = 8 + 1 + 1;** Define una constante para el tamaño total. Son 8 bytes (discriminador) + 1 byte (vault_bump) + 1 byte (state_bump).

### Implementación de métodos:
Este bloque de código contiene la lógica de negocio principal del programa, implementa los métodos asociados a las estructuras de cuentas, realizando las acciones reales (transferencias, almacenamiento de datos, cálculos) de las instrucciones del smart contract. Este bloque es donde se definen y ejecutan los pasos atómicos (como las CPIs) para lograr la funcionalidad de la bóveda.

**Initialize:** Almacena los valores de **bump** necesarios para futuras firmas de PDA en la cuenta de estado. Calcula el mínimo SOL requerido para el **rent-exempt** y transfiere esa cantidad desde el usuario a la cuenta **vault** (la PDA de la bóveda) mediante una Llamada entre Programas (CPI).

    impl<'info> Initialize<'info> {
        pub fn initialize(&mut self, vault_bump: u8, state_bump: u8) -> Result<()> {
            self.vault_state.vault_bump = vault_bump;
            self.vault_state.state_bump = state_bump;

            let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

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

* **impl<'info> Initialize<'info>** {	Inicia el bloque de implementación de métodos para la estructura Initialize.
    * **pub fn initialize(&mut self, vault_bump: u8, state_bump: u8) -> Result<()>** Define el método initialize que ejecuta la lógica de la instrucción. Recibe los bumps.
        * **self.vault_state.vault_bump = vault_bump;** Asigna el bump de vault a la cuenta de estado.
        * **self.vault_state.state_bump = state_bump;** Asigna el bump de vault_state a la cuenta de estado.
        122	let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.* **ccount_info().data_len());** Consulta la Sysvar de Rent (Rent::get()?) para calcular la cantidad mínima de lamports (minimum_balance) que debe tener la cuenta vault para estar exenta de alquiler. Se usa el tamaño de datos actual de la cuenta vault (que es 0 para una SystemAccount).
        * **let cpi_ctx = CpiContext::new(** Crea un nuevo contexto CPI (Llamada entre Programas)
        * **self.system_program.to_account_info(),** El programa al que se llama: el Programa del Sistema.
        * **Transfer {** Define los datos de la instrucción que se llama (una Transfer del Programa del Sistema).
        * **from: self.user.to_account_info(),** La cuenta de origen de la transferencia: el user.
        * **to: self.vault.to_account_info(),** La cuenta de destino de la transferencia: el vault.
        * **system_program::transfer(cpi_ctx, rent_exempt)?;** Ejecuta la CPI de transferencia, enviando el monto rent_exempt desde el user al vault (lo cual es necesario para que el vault exista como una cuenta rent-exempt).
        * **Ok(())** Retorna éxito.

**Deposit:** Ingresa SOL a la bóveda. Realiza una CPI al **system-program** para transferir la cantidad especificada (amount) de SOL desde la cuenta del user (firmante) a la cuenta **vault** (la PDA).

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

* **impl<'info> Deposit<'info> {** Inicia el bloque de implementación para Deposit.
    * **pub fn deposit(&mut self, amount: u64) -> Result<()> {** Define el método deposit que recibe la cantidad (amount) a depositar.
        * **let cpi_ctx = CpiContext::new(** Crea el contexto CPI para la transferencia.
        * **self.system_program.to_account_info(),** Programa del Sistema.
        * **Transfer {** Datos de la instrucción Transfer.
        * **from: self.user.to_account_info(),** Origen: el user.
        * **to: self.vault.to_account_info(),** Destino: el vault.
        * **system_program::transfer(cpi_ctx, amount)?;** Ejecuta la CPI de transferencia, enviando el amount de SOL/lamports desde el user al vault.
        * **Ok(())** Retorna éxito.


**Withdraw:** Retira SOL de la bóveda. Realiza una CPI para transferir la cantidad especificada (amount) de SOL desde la cuenta **vault** a la cuenta del user. Esta transferencia se realiza con la firma de la PDA (vault), lo cual es posible gracias a los bumps almacenados.

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

* **impl<'info> Withdraw<'info> {**Inicia el bloque de implementación para Withdraw.
    * **pub fn withdraw(&mut self, amount: u64) -> Result<()> {**Define el método withdraw que recibe la cantidad (amount) a retirar.
    * **let cpi_ctx = CpiContext::new(**Crea el contexto CPI para la transferencia.
    * **self.system_program.to_account_info(),**Programa del Sistema.
    * **Transfer {**Datos de la instrucción Transfer.
    * **from: self.vault.to_account_info(),**Origen: el vault (la PDA, que actuará como firmante a través de su bump).
    * **to: self.user.to_account_info(),**Destino: el user.
    * **system_program::transfer(cpi_ctx, amount)?;**Ejecuta la CPI de transferencia, enviando el amount de SOL/lamports desde el vault (PDA) al user.
    * **Ok(())**Retorna éxito.

**Close:** Finaliza la cuenta de estado. La función no contiene código de transferencia explícito. Simplemente regresa Ok(()), ya que el cierre de la cuenta **vault_state** y la devolución de sus lamports al usuario son manejados automáticamente por Anchor debido al atributo **close = user** definido en la estructura de cuentas **Close**.

    impl<'info> Close<'info> {
        pub fn close(&mut self) -> Result<()> {
            Ok(())
        }
    }

* **impl<'info> Close<'info> {** Inicia el bloque de implementación para Close.
    * **pub fn close(&mut self) -> Result<()> {** Define el método close. El atributo **close = user** en vault_state hace que los lamports de esta cuenta se envíen automáticamente al user cuando se cierra. No necesitamos transferir lamports manualmente aquí. La lógica de cierre y devolución de lamports para vault_state es manejada por el atributo #[account(mut, close = user)].
        * **Ok(())** Retorna éxito.