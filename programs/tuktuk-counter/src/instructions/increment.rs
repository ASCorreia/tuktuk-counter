use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"counter"],
        bump,
        space = Counter::INIT_SPACE,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

impl<'info> Increment<'info> {
    pub fn increment_counter(&mut self) -> Result<()> {
        self.counter.count += 1;
        Ok(())
    }
}