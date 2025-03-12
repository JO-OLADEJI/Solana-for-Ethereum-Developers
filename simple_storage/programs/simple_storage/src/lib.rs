use anchor_lang::prelude::*;

declare_id!("Dee7xyY3WfLfhFQQ1wYuJGGeY13wFwnAymVk9UvYw2d4");

#[program]
pub mod simple_storage {
    use super::*;

    pub fn set(ctx: Context<SetCalldata>, value: u64) -> Result<()> {
        let x_account = &mut ctx.accounts.x_account;
        x_account.value = value;

        return Ok(());
    }
}

#[derive(Accounts)]
pub struct SetCalldata<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + XState::INIT_SPACE,
        seeds = [b"x-account".as_ref()],
        bump
    )]
    pub x_account: Account<'info, XState>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct XState {
    value: u64,
}
