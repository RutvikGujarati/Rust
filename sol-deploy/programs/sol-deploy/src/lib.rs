use anchor_lang::prelude::*;

declare_id!("BNLCU6V4FKkSzHt8TS3sUQGuo9auSfWTYoWN8cW27bJ");

#[program]
pub mod sol_deploy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
