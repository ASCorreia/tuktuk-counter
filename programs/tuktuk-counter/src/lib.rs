use anchor_lang::prelude::*;

declare_id!("7j34iqa2ZFMS79QXPuZ3FieLoE6WQUDJ8wJqZr4X86UF");

mod state;
mod instructions;
pub use instructions::*;

#[program]
pub mod tuktuk_counter {
    use super::*;

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment_counter()
    }

    pub fn schedule(ctx: Context<Schedule>, task_id: u16) -> Result<()> {
        ctx.accounts.schedule(task_id, ctx.bumps)
    }
}
