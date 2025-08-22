use anchor_lang::prelude::*;

declare_id!("2q2R6AF21gcf668Wbt9ymyibnnAYQnSE9tMewknhqTT3");

#[program]
pub mod tipjar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
