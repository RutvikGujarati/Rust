use anchor_lang::prelude::*;

declare_id!("6jLeF3nRzYtigwpaaRMM6jpvCLyMxphvonQX51q2QvXo");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
