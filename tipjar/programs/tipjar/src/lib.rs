use anchor_lang::prelude::*;

declare_id!("2q2R6AF21gcf668Wbt9ymyibnnAYQnSE9tMewknhqTT3");

#[program]
pub mod tipjar {
    use super::*;

    // Initialize PDA to store tip history
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let tips_account = &mut ctx.accounts.tips_account;
        tips_account.total_tips = 0;
        Ok(())
    }

    // Send a tip (SOL) to the creator & log it
    pub fn send_tip(ctx: Context<SendTip>, amount: u64) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.creator.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let tips_account = &mut ctx.accounts.tips_account;
        tips_account.total_tips += 1;

        tips_account.history.push(Tip {
            sender: ctx.accounts.user.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + (100 * (32 + 8 + 8)))] // max 100 tips
    pub tips_account: Account<'info, TipsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendTip<'info> {
    #[account(mut)]
    pub tips_account: Account<'info, TipsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: Creator can be any wallet
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TipsAccount {
    pub total_tips: u64,
    pub history: Vec<Tip>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Tip {
    pub sender: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}
